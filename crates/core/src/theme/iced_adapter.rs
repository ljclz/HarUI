//! iced 适配层 — 把 har-ui Theme 映射为 iced::Theme
//!
//! iced 0.13.1 的 widget style 闭包签名为 `Fn(&iced::Theme, Status) -> Style`。
//! 本模块提供 `to_iced_theme(&Theme) -> iced::Theme`，把 har-ui 5 调色板 + 中性色
//! 映射成 `iced::Theme::custom(Palette)`，让 iced widget 闭包能通过
//! `iced::Theme::palette()` / `.extended_palette()` 拿到颜色。
//!
//! 注意：`iced::theme::Palette` 只有 5 个字段（background/text/primary/success/danger），
//! 没有 warning/info/中性色细节。完整的 har-ui 颜色访问请使用
//! [`style_sheets`](super::style_sheets) 模块的 style 函数（直接接受 `&Theme`）。

use crate::theme::Theme;
use iced::Color;
use iced::theme::Palette;

/// 把 har-ui Theme 转换为 iced::Theme（custom 变体）
///
/// 映射规则：
/// - `background` ← `neutral.bg_overlay`（浅色）/ `neutral.bg_page`（深色）
/// - `text` ← `neutral.text_primary`
/// - `primary` ← `primary.base`
/// - `success` ← `success.base`
/// - `danger` ← `danger.base`
///
/// # 示例
/// ```no_run
/// use har_ui_core::Theme;
/// use har_ui_core::theme::iced_adapter;
/// let t = Theme::element_light();
/// let _iced_theme = iced_adapter::to_iced_theme(&t);
/// ```
pub fn to_iced_theme(theme: &Theme) -> iced::Theme {
    let bg = if theme.is_dark {
        theme.neutral.bg_page
    } else {
        theme.neutral.bg_overlay
    };

    let palette = Palette {
        background: Color::from(bg),
        text: Color::from(theme.neutral.text_primary),
        primary: Color::from(theme.primary.base),
        success: Color::from(theme.success.base),
        warning: Color::from(theme.warning.base),
        danger: Color::from(theme.danger.base),
    };

    let name = if theme.is_dark {
        "HarUI Dark"
    } else {
        "HarUI Light"
    };

    iced::Theme::custom(name.to_string(), palette)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_test_to_iced_theme_returns_custom() {
        let t = Theme::element_light();
        let iced_theme = to_iced_theme(&t);
        let display = format!("{}", iced_theme);
        assert!(display.contains("HarUI"));
    }
}
