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
