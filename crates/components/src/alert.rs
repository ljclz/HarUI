//! Alert 警告 — 参考 Element Plus `<el-alert>`。
//!
//! 支持：4 种 type、title/description、closable、center、show-icon、effect（light/dark）、可见性切换。

use har_ui_core::theme::Theme;
use iced::widget::{button, container, text};
use iced::{Color, Element, Length, Padding};

/// 类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlertType {
    Success,
    Warning,
    #[default]
    Info,
    Error,
}

/// 视觉效果
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlertEffect {
    #[default]
    Light,
    Dark,
}

/// Alert 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlertMessage {
    Close,
    Open,
}

/// Alert 组件
#[derive(Debug, Clone)]
pub struct Alert {
    alert_type: AlertType,
    title: String,
    description: Option<String>,
    closable: bool,
    center: bool,
    show_icon: bool,
    effect: AlertEffect,
    visible: bool,
}

impl Default for Alert {
    fn default() -> Self {
        Self::new()
    }
}

impl Alert {
    pub fn new() -> Self {
        Self {
            alert_type: AlertType::Info,
            title: String::new(),
            description: None,
            closable: true,
            center: false,
            show_icon: true,
            effect: AlertEffect::Light,
            visible: true,
        }
    }

    pub fn with_type(mut self, t: AlertType) -> Self {
        self.alert_type = t;
        self
    }

    pub fn with_title(mut self, t: impl Into<String>) -> Self {
        self.title = t.into();
        self
    }

    pub fn with_description(mut self, d: impl Into<String>) -> Self {
        self.description = Some(d.into());
        self
    }

    pub fn with_closable(mut self, v: bool) -> Self {
        self.closable = v;
        self
    }

    pub fn with_center(mut self, v: bool) -> Self {
        self.center = v;
        self
    }

    pub fn with_show_icon(mut self, v: bool) -> Self {
        self.show_icon = v;
        self
    }

    pub fn with_effect(mut self, e: AlertEffect) -> Self {
        self.effect = e;
        self
    }

    pub fn alert_type(&self) -> AlertType {
        self.alert_type
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn closable(&self) -> bool {
        self.closable
    }

    pub fn center(&self) -> bool {
        self.center
    }

    pub fn show_icon(&self) -> bool {
        self.show_icon
    }

    pub fn effect(&self) -> AlertEffect {
        self.effect
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn handle(&mut self, msg: AlertMessage) {
        match msg {
            AlertMessage::Close => {
                if self.closable {
                    self.visible = false;
                }
            }
            AlertMessage::Open => {
                self.visible = true;
            }
        }
    }

    /// 渲染 Alert 为 iced::Element
    ///
    /// # 参数
    /// - `theme`: HarUI 主题引用
    /// - `on_close`: 点击关闭按钮时发出消息
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        on_close: impl Fn() -> Message + 'a,
    ) -> Element<'a, Message> {
        if !self.visible {
            return container(text("")).into();
        }

        let accent_token = match self.alert_type {
            AlertType::Success => theme.success.base,
            AlertType::Warning => theme.warning.base,
            AlertType::Error => theme.danger.base,
            AlertType::Info => theme.info.base,
        };
        let (icon_str, accent) = match self.alert_type {
            AlertType::Success => ("✓", Color::from(theme.success.base)),
            AlertType::Warning => ("⚠", Color::from(theme.warning.base)),
            AlertType::Error => ("✕", Color::from(theme.danger.base)),
            AlertType::Info => ("ℹ", Color::from(theme.info.base)),
        };

        let title_color = Color::from(theme.neutral.text_primary);
        let desc_color = Color::from(theme.neutral.text_regular);
        let bg = if theme.is_dark {
            Color::from(theme.neutral.bg_overlay)
        } else {
            // 浅色：Element Plus 公式 — accent 10% 混白色 90%
            Color::from(har_ui_core::utils::color_utils::mix_colors(
                accent_token,
                har_ui_core::theme::color::ThemeColor::from_rgb(255, 255, 255), // HARUI-EXCEPTION: EP 规范字面白（SCSS $color-white），混色公式常量
                0.1,
            ))
        };

        let mut row_children: Vec<Element<'a, Message>> = Vec::new();
        if self.show_icon {
            row_children.push(text(icon_str.to_string()).color(accent).size(16.0).into());
            row_children.push(iced::widget::Space::with_width(Length::Fixed(8.0)).into());
        }

        let mut text_col_children: Vec<Element<'a, Message>> = Vec::new();
        text_col_children.push(
            text(self.title.clone())
                .color(title_color)
                .size(15.0)
                .into(),
        );
        if let Some(d) = &self.description {
            text_col_children.push(text(d.clone()).color(desc_color).size(13.0).into());
        }
        let text_col = iced::widget::Column::with_children(text_col_children)
            .spacing(4)
            .width(Length::Fill);
        row_children.push(text_col.into());

        if self.closable {
            let close_btn = button(text("✕").color(desc_color).size(13.0))
                .padding(Padding::from([2u16, 6u16]))
                .style(move |_t, _status| iced::widget::button::Style {
                    background: None,
                    text_color: desc_color,
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                })
                .on_press(on_close());
            row_children.push(close_btn.into());
        }

        let mut row = iced::widget::Row::with_children(row_children)
            .spacing(0)
            .align_y(iced::Alignment::Center);
        if self.center {
            row = row.align_y(iced::Alignment::Center);
        }

        container(row)
            .width(Length::Fill)
            .padding(Padding::from([10u16, 16u16]))
            .style(move |_t| iced::widget::container::Style {
                text_color: None,
                background: Some(iced::Background::Color(bg)),
                border: iced::Border {
                    color: accent,
                    width: 1.0,
                    radius: iced::border::radius(4.0),
                },
                shadow: iced::Shadow::default(),
            })
            .into()
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_alert_default_visible() {
        let a = Alert::new();
        assert!(a.visible());
        assert!(a.closable());
        assert!(a.show_icon());
        assert!(!a.center());
    }
}
