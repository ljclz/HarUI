//! R.2.P1.4 Progress view() 测试 — TDD RED 阶段

use har_ui_components::progress::{Progress, ProgressMessage, ProgressStatus, ProgressType};
use har_ui_core::theme::Theme;

#[test]
fn test_progress_view_default_renders() {
    let theme = Theme::element_light();
    let p = Progress::new();
    let _element = p.view(&theme);
}

#[test]
fn test_progress_view_with_percentage_renders() {
    let theme = Theme::element_light();
    let mut p = Progress::new();
    p.handle(ProgressMessage::SetPercentage(50));
    let _element = p.view(&theme);
}

#[test]
fn test_progress_view_full_renders() {
    let theme = Theme::element_light();
    let mut p = Progress::new();
    p.handle(ProgressMessage::SetPercentage(100));
    let _element = p.view(&theme);
}

#[test]
fn test_progress_view_circle_type_renders() {
    let theme = Theme::element_light();
    let mut p = Progress::new().with_type(ProgressType::Circle);
    p.handle(ProgressMessage::SetPercentage(60));
    let _element = p.view(&theme);
}

#[test]
fn test_progress_view_success_status_renders() {
    let theme = Theme::element_light();
    let p = Progress::new().with_status(ProgressStatus::Success);
    let _element = p.view(&theme);
}

#[test]
fn test_progress_view_exception_status_renders() {
    let theme = Theme::element_light();
    let p = Progress::new().with_status(ProgressStatus::Exception);
    let _element = p.view(&theme);
}

#[test]
fn test_progress_view_no_text_renders() {
    let theme = Theme::element_light();
    let p = Progress::new().with_show_text(false);
    let _element = p.view(&theme);
}

#[test]
fn test_progress_view_custom_color_renders() {
    let theme = Theme::element_light();
    let p = Progress::new().with_color("#13CE66");
    let _element = p.view(&theme);
}

#[test]
fn test_progress_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let p = Progress::new();
    let _element = p.view(&theme);
}
