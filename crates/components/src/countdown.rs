//! Countdown 倒计时 — 参考 Element Plus `<el-countdown>`
//!
//! 支持：倒计时驱动（应用层经 `Tick(delta_ms)` 喂时间，配 `iced::time::every`
//! 或 window::frames）、暂停/恢复/重启、结束态（finished 后 Tick 不再生效）。
//!
//! ## 驱动说明
//! 组件不自带定时器（与 Payment 等逻辑组件一致），应用层订阅 `iced::time::every(1s)`
//! 并把流逝毫秒经 `Tick` 喂入；remaining 用 i64 内部表示防止下溢。

use har_ui_core::theme::Theme;
use iced::Element;

/// Countdown 消息
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CountdownMessage {
    /// 时间流逝（毫秒，应用层喂入；仅在运行中且未结束时生效）
    Tick(u64),
    /// 暂停
    Pause,
    /// 恢复
    Resume,
    /// 重启（remaining 重置为 total 并进入运行态）
    Restart,
}

/// Countdown 组件
#[derive(Debug, Clone)]
pub struct Countdown {
    total_ms: u64,
    remaining_ms: i64,
    running: bool,
    finished: bool,
}

impl Countdown {
    pub fn new(total_ms: u64) -> Self {
        Self {
            total_ms,
            remaining_ms: total_ms as i64,
            running: total_ms > 0,
            finished: total_ms == 0,
        }
    }

    pub fn paused(mut self) -> Self {
        self.running = false;
        self
    }

    pub fn total_ms(&self) -> u64 {
        self.total_ms
    }

    pub fn remaining_ms(&self) -> i64 {
        self.remaining_ms
    }

    /// 剩余整秒（向上取整，保证 1ms 也显示 1 秒）
    pub fn remaining_secs(&self) -> u64 {
        ((self.remaining_ms + 999) / 1000).max(0) as u64
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }

    /// 处理消息
    pub fn handle(&mut self, msg: CountdownMessage) {
        match msg {
            CountdownMessage::Tick(delta_ms) => {
                if !self.running || self.finished {
                    return;
                }
                self.remaining_ms -= delta_ms as i64;
                if self.remaining_ms <= 0 {
                    self.remaining_ms = 0;
                    self.finished = true;
                    self.running = false;
                }
            }
            CountdownMessage::Pause => {
                if !self.finished {
                    self.running = false;
                }
            }
            CountdownMessage::Resume => {
                if !self.finished {
                    self.running = true;
                }
            }
            CountdownMessage::Restart => {
                self.remaining_ms = self.total_ms as i64;
                self.finished = self.total_ms == 0;
                self.running = !self.finished;
            }
        }
    }

    /// 渲染 mm:ss（或 hh:mm:ss，超过 1 小时时）
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        on_msg: impl Fn(CountdownMessage) -> Message + Clone + 'a,
    ) -> Element<'a, Message> {
        use iced::widget::{button, column, container, text};
        use iced::{Color, Padding};
        let _ = &on_msg;
        let secs = self.remaining_secs();
        let label = if self.total_ms >= 3_600_000 {
            format!(
                "{:02}:{:02}:{:02}",
                secs / 3600,
                (secs % 3600) / 60,
                secs % 60
            )
        } else {
            format!("{:02}:{:02}", secs / 60, secs % 60)
        };
        let color = if self.finished {
            Color::from(theme.neutral.text_placeholder)
        } else {
            Color::from(theme.primary.base)
        };
        let mut col = column![
            text(label).size(28).color(color),
            text(if self.finished {
                "已结束"
            } else if self.running {
                "进行中"
            } else {
                "已暂停"
            })
            .size(12)
            .color(Color::from(theme.neutral.text_secondary)),
        ]
        .spacing(4);
        col = col.push(
            container(
                button(text("重启").size(12))
                    .on_press(on_msg(CountdownMessage::Restart))
                    .padding(Padding::from([4u16, 10u16])),
            )
            .padding(Padding::new(0.0).top(4.0)),
        );
        container(col).into()
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_initial_running() {
        let c = Countdown::new(10_000);
        assert_eq!(c.remaining_ms(), 10_000);
        assert!(c.is_running());
        assert!(!c.is_finished());
        assert_eq!(c.remaining_secs(), 10);
    }

    #[test]
    fn test_tick_decrements() {
        let mut c = Countdown::new(10_000);
        c.handle(CountdownMessage::Tick(2_500));
        assert_eq!(c.remaining_ms(), 7_500);
        assert_eq!(c.remaining_secs(), 8); // 向上取整
    }

    #[test]
    fn test_finish_at_zero() {
        let mut c = Countdown::new(1_000);
        c.handle(CountdownMessage::Tick(600));
        assert!(!c.is_finished());
        c.handle(CountdownMessage::Tick(400));
        assert!(c.is_finished());
        assert_eq!(c.remaining_ms(), 0);
        assert!(!c.is_running());
        // 结束后 Tick 不再生效（也不会变负）
        c.handle(CountdownMessage::Tick(5_000));
        assert_eq!(c.remaining_ms(), 0);
        assert!(c.is_finished());
    }

    #[test]
    fn test_overshoot_clamped_to_zero() {
        let mut c = Countdown::new(1_000);
        c.handle(CountdownMessage::Tick(9_999));
        assert_eq!(c.remaining_ms(), 0);
        assert!(c.is_finished());
    }

    #[test]
    fn test_pause_resume() {
        let mut c = Countdown::new(10_000).paused();
        assert!(!c.is_running());
        c.handle(CountdownMessage::Tick(1_000));
        assert_eq!(c.remaining_ms(), 10_000, "暂停中 Tick 无效");
        c.handle(CountdownMessage::Resume);
        assert!(c.is_running());
        c.handle(CountdownMessage::Tick(1_000));
        assert_eq!(c.remaining_ms(), 9_000);
        c.handle(CountdownMessage::Pause);
        c.handle(CountdownMessage::Pause); // 重复暂停幂等
        assert!(!c.is_running());
        c.handle(CountdownMessage::Tick(9_000));
        assert_eq!(c.remaining_ms(), 9_000, "暂停中 Tick 无效");
        // 恢复后跑完
        c.handle(CountdownMessage::Resume);
        c.handle(CountdownMessage::Tick(9_000));
        assert!(c.is_finished());
        // 结束后 Resume/Restart(非零总长除外) 语义
        c.handle(CountdownMessage::Resume);
        assert!(!c.is_running());
    }

    #[test]
    fn test_restart_resets() {
        let mut c = Countdown::new(5_000);
        c.handle(CountdownMessage::Tick(4_999));
        c.handle(CountdownMessage::Restart);
        assert_eq!(c.remaining_ms(), 5_000);
        assert!(c.is_running());
        assert!(!c.is_finished());
        // 零时长倒计时：初始即结束
        let z = Countdown::new(0);
        assert!(z.is_finished());
        let mut z2 = Countdown::new(0);
        z2.handle(CountdownMessage::Restart);
        assert!(z2.is_finished());
    }
}
