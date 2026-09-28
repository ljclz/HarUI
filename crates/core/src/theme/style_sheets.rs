//! 组件 style 函数集 — R.1 阶段实现 button/input/container 三个基础 style
//!
//! 这些函数接受 [`Theme`] 引用，输出 `iced::widget::xxx::Style`，
//! 被 widget 的 `.style(move |_t, status| ...)` 闭包调用。
//!
//! R.2 阶段每个组件 view() 实现时，会按需补齐其他组件的 style 函数到本模块。

use crate::theme::Theme;
use crate::theme::color::ColorPalette;
use iced::widget::{button, container, text_input};
use iced::{Background, Border, Color, Shadow};

/// Button 视觉类型
///
/// 与 `components::button::ButtonType` 对应，但 style_sheets 模块独立定义此 enum，
/// 避免核心 crate 依赖组件 crate 造成循环依赖。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonKind {
    Default,
    Primary,
    Success,
    Warning,
    Danger,
    Info,
    Text,
    Link,
}

/// 计算 `button::Style`
///
/// 根据 [`ButtonKind`] + `plain` + [`button::Status`] 输出 Element Plus 风格的样式：
/// - 彩色按钮（Primary/Success/Warning/Danger/Info）：背景 = 调色板 base，文字白色
/// - Default：白底，文字 text_regular，边框 border_base
/// - Text/Link：透明背景，文字 = primary
/// - Disabled：所有颜色 alpha=0.5
/// - Plain：透明背景，文字/边框 = 调色板 base
pub fn button_style(
    theme: &Theme,
    kind: ButtonKind,
    plain: bool,
    status: button::Status,
) -> button::Style {
    let disabled = matches!(status, button::Status::Disabled);

    match kind {
        ButtonKind::Primary => colored(theme, &theme.primary, plain, status, disabled),
        ButtonKind::Success => colored(theme, &theme.success, plain, status, disabled),
        ButtonKind::Warning => colored(theme, &theme.warning, plain, status, disabled),
        ButtonKind::Danger => colored(theme, &theme.danger, plain, status, disabled),
        ButtonKind::Info => colored(theme, &theme.info, plain, status, disabled),
        ButtonKind::Default => default_button(theme, status, disabled),
        ButtonKind::Text => text_button(theme, status, disabled),
        ButtonKind::Link => link_button(theme, status, disabled),
    }
}

fn colored(
    _theme: &Theme,
    palette: &ColorPalette,
    plain: bool,
    status: button::Status,
    disabled: bool,
) -> button::Style {
    let base = Color::from(palette.base);
    if plain {
        let alpha = if disabled { 0.5 } else { 1.0 };
        let tinted = Color { a: alpha, ..base };
        return button::Style {
            background: Some(Background::Color(Color { a: 0.0, ..base })),
            text_color: tinted,
            border: Border {
                color: tinted,
                width: 1.0,
                radius: iced::border::Radius::default(),
            },
            shadow: Shadow::default(),
        };
    }
    if disabled {
        return button::Style {
            background: Some(Background::Color(Color { a: 0.5, ..base })),
            text_color: Color {
                a: 0.7,
                ..Color::WHITE
            },
            border: Border::default(),
            shadow: Shadow::default(),
        };
    }
    let bg = match status {
        button::Status::Hovered => Color::from(palette.light(3)),
        button::Status::Pressed => Color::from(palette.dark(2)),
        _ => base,
    };
    button::Style {
        background: Some(Background::Color(bg)),
        text_color: Color::WHITE,
        border: Border::default(),
        shadow: Shadow::default(),
    }
}

fn default_button(theme: &Theme, status: button::Status, disabled: bool) -> button::Style {
    let (bg, text_color, border_color) = if disabled {
        (
            Some(Background::Color(Color {
                a: 0.5,
                ..Color::from(theme.neutral.bg_base)
            })),
            Color::from(theme.neutral.text_disabled),
            Color::from(theme.neutral.border_light),
        )
    } else {
        let bg = match status {
            button::Status::Hovered => Some(Background::Color(Color {
                a: 0.1,
                ..Color::from(theme.primary.base)
            })),
            _ => Some(Background::Color(Color::from(theme.neutral.bg_overlay))),
        };
        (
            bg,
            Color::from(theme.neutral.text_regular),
            Color::from(theme.neutral.border_base),
        )
    };
    button::Style {
        background: bg,
        text_color,
        border: Border {
            color: border_color,
            width: 1.0,
            radius: iced::border::Radius::default(),
        },
        shadow: Shadow::default(),
    }
}

