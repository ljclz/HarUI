//! Form 组件 — 表单
//!
//! 参考 Element Plus `<el-form>`。
//! 支持：label-position(left/right/top)、label-width、rules 验证、required、
//! 自定义 validator、inline 模式、表单重置、异步校验。

use har_ui_components::form::{Form, FormItem, FormRule, FormState, ValidateTrigger};

// ---------- FormRule ----------

#[test]
fn test_form_rule_required() {
    let rule = FormRule::new("name").required(true);
    assert!(rule.required);
    assert_eq!(rule.field, "name");
}

#[test]
fn test_form_rule_with_message() {
    let rule = FormRule::new("name").required(true).with_message("姓名必填");
    assert_eq!(rule.message, Some("姓名必填".to_string()));
}

#[test]
fn test_form_rule_with_min_max() {
    let rule = FormRule::new("age").with_min(1).with_max(120);
    assert_eq!(rule.min, Some(1));
    assert_eq!(rule.max, Some(120));
}

#[test]
fn test_form_rule_with_trigger() {
    let rule = FormRule::new("name").with_trigger(ValidateTrigger::Blur);
    assert_eq!(rule.trigger, ValidateTrigger::Blur);
}

// ---------- FormItem ----------

#[test]
fn test_form_item_default() {
    let item = FormItem::new("name", "姓名");
    assert_eq!(item.field(), "name");
    assert_eq!(item.label(), "姓名");
    assert!(!item.required());
    assert!(item.rules().is_empty());
}

#[test]
fn test_form_item_with_rules() {
    let item = FormItem::new("name", "姓名")
        .with_rule(FormRule::new("name").required(true))
        .with_rule(FormRule::new("name").with_min(2).with_max(20));
    assert_eq!(item.rules().len(), 2);
}

#[test]
fn test_form_item_required_flag() {
    let item = FormItem::new("name", "姓名").set_required(true);
    assert!(item.required());
}

// ---------- Form ----------

#[test]
fn test_form_default() {
    let f = Form::new();
    assert!(!f.inline());
    assert_eq!(f.label_position(), "right");
    assert_eq!(f.label_width(), None);
    assert!(f.items().is_empty());
    assert_eq!(f.state(), FormState::Idle);
}

#[test]
fn test_form_with_inline() {
    let f = Form::new().with_inline(true);
    assert!(f.inline());
}

#[test]
fn test_form_with_label_position() {
    let f = Form::new().with_label_position("top");
    assert_eq!(f.label_position(), "top");
}

#[test]
fn test_form_with_label_width() {
    let f = Form::new().with_label_width(120);
    assert_eq!(f.label_width(), Some(120));
}

#[test]
fn test_form_with_items() {
    let f = Form::new()
        .with_item(FormItem::new("name", "姓名"))
        .with_item(FormItem::new("age", "年龄"));
    assert_eq!(f.items().len(), 2);
}

// ---------- 验证：必填 ----------

#[test]
fn test_form_validate_required_pass() {
    let mut f = Form::new()
        .with_item(FormItem::new("name", "姓名").with_rule(FormRule::new("name").required(true)));
    f.set_value("name", "张三");
    let result = f.validate();
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}

#[test]
fn test_form_validate_required_fail() {
    let mut f = Form::new()
        .with_item(FormItem::new("name", "姓名").with_rule(FormRule::new("name").required(true)));
    f.set_value("name", "");
    let result = f.validate();
    assert!(result.is_ok());
    let errors = result.unwrap();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].field, "name");
}

// ---------- 验证：min/max 长度 ----------

#[test]
fn test_form_validate_min_length_fail() {
    let mut f = Form::new().with_item(
        FormItem::new("name", "姓名")
            .with_rule(FormRule::new("name").with_min(3)),
    );
    f.set_value("name", "ab"); // 长度 2 < 3
    let result = f.validate().unwrap();
    assert_eq!(result.len(), 1);
}

#[test]
fn test_form_validate_max_length_fail() {
    let mut f = Form::new().with_item(
        FormItem::new("name", "姓名")
            .with_rule(FormRule::new("name").with_max(5)),
    );
    f.set_value("name", "abcdef"); // 长度 6 > 5
    let result = f.validate().unwrap();
    assert_eq!(result.len(), 1);
}

