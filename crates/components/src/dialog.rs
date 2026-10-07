//! Dialog 组件 — 对话框
//!
//! 参考 Element Plus `<el-dialog>` 组件。
//! 支持可见性状态机、遮罩点击关闭、Escape 关闭、满屏、拖拽。
//!
//! ## 状态机
//! ```text
//! Closed ──Open──► Opening ──AnimationFinished──► Open
//!    ▲                                              │
//!    │                                              │ Close / Overlay / Escape
//!    └──AnimationFinished──◄── Closing ◄───────────┘
//! ```

use har_ui_core::theme::Theme;
use har_ui_core::theme::style_sheets::{self, ButtonKind};
use iced::widget::{button, container, text};
use iced::{Color, Element, Length, Padding};

/// Dialog 状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DialogState {
    #[default]
    Closed,
    Opening,
    Open,
    Closing,
}

/// Dialog Props — 配置
#[derive(Debug, Clone)]
pub struct DialogProps {
    pub close_on_click_modal: bool,
    pub close_on_press_escape: bool,
    pub fullscreen: bool,
    pub draggable: bool,
}

impl Default for DialogProps {
    fn default() -> Self {
        Self {
            close_on_click_modal: true,
            close_on_press_escape: true,
            fullscreen: false,
            draggable: false,
        }
    }
}

impl DialogProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_close_on_click_modal(mut self, v: bool) -> Self {
        self.close_on_click_modal = v;
        self
    }

    pub fn with_close_on_press_escape(mut self, v: bool) -> Self {
        self.close_on_press_escape = v;
        self
    }

    pub fn with_fullscreen(mut self, v: bool) -> Self {
        self.fullscreen = v;
        self
    }

    pub fn with_draggable(mut self, v: bool) -> Self {
        self.draggable = v;
        self
    }
}

/// Dialog 消息
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DialogMessage {
    /// 打开对话框
    Open,
    /// 关闭对话框
    Close,
    /// 动画完成
    AnimationFinished,
    /// 遮罩被点击
    OverlayClicked,
    /// Escape 键按下
    EscapePressed,
    /// 拖拽到新位置
    Dragged(f32, f32),
}

/// Dialog 组件
#[derive(Debug, Clone)]
pub struct Dialog {
    title: String,
    content: String,
    state: DialogState,
    props: DialogProps,
    /// 拖拽位置（None 表示默认居中）
    position: Option<(f32, f32)>,
}

