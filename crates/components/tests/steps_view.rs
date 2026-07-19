//! R.2.P1.13 Steps view() 测试 — TDD RED 阶段

use har_ui_components::steps::{Step, Steps, StepsDirection, StepsMessage};
use har_ui_core::theme::Theme;

#[test]
fn test_steps_view_empty_renders() {
    let theme = Theme::element_light();
    let s = Steps::new();
    let _element = s.view(&theme);
}

#[test]
fn test_steps_view_default_renders() {
    let theme = Theme::element_light();
    let s = Steps::new()
        .with_step(Step::new("Step 1"))
        .with_step(Step::new("Step 2"))
        .with_step(Step::new("Step 3"));
    let _element = s.view(&theme);
}

#[test]
fn test_steps_view_with_description_renders() {
    let theme = Theme::element_light();
    let s = Steps::new()
        .with_step(Step::new("Step 1").with_description("desc 1"))
        .with_step(Step::new("Step 2").with_description("desc 2"));
    let _element = s.view(&theme);
}

#[test]
fn test_steps_view_with_icon_renders() {
    let theme = Theme::element_light();
    let s = Steps::new()
        .with_step(Step::new("Step 1").with_icon("✓"))
        .with_step(Step::new("Step 2"));
    let _element = s.view(&theme);
}

#[test]
fn test_steps_view_current_step_renders() {
    let theme = Theme::element_light();
    let mut s = Steps::new()
        .with_step(Step::new("Step 1"))
        .with_step(Step::new("Step 2"))
        .with_step(Step::new("Step 3"));
    s.handle(StepsMessage::Next);
    let _element = s.view(&theme);
}

#[test]
fn test_steps_view_finished_renders() {
    let theme = Theme::element_light();
    let mut s = Steps::new()
        .with_step(Step::new("Step 1"))
        .with_step(Step::new("Step 2"));
    s.handle(StepsMessage::Finish);
    let _element = s.view(&theme);
}

#[test]
fn test_steps_view_jump_to_renders() {
    let theme = Theme::element_light();
    let mut s = Steps::new()
        .with_step(Step::new("Step 1"))
        .with_step(Step::new("Step 2"))
        .with_step(Step::new("Step 3"))
        .with_step(Step::new("Step 4"));
    s.handle(StepsMessage::JumpTo(2));
    let _element = s.view(&theme);
}

#[test]
fn test_steps_view_vertical_direction_renders() {
    let theme = Theme::element_light();
    let s = Steps::new()
        .with_direction(StepsDirection::Vertical)
        .with_step(Step::new("Step 1"))
        .with_step(Step::new("Step 2"));
    let _element = s.view(&theme);
}

#[test]
fn test_steps_view_simple_mode_renders() {
    let theme = Theme::element_light();
    let s = Steps::new()
        .with_simple(true)
        .with_step(Step::new("Step 1"))
        .with_step(Step::new("Step 2"));
    let _element = s.view(&theme);
}

#[test]
fn test_steps_view_reset_renders() {
    let theme = Theme::element_light();
    let mut s = Steps::new()
        .with_step(Step::new("Step 1"))
        .with_step(Step::new("Step 2"));
    s.handle(StepsMessage::Next);
    s.handle(StepsMessage::Finish);
    s.handle(StepsMessage::Reset);
    let _element = s.view(&theme);
}

#[test]
fn test_steps_view_dark_theme_renders() {
    let theme = Theme::element_dark();
    let s = Steps::new()
        .with_step(Step::new("Step 1"))
        .with_step(Step::new("Step 2"));
    let _element = s.view(&theme);
}
