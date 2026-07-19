//! HarUI 图标系统测试

use har_ui_core::icon::{Icon, IconName, IconLibrary};

#[test]
fn test_icon_name_variants() {
    // 至少 40 个 POS 常用图标
    let icons = vec![
        IconName::Plus, IconName::Minus, IconName::Delete, IconName::Edit,
        IconName::Search, IconName::Refresh, IconName::Print, IconName::Setting,
        IconName::User, IconName::WeChat, IconName::Alipay, IconName::UnionPay,
        IconName::Cash, IconName::BankCard, IconName::Coupon,
        IconName::HangOrder, IconName::TakeOrder, IconName::Clear,
        IconName::Back, IconName::Close, IconName::Expand, IconName::Fold,
        IconName::Success, IconName::Warning, IconName::Error, IconName::Info,
        IconName::Loading, IconName::More, IconName::Upload, IconName::Image,
        IconName::File, IconName::Data, IconName::Statistic,
        IconName::Calendar, IconName::Clock, IconName::Filter,
        IconName::ArrowLeft, IconName::ArrowRight, IconName::ArrowUp, IconName::ArrowDown,
    ];
    assert!(icons.len() >= 40, "至少 40 个图标，实际 {}", icons.len());
}

#[test]
fn test_icon_library_contains_all_names() {
    let lib = IconLibrary::default();
    // 测试每个 IconName 都能返回非空 SVG
    for name in IconName::all() {
        let svg = lib.get(name);
        assert!(!svg.is_empty(), "icon {:?} should have svg data", name);
        assert!(svg.starts_with("<svg"), "icon {:?} svg should start with <svg", name);
    }
}

#[test]
fn test_icon_library_size() {
    let lib = IconLibrary::default();
    assert!(lib.len() >= 40, "图标库至少 40 个，实际 {}", lib.len());
}

#[test]
fn test_icon_creation() {
    let icon = Icon::new(IconName::Search);
    assert_eq!(icon.name, IconName::Search);
    assert_eq!(icon.size, 16.0);  // 默认 16px
    assert!(!icon.loading);  // 默认不旋转
}

#[test]
fn test_icon_with_size() {
    let icon = Icon::new(IconName::Success).size(24.0);
    assert_eq!(icon.size, 24.0);
}

#[test]
fn test_icon_with_loading() {
    let icon = Icon::new(IconName::Loading).loading(true);
    assert!(icon.loading);
}

#[test]
fn test_icon_svg_data_not_empty() {
    let lib = IconLibrary::default();
    let search_svg = lib.get(IconName::Search);
    assert!(search_svg.contains("svg"));
    assert!(search_svg.len() > 50, "svg should not be empty, len={}", search_svg.len());
}

#[test]
fn test_icon_all_names_unique() {
    let all = IconName::all();
    let mut seen = std::collections::HashSet::new();
    for name in &all {
        assert!(seen.insert(name), "duplicate icon name: {:?}", name);
    }
}

#[test]
fn test_icon_svg_contains_path() {
    let lib = IconLibrary::default();
    for name in IconName::all() {
        let svg = lib.get(name);
        assert!(
            svg.contains("<path") || svg.contains("<circle") || svg.contains("<rect") || svg.contains("<line") || svg.contains("<polygon"),
            "icon {:?} svg should contain a drawable element, got: {}",
            name,
            &svg[..svg.len().min(100)]
        );
    }
}