impl Dialog {
    pub fn new(title: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            content: content.into(),
            state: DialogState::Closed,
            props: DialogProps::default(),
            position: None,
        }
    }

    pub fn with_props(mut self, props: DialogProps) -> Self {
        self.props = props;
        self
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn state(&self) -> DialogState {
        self.state
    }

    pub fn props(&self) -> &DialogProps {
        &self.props
    }

    pub fn is_visible(&self) -> bool {
        self.state != DialogState::Closed
    }

    pub fn position(&self) -> Option<(f32, f32)> {
        self.position
    }

    /// 计算对话框的默认居中矩形（委托 core 行为层 `overlay::centered`，ADR-009；
    /// Element Plus dialog 默认语义：视窗水平垂直居中，内容超出视窗时钳到起点）
    pub fn centered_rect(
        &self,
        viewport: har_ui_core::behavior::overlay::Rect,
        content: har_ui_core::behavior::overlay::Size,
    ) -> har_ui_core::behavior::overlay::Rect {
        har_ui_core::behavior::overlay::centered(viewport, content)
    }

    /// 渲染对话框为 iced::Element
    ///
    /// # 参数
    /// - `theme`: HarUI 主题引用
    /// - `on_close`: 关闭按钮/遮罩点击时发出的消息
    ///
    /// # 行为
    /// - Closed：返回空 container（占位）
    /// - Opening/Open/Closing：渲染遮罩 + 对话框（标题 + 内容 + 关闭按钮）
    /// - fullscreen：对话框填满视窗
    /// - draggable：position 决定对话框偏移
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        on_close: Message,
    ) -> Element<'a, Message> {
        if !self.is_visible() {
            // 关闭时返回空 container
            return container(text("")).into();
        }

        // 标题栏
        let title_text = text(self.title.clone())
            .color(Color::from(theme.neutral.text_primary))
            .size(16);
        let close_btn = button(text("×").color(Color::from(theme.neutral.text_regular)))
            .padding(Padding::from([2u16, 8u16]))
            .on_press(on_close.clone())
            .style(move |_t, status| {
                style_sheets::button_style(theme, ButtonKind::Text, false, status)
            });
        let header = iced::widget::Row::new()
            .push(title_text)
            .push(iced::widget::Space::with_width(Length::Fill))
            .push(close_btn)
            .align_y(iced::Alignment::Center)
            .padding(Padding::from([12u16, 16u16]));

        // 内容区
        let body =
            container(text(self.content.clone()).color(Color::from(theme.neutral.text_regular)))
                .width(Length::Fill)
                .padding(Padding::from(16u16));

        // 对话框主体（用 container 包裹以应用样式，因为 Column 无 style 方法）
        let dialog_inner = iced::widget::Column::new().push(header).push(body);
        let dialog_box = container(dialog_inner)
            .max_width(if self.props.fullscreen {
                100000.0
            } else {
                500.0
            })
            .style(move |_t| iced::widget::container::Style {
                text_color: Some(Color::from(theme.neutral.text_primary)),
                background: Some(iced::Background::Color(Color::from(
                    theme.neutral.bg_overlay,
                ))),
                border: iced::Border {
                    color: Color::from(theme.neutral.border_lighter),
                    width: 1.0,
                    radius: iced::border::radius(4.0),
                },
                shadow: iced::Shadow {
                    color: Color {
                        a: 0.3,
                        ..Color::BLACK
                    },
                    offset: iced::Vector::new(0.0, 4.0),
                    blur_radius: 16.0,
                },
            });

        // 注：position 偏移在 iced 中需要绝对定位支持，此处简化为中心对齐
        let _ = self.position;
        let dialog_container = if self.props.fullscreen {
            container(dialog_box)
                .width(Length::Fill)
                .height(Length::Fill)
        } else {
            container(dialog_box)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
        };

        // 遮罩层 + 对话框
        let overlay = container(dialog_container)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_t| style_sheets::container_dialog_mask_style(theme));

        overlay.into()
    }

    /// 处理消息
    pub fn handle(&mut self, msg: DialogMessage) {
        match msg {
            DialogMessage::Open => {
                if self.state == DialogState::Closed {
                    self.state = DialogState::Opening;
                }
            }
            DialogMessage::Close => match self.state {
                DialogState::Opening | DialogState::Open => {
                    self.state = DialogState::Closing;
                }
                _ => {}
            },
            DialogMessage::AnimationFinished => {
                match self.state {
                    DialogState::Opening => self.state = DialogState::Open,
                    DialogState::Closing => {
                        self.state = DialogState::Closed;
                        // 关闭后清除拖拽位置
                        self.position = None;
                    }
                    _ => {}
                }
            }
            DialogMessage::OverlayClicked => {
                if self.state == DialogState::Open && self.props.close_on_click_modal {
                    self.state = DialogState::Closing;
                }
            }
            DialogMessage::EscapePressed => {
                if self.state == DialogState::Open && self.props.close_on_press_escape {
                    self.state = DialogState::Closing;
                }
            }
            DialogMessage::Dragged(x, y) => {
                if self.props.draggable && self.state == DialogState::Open {
                    self.position = Some((x, y));
                }
            }
        }
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_dialog_default_props() {
        let dlg = Dialog::new("T", "C");
        assert_eq!(dlg.state(), DialogState::Closed);
        assert!(!dlg.is_visible());
        assert_eq!(dlg.position(), None);
        assert!(dlg.props().close_on_click_modal);
    }

    #[test]
    fn test_dialog_props_builder() {
        let p = DialogProps::new()
            .with_close_on_click_modal(false)
            .with_close_on_press_escape(false)
            .with_fullscreen(true)
            .with_draggable(true);
        assert!(!p.close_on_click_modal);
        assert!(!p.close_on_press_escape);
        assert!(p.fullscreen);
        assert!(p.draggable);
    }

    // ============ 定位引擎接入（ADR-009） ============

    #[test]
    fn test_dialog_centered_rect_in_viewport() {
        let d = Dialog::new("标题", "内容");
        let r = d.centered_rect(
            har_ui_core::behavior::overlay::Rect {
                x: 0.0,
                y: 0.0,
                width: 800.0,
                height: 600.0,
            },
            har_ui_core::behavior::overlay::Size::new(400.0, 300.0),
        );
        assert_eq!(r.x, 200.0);
        assert_eq!(r.y, 150.0);
    }
}
