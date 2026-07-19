//! R.2.P1.20 Slider view() 测试

use har_ui_components::slider::{Slider, SliderMessage};
use har_ui_core::theme::Theme;

#[test]
fn test_slider_view_default_renders() {
    let theme = Theme::element_light();
    let s = Slider::new();
    let _element = s.view(&theme, |_| ());
}

#[test]
fn test_slider_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let s = Slider::new();
    let _element = s.view(&theme, |_| ());
}

#[test]
fn test_slider_view_with_value_renders() {
    let theme = Theme::element_light();
    let mut s = Slider::new();
    s.handle(SliderMessage::SetValue(50.0));
    let _element = s.view(&theme, |_| ());
}

#[test]
fn test_slider_view_at_min_renders() {
    let theme = Theme::element_light();
    let s = Slider::new().with_min(0.0).with_max(100.0);
    let _element = s.view(&theme, |_| ());
}

#[test]
fn test_slider_view_at_max_renders() {
    let theme = Theme::element_light();
    let mut s = Slider::new().with_max(100.0);
    s.handle(SliderMessage::SetValue(100.0));
    let _element = s.view(&theme, |_| ());
}

#[test]
fn test_slider_view_disabled_renders() {
    let theme = Theme::element_light();
    let s = Slider::new().with_disabled(true);
    let _element = s.view(&theme, |_| ());
}

#[test]
fn test_slider_view_show_input_renders() {
    let theme = Theme::element_light();
    let s = Slider::new().with_show_input(true);
    let _element = s.view(&theme, |_| ());
}

#[test]
fn test_slider_view_show_stops_renders() {
    let theme = Theme::element_light();
    let s = Slider::new().with_show_stops(true).with_step(10.0);
    let _element = s.view(&theme, |_| ());
}

#[test]
fn test_slider_view_vertical_renders() {
    let theme = Theme::element_light();
    let s = Slider::new().with_vertical(true);
    let _element = s.view(&theme, |_| ());
}

#[test]
fn test_slider_view_with_step_renders() {
    let theme = Theme::element_light();
    let s = Slider::new().with_min(0.0).with_max(100.0).with_step(5.0);
    let _element = s.view(&theme, |_| ());
}

#[test]
fn test_slider_view_zero_span_renders() {
    let theme = Theme::element_light();
    let s = Slider::new().with_min(50.0).with_max(50.0);
    let _element = s.view(&theme, |_| ());
}

#[test]
fn test_slider_view_negative_range_renders() {
    let theme = Theme::element_light();
    let s = Slider::new().with_min(-50.0).with_max(50.0);
    let _element = s.view(&theme, |_| ());
}

#[test]
fn test_slider_view_show_tooltip_renders() {
    let theme = Theme::element_light();
    let s = Slider::new().with_show_tooltip(false);
    let _element = s.view(&theme, |_| ());
}

#[test]
fn test_slider_view_with_custom_message_type() {
    let theme = Theme::element_light();
    let s = Slider::new();
    #[derive(Clone, Debug)]
    #[allow(dead_code)]
    enum AppMsg {
        Change(f64),
    }
    let _element = s.view(&theme, |v| AppMsg::Change(v));
}
