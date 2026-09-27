//! Form 组件 — 表单
//!
//! 参考 Element Plus `<el-form>`。
//! 支持：label-position/label-width、rules 验证（required/min/max/自定义 validator）、
//! inline 模式、表单重置、单字段验证、整体验证。

use har_ui_core::theme::Theme;
use iced::widget::{container, text};
use iced::{Color, Element, Length, Padding};

/// 验证触发时机
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ValidateTrigger {
    #[default]
    Change,
    Blur,
}

/// Form 状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FormState {
    #[default]
    Idle,
    Validating,
    Passed,
    Failed,
}

/// 自定义验证函数类型
pub type FormValidator = Box<dyn Fn(&str) -> bool + Send + Sync>;

/// 验证规则（不可 Clone，因含函数指针对象）
pub struct FormRule {
    pub field: String,
    pub required: bool,
    pub min: Option<usize>,
    pub max: Option<usize>,
    pub message: Option<String>,
    pub trigger: ValidateTrigger,
    pub validator: Option<FormValidator>,
}

impl std::fmt::Debug for FormRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FormRule")
            .field("field", &self.field)
            .field("required", &self.required)
            .field("min", &self.min)
            .field("max", &self.max)
            .field("message", &self.message)
            .field("trigger", &self.trigger)
            .field("validator", &self.validator.is_some())
            .finish()
    }
}

impl FormRule {
    pub fn new(field: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            required: false,
            min: None,
            max: None,
            message: None,
            trigger: ValidateTrigger::Change,
            validator: None,
        }
    }

    pub fn required(mut self, v: bool) -> Self {
        self.required = v;
        self
    }

    pub fn with_min(mut self, m: usize) -> Self {
        self.min = Some(m);
        self
    }

    pub fn with_max(mut self, m: usize) -> Self {
        self.max = Some(m);
        self
    }

    pub fn with_message(mut self, m: impl Into<String>) -> Self {
        self.message = Some(m.into());
        self
    }

    pub fn with_trigger(mut self, t: ValidateTrigger) -> Self {
        self.trigger = t;
        self
    }

    pub fn with_validator(mut self, v: Box<dyn Fn(&str) -> bool + Send + Sync>) -> Self {
        self.validator = Some(v);
        self
    }
}

/// Form 项
#[derive(Debug)]
pub struct FormItem {
    field: String,
    label: String,
    required: bool,
    rules: Vec<FormRule>,
}

impl FormItem {
    pub fn new(field: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            label: label.into(),
            required: false,
            rules: Vec::new(),
        }
    }

    pub fn field(&self) -> &str {
        &self.field
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn required(&self) -> bool {
        self.required
    }

    pub fn rules(&self) -> &[FormRule] {
        &self.rules
    }

    pub fn set_required(mut self, v: bool) -> Self {
        self.required = v;
        self
    }

    pub fn with_rule(mut self, r: FormRule) -> Self {
        if r.required {
            self.required = true;
        }
        self.rules.push(r);
        self
    }
}

/// 验证错误
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

/// Form 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormMessage {
    Validate,
    ValidateField(String),
    Reset,
    SetValue(String, String),
}

/// Form 组件
#[derive(Debug, Default)]
pub struct Form {
    items: Vec<FormItem>,
    values: std::collections::HashMap<String, String>,
    errors: Vec<ValidationError>,
    inline: bool,
    label_position: String,
    label_width: Option<u32>,
    state: FormState,
}

