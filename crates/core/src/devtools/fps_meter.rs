//! FPS 实时测量 — 订阅 [`iced::window::frames`] 逐帧采样，统计当前/平均/最差帧耗
//!
//! 用途：把"60 FPS"的性能承诺变成现场可见的数字（showcase / table_demo overlay）。
//!
//! ```no_run
//! # use har_ui_core::devtools::fps_meter::FpsMeter;
//! # use iced::{Element, Task};
//! # struct State { fps: FpsMeter }
//! # #[derive(Debug, Clone)] enum Message { Frame(std::time::Instant) }
//! # fn view(state: &State) -> Element<'_, Message> { iced::widget::text("").into() }
//! # fn update(state: &mut State, msg: Message) -> Task<Message> {
//! #     Task::none()
//! # }
//! # fn main() -> iced::Result {
//! iced::application(
//!     || {
//!         let state = State { fps: FpsMeter::new() };
//!         (state, Task::none())
//!     },
//!     update,
//!     view,
//! )
//! .title(|_state: &State| String::from("Demo"))
//! .subscription(|_state| iced::window::frames().map(Message::Frame))
//! .run()
//! # }
//! ```

use std::time::Instant;

use iced::widget::text;
use iced::{Element, Length};

/// 单帧采样环形缓冲容量（默认 120 ≈ 2 秒 @60FPS）
const DEFAULT_CAPACITY: usize = 120;

/// FPS 测量器 — 记录帧间隔并统计均值/最差值
#[derive(Debug, Clone)]
pub struct FpsMeter {
    last: Option<Instant>,
    /// 最近 N 帧的帧耗时（毫秒），环形缓冲
    deltas_ms: Vec<f32>,
    head: usize,
    filled: usize,
    max_ms: f32,
    frames: u64,
}

impl Default for FpsMeter {
    fn default() -> Self {
        Self::new()
    }
}

