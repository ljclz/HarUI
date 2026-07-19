//! Button 组件 — 按钮控件
//!
//! 参考 Element Plus `<el-button>` 组件。
//!
//! ## Props
//! - text: 按钮文本
//! - button_type: 8 种类型（Default/Primary/Success/Warning/Danger/Info/Text/Link）
//! - size: 3 种尺寸（Large/Default/Small）
//! - disabled: 禁用态
//! - loading: 加载中
//! - plain: 朴素按钮
//! - round: 圆角按钮
//! - circle: 圆形按钮
//!
//! ## State 状态机
//! ```text
//! Normal ──hover──► Hover ──press──► Active
//!   ▲                 ▲                 │
//!   │                 └──release────────┘
//!   └──────────unhover────────────────────
//! ```

use har_ui_core::theme::style_sheets::{self, ButtonKind};
use har_ui_core::theme::Theme;
use iced::widget::{button, text};
use iced::{Element, Padding};

/// 按钮类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonType {
    #[default]
    Default,
    Primary,
    Success,
    Warning,
    Danger,
    Info,
    /// 文字按钮（无背景无边框）
    Text,
    /// 链接按钮
    Link,
}

/// 按钮尺寸
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonSize {
    Large,
    #[default]
    Default,
    Small,
}

/// 按钮交互状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonState {
    #[default]
    Normal,
    Hover,
    Active,
}

/// Button Props — 所有配置属性
#[derive(Debug, Clone)]
pub struct ButtonProps {
    pub text: String,
    pub button_type: ButtonType,
    pub size: ButtonSize,
    pub disabled: bool,
    pub loading: bool,
    pub plain: bool,
    pub round: bool,
    pub circle: bool,
}

impl Default for ButtonProps {
    fn default() -> Self {
        Self {
            text: String::new(),
            button_type: ButtonType::Default,
            size: ButtonSize::Default,
            disabled: false,
            loading: false,
            plain: false,
            round: false,
            circle: false,
        }
    }
}

/// Button 消息（事件）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonMessage {
    Clicked,
    Hovered,
    Unhovered,
    Pressed,
    Released,
    /// 无状态变化
    NoChange,
}

/// Button 组件
#[derive(Debug, Clone)]
pub struct Button {
    props: ButtonProps,
    state: ButtonState,
}