impl Form {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            values: std::collections::HashMap::new(),
            errors: Vec::new(),
            inline: false,
            label_position: "right".to_string(),
            label_width: None,
            state: FormState::Idle,
        }
    }

    pub fn with_inline(mut self, v: bool) -> Self {
        self.inline = v;
        self
    }

    pub fn with_label_position(mut self, p: impl Into<String>) -> Self {
        self.label_position = p.into();
        self
    }

    pub fn with_label_width(mut self, w: u32) -> Self {
        self.label_width = Some(w);
        self
    }

    pub fn with_item(mut self, item: FormItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn inline(&self) -> bool {
        self.inline
    }

    pub fn label_position(&self) -> &str {
        &self.label_position
    }

    pub fn label_width(&self) -> Option<u32> {
        self.label_width
    }

    pub fn items(&self) -> &[FormItem] {
        &self.items
    }

    pub fn state(&self) -> FormState {
        self.state
    }

    pub fn errors(&self) -> &[ValidationError] {
        &self.errors
    }

    pub fn value_of(&self, field: &str) -> Option<&String> {
        self.values.get(field)
    }

    pub fn set_value(&mut self, field: impl Into<String>, value: impl Into<String>) {
        self.values.insert(field.into(), value.into());
    }

    /// 重置表单（清空值和错误）
    pub fn reset(&mut self) {
        self.values.clear();
        self.errors.clear();
        self.state = FormState::Idle;
    }

    /// 验证单个字段，返回 Some(error) 或 None
    pub fn validate_field(&mut self, field: &str) -> Option<ValidationError> {
        let item = self.items.iter().find(|i| i.field() == field)?;
        let value = self.values.get(field).cloned().unwrap_or_default();
        for rule in item.rules() {
            if rule.required && value.is_empty() {
                return Some(ValidationError {
                    field: field.to_string(),
                    message: rule
                        .message
                        .clone()
                        .unwrap_or_else(|| format!("{} is required", field)),
                });
            }
            if let Some(min) = rule.min
                && value.len() < min
            {
                return Some(ValidationError {
                    field: field.to_string(),
                    message: rule
                        .message
                        .clone()
                        .unwrap_or_else(|| format!("{} length must be >= {}", field, min)),
                });
            }
            if let Some(max) = rule.max
                && value.len() > max
            {
                return Some(ValidationError {
                    field: field.to_string(),
                    message: rule
                        .message
                        .clone()
                        .unwrap_or_else(|| format!("{} length must be <= {}", field, max)),
                });
            }
            if let Some(validator) = &rule.validator
                && !validator(&value)
            {
                return Some(ValidationError {
                    field: field.to_string(),
                    message: rule
                        .message
                        .clone()
                        .unwrap_or_else(|| format!("{} validation failed", field)),
                });
            }
        }
        None
    }

    /// 验证整个表单，返回 Ok(errors) （errors 为空表示全部通过）
    pub fn validate(&mut self) -> Result<Vec<ValidationError>, String> {
        self.state = FormState::Validating;
        let mut errors = Vec::new();
        let fields: Vec<String> = self.items.iter().map(|i| i.field().to_string()).collect();
        for field in fields {
            if let Some(e) = self.validate_field(&field) {
                errors.push(e);
            }
        }
        self.errors = errors.clone();
        self.state = if errors.is_empty() {
            FormState::Passed
        } else {
            FormState::Failed
        };
        Ok(errors)
    }

    /// 处理消息
    pub fn handle(&mut self, msg: FormMessage) {
        match msg {
            FormMessage::Validate => {
                let _ = self.validate();
            }
            FormMessage::ValidateField(field) => {
                if let Some(e) = self.validate_field(&field) {
                    self.errors.retain(|x| x.field != field);
                    self.errors.push(e);
                    self.state = FormState::Failed;
                } else {
                    self.errors.retain(|x| x.field != field);
                }
            }
            FormMessage::Reset => self.reset(),
            FormMessage::SetValue(f, v) => self.set_value(f, v),
        }
    }

    /// 渲染表单为 iced::Element
    ///
    /// # 参数
    /// - `theme`: HarUI 主题引用
    /// - `field_renderer`: 字段渲染闭包（接收 FormItem 引用，返回该字段的 input Element）
    ///
    /// # 行为
    /// - 每个 FormItem 渲染为一行：label + field + error（如有）
    /// - label_position：left/right/top
    /// - label_width：固定 label 宽度
    /// - inline：所有字段横排
    /// - 错误信息用 danger 色
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        field_renderer: impl Fn(&FormItem) -> Element<'a, Message> + 'a,
    ) -> Element<'a, Message> {
        let label_color = Color::from(theme.neutral.text_primary);
        let error_color = Color::from(theme.danger.base);
        let label_width = self
            .label_width
            .map(|w| Length::Fixed(w as f32))
            .unwrap_or(Length::Fixed(80.0));

        let mut item_elements: Vec<Element<'a, Message>> = Vec::with_capacity(self.items.len());

        for item in &self.items {
            // label（必填项前加 *）
            let label_text = if item.required() {
                format!("* {}", item.label())
            } else {
                item.label().to_string()
            };
            let label_widget = text(label_text).color(label_color);

            // 字段 input
            let field_widget = field_renderer(item);

            // 错误信息
            let error_msg = self.errors.iter().find(|e| e.field == item.field());
            let error_widget =
                error_msg.map(|e| text(e.message.clone()).color(error_color).size(12));

            // 按标签位置布局
            let item_element: Element<'a, Message> = match self.label_position.as_str() {
                "top" => {
                    let mut col = iced::widget::Column::new()
                        .push(label_widget)
                        .push(field_widget)
                        .spacing(4);
                    if let Some(err) = error_widget {
                        col = col.push(err);
                    }
                    container(col).padding(Padding::from([8u16, 12u16])).into()
                }
                "left" => {
                    let mut right_col = iced::widget::Column::new().push(field_widget).spacing(2);
                    if let Some(err) = error_widget {
                        right_col = right_col.push(err);
                    }
                    let row = iced::widget::Row::new()
                        .push(
                            container(label_widget)
                                .width(label_width)
                                .align_y(iced::alignment::Vertical::Top)
                                .padding(Padding::from([8u16, 0u16])),
                        )
                        .push(right_col);
                    container(row).padding(Padding::from([8u16, 12u16])).into()
                }
                _ => {
                    // right（默认）
                    let mut right_col = iced::widget::Column::new().push(field_widget).spacing(2);
                    if let Some(err) = error_widget {
                        right_col = right_col.push(err);
                    }
                    let row = iced::widget::Row::new()
                        .push(
                            container(label_widget)
                                .width(label_width)
                                .align_y(iced::alignment::Vertical::Top)
                                .padding(Padding::from([8u16, 0u16])),
                        )
                        .push(right_col);
                    container(row).padding(Padding::from([8u16, 12u16])).into()
                }
            };
            item_elements.push(item_element);
        }

        if self.inline {
            // 横排
            iced::widget::Row::with_children(item_elements)
                .spacing(16)
                .align_y(iced::Alignment::Center)
                .into()
        } else {
            // 竖排
            iced::widget::Column::with_children(item_elements)
                .spacing(0)
                .into()
        }
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_form_rule_builder() {
        let r = FormRule::new("name")
            .required(true)
            .with_min(2)
            .with_max(20)
            .with_message("invalid")
            .with_trigger(ValidateTrigger::Blur);
        assert!(r.required);
        assert_eq!(r.min, Some(2));
        assert_eq!(r.max, Some(20));
        assert_eq!(r.message, Some("invalid".to_string()));
        assert_eq!(r.trigger, ValidateTrigger::Blur);
    }

    #[test]
    fn test_form_item_with_rule_marks_required() {
        let item = FormItem::new("name", "Name").with_rule(FormRule::new("name").required(true));
        assert!(item.required());
    }

    #[test]
    fn test_form_validate_collects_all_errors() {
        let mut f = Form::new()
            .with_item(FormItem::new("a", "A").with_rule(FormRule::new("a").required(true)))
            .with_item(FormItem::new("b", "B").with_rule(FormRule::new("b").required(true)));
        let errors = f.validate().unwrap();
        assert_eq!(errors.len(), 2);
    }
}
