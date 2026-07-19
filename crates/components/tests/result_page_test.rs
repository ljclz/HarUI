//! Result 结果页 — 参考 Element Plus `<el-result>`。
//!
//! 覆盖：4 种 type、自定义 icon/title/subtitle、额外内容 slot。

use har_ui_components::result_page::{ResultPage, ResultType};

#[test]
fn test_result_default() {
    let r = ResultPage::new();
    assert_eq!(r.result_type(), ResultType::Info);
    assert_eq!(r.title(), "提示");
    assert_eq!(r.sub_title(), None);
    assert!(!r.has_extra());
}

#[test]
fn test_result_success_type() {
    let r = ResultPage::new().with_type(ResultType::Success).with_title("操作成功");
    assert_eq!(r.result_type(), ResultType::Success);
    assert_eq!(r.title(), "操作成功");
}

#[test]
fn test_result_warning_type() {
    let r = ResultPage::new().with_type(ResultType::Warning);
    assert_eq!(r.result_type(), ResultType::Warning);
}

#[test]
fn test_result_error_type() {
    let r = ResultPage::new().with_type(ResultType::Error).with_title("提交失败");
    assert_eq!(r.result_type(), ResultType::Error);
    assert_eq!(r.title(), "提交失败");
}

#[test]
fn test_result_custom_subtitle() {
    let r = ResultPage::new().with_sub_title("请稍后重试");
    assert_eq!(r.sub_title(), Some("请稍后重试"));
}

#[test]
fn test_result_custom_icon_url() {
    let r = ResultPage::new().with_icon_url("/img/custom.png");
    assert_eq!(r.icon_url(), Some("/img/custom.png"));
}

#[test]
fn test_result_extra_slot() {
    let r = ResultPage::new().with_has_extra(true);
    assert!(r.has_extra());
}

#[test]
fn test_result_safe_default_title_per_type() {
    // 每种 type 切换后标题默认值不变（除非显式设置）
    let s = ResultPage::new().with_type(ResultType::Success);
    assert_eq!(s.title(), "提示"); // 标题独立于 type
}