impl Button {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            props: ButtonProps {
                text: text.into(),
                ..Default::default()
            },
            state: ButtonState::Normal,
        }
    }

    pub fn with_type(mut self, t: ButtonType) -> Self {
        self.props.button_type = t;
        self
    }

    pub fn with_size(mut self, s: ButtonSize) -> Self {
        self.props.size = s;
        self
    }

    pub fn disabled(mut self, v: bool) -> Self {
        self.props.disabled = v;
        self
    }

    pub fn loading(mut self, v: bool) -> Self {
        self.props.loading = v;
        self
    }

    pub fn plain(mut self, v: bool) -> Self {
        self.props.plain = v;
        self
    }

    pub fn round(mut self, v: bool) -> Self {
        self.props.round = v;
        self
    }

    pub fn circle(mut self, v: bool) -> Self {
        self.props.circle = v;
        self
    }

    pub fn props(&self) -> &ButtonProps {
        &self.props
    }

    pub fn state(&self) -> ButtonState {
        self.state
    }

    /// 将 ButtonType 映射到 style_sheets::ButtonKind
    ///
    /// style_sheets 模块在 core crate 中独立定义 ButtonKind 以避免循环依赖，
    /// 此方法负责两层枚举的一一映射。
    pub fn kind(&self) -> ButtonKind {
        match self.props.button_type {
            ButtonType::Default => ButtonKind::Default,
            ButtonType::Primary => ButtonKind::Primary,
            ButtonType::Success => ButtonKind::Success,
            ButtonType::Warning => ButtonKind::Warning,
            ButtonType::Danger => ButtonKind::Danger,
            ButtonType::Info => ButtonKind::Info,
            ButtonType::Text => ButtonKind::Text,
            ButtonType::Link => ButtonKind::Link,
        }
    }

    /// 按尺寸返回 padding（参考 Element Plus 数值）
    ///
    /// - Large: 12px × 19px
    /// - Default: 8px × 15px
    /// - Small: 5px × 11px
    pub fn padding_for_size(size: ButtonSize) -> Padding {
        match size {
            ButtonSize::Large => Padding::from([12u16, 19u16]),
            ButtonSize::Default => Padding::from([8u16, 15u16]),
            ButtonSize::Small => Padding::from([5u16, 11u16]),
        }
    }

    /// 按形状（round/circle）返回 border radius
    ///
    /// - 默认：0
    /// - round：20px（足够圆角）
    /// - circle：50px（圆形优先级高于 round）
    pub fn radius_for_shape(round: bool, circle: bool) -> iced::border::Radius {
        if circle {
            iced::border::radius(50.0)
        } else if round {
            iced::border::radius(20.0)
        } else {
            iced::border::Radius::default()
        }
    }

    /// 计算完整的 button::Style（包含 round/circle 半径）
    ///
    /// 调用 core crate 的 `style_sheets::button_style`，并叠加形状半径。
    pub fn compute_style(&self, theme: &Theme, status: button::Status) -> button::Style {
        let kind = self.kind();
        let mut style = style_sheets::button_style(theme, kind, self.props.plain, status);
        style.border.radius = Self::radius_for_shape(self.props.round, self.props.circle);
        style
    }

    /// 渲染按钮为 iced::Element
    ///
    /// # 参数
    /// - `theme`: HarUI 主题引用（用于注入 style）
    /// - `on_press`: 按钮按下时发出的消息
    ///
    /// # 行为
    /// - `disabled` 或 `loading` 时不附加 on_press，iced 会自动设置 Status::Disabled
    /// - style 闭包捕获 theme 引用，按 status 动态计算样式
    /// - 圆角（round/circle）通过 border.radius 注入
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        on_press: Message,
    ) -> Element<'a, Message> {
        let content = text(&self.props.text);
        let mut btn = button(content).padding(Self::padding_for_size(self.props.size));

        let disabled = self.props.disabled || self.props.loading;
        if !disabled {
            btn = btn.on_press(on_press);
        }

        let kind = self.kind();
        let plain = self.props.plain;
        let round = self.props.round;
        let circle = self.props.circle;
        btn = btn.style(move |_t, status| {
            let mut s = style_sheets::button_style(theme, kind, plain, status);
            s.border.radius = Self::radius_for_shape(round, circle);
            s
        });

        btn.into()
    }

    /// 处理消息事件，返回对外发出的消息
    pub fn handle(&mut self, msg: ButtonMessage) -> ButtonMessage {
        // 禁用或 loading 状态下忽略所有交互
        if self.props.disabled || self.props.loading {
            match msg {
                ButtonMessage::Clicked => return ButtonMessage::NoChange,
                _ => return ButtonMessage::NoChange,
            }
        }

        match msg {
            ButtonMessage::Clicked => ButtonMessage::Clicked,
            ButtonMessage::Hovered => {
                self.state = ButtonState::Hover;
                ButtonMessage::Hovered
            }
            ButtonMessage::Unhovered => {
                self.state = ButtonState::Normal;
                ButtonMessage::Unhovered
            }
            ButtonMessage::Pressed => {
                self.state = ButtonState::Active;
                ButtonMessage::Pressed
            }
            ButtonMessage::Released => {
                self.state = ButtonState::Hover;
                ButtonMessage::Released
            }
            ButtonMessage::NoChange => ButtonMessage::NoChange,
        }
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_default_props_via_default() {
        let p = ButtonProps::default();
        assert_eq!(p.button_type, ButtonType::Default);
        assert_eq!(p.size, ButtonSize::Default);
        assert!(!p.disabled);
    }
}