fn text_button(theme: &Theme, status: button::Status, disabled: bool) -> button::Style {
    let alpha = if disabled { 0.5 } else { 1.0 };
    let text_color = if matches!(status, button::Status::Hovered) {
        Color::from(theme.primary.light(3))
    } else {
        Color::from(theme.primary.base)
    };
    button::Style {
        background: None,
        text_color: Color {
            a: alpha,
            ..text_color
        },
        border: Border::default(),
        shadow: Shadow::default(),
    }
}

fn link_button(theme: &Theme, status: button::Status, disabled: bool) -> button::Style {
    text_button(theme, status, disabled)
}

/// Input（text_input）style
///
/// - Active/Hovered：bg_overlay 背景，border_base 边框
/// - Focused：bg_overlay 背景，primary.base 边框
/// - Disabled：bg_base 背景，border_light 边框
pub fn input_style(theme: &Theme, status: text_input::Status) -> text_input::Style {
    let (bg, border_color, border_width) = match status {
        text_input::Status::Focused => (
            Color::from(theme.neutral.bg_overlay),
            Color::from(theme.primary.base),
            1.0,
        ),
        text_input::Status::Disabled => (
            Color::from(theme.neutral.bg_base),
            Color::from(theme.neutral.border_light),
            1.0,
        ),
        _ => (
            Color::from(theme.neutral.bg_overlay),
            Color::from(theme.neutral.border_base),
            1.0,
        ),
    };
    text_input::Style {
        background: Background::Color(bg),
        border: Border {
            color: border_color,
            width: border_width,
            radius: iced::border::Radius::default(),
        },
        icon: Color::from(theme.neutral.text_secondary),
        placeholder: Color::from(theme.neutral.text_placeholder),
        value: Color::from(theme.neutral.text_regular),
        selection: Color::from(theme.primary.base),
    }
}

/// Container style：Card（带阴影边框）
pub fn container_card_style(theme: &Theme) -> container::Style {
    container::Style {
        text_color: Some(Color::from(theme.neutral.text_primary)),
        background: Some(Background::Color(Color::from(theme.neutral.bg_overlay))),
        border: Border {
            color: Color::from(theme.neutral.border_lighter),
            width: 1.0,
            radius: iced::border::Radius::default(),
        },
        shadow: Shadow {
            color: Color {
                a: 0.1,
                ..Color::BLACK
            },
            offset: iced::Vector::new(0.0, 2.0),
            blur_radius: 8.0,
        },
    }
}

/// Container style：Dialog 遮罩（半透明黑）
pub fn container_dialog_mask_style(_theme: &Theme) -> container::Style {
    container::Style {
        text_color: None,
        background: Some(Background::Color(Color {
            a: 0.5,
            ..Color::BLACK
        })),
        border: Border::default(),
        shadow: Shadow::default(),
    }
}

/// Container style：默认（无背景无边框）
pub fn container_default_style(_theme: &Theme) -> container::Style {
    container::Style::default()
}

/// ShadowSpec → iced::Shadow 通用映射（气泡/弹层组件共用）
///
/// 供 popconfirm/popover/tooltip 等组件从 [`Theme`] 的阴影令牌
/// （base/light/lighter/dark 四档，对应 Element Plus box-shadow 变量）
/// 生成 iced 阴影，替代各组件散落的硬编码阴影。
pub fn iced_shadow(spec: &crate::theme::shadow::ShadowSpec) -> Shadow {
    Shadow {
        color: Color::from(spec.color),
        offset: iced::Vector::new(spec.offset_x, spec.offset_y),
        blur_radius: spec.blur,
    }
}
