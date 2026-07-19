//! D.14 截图测试 — 组件渲染验证
//!
//! 本测试验证所有纯展示组件能在 light/dark 两种主题下成功渲染（不实际截图）。
//! 真机像素级截图测试需运行 `cargo run -p screenshot_test` 手动进行，
//! 详见 `docs/screenshot_test.md`。

use har_ui_components::alert::Alert;
use har_ui_components::badge::{Badge, BadgeValue};
use har_ui_components::button::{Button, ButtonType};
use har_ui_components::card::Card;
use har_ui_components::divider::Divider;
use har_ui_components::empty::Empty;
use har_ui_components::link::Link;
use har_ui_components::progress::{Progress, ProgressType};
use har_ui_components::tag::{Tag, TagType};
use har_ui_components::text::Text;
use har_ui_core::theme::Theme;

#[test]
fn test_all_components_render_in_light_theme() {
    let theme = Theme::element_light();
    let _ = Button::new("test").view(&theme, ());
    let _ = Card::new("body").view(&theme);
    let _ = Empty::new().view(&theme);
    let _ = Tag::new("tag").view(&theme, || ());
    let _ = Badge::new(BadgeValue::Number(5)).view(&theme);
    let _ = Progress::new().view(&theme);
    let _ = Divider::new().view(&theme);
    let _ = Alert::new().with_title("alert").view(&theme, || ());
    let _ = Link::new().with_text("link").view(&theme, ());
    let _ = Text::new().with_content("text").view(&theme);
}

#[test]
fn test_all_components_render_in_dark_theme() {
    let theme = Theme::element_dark();
    let _ = Button::new("test").view(&theme, ());
    let _ = Card::new("body").view(&theme);
    let _ = Empty::new().view(&theme);
    let _ = Tag::new("tag").view(&theme, || ());
    let _ = Badge::new(BadgeValue::Number(5)).view(&theme);
    let _ = Progress::new().view(&theme);
    let _ = Divider::new().view(&theme);
    let _ = Alert::new().with_title("alert").view(&theme, || ());
    let _ = Link::new().with_text("link").view(&theme, ());
    let _ = Text::new().with_content("text").view(&theme);
}

#[test]
fn test_button_all_types_render_in_both_themes() {
    let types = [
        ButtonType::Default,
        ButtonType::Primary,
        ButtonType::Success,
        ButtonType::Warning,
        ButtonType::Danger,
        ButtonType::Info,
    ];
    for theme in [Theme::element_light(), Theme::element_dark()] {
        for t in types {
            let btn = Button::new("test").with_type(t);
            let _ = btn.view(&theme, ());
        }
    }
}

#[test]
fn test_tag_all_types_render_in_both_themes() {
    let types = [
        TagType::Default,
        TagType::Primary,
        TagType::Success,
        TagType::Warning,
        TagType::Danger,
        TagType::Info,
    ];
    for theme in [Theme::element_light(), Theme::element_dark()] {
        for t in types {
            let tag = Tag::new("tag").with_type(t);
            let _ = tag.view(&theme, || ());
        }
    }
}

#[test]
fn test_progress_all_types_render_in_both_themes() {
    for theme in [Theme::element_light(), Theme::element_dark()] {
        let _ = Progress::new().with_type(ProgressType::Line).view(&theme);
        let _ = Progress::new().with_type(ProgressType::Circle).view(&theme);
        let _ = Progress::new().with_type(ProgressType::Dashboard).view(&theme);
    }
}
