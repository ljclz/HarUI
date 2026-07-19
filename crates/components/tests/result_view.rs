//! R.2.P1.7 Result view() 测试 — TDD RED 阶段

use har_ui_components::result_page::{ResultPage, ResultType};
use har_ui_core::theme::Theme;

#[test]
fn test_result_view_default_renders() {
    let theme = Theme::element_light();
    let r = ResultPage::new();
    let _element = r.view(&theme);
}

#[test]
fn test_result_view_success_renders() {
    let theme = Theme::element_light();
    let r = ResultPage::new().with_type(ResultType::Success);
    let _element = r.view(&theme);
}

#[test]
fn test_result_view_warning_renders() {
    let theme = Theme::element_light();
    let r = ResultPage::new().with_type(ResultType::Warning);
    let _element = r.view(&theme);
}

#[test]
fn test_result_view_error_renders() {
    let theme = Theme::element_light();
    let r = ResultPage::new().with_type(ResultType::Error);
    let _element = r.view(&theme);
}

#[test]
fn test_result_view_with_title_renders() {
    let theme = Theme::element_light();
    let r = ResultPage::new().with_title("操作成功");
    let _element = r.view(&theme);
}

#[test]
fn test_result_view_with_sub_title_renders() {
    let theme = Theme::element_light();
    let r = ResultPage::new().with_title("OK").with_sub_title("请稍后");
    let _element = r.view(&theme);
}

#[test]
fn test_result_view_with_icon_url_renders() {
    let theme = Theme::element_light();
    let r = ResultPage::new().with_icon_url("/icon.png");
    let _element = r.view(&theme);
}

#[test]
fn test_result_view_with_extra_renders() {
    let theme = Theme::element_light();
    let r = ResultPage::new().with_has_extra(true);
    let _element = r.view(&theme);
}

#[test]
fn test_result_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let r = ResultPage::new().with_type(ResultType::Success);
    let _element = r.view(&theme);
}
