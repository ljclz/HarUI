//! Text view() 测试

use har_ui_components::text::{Text, TextSize, TextTag, TextType};
use har_ui_core::theme::Theme;

#[test]
fn test_text_view_default_renders() {
    let theme = Theme::element_light();
    let t = Text::new().with_content("普通文本");
    let _element = t.view(&theme);
}

#[test]
fn test_text_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let t = Text::new().with_content("深色文本");
    let _element = t.view(&theme);
}

#[test]
fn test_text_view_primary_type_renders() {
    let theme = Theme::element_light();
    let t = Text::new()
        .with_content("主要")
        .with_type(TextType::Primary);
    let _element = t.view(&theme);
}

#[test]
fn test_text_view_success_type_renders() {
    let theme = Theme::element_light();
    let t = Text::new()
        .with_content("成功")
        .with_type(TextType::Success);
    let _element = t.view(&theme);
}

#[test]
fn test_text_view_warning_type_renders() {
    let theme = Theme::element_light();
    let t = Text::new()
        .with_content("警告")
        .with_type(TextType::Warning);
    let _element = t.view(&theme);
}

#[test]
fn test_text_view_danger_type_renders() {
    let theme = Theme::element_light();
    let t = Text::new()
        .with_content("危险")
        .with_type(TextType::Danger);
    let _element = t.view(&theme);
}

#[test]
fn test_text_view_info_type_renders() {
    let theme = Theme::element_light();
    let t = Text::new()
        .with_content("信息")
        .with_type(TextType::Info);
    let _element = t.view(&theme);
}

#[test]
fn test_text_view_large_size_renders() {
    let theme = Theme::element_light();
    let t = Text::new()
        .with_content("大号")
        .with_size(TextSize::Large);
    let _element = t.view(&theme);
}

#[test]
fn test_text_view_small_size_renders() {
    let theme = Theme::element_light();
    let t = Text::new()
        .with_content("小号")
        .with_size(TextSize::Small);
    let _element = t.view(&theme);
}

#[test]
fn test_text_view_truncated_renders() {
    let theme = Theme::element_light();
    let t = Text::new()
        .with_content("截断文本")
        .with_truncated(true);
    let _element = t.view(&theme);
}

#[test]
fn test_text_view_copyable_renders() {
    let theme = Theme::element_light();
    let t = Text::new()
        .with_content("可复制")
        .with_copyable(true);
    let _element = t.view(&theme);
}

#[test]
fn test_text_view_h1_tag_renders() {
    let theme = Theme::element_light();
    let t = Text::new()
        .with_content("标题1")
        .with_tag(TextTag::H1);
    let _element = t.view(&theme);
}

#[test]
fn test_text_view_h2_h3_tags_renders() {
    let theme = Theme::element_light();
    let t2 = Text::new()
        .with_content("标题2")
        .with_tag(TextTag::H2);
    let t3 = Text::new()
        .with_content("标题3")
        .with_tag(TextTag::H3);
    let _e2 = t2.view(&theme);
    let _e3 = t3.view(&theme);
}

#[test]
fn test_text_view_paragraph_tag_renders() {
    let theme = Theme::element_light();
    let t = Text::new()
        .with_content("段落")
        .with_tag(TextTag::P);
    let _element = t.view(&theme);
}

#[test]
fn test_text_view_div_tag_renders() {
    let theme = Theme::element_light();
    let t = Text::new()
        .with_content("块级")
        .with_tag(TextTag::Div);
    let _element = t.view(&theme);
}

#[test]
fn test_text_view_empty_content_renders() {
    let theme = Theme::element_light();
    let t = Text::new();
    let _element = t.view(&theme);
}

#[test]
fn test_text_view_with_max_lines_renders() {
    let theme = Theme::element_light();
    let t = Text::new()
        .with_content("多行文本")
        .with_max_lines(3);
    let _element = t.view(&theme);
}
