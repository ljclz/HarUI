//! Select 组件 — 选择器
//!
//! 参考 Element Plus `<el-select>`。
//! 支持：单选/多选、禁用选项、可搜索(filterable)、clearable、1000 选项虚拟列表。

use har_ui_core::theme::Theme;
use har_ui_core::theme::style_sheets::{self, ButtonKind};
use iced::widget::{button, container, scrollable, text, text_input};
use iced::{Color, Element, Length, Padding};

/// 选项
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectOption {
    value: String,
    label: String,
    disabled: bool,
}

impl SelectOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }

    pub fn set_disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }
}

/// Select 状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectState {
    #[default]
    Closed,
    Open,
}

/// Select 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectMessage {
    Open,
    Close,
    Choose(String),
    Clear,
    Query(String),
}

/// Select 组件
#[derive(Debug, Clone)]
pub struct Select {
    options: Vec<SelectOption>,
    /// 单选时的值
    value: Option<String>,
    /// 多选时的值列表
    values: Vec<String>,
    multiple: bool,
    filterable: bool,
    clearable: bool,
    disabled: bool,
    state: SelectState,
    query: Option<String>,
}

impl Default for Select {
    fn default() -> Self {
        Self::new()
    }
}

impl Select {
    pub fn new() -> Self {
        Self {
            options: Vec::new(),
            value: None,
            values: Vec::new(),
            multiple: false,
            filterable: false,
            clearable: false,
            disabled: false,
            state: SelectState::Closed,
            query: None,
        }
    }

    pub fn with_option(mut self, opt: SelectOption) -> Self {
        self.options.push(opt);
        self
    }

    pub fn with_multiple(mut self, v: bool) -> Self {
        self.multiple = v;
        self
    }

    pub fn with_filterable(mut self, v: bool) -> Self {
        self.filterable = v;
        self
    }

    pub fn with_clearable(mut self, v: bool) -> Self {
        self.clearable = v;
        self
    }