#[test]
fn test_form_validate_min_max_pass() {
    let mut f = Form::new().with_item(
        FormItem::new("name", "姓名")
            .with_rule(FormRule::new("name").with_min(2).with_max(5)),
    );
    f.set_value("name", "abc"); // 长度 3 在 [2, 5]
    let result = f.validate().unwrap();
    assert_eq!(result.len(), 0);
}

// ---------- 验证：自定义 validator ----------

#[test]
fn test_form_validate_custom_validator_pass() {
    let mut f = Form::new().with_item(
        FormItem::new("age", "年龄").with_rule(
            FormRule::new("age").with_validator(Box::new(|v: &str| {
                v.parse::<u32>().map(|n| n <= 150).unwrap_or(false)
            })),
        ),
    );
    f.set_value("age", "30");
    let result = f.validate().unwrap();
    assert_eq!(result.len(), 0);
}

#[test]
fn test_form_validate_custom_validator_fail() {
    let mut f = Form::new().with_item(
        FormItem::new("age", "年龄").with_rule(
            FormRule::new("age").with_validator(Box::new(|v: &str| {
                v.parse::<u32>().map(|n| n <= 150).unwrap_or(false)
            })),
        ),
    );
    f.set_value("age", "200"); // > 150
    let result = f.validate().unwrap();
    assert_eq!(result.len(), 1);
}

// ---------- 验证：多字段组合 ----------

#[test]
fn test_form_validate_multiple_fields() {
    let mut f = Form::new()
        .with_item(
            FormItem::new("name", "姓名")
                .with_rule(FormRule::new("name").required(true)),
        )
        .with_item(
            FormItem::new("age", "年龄")
                .with_rule(FormRule::new("age").required(true)),
        );
    f.set_value("name", "");
    f.set_value("age", "");
    let result = f.validate().unwrap();
    assert_eq!(result.len(), 2);
}

#[test]
fn test_form_validate_partial_failure() {
    let mut f = Form::new()
        .with_item(
            FormItem::new("name", "姓名")
                .with_rule(FormRule::new("name").required(true)),
        )
        .with_item(
            FormItem::new("age", "年龄")
                .with_rule(FormRule::new("age").required(true)),
        );
    f.set_value("name", "张三");
    f.set_value("age", "");
    let result = f.validate().unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].field, "age");
}

// ---------- 重置 ----------

#[test]
fn test_form_reset_clears_values_and_errors() {
    let mut f = Form::new()
        .with_item(
            FormItem::new("name", "姓名")
                .with_rule(FormRule::new("name").required(true)),
        );
    f.set_value("name", "张三");
    let _ = f.validate();
    f.reset();
    assert_eq!(f.value_of("name"), None);
}

// ---------- 获取/设置值 ----------

#[test]
fn test_form_get_set_value() {
    let mut f = Form::new().with_item(FormItem::new("name", "姓名"));
    f.set_value("name", "张三");
    assert_eq!(f.value_of("name"), Some(&"张三".to_string()));
}

#[test]
fn test_form_get_nonexistent_value_returns_none() {
    let f = Form::new();
    assert_eq!(f.value_of("nonexistent"), None);
}

// ---------- 验证状态 ----------

#[test]
fn test_form_state_transitions() {
    let mut f = Form::new()
        .with_item(
            FormItem::new("name", "姓名")
                .with_rule(FormRule::new("name").required(true)),
        );
    assert_eq!(f.state(), FormState::Idle);
    f.set_value("name", "");
    let _ = f.validate();
    assert_eq!(f.state(), FormState::Failed);
    f.set_value("name", "张三");
    let _ = f.validate();
    assert_eq!(f.state(), FormState::Passed);
}

// ---------- 单字段验证（按 trigger） ----------

#[test]
fn test_form_validate_field_by_name() {
    let mut f = Form::new()
        .with_item(
            FormItem::new("name", "姓名")
                .with_rule(FormRule::new("name").required(true)),
        );
    f.set_value("name", "");
    let result = f.validate_field("name");
    assert!(result.is_some()); // 有错误
    f.set_value("name", "张三");
    let result = f.validate_field("name");
    assert!(result.is_none()); // 无错误
}

#[test]
fn test_form_validate_nonexistent_field_returns_none() {
    let mut f = Form::new();
    f.set_value("name", "");
    let result = f.validate_field("nonexistent");
    assert!(result.is_none());
}
