//! Backtop 回到顶部 — 参考 Element Plus `<el-backtop>`。
//!
//! 支持：visibility_height 阈值、scroll_y 跟踪、Click 回顶、right/bottom、smooth。

use har_ui_core::theme::Theme;
use iced::widget::{button, container, text};
use iced::{Color, Element, Length, Padding};

/// Backtop 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BacktopMessage {
    /// 滚动事件，更新当前 scroll_y
    Scroll(u32),
    /// 点击回到顶部
    Click,
    /// 设置可见阈值
    SetVisibilityHeight(u32),
    /// 设置是否平滑滚动
    SetSmooth(bool),
}

/// Backtop 组件
#[derive(Debug, Clone)]
pub struct Backtop {
    visibility_height: u32,
    right: u32,
    bottom: u32,
    smooth: bool,
    scroll_y: u32,
}

impl Default for Backtop {
    fn default() -> Self {
        Self::new()
    }
}

impl Backtop {
    pub fn new() -> Self {
        Self {
            visibility_height: 200,
            right: 40,
            bottom: 40,
            smooth: false,
            scroll_y: 0,
        }
    }

    pub fn with_visibility_height(mut self, v: u32) -> Self {
        self.visibility_height = v;
        self
    }

    pub fn with_right(mut self, v: u32) -> Self {
        self.right = v;
        self
    }

    pub fn with_bottom(mut self, v: u32) -> Self {
        self.bottom = v;
        self
    }

    pub fn with_smooth(mut self, v: bool) -> Self {
        self.smooth = v;
        self
    }

    pub fn visibility_height(&self) -> u32 {
        self.visibility_height
    }

    pub fn right(&self) -> u32 {
        self.right
    }

    pub fn bottom(&self) -> u32 {
        self.bottom
    }

    pub fn smooth(&self) -> bool {
        self.smooth
    }

    pub fn scroll_y(&self) -> u32 {
        self.scroll_y
    }

    /// 是否可见（滚动距离严格大于阈值时显示）
    pub fn visible(&self) -> bool {
        self.scroll_y > self.visibility_height
    }

    pub fn handle(&mut self, msg: BacktopMessage) {
        match msg {
            BacktopMessage::Scroll(y) => {
                self.scroll_y = y;
            }
            BacktopMessage::Click => {
                self.scroll_y = 0;
            }
            BacktopMessage::SetVisibilityHeight(v) => {
                self.visibility_height = v;
            }
            BacktopMessage::SetSmooth(v) => {
                self.smooth = v;
            }
        }
    }

    /// 渲染 Backtop 为 iced::Element
    ///
    /// 仅当 `visible()` 返回 true 时渲染圆形按钮（含上箭头 ↑），否则渲染空占位。
    ///
    /// # 参数
    /// - `theme`: HarUI 主题引用
    /// - `on_click`: 点击按钮时发出的消息
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        on_click: Message,
    ) -> Element<'a, Message> {
        if !self.visible() {
            return container(text("")).width(Length::Fixed(0.0)).into();
        }

        let primary = Color::from(theme.primary.base);
        let arrow_color = Color::WHITE;

        let arrow_text = text("↑").color(arrow_color).size(20.0);

        let btn = button(arrow_text)
            .padding(Padding::from(0u16))
            .style(move |_t, _status| iced::widget::button::Style {
                background: Some(iced::Background::Color(primary)),
                text_color: arrow_color,
                border: iced::Border {
                    color: primary,
                    width: 0.0,
                    radius: iced::border::radius(20.0),
                },
                shadow: iced::Shadow::default(),
                snap: false,
            })
            .on_press(on_click);

        let btn_wrap = container(btn)
            .width(Length::Fixed(40.0))
            .height(Length::Fixed(40.0))
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center)
            .style(move |_t| iced::widget::container::Style {
                text_color: None,
                background: None,
                border: iced::Border {
                    color: primary,
                    width: 0.0,
                    radius: iced::border::radius(20.0),
                },
                shadow: iced::Shadow::default(),
                snap: false,
            });

        btn_wrap.into()
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_backtop_visibility_threshold_strict() {
        let b = Backtop::new().with_visibility_height(100);
        // 100 不显示，101 显示
        let mut b1 = b.clone();
        b1.handle(BacktopMessage::Scroll(100));
        assert!(!b1.visible());
        let mut b2 = b.clone();
        b2.handle(BacktopMessage::Scroll(101));
        assert!(b2.visible());
    }
}
