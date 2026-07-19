//! Steps 步骤条 — 参考 Element Plus `<el-steps>`。
//!
//! 覆盖：基础步骤、当前步骤切换、方向、simple 模式、状态、finish 状态。

use har_ui_components::steps::{Step, StepStatus, Steps, StepsDirection, StepsMessage};

#[test]
fn test_steps_empty() {
    let s = Steps::new();
    assert_eq!(s.steps().len(), 0);
    assert_eq!(s.current(), 0);
}

#[test]
fn test_steps_with_steps() {
    let s = Steps::new()
        .with_step(Step::new("第一步").with_description("开始"))
        .with_step(Step::new("第二步"))
        .with_step(Step::new("完成"));
    assert_eq!(s.steps().len(), 3);
    assert_eq!(s.steps()[0].title(), "第一步");
}

#[test]
fn test_steps_default_direction_horizontal() {
    let s = Steps::new();
    assert_eq!(s.direction(), StepsDirection::Horizontal);
}

#[test]
fn test_steps_vertical_direction() {
    let s = Steps::new().with_direction(StepsDirection::Vertical);
    assert_eq!(s.direction(), StepsDirection::Vertical);
}

#[test]
fn test_steps_current_advances() {
    let mut s = Steps::new()
        .with_step(Step::new("a"))
        .with_step(Step::new("b"))
        .with_step(Step::new("c"));
    assert_eq!(s.current(), 0);

    s.handle(StepsMessage::Next);
    assert_eq!(s.current(), 1);

    s.handle(StepsMessage::Next);
    assert_eq!(s.current(), 2);

    // 末尾不再前进
    s.handle(StepsMessage::Next);
    assert_eq!(s.current(), 2);
}

#[test]
fn test_steps_prev_go_back() {
    let mut s = Steps::new()
        .with_step(Step::new("a"))
        .with_step(Step::new("b"));
    s.handle(StepsMessage::Next);
    assert_eq!(s.current(), 1);
    s.handle(StepsMessage::Prev);
    assert_eq!(s.current(), 0);
    // 起点不回退
    s.handle(StepsMessage::Prev);
    assert_eq!(s.current(), 0);
}

#[test]
fn test_steps_jump_to_index() {
    let mut s = Steps::new()
        .with_step(Step::new("a"))
        .with_step(Step::new("b"))
        .with_step(Step::new("c"));
    s.handle(StepsMessage::JumpTo(2));
    assert_eq!(s.current(), 2);
    s.handle(StepsMessage::JumpTo(99));
    assert_eq!(s.current(), 2); // 越界钳制
}

#[test]
fn test_steps_status_for_each_step() {
    let mut s = Steps::new()
        .with_step(Step::new("a"))
        .with_step(Step::new("b"))
        .with_step(Step::new("c"));
    s.handle(StepsMessage::JumpTo(1));
    // current=1：a 已完成，b 进行中，c 等待
    assert_eq!(s.step_status(0), StepStatus::Finish);
    assert_eq!(s.step_status(1), StepStatus::Process);
    assert_eq!(s.step_status(2), StepStatus::Wait);
}

#[test]
fn test_steps_simple_mode() {
    let s = Steps::new().with_simple(true);
    assert!(s.simple());
}

#[test]
fn test_steps_finish_flag() {
    let mut s = Steps::new()
        .with_step(Step::new("a"))
        .with_step(Step::new("b"));
    assert!(!s.is_finished());
    s.handle(StepsMessage::Finish);
    assert!(s.is_finished());
}

#[test]
fn test_steps_custom_icon() {
    let step = Step::new("a").with_icon("edit");
    assert_eq!(step.icon(), Some("edit"));
}

#[test]
fn test_steps_reset() {
    let mut s = Steps::new()
        .with_step(Step::new("a"))
        .with_step(Step::new("b"));
    s.handle(StepsMessage::Next);
    s.handle(StepsMessage::Finish);
    s.handle(StepsMessage::Reset);
    assert_eq!(s.current(), 0);
    assert!(!s.is_finished());
}
