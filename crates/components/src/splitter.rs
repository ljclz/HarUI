//! Splitter 分栏面板 — 参考 Element Plus `<el-splitter>`（2.8+）
//!
//! 支持：水平/垂直分栏、拖拽分隔条调整相邻面板占比（Start/Move/End 三元组，
//! 与 Table 列宽拖拽同款模式）、min 占比钳制、总和守恒、程序化 Resize。
//!
//! ## 模型
//! `sizes` 为各面板的**百分比**（合计恒为 100）；Resize(idx, delta) 把 delta%
//! 从 `idx` 面板转移到 `idx+1` 面板（负值反向），两端受 min_sizes 钳制。

use har_ui_core::theme::Theme;
use iced::Element;

/// 分栏方向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SplitterDirection {
    /// 水平分栏（面板左右排列）
    #[default]
    Horizontal,
    /// 垂直分栏（面板上下排列）
    Vertical,
}

/// Splitter 消息
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SplitterMessage {
    /// 开始拖拽：分隔条索引（第 idx 与 idx+1 面板之间）
    DragStart(usize),
    /// 拖拽移动：累计位移（像素，正=向右/向下）
    DragMove(f32),
    /// 拖拽结束
    DragEnd,
    /// 程序化调整：面板 idx 与 idx+1 之间转移 delta 百分比
    Resize(usize, f32),
    /// 重置为初始均分
    Reset,
}

/// Splitter 组件
#[derive(Debug, Clone)]
pub struct Splitter {
    direction: SplitterDirection,
    sizes: Vec<f32>,
    initial_sizes: Vec<f32>,
    min_sizes: Vec<f32>,
    /// 拖拽会话（分隔条索引, 上次光标位置）
    drag: Option<(usize, Option<f32>)>,
    /// 归一化基准：像素 → 百分比的换算总长（视窗尺寸）
    viewport_extent: f32,
}

impl Default for Splitter {
    fn default() -> Self {
        Self::new(2)
    }
}

impl Splitter {
    /// n 个等分面板（n ≥ 2）
    pub fn new(panes: usize) -> Self {
        let panes = panes.max(2);
        let each = 100.0 / panes as f32;
        Self {
            direction: SplitterDirection::Horizontal,
            sizes: vec![each; panes],
            initial_sizes: vec![each; panes],
            min_sizes: vec![5.0; panes],
            drag: None,
            viewport_extent: 800.0,
        }
    }

    pub fn with_direction(mut self, d: SplitterDirection) -> Self {
        self.direction = d;
        self
    }

    /// 各面板最小百分比（默认 5%）
    pub fn with_min_sizes(mut self, mins: Vec<f32>) -> Self {
        if mins.len() == self.sizes.len() {
            self.min_sizes = mins.into_iter().map(|m| m.max(0.0)).collect();
        }
        self
    }

    /// 视窗总长（像素）——DragMove 像素位移换算百分比用
    pub fn with_viewport_extent(mut self, extent: f32) -> Self {
        self.viewport_extent = extent.max(1.0);
        self
    }

    pub fn direction(&self) -> SplitterDirection {
        self.direction
    }

    /// 各面板百分比（合计恒为 100 ± 1e-6）
    pub fn sizes(&self) -> &[f32] {
        &self.sizes
    }

    /// 处理消息
    pub fn handle(&mut self, msg: SplitterMessage) {
        match msg {
            SplitterMessage::DragStart(idx) => {
                if idx + 1 < self.sizes.len() {
                    self.drag = Some((idx, None));
                }
            }
            SplitterMessage::DragMove(pos) => {
                if let Some((idx, last)) = self.drag {
                    match last {
                        // 首次 move：锚定，不调整
                        None => self.drag = Some((idx, Some(pos))),
                        Some(last_pos) => {
                            let delta_px = pos - last_pos;
                            let delta_pct = delta_px / self.viewport_extent * 100.0;
                            self.resize(idx, delta_pct);
                            self.drag = Some((idx, Some(pos)));
                        }
                    }
                }
            }
            SplitterMessage::DragEnd => {
                self.drag = None;
            }
            SplitterMessage::Resize(idx, delta) => {
                self.resize(idx, delta);
            }
            SplitterMessage::Reset => {
                self.sizes = self.initial_sizes.clone();
            }
        }
    }

