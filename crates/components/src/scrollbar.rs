//! Scrollbar 滚动条 — 参考 Element Plus `<el-scrollbar>`。
//!
//! 支持：height/max_height、native、always_visible、滚动位置（scroll_x/scroll_y）、
//! 最大滚动范围钳制、增量滚动、重置。

use har_ui_core::theme::Theme;
use iced::widget::{container, scrollable};
use iced::{Color, Element, Length};

/// Scrollbar 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScrollbarMessage {
    /// 滚动到绝对位置
    ScrollTo { x: u32, y: u32 },
    /// 滚动增量（可为负）
    ScrollBy { dx: i32, dy: i32 },
    /// 重置到原点
    Reset,
}

/// Scrollbar 组件
#[derive(Debug, Clone)]
pub struct Scrollbar {
    height: Option<u32>,
    max_height: Option<u32>,
    always_visible: bool,
    native: bool,
    scroll_x: u32,
    scroll_y: u32,
    max_scroll_x: u32,
    max_scroll_y: u32,
}

impl Default for Scrollbar {
    fn default() -> Self {
        Self::new()
    }
}

impl Scrollbar {
    pub fn new() -> Self {
        Self {
            height: None,
            max_height: None,
            always_visible: false,
            native: false,
            scroll_x: 0,
            scroll_y: 0,
            max_scroll_x: 0,
            max_scroll_y: 0,
        }
    }

    pub fn with_height(mut self, h: u32) -> Self {
        self.height = Some(h);
        self
    }

    pub fn with_max_height(mut self, h: u32) -> Self {
        self.max_height = Some(h);
        self
    }

    pub fn with_always_visible(mut self, v: bool) -> Self {
        self.always_visible = v;
        self
    }

    pub fn with_native(mut self, v: bool) -> Self {
        self.native = v;
        self
    }

    pub fn with_max_scroll(mut self, max_x: u32, max_y: u32) -> Self {
        self.max_scroll_x = max_x;
        self.max_scroll_y = max_y;
        self
    }

    pub fn set_max_scroll(&mut self, max_x: u32, max_y: u32) {
        self.max_scroll_x = max_x;
        self.max_scroll_y = max_y;
        // 钳制当前滚动位置到新范围
        self.clamp_position();
    }

    pub fn height(&self) -> Option<u32> {
        self.height
    }

    pub fn max_height(&self) -> Option<u32> {
        self.max_height
    }

    pub fn always_visible(&self) -> bool {
        self.always_visible
    }

    pub fn native(&self) -> bool {
        self.native
    }

    pub fn scroll_x(&self) -> u32 {
        self.scroll_x
    }

    pub fn scroll_y(&self) -> u32 {
        self.scroll_y
    }

    pub fn max_scroll_x(&self) -> u32 {
        self.max_scroll_x
    }

    pub fn max_scroll_y(&self) -> u32 {
        self.max_scroll_y
    }

    pub fn handle(&mut self, msg: ScrollbarMessage) {
        match msg {
            ScrollbarMessage::ScrollTo { x, y } => {
                self.scroll_x = x;
                self.scroll_y = y;
                self.clamp_position();
            }
            ScrollbarMessage::ScrollBy { dx, dy } => {
                self.scroll_x = apply_delta(self.scroll_x, dx);
                self.scroll_y = apply_delta(self.scroll_y, dy);
                self.clamp_position();
            }
            ScrollbarMessage::Reset => {
                self.scroll_x = 0;
                self.scroll_y = 0;
            }
        }
    }

    fn clamp_position(&mut self) {
        // max 为 0 表示未设置上限，不进行钳制
        if self.max_scroll_x > 0 && self.scroll_x > self.max_scroll_x {
            self.scroll_x = self.max_scroll_x;
        }
        if self.max_scroll_y > 0 && self.scroll_y > self.max_scroll_y {
            self.scroll_y = self.max_scroll_y;
        }
    }

    /// 渲染 Scrollbar 为 iced::Element
    ///
    /// 用 `iced::widget::scrollable` 包裹 content，
    /// 按 height/max_height 配置容器尺寸。
    ///
    /// # 参数
    /// - `theme`: HarUI 主题引用
    /// - `content`: 被包裹的内容
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        content: Element<'a, Message>,
    ) -> Element<'a, Message> {
        let border_color = Color::from(theme.neutral.border_lighter);

        let scroller = scrollable(content);
        let with_height = match self.height {
            Some(h) => scroller.height(Length::Fixed(h as f32)),
            None => scroller,
        };
        let with_max_height = match self.max_height {
            Some(h) => with_height.height(Length::Fixed(h as f32)),
            None => with_height,
        };

        container(with_max_height)
            .width(Length::Fill)
            .style(move |_t| iced::widget::container::Style {
                text_color: None,
                background: None,
                border: iced::Border {
                    color: border_color,
                    width: 1.0,
                    radius: iced::border::radius(4.0),
                },
                shadow: iced::Shadow::default(),
                snap: false,
            })
            .into()
    }
}

/// 将 i32 增量应用在 u32 上，负值钳制到 0
fn apply_delta(current: u32, delta: i32) -> u32 {
    if delta >= 0 {
        current.saturating_add(delta as u32)
    } else {
        current.saturating_sub((-delta) as u32)
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_apply_delta_positive() {
        assert_eq!(apply_delta(10, 5), 15);
    }

    #[test]
    fn test_apply_delta_negative() {
        assert_eq!(apply_delta(10, -5), 5);
    }

    #[test]
    fn test_apply_delta_negative_clamp() {
        assert_eq!(apply_delta(3, -10), 0);
    }
}
