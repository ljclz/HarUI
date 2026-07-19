//! Drawer 抽屉组件 — 参考 Element Plus `<el-drawer>`。
//! 支持：4 种 direction（rtl/ltr/ttb/btt）、状态机（Closed→Opening→Open→Closing→Closed）、
//! close_on_click_modal/close_on_press_escape、modal、show_close、destroy_on_close。

use har_ui_core::theme::Theme;
use iced::widget::{button, container, text};
use iced::{Color, Element, Length, Padding};

/// Drawer 弹出方向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DrawerDirection {
    /// 右往左
    #[default]
    Rtl,
    /// 左往右
    Ltr,
    /// 上往下
    Ttb,
    /// 下往上
    Btt,
}

impl DrawerDirection {
    pub fn as_str(self) -> &'static str {
        match self {
            DrawerDirection::Rtl => "rtl",
            DrawerDirection::Ltr => "ltr",
            DrawerDirection::Ttb => "ttb",
            DrawerDirection::Btt => "btt",
        }
    }
}

/// Drawer 状态机
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DrawerState {
    #[default]
    Closed,
    Opening,
    Open,
    Closing,
}

/// Drawer 消息
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawerMessage {
    /// 打开抽屉
    Open,
    /// 关闭抽屉
    Close,
    /// 动画结束
    AnimationEnd,
    /// 点击遮罩
    ClickModal,
    /// 按下 Escape
    PressEscape,
    /// 点击关闭按钮
    ClickClose,
}

/// Drawer 组件
#[derive(Debug, Clone)]
pub struct Drawer {
    /// 当前状态
    state: DrawerState,
    /// 标题
    title: Option<String>,
    /// 弹出方向
    direction: DrawerDirection,
    /// 尺寸（百分比或 px 字符串）
    size: String,
    /// 是否显示关闭按钮
    show_close: bool,
    /// 是否显示遮罩
    modal: bool,
    /// 是否点击遮罩关闭
    close_on_click_modal: bool,
    /// 是否按 Escape 关闭
    close_on_press_escape: bool,
    /// 是否关闭时销毁内容
    destroy_on_close: bool,
}

impl Default for Drawer {
    fn default() -> Self {
        Self::new()
    }
}

impl Drawer {
    pub fn new() -> Self {
        Self {
            state: DrawerState::Closed,
            title: None,
            direction: DrawerDirection::Rtl,
            size: "30%".to_string(),
            show_close: true,
            modal: true,
            close_on_click_modal: true,
            close_on_press_escape: true,
            destroy_on_close: false,
        }
    }

    // ---------- Builder ----------

    pub fn with_title(mut self, t: &str) -> Self {
        self.title = Some(t.to_string());
        self
    }

    pub fn with_direction(mut self, d: DrawerDirection) -> Self {
        self.direction = d;
        self
    }

    pub fn with_size(mut self, s: &str) -> Self {
        self.size = s.to_string();
        self
    }

    pub fn with_show_close(mut self, s: bool) -> Self {
        self.show_close = s;
        self
    }

    pub fn with_modal(mut self, m: bool) -> Self {
        self.modal = m;
        self
    }

    pub fn with_close_on_click_modal(mut self, c: bool) -> Self {
        self.close_on_click_modal = c;
        self
    }

    pub fn with_close_on_press_escape(mut self, c: bool) -> Self {
        self.close_on_press_escape = c;
        self
    }

    pub fn with_destroy_on_close(mut self, d: bool) -> Self {
        self.destroy_on_close = d;
        self
    }

    // ---------- Getter ----------

    pub fn state(&self) -> DrawerState {
        self.state
    }

    pub fn visible(&self) -> bool {
        match self.state {
            DrawerState::Closed => false,
            DrawerState::Opening | DrawerState::Open | DrawerState::Closing => true,
        }
    }

    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    pub fn direction(&self) -> DrawerDirection {
        self.direction
    }

    pub fn size(&self) -> &str {
        &self.size
    }

    pub fn show_close(&self) -> bool {
        self.show_close
    }

    pub fn modal(&self) -> bool {
        self.modal
    }

    pub fn close_on_click_modal(&self) -> bool {
        self.close_on_click_modal
    }

    pub fn close_on_press_escape(&self) -> bool {
        self.close_on_press_escape
    }

    pub fn destroy_on_close(&self) -> bool {
        self.destroy_on_close
    }

    // ---------- 消息处理 ----------