impl FpsMeter {
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_CAPACITY)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            last: None,
            deltas_ms: vec![0.0; capacity.max(1)],
            head: 0,
            filled: 0,
            max_ms: 0.0,
            frames: 0,
        }
    }

    /// 记录一帧（由 `window::frames()` 订阅的 Instant 驱动）
    ///
    /// 首帧只有时间戳没有 delta；后续帧计算与上一帧的间隔。
    pub fn record(&mut self, now: Instant) {
        if let Some(last) = self.last {
            let dt_ms = now.duration_since(last).as_secs_f32() * 1000.0;
            // 忽略挂起恢复等异常大间隔（> 1s），避免污染统计
            if dt_ms < 1000.0 {
                self.deltas_ms[self.head] = dt_ms;
                self.head = (self.head + 1) % self.deltas_ms.len();
                self.filled = (self.filled + 1).min(self.deltas_ms.len());
                if dt_ms > self.max_ms {
                    self.max_ms = dt_ms;
                }
            }
        }
        self.last = Some(now);
        self.frames += 1;
    }

    /// 平均帧耗时（毫秒）；样本不足时返回 None
    pub fn avg_ms(&self) -> Option<f32> {
        if self.filled == 0 {
            return None;
        }
        let sum: f32 = self.deltas_ms.iter().take(self.filled).sum();
        Some(sum / self.filled as f32)
    }

    /// 平均 FPS（基于平均帧耗时）；样本不足时返回 None
    pub fn fps(&self) -> Option<f32> {
        self.avg_ms()
            .map(|avg| if avg > 0.0 { 1000.0 / avg } else { 0.0 })
    }

    /// 缓冲期内最差帧耗时（毫秒）
    pub fn max_ms(&self) -> f32 {
        self.max_ms
    }

    /// 累计帧数
    pub fn frames(&self) -> u64 {
        self.frames
    }

    /// 是否已有有效样本
    pub fn has_samples(&self) -> bool {
        self.filled > 0
    }

    /// 清空统计（max 一并归零）
    pub fn reset(&mut self) {
        self.deltas_ms.iter_mut().for_each(|d| *d = 0.0);
        self.head = 0;
        self.filled = 0;
        self.max_ms = 0.0;
    }

    /// 渲染右上角 FPS overlay（半透明底、健康度配色文本）
    ///
    /// 尚无有效样本时返回 None，调用方可直接跳过叠放。
    /// 帧耗健康度配色：≤20ms 绿（60FPS 预算）→ ≤33.4ms 黄（30FPS）→ 更高红。
    pub fn overlay_view<'a, Message: Clone + 'static>(
        &self,
        theme: &crate::theme::Theme,
    ) -> Option<Element<'a, Message>> {
        use iced::widget::container;
        use iced::{Background, Color};
        let avg = self.avg_ms()?;
        let fps = if avg > 0.0 { 1000.0 / avg } else { 0.0 };
        let color = if avg <= 20.0 {
            Color::from(theme.success.base)
        } else if avg <= 33.4 {
            Color::from(theme.warning.base)
        } else {
            Color::from(theme.danger.base)
        };
        let label = text(format!(
            "FPS {:.0} | avg {:.1}ms | max {:.1}ms",
            fps, avg, self.max_ms
        ))
        .size(12)
        .color(color);
        Some(
            container(label)
                .padding(iced::Padding::from([4u16, 8u16]))
                .style(move |_t| container::Style {
                    background: Some(Background::Color(Color {
                        a: 0.75,
                        ..Color::BLACK
                    })),
                    border: iced::Border::default(),
                    ..container::Style::default()
                })
                .width(Length::Shrink)
                .into(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn at(ms: u64) -> Instant {
        Instant::now() + Duration::from_millis(ms)
    }

    #[test]
    fn test_first_frame_no_sample() {
        let mut m = FpsMeter::new();
        m.record(at(0));
        assert!(!m.has_samples());
        assert_eq!(m.avg_ms(), None);
        assert_eq!(m.fps(), None);
        assert_eq!(m.frames(), 1);
    }

    #[test]
    fn test_60fps_stats() {
        let mut m = FpsMeter::new();
        m.record(at(0));
        // 100 帧 × 16.6ms
        for i in 1..=100u64 {
            m.record(at(i * 16));
        }
        let avg = m.avg_ms().unwrap();
        assert!((avg - 16.0).abs() < 0.5, "avg {} ≈ 16ms", avg);
        let fps = m.fps().unwrap();
        assert!((fps - 62.5).abs() < 2.0, "fps {} ≈ 62.5", fps);
        assert!(m.max_ms() <= 16.0 + 0.5);
    }

    #[test]
    fn test_max_tracks_worst_frame() {
        let mut m = FpsMeter::new();
        m.record(at(0));
        for i in 1..10u64 {
            m.record(at(i * 16));
        }
        m.record(at(10 * 16 + 200)); // 一帧卡顿 +200ms
        assert!(m.max_ms() >= 200.0, "max {} 应捕获卡顿帧", m.max_ms());
        // 统计回归正常
        let after = m.record_probe(11);
        assert!(after < 200.0);
    }

    impl FpsMeter {
        // 测试辅助：模拟下一帧 16ms
        fn record_probe(&mut self, _i: u64) -> f32 {
            self.record(at(11 * 16 + 200));
            self.deltas_ms[(self.head + self.deltas_ms.len() - 1) % self.deltas_ms.len()]
        }
    }

    #[test]
    fn test_huge_gap_ignored() {
        let mut m = FpsMeter::new();
        m.record(at(0));
        m.record(at(16));
        m.record(at(5000)); // 挂起恢复 > 1s → 忽略
        assert_eq!(m.filled, 1);
        assert!(m.max_ms() < 1000.0);
    }

    #[test]
    fn test_ring_buffer_capacity() {
        let mut m = FpsMeter::with_capacity(10);
        m.record(at(0));
        for i in 1..=50u64 {
            m.record(at(i * 16));
        }
        assert_eq!(m.filled, 10, "环形缓冲只保留最近 10 帧");
        let avg = m.avg_ms().unwrap();
        assert!((avg - 16.0).abs() < 0.5);
    }

    #[test]
    fn test_reset() {
        let mut m = FpsMeter::new();
        m.record(at(0));
        m.record(at(16));
        m.reset();
        assert!(!m.has_samples());
        assert_eq!(m.max_ms(), 0.0);
        // reset 后可继续采样
        m.record(at(32));
        m.record(at(48));
        assert!(m.has_samples());
    }
}