    pub fn with_disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }

    pub fn options(&self) -> &[SelectOption] {
        &self.options
    }

    pub fn value(&self) -> Option<&String> {
        self.value.as_ref()
    }

    pub fn values(&self) -> &[String] {
        &self.values
    }

    pub fn multiple(&self) -> bool {
        self.multiple
    }

    pub fn filterable(&self) -> bool {
        self.filterable
    }

    pub fn clearable(&self) -> bool {
        self.clearable
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn state(&self) -> SelectState {
        self.state
    }

    pub fn query(&self) -> Option<String> {
        self.query.clone()
    }

    /// 显示的 label（单选）
    pub fn display_label(&self) -> Option<String> {
        let v = self.value.as_ref()?;
        self.options
            .iter()
            .find(|o| &o.value == v)
            .map(|o| o.label.clone())
    }

    /// 多选时显示的 label 列表
    pub fn display_labels(&self) -> Vec<String> {
        self.values
            .iter()
            .filter_map(|v| self.options.iter().find(|o| &o.value == v))
            .map(|o| o.label.clone())
            .collect()
    }

    /// 按查询文本过滤选项
    pub fn filter(&self, q: &str) -> Vec<&SelectOption> {
        if q.is_empty() {
            return self.options.iter().collect();
        }
        let q_lower = q.to_lowercase();
        self.options
            .iter()
            .filter(|o| {
                o.label.to_lowercase().contains(&q_lower)
                    || o.value.to_lowercase().contains(&q_lower)
            })
            .collect()
    }

    /// 处理消息
    pub fn handle(&mut self, msg: SelectMessage) {
        match msg {
            SelectMessage::Open => {
                if !self.disabled {
                    self.state = SelectState::Open;
                    self.query = Some(String::new());
                }
            }
            SelectMessage::Close => {
                self.state = SelectState::Closed;
            }
            SelectMessage::Choose(v) => {
                // 检查选项存在且未禁用
                let opt = match self.options.iter().find(|o| o.value == v) {
                    Some(o) if !o.disabled => o.clone(),
                    _ => return,
                };
                let _ = opt;
                if self.multiple {
                    // 切换：已选则取消，未选则添加
                    if let Some(pos) = self.values.iter().position(|x| x == &v) {
                        self.values.remove(pos);
                    } else {
                        self.values.push(v);
                    }
                    // 多选不关闭下拉
                } else {
                    self.value = Some(v);
                    // 单选关闭下拉
                    self.state = SelectState::Closed;
                }
            }
            SelectMessage::Clear => {
                if self.clearable {
                    self.value = None;
                    self.values.clear();
                }
            }
            SelectMessage::Query(q) => {
                self.query = Some(q);
            }
        }
    }

    /// 渲染 Select 为 iced::Element
    ///
    /// # 参数
    /// - `theme`: HarUI 主题引用
    /// - `on_choose`: 交互回调。参数为选项 value 表示选中；
    ///   约定字符串：`"__trigger__"` 触发器点击、`"__clear__"` 清除、
    ///   `"__query:<text>"` 搜索框输入。
    ///
    /// # 行为
    /// - Closed：仅渲染触发器（显示当前值或 placeholder）
    /// - Open：渲染触发器 + 下拉面板（filterable 时含搜索框 + 选项列表）
    /// - disabled：触发器无 on_press
    /// - clearable + 有值：触发器右侧显示 × 按钮
    /// - 多选：触发器显示已选 label 拼接；选项前加 ✓
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        on_choose: impl Fn(String) -> Message + 'a,
    ) -> Element<'a, Message> {
        let text_color = Color::from(theme.neutral.text_regular);
        let placeholder_color = Color::from(theme.neutral.text_placeholder);
        let border_color = Color::from(theme.neutral.border_base);
        let primary_color = Color::from(theme.primary.base);
        let disabled_color = Color::from(theme.neutral.text_disabled);
        let bg_overlay = Color::from(theme.neutral.bg_overlay);

        // 触发器显示文本
        let trigger_text = if self.multiple {
            let labels = self.display_labels();
            if labels.is_empty() {
                "请选择".to_string()
            } else {
                labels.join(", ")
            }
        } else {
            self.display_label().unwrap_or_else(|| "请选择".to_string())
        };
        let is_empty = if self.multiple {
            self.values.is_empty()
        } else {
            self.value.is_none()
        };
        let trigger_text_color = if is_empty {
            placeholder_color
        } else if self.disabled {
            disabled_color
        } else {
            text_color
        };

        // 触发器按钮（Row 包裹 文本 + Fill + 箭头）
        let arrow = if self.state == SelectState::Open {
            "▲"
        } else {
            "▼"
        };
        let trigger_content = iced::widget::Row::new()
            .push(text(trigger_text).color(trigger_text_color))
            .push(iced::widget::Space::with_width(Length::Fill))
            .push(text(arrow).color(text_color));

        let mut trigger_btn = button(trigger_content)
            .padding(Padding::from([8u16, 12u16]))
            .width(Length::Fill)
            .style(move |_t, status| {
                style_sheets::button_style(theme, ButtonKind::Default, false, status)
            });

        if !self.disabled {
            trigger_btn = trigger_btn.on_press(on_choose("__trigger__".to_string()));
        }

        // clearable + 有值：右侧追加 × 按钮
        let trigger_elem: Element<'a, Message> = if self.clearable && !is_empty && !self.disabled {
            let clear_btn = button(text("×").color(text_color))
                .padding(Padding::from([2u16, 6u16]))
                .on_press(on_choose("__clear__".to_string()))
                .style(move |_t, status| {
                    style_sheets::button_style(theme, ButtonKind::Text, false, status)
                });
            iced::widget::Row::new()
                .push(trigger_btn)
                .push(clear_btn)
                .spacing(4)
                .align_y(iced::Alignment::Center)
                .into()
        } else {
            trigger_btn.into()
        };

        // Closed：仅触发器
        if self.state != SelectState::Open {
            return container(trigger_elem).into();
        }

        // Open：下拉面板
        let mut dropdown_children: Vec<Element<'a, Message>> = Vec::new();

        // 选项按钮（先构建，避免 on_choose 被移动后无法调用）
        let q = self.query.clone().unwrap_or_default();
        let filtered: Vec<&SelectOption> = self.filter(&q);

        if filtered.is_empty() {
            dropdown_children.push(
                container(text("无匹配项").color(placeholder_color))
                    .padding(Padding::from(12u16))
                    .width(Length::Fill)
                    .into(),
            );
        } else {
            let mut options_children: Vec<Element<'a, Message>> =
                Vec::with_capacity(filtered.len());
            for opt in &filtered {
                let is_selected = if self.multiple {
                    self.values.iter().any(|v| v == &opt.value)
                } else {
                    self.value.as_ref() == Some(&opt.value)
                };
                let opt_color = if opt.disabled {
                    disabled_color
                } else if is_selected {
                    primary_color
                } else {
                    text_color
                };
                let opt_label = if is_selected {
                    format!("✓ {}", opt.label)
                } else {
                    opt.label.clone()
                };

                let mut opt_btn = button(text(opt_label).color(opt_color))
                    .padding(Padding::from([6u16, 12u16]))
                    .width(Length::Fill)
                    .style(move |_t, status| {
                        let mut s =
                            style_sheets::button_style(theme, ButtonKind::Text, false, status);
                        if is_selected {
                            s.background = Some(iced::Background::Color(Color {
                                a: 0.05,
                                ..primary_color
                            }));
                        }
                        s
                    });

                if !opt.disabled {
                    opt_btn = opt_btn.on_press(on_choose(opt.value.clone()));
                }
                options_children.push(opt_btn.into());
            }

            let options_col = iced::widget::Column::with_children(options_children).spacing(0);
            let scroll = scrollable(options_col).height(Length::Fixed(300.0));
            dropdown_children.push(scroll.into());
        }

        // filterable：搜索框（最后构建，on_choose 被 move 进闭包）
        if self.filterable {
            let query_text = self.query.clone().unwrap_or_default();
            let search_input = text_input("搜索...", &query_text)
                .on_input(move |t| on_choose(format!("__query:{}", t)))
                .style(move |_t, status| style_sheets::input_style(theme, status));
            dropdown_children.insert(0, search_input.into());
        }

        let dropdown = container(iced::widget::Column::with_children(dropdown_children).spacing(0))
            .width(Length::Fill)
            .max_height(400.0)
            .style(move |_t| iced::widget::container::Style {
                text_color: Some(text_color),
                background: Some(iced::Background::Color(bg_overlay)),
                border: iced::Border {
                    color: border_color,
                    width: 1.0,
                    radius: iced::border::radius(4.0),
                },
                shadow: iced::Shadow {
                    color: Color {
                        a: 0.2,
                        ..Color::BLACK
                    },
                    offset: iced::Vector::new(0.0, 2.0),
                    blur_radius: 8.0,
                },
            });

        container(
            iced::widget::Column::new()
                .push(trigger_elem)
                .push(dropdown)
                .spacing(4),
        )
        .into()
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_select_default_no_value() {
        let s = Select::new();
        assert_eq!(s.value(), None);
        assert!(s.values().is_empty());
    }

    #[test]
    fn test_select_filter_by_label_or_value() {
        let s = Select::new()
            .with_option(SelectOption::new("a", "Apple"))
            .with_option(SelectOption::new("b", "Banana"));
        let filtered = s.filter("a");
        // "a" 同时匹配 value "a" 和 label "Apple"/"Banana"
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_select_choose_disabled_option_ignored() {
        let mut s = Select::new().with_option(SelectOption::new("a", "A").set_disabled(true));
        s.handle(SelectMessage::Choose("a".to_string()));
        assert_eq!(s.value(), None);
    }
}