    pub fn handle(&mut self, msg: DrawerMessage) {
        match (self.state, msg) {
            // Closed 状态：只响应 Open
            (DrawerState::Closed, DrawerMessage::Open) => {
                self.state = DrawerState::Opening;
            }

            // Opening 状态：响应 AnimationEnd → Open
            (DrawerState::Opening, DrawerMessage::AnimationEnd) => {
                self.state = DrawerState::Open;
            }

            // Open 状态：响应 Close → Closing
            (DrawerState::Open, DrawerMessage::Close) => {
                self.state = DrawerState::Closing;
            }
            // Open 状态：点击关闭按钮
            (DrawerState::Open, DrawerMessage::ClickClose) => {
                if self.show_close {
                    self.state = DrawerState::Closing;
                }
            }
            // Open 状态：点击遮罩
            (DrawerState::Open, DrawerMessage::ClickModal) => {
                if self.modal && self.close_on_click_modal {
                    self.state = DrawerState::Closing;
                }
            }
            // Open 状态：Escape
            (DrawerState::Open, DrawerMessage::PressEscape) => {
                if self.close_on_press_escape {
                    self.state = DrawerState::Closing;
                }
            }

            // Closing 状态：响应 AnimationEnd → Closed
            (DrawerState::Closing, DrawerMessage::AnimationEnd) => {
                self.state = DrawerState::Closed;
            }

            _ => {}
        }
    }

    /// 渲染 Drawer 为 iced::Element
    ///
    /// - `visible() == false` 时返回空容器
    /// - 否则按 direction 定位 + 标题 + 关闭按钮 + 占位内容
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        on_close: impl Fn() -> Message + 'a,
    ) -> Element<'a, Message> {
        if !self.visible() {
            return container(text("")).into();
        }

        let title_color = Color::from(theme.neutral.text_primary);
        let content_color = Color::from(theme.neutral.text_regular);
        let bg = Color::from(theme.neutral.bg_overlay);
        let border_color = Color::from(theme.neutral.border_lighter);
        let mask = Color { a: 0.5, ..Color::BLACK };

        // 标题栏
        let mut header_children: Vec<Element<'a, Message>> = Vec::new();
        if let Some(t) = &self.title {
            header_children.push(text(t.clone()).color(title_color).size(16.0).into());
        } else {
            header_children.push(text("").into());
        }
        header_children.push(iced::widget::Space::with_width(Length::Fill).into());
        if self.show_close {
            let close_btn = button(text("×").color(content_color).size(18.0))
                .padding(Padding::from([2u16, 8u16]))
                .on_press(on_close())
                .style(move |_t, _status| iced::widget::button::Style {
                    background: None,
                    text_color: content_color,
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                });
            header_children.push(close_btn.into());
        }
        let header = iced::widget::Row::with_children(header_children)
            .align_y(iced::Alignment::Center)
            .spacing(0);

        // 占位内容
        let body_text = text("[drawer content]")
            .color(Color::from(theme.neutral.text_placeholder))
            .size(14.0);
        let body = container(body_text)
            .width(Length::Fill)
            .padding(Padding::from([16u16, 16u16]));

        let inner = iced::widget::Column::new()
            .push(header)
            .push(iced::widget::Space::with_height(Length::Fixed(8.0)))
            .push(body);

        // 按 direction 计算尺寸
        let (drawer_width, drawer_height): (Length, Length) = match self.direction {
            DrawerDirection::Rtl | DrawerDirection::Ltr => {
                (Length::Fixed(360.0), Length::Fill)
            }
            DrawerDirection::Ttb | DrawerDirection::Btt => {
                (Length::Fill, Length::Fixed(280.0))
            }
        };

        let drawer = container(inner)
            .width(drawer_width)
            .height(drawer_height)
            .style(move |_t| iced::widget::container::Style {
                text_color: Some(title_color),
                background: Some(iced::Background::Color(bg)),
                border: iced::Border {
                    color: border_color,
                    width: 1.0,
                    radius: iced::border::radius(0.0),
                },
                shadow: iced::Shadow::default(),
            });

        // 按 direction 决定整体容器的对齐方式
        let positioned = match self.direction {
            DrawerDirection::Rtl => container(drawer)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(iced::alignment::Horizontal::Right),
            DrawerDirection::Ltr => container(drawer)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(iced::alignment::Horizontal::Left),
            DrawerDirection::Ttb => container(drawer)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_y(iced::alignment::Vertical::Top),
            DrawerDirection::Btt => container(drawer)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_y(iced::alignment::Vertical::Bottom),
        };

        container(positioned)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_t| iced::widget::container::Style {
                text_color: None,
                background: Some(iced::Background::Color(mask)),
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
            })
            .into()
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_drawer_internal_default() {
        let d = Drawer::new();
        assert_eq!(d.state, DrawerState::Closed);
        assert_eq!(d.direction, DrawerDirection::Rtl);
        assert_eq!(d.size, "30%");
    }

    #[test]
    fn test_drawer_internal_visible_map() {
        let mut d = Drawer::new();
        assert!(!d.visible());
        d.handle(DrawerMessage::Open);
        assert!(d.visible());
        d.handle(DrawerMessage::AnimationEnd);
        assert!(d.visible());
        d.handle(DrawerMessage::Close);
        assert!(d.visible());
        d.handle(DrawerMessage::AnimationEnd);
        assert!(!d.visible());
    }

    #[test]
    fn test_drawer_internal_close_button_disabled() {
        let mut d = Drawer::new().with_show_close(false);
        d.handle(DrawerMessage::Open);
        d.handle(DrawerMessage::AnimationEnd);
        d.handle(DrawerMessage::ClickClose);
        assert_eq!(d.state, DrawerState::Open, "show_close=false 时 ClickClose 不应关闭");
    }
}
