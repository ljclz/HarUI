//! Avatar 头像 — 参考 Element Plus `<el-avatar>`。
//!
//! 覆盖：3 种类型（icon/text/image）、尺寸、形状、fit、fallback。

use har_ui_components::avatar::{Avatar, AvatarFit, AvatarShape, AvatarSize, AvatarSource};

#[test]
fn test_avatar_default() {
    let a = Avatar::new();
    assert_eq!(a.size(), AvatarSize::Default);
    assert_eq!(a.shape(), AvatarShape::Circle);
    assert_eq!(a.fit(), AvatarFit::Cover);
    assert_eq!(a.source(), AvatarSource::Icon);
}

#[test]
fn test_avatar_text_source() {
    let a = Avatar::new().with_text("U");
    assert_eq!(a.source(), AvatarSource::Text);
    assert_eq!(a.text(), Some("U"));
}

#[test]
fn test_avatar_image_source() {
    let a = Avatar::new().with_image_url("/img/user.png");
    assert_eq!(a.source(), AvatarSource::Image);
    assert_eq!(a.image_url(), Some("/img/user.png"));
}

#[test]
fn test_avatar_icon_source() {
    let a = Avatar::new().with_icon("user");
    assert_eq!(a.source(), AvatarSource::Icon);
    assert_eq!(a.icon(), Some("user"));
}

#[test]
fn test_avatar_sizes() {
    let a = Avatar::new().with_size(AvatarSize::Large);
    assert_eq!(a.size(), AvatarSize::Large);
    let a2 = Avatar::new().with_size(AvatarSize::Small);
    assert_eq!(a2.size(), AvatarSize::Small);
}

#[test]
fn test_avatar_custom_pixel_size() {
    let a = Avatar::new().with_pixel_size(80);
    assert_eq!(a.pixel_size(), Some(80));
}

#[test]
fn test_avatar_shape_square() {
    let a = Avatar::new().with_shape(AvatarShape::Square);
    assert_eq!(a.shape(), AvatarShape::Square);
}

#[test]
fn test_avatar_fit_modes() {
    let a = Avatar::new().with_fit(AvatarFit::Contain);
    assert_eq!(a.fit(), AvatarFit::Contain);
    let a2 = Avatar::new().with_fit(AvatarFit::Fill);
    assert_eq!(a2.fit(), AvatarFit::Fill);
}

#[test]
fn test_avatar_fallback_on_error() {
    let mut a = Avatar::new().with_image_url("/img/missing.png")
        .with_fallback_text("U");
    assert_eq!(a.source(), AvatarSource::Image);
    // 模拟图片加载失败
    a.handle_load_error();
    // 自动 fallback 到文本
    assert_eq!(a.source(), AvatarSource::Text);
    assert_eq!(a.text(), Some("U"));
}

#[test]
fn test_avatar_no_fallback_silently_fails() {
    let mut a = Avatar::new().with_image_url("/img/missing.png");
    a.handle_load_error();
    // 无 fallback 时保持 image（外部可见错误态由源标记）
    // 但 has_error 应为 true
    assert!(a.has_error());
}

#[test]
fn test_avatar_initials_helper() {
    // 从 "张三" 取首字 → "张"
    let a = Avatar::new().with_initials("张三");
    assert_eq!(a.text(), Some("张"));
    // 从 "John Doe" 取首字母 → "J"
    let a2 = Avatar::new().with_initials("John Doe");
    assert_eq!(a2.text(), Some("J"));
}

#[test]
fn test_avatar_safe_default_render() {
    let a = Avatar::new();
    // 默认 Icon 类型不 panic
    assert_eq!(a.source(), AvatarSource::Icon);
}