    /// 渲染：按 sizes 比例布局面板内容槽（panes 顺序对应面板索引；
    /// 分隔条为静态视觉条——拖拽由应用层经 DragStart/Move/End 消息驱动，
    /// 分隔条定位在面板之间（本视图不做绝对定位，间距由容器 margin 呈现）。
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        panes: Vec<Element<'a, Message>>,
    ) -> Element<'a, Message> {
        use iced::widget::{container, text};
        use iced::{Color, Length, Padding};
        let border = Color::from(theme.neutral.border_lighter);
        let bg = Color::from(theme.neutral.bg_overlay);
        let n = panes.len().min(self.sizes.len());
        let mut items: Vec<Element<'a, Message>> = Vec::new();
        // Element 非 Clone：用 into_iter 逐个消费，分隔条插入相邻面板之间
        for (i, content) in panes.into_iter().take(n).enumerate() {
            let pane = container(content)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(move |_t| iced::widget::container::Style {
                    text_color: None,
                    background: Some(iced::Background::Color(bg)),
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                    snap: false,
                });
            items.push(container(pane).width(Length::Fill).into());
            if i + 1 < n {
                // 分隔条视觉条（拖拽事件由应用层在容器上接线）
                let bar = container(text("").width(Length::Fixed(2.0)).height(Length::Fill))
                    .padding(Padding::from(2u16))
                    .style(move |_t| iced::widget::container::Style {
                        text_color: None,
                        background: Some(iced::Background::Color(border)),
                        border: iced::Border::default(),
                        shadow: iced::Shadow::default(),
                        snap: false,
                    });
                items.push(bar.into());
            }
        }
        match self.direction {
            SplitterDirection::Horizontal => iced::widget::Row::with_children(items).into(),
            SplitterDirection::Vertical => iced::widget::Column::with_children(items).into(),
        }
    }

    /// 核心：idx 与 idx+1 之间转移 delta 百分比（min 钳制，总和守恒）
    fn resize(&mut self, idx: usize, delta: f32) {
        if idx + 1 >= self.sizes.len() {
            return;
        }
        let min_a = self.min_sizes[idx];
        let min_b = self.min_sizes[idx + 1];
        // 钳制 delta：双向都不能越 min
        let max_left = self.sizes[idx] - min_a; // idx 最多让出
        let max_right = self.sizes[idx + 1] - min_b; // idx+1 最多吸收
        let clamped = if delta >= 0.0 {
            delta.min(max_right)
        } else {
            -((-delta).min(max_left))
        };
        self.sizes[idx] += clamped;
        self.sizes[idx + 1] -= clamped;
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_equal_split_default() {
        let s = Splitter::new(3);
        assert_eq!(s.sizes(), [100.0 / 3.0; 3].to_vec().as_slice());
        assert!((s.sizes().iter().sum::<f32>() - 100.0).abs() < 1e-4);
    }

    #[test]
    fn test_resize_transfers_between_neighbors() {
        let mut s = Splitter::new(2);
        s.handle(SplitterMessage::Resize(0, 10.0));
        assert!((s.sizes()[0] - 60.0).abs() < 1e-4);
        assert!((s.sizes()[1] - 40.0).abs() < 1e-4);
        // 总和守恒
        assert!((s.sizes().iter().sum::<f32>() - 100.0).abs() < 1e-4);
        // 负向
        s.handle(SplitterMessage::Resize(0, -20.0));
        assert!((s.sizes()[0] - 40.0).abs() < 1e-4);
    }

    #[test]
    fn test_min_clamped_both_directions() {
        let mut s = Splitter::new(2).with_min_sizes(vec![20.0, 30.0]);
        s.handle(SplitterMessage::Resize(0, -50.0));
        // 左最小 20：只能让出 30
        assert!((s.sizes()[0] - 20.0).abs() < 1e-4);
        assert!((s.sizes()[1] - 80.0).abs() < 1e-4);
        s.handle(SplitterMessage::Resize(0, 200.0));
        // 右最小 30：左最多 70
        assert!((s.sizes()[0] - 70.0).abs() < 1e-4);
        assert!((s.sizes()[1] - 30.0).abs() < 1e-4);
    }

    #[test]
    fn test_drag_session_incremental() {
        let mut s = Splitter::new(2).with_viewport_extent(1000.0);
        s.handle(SplitterMessage::DragStart(0));
        s.handle(SplitterMessage::DragMove(100.0)); // 锚定
        let before = s.sizes()[0];
        s.handle(SplitterMessage::DragMove(150.0)); // +50px = +5%
        assert!((s.sizes()[0] - (before + 5.0)).abs() < 1e-4);
        s.handle(SplitterMessage::DragEnd);
        s.handle(SplitterMessage::DragMove(300.0)); // 会话外无效
        assert!((s.sizes()[0] - (before + 5.0)).abs() < 1e-4);
    }

    #[test]
    fn test_out_of_range_idx_ignored() {
        let mut s = Splitter::new(2);
        s.handle(SplitterMessage::Resize(1, 10.0)); // 最后一个面板无右邻居
        assert_eq!(s.sizes(), [50.0, 50.0]);
        s.handle(SplitterMessage::Resize(9, 10.0));
        assert_eq!(s.sizes(), [50.0, 50.0]);
    }

    #[test]
    fn test_reset() {
        let mut s = Splitter::new(2);
        s.handle(SplitterMessage::Resize(0, 30.0));
        s.handle(SplitterMessage::Reset);
        assert_eq!(s.sizes(), [50.0, 50.0]);
    }

    #[test]
    fn test_three_panes_middle_resize() {
        let mut s = Splitter::new(3);
        s.handle(SplitterMessage::Resize(1, 5.0));
        assert!((s.sizes()[1] - (100.0 / 3.0 + 5.0)).abs() < 1e-4);
        assert!((s.sizes()[2] - (100.0 / 3.0 - 5.0)).abs() < 1e-4);
        assert!(
            (s.sizes()[0] - 100.0 / 3.0).abs() < 1e-4,
            "非相邻面板不受影响"
        );
    }
}
