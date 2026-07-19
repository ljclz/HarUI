//! Slider 滑块 — 参考 Element Plus `<el-slider>`。
//!
//! 覆盖：min/max/step、value、disabled、vertical、show_input、range、increase/decrease。

use har_ui_components::slider::{Slider, SliderMessage};

#[test]
fn test_slider_default() {
    let s = Slider::new();
    assert_eq!(s.min(), 0.0);
    assert_eq!(s.max(), 100.0);
    assert_eq!(s.step(), 1.0);
    assert_eq!(s.value(), 0.0);
    assert!(!s.disabled());
    assert!(!s.vertical());
    assert!(!s.show_input());
    assert!(!s.range());
}

#[test]
fn test_slider_with_min_max() {
    let s = Slider::new().with_min(10.0).with_max(50.0);
    assert_eq!(s.min(), 10.0);
    assert_eq!(s.max(), 50.0);
}

#[test]
fn test_slider_with_step() {
    let s = Slider::new().with_step(5.0);
    assert_eq!(s.step(), 5.0);
}

#[test]
fn test_slider_set_value() {
    let mut s = Slider::new();
    s.handle(SliderMessage::SetValue(42.0));
    assert_eq!(s.value(), 42.0);
}

#[test]
fn test_slider_clamp_to_min() {
    let mut s = Slider::new().with_min(10.0).with_max(100.0);
    s.handle(SliderMessage::SetValue(5.0));
    assert_eq!(s.value(), 10.0);
}

#[test]
fn test_slider_clamp_to_max() {
    let mut s = Slider::new().with_min(0.0).with_max(100.0);
    s.handle(SliderMessage::SetValue(150.0));
    assert_eq!(s.value(), 100.0);
}

#[test]
fn test_slider_set_value_snaps_to_step() {
    let mut s = Slider::new().with_min(0.0).with_max(100.0).with_step(10.0);
    s.handle(SliderMessage::SetValue(23.0));
    // 23 应吸附到最近的 10 的倍数 20
    assert_eq!(s.value(), 20.0);
}

#[test]
fn test_slider_disabled_noop() {
    let mut s = Slider::new().with_disabled(true);
    s.handle(SliderMessage::SetValue(50.0));
    assert_eq!(s.value(), 0.0);
}

#[test]
fn test_slider_with_vertical() {
    let s = Slider::new().with_vertical(true);
    assert!(s.vertical());
}

#[test]
fn test_slider_with_show_input() {
    let s = Slider::new().with_show_input(true);
    assert!(s.show_input());
}

#[test]
fn test_slider_increase() {
    let mut s = Slider::new().with_step(5.0).with_max(20.0);
    s.handle(SliderMessage::SetValue(5.0));
    s.handle(SliderMessage::Increase);
    assert_eq!(s.value(), 10.0);
}

#[test]
fn test_slider_decrease() {
    let mut s = Slider::new().with_step(5.0).with_min(0.0);
    s.handle(SliderMessage::SetValue(10.0));
    s.handle(SliderMessage::Decrease);
    assert_eq!(s.value(), 5.0);
}

#[test]
fn test_slider_increase_clamp_at_max() {
    let mut s = Slider::new().with_step(10.0).with_max(15.0);
    s.handle(SliderMessage::SetValue(10.0));
    s.handle(SliderMessage::Increase);
    // 10 + 10 = 20，应钳制到 max=15
    assert_eq!(s.value(), 15.0);
}

#[test]
fn test_slider_decrease_clamp_at_min() {
    let mut s = Slider::new().with_step(10.0).with_min(5.0);
    s.handle(SliderMessage::SetValue(10.0));
    s.handle(SliderMessage::Decrease);
    // 10 - 10 = 0，应钳制到 min=5
    assert_eq!(s.value(), 5.0);
}

#[test]
fn test_slider_range_set() {
    let mut s = Slider::new().with_range(true);
    s.handle(SliderMessage::SetRange(20.0, 80.0));
    assert_eq!(s.range_value(), (20.0, 80.0));
}

#[test]
fn test_slider_range_clamp_order() {
    let mut s = Slider::new().with_range(true).with_min(0.0).with_max(100.0);
    // 左 > 右时应自动交换
    s.handle(SliderMessage::SetRange(80.0, 20.0));
    assert_eq!(s.range_value(), (20.0, 80.0));
}

#[test]
fn test_slider_range_clamp_to_bounds() {
    let mut s = Slider::new().with_range(true).with_min(10.0).with_max(90.0);
    s.handle(SliderMessage::SetRange(0.0, 100.0));
    assert_eq!(s.range_value(), (10.0, 90.0));
}
