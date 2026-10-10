//! Autocomplete 自动完成 — 参考 Element Plus `<el-autocomplete>`
//!
//! 支持：本地建议列表（contains 过滤，大小写不敏感）或自定义 provider、
//! 键盘上下移动 + Enter 选中、禁用、清空。
//!
//! ## 五段式
//! 输入经 `Char/Backspace` 消息驱动（与 Keypad 同款字符流模式）；
//! 过滤结果经 `SetSuggestions` 回填（支持异步 provider 的应用侧回填）。

/// 建议项
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AutocompleteItem {
    pub value: String,
}

impl AutocompleteItem {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

/// Autocomplete 消息
#[derive(Debug, Clone, PartialEq)]
pub enum AutocompleteMessage {
    /// 输入字符
    Char(char),
    /// 退格
    Backspace,
    /// 清空输入与建议
    Clear,
    /// 应用侧回填建议结果（异步 provider 完成后）
    SetSuggestions(Vec<String>),
    /// 键盘下移（环绕）
    MoveDown,
    /// 键盘上移（环绕）
    MoveUp,
    /// 选中当前高亮项（无高亮时选第一项）
    Enter,
    /// 直接选中第 i 项
    Choose(usize),
    /// 隐藏建议面板
    Hide,
}

/// 面板状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AutocompleteState {
    #[default]
    Idle,
    /// 建议面板可见
    Opened,
}

/// Autocomplete 组件
#[derive(Debug, Clone)]
pub struct Autocomplete {
    value: String,
    items: Vec<AutocompleteItem>,
    highlighted: Option<usize>,
    visible: bool,
    loading: bool,
    /// 本地建议源（未设置 provider 时做 contains 过滤）
    local_suggestions: Vec<String>,
    /// 自定义建议 provider（优先于本地过滤）
    provider: Option<fn(&str) -> Vec<String>>,
    placeholder: String,
    disabled: bool,
}

impl Default for Autocomplete {
    fn default() -> Self {
        Self::new()
    }
}

impl Autocomplete {
    pub fn new() -> Self {
        Self {
            value: String::new(),
            items: Vec::new(),
            highlighted: None,
            visible: false,
            loading: false,
            local_suggestions: Vec::new(),
            provider: None,
            placeholder: String::new(),
            disabled: false,
        }
    }

    pub fn with_placeholder(mut self, p: impl Into<String>) -> Self {
        self.placeholder = p.into();
        self
    }

    pub fn with_disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }

    /// 设置本地建议全集（未设 provider 时的过滤源）
    pub fn with_suggestions(mut self, items: Vec<String>) -> Self {
        self.local_suggestions = items;
        self
    }

    /// 设置自定义建议 provider（优先于本地过滤）
    pub fn with_provider(mut self, f: fn(&str) -> Vec<String>) -> Self {
        self.provider = Some(f);
        self
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn items(&self) -> &[AutocompleteItem] {
        &self.items
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn loading(&self) -> bool {
        self.loading
    }

    pub fn state(&self) -> AutocompleteState {
        if self.visible {
            AutocompleteState::Opened
        } else {
            AutocompleteState::Idle
        }
    }

    pub fn highlighted_index(&self) -> Option<usize> {
        self.highlighted
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    /// 处理消息
    pub fn handle(&mut self, msg: AutocompleteMessage) {
        match msg {
            AutocompleteMessage::Char(c) => {
                if self.disabled {
                    return;
                }
                self.value.push(c);
                self.fetch();
            }
            AutocompleteMessage::Backspace => {
                if self.disabled {
                    return;
                }
                self.value.pop();
                self.fetch();
            }
            AutocompleteMessage::Clear => {
                self.value.clear();
                self.items.clear();
                self.visible = false;
                self.highlighted = None;
                self.loading = false;
            }
            AutocompleteMessage::SetSuggestions(values) => {
                self.items = values.into_iter().map(AutocompleteItem::new).collect();
                self.highlighted = if self.items.is_empty() { None } else { Some(0) };
                self.visible = !self.items.is_empty();
                self.loading = false;
            }
            AutocompleteMessage::MoveDown => {
                if !self.visible || self.items.is_empty() {
                    return;
                }
                let n = self.items.len();
                self.highlighted = Some(match self.highlighted {
                    Some(i) => (i + 1) % n,
                    None => 0,
                });
            }
            AutocompleteMessage::MoveUp => {
                if !self.visible || self.items.is_empty() {
                    return;
                }
                let n = self.items.len();
                self.highlighted = Some(match self.highlighted {
                    Some(0) | None => n - 1,
                    Some(i) => i - 1,
                });
            }
            AutocompleteMessage::Enter => {
                if self.disabled || !self.visible {
                    return;
                }
                let idx = self.highlighted.unwrap_or(0);
                if idx < self.items.len() {
                    let v = self.items[idx].value.clone();
                    self.choose_value(&v);
                }
            }
            AutocompleteMessage::Choose(i) => {
                if self.disabled || i >= self.items.len() {
                    return;
                }
                let v = self.items[i].value.clone();
                self.choose_value(&v);
            }
            AutocompleteMessage::Hide => {
                self.visible = false;
                self.highlighted = None;
            }
        }
    }

    /// 触发过滤：provider 优先，否则本地 contains 过滤（大小写不敏感）
    fn fetch(&mut self) {
        if self.provider.is_none() && self.local_suggestions.is_empty() {
            self.items.clear();
            self.visible = false;
            return;
        }
        self.loading = true;
        let items = match self.provider {
            Some(f) => f(&self.value),
            None => {
                let q = self.value.to_lowercase();
                self.local_suggestions
                    .iter()
                    .filter(|s| !q.is_empty() && s.to_lowercase().contains(&q))
                    .cloned()
                    .collect()
            }
        };
        self.handle(AutocompleteMessage::SetSuggestions(items));
    }

    fn choose_value(&mut self, v: &str) {
        self.value = v.to_string();
        self.visible = false;
        self.highlighted = None;
        self.items.clear();
    }

    /// 渲染（逻辑视图：当前值 + 可见建议项列表；下拉定位经 behavior::overlay）
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a har_ui_core::theme::Theme,
        on_msg: impl Fn(AutocompleteMessage) -> Message + 'a,
    ) -> Element<'a, Message> {
        use iced::widget::{container, text};
        use iced::{Color, Length, Padding};
        let _ = on_msg;
        let text_color = Color::from(theme.neutral.text_regular);
        let placeholder_color = Color::from(theme.neutral.text_placeholder);
        let border = Color::from(theme.neutral.border_lighter);
        let shown: Vec<&str> = if self.visible {
            self.items
                .iter()
                .map(|i| i.value.as_str())
                .take(8)
                .collect()
        } else {
            Vec::new()
        };
        let mut col = iced::widget::Column::new().spacing(2);
        let display: Element<'a, Message> = if self.value.is_empty() {
            text(self.placeholder.clone())
                .color(placeholder_color)
                .into()
        } else {
            text(self.value.clone()).color(text_color).into()
        };
        col = col.push(
            container(display)
                .width(Length::Fill)
                .padding(Padding::from([8u16, 12u16]))
                .style(move |_t| iced::widget::container::Style {
                    text_color: None,
                    background: Some(iced::Background::Color(Color::from(
                        theme.neutral.bg_overlay,
                    ))),
                    border: iced::Border {
                        color: border,
                        width: 1.0,
                        radius: iced::border::Radius::default(),
                    },
                    shadow: iced::Shadow::default(),
                    snap: false,
                }),
        );
        for (i, v) in shown.iter().enumerate() {
            let hl = self.highlighted == Some(i);
            let bg = if hl {
                Color::from(theme.neutral.bg_base)
            } else {
                Color::TRANSPARENT
            };
            col = col.push(
                container(text((*v).to_string()).color(text_color))
                    .width(Length::Fill)
                    .padding(Padding::from([6u16, 12u16]))
                    .style(move |_t| iced::widget::container::Style {
                        text_color: None,
                        background: Some(iced::Background::Color(bg)),
                        border: iced::Border::default(),
                        shadow: iced::Shadow::default(),
                        snap: false,
                    }),
            );
        }
        col.into()
    }
}

use iced::Element;

#[cfg(test)]
mod internal_tests {
    use super::*;

    fn sample() -> Autocomplete {
        Autocomplete::new().with_suggestions(vec![
            "Beijing".to_string(),
            "Shanghai".to_string(),
            "Shenzhen".to_string(),
        ])
    }

    #[test]
    fn test_typing_filters_contains_case_insensitive() {
        let mut a = sample();
        for c in "sh".chars() {
            a.handle(AutocompleteMessage::Char(c));
        }
        assert_eq!(a.value(), "sh");
        assert!(a.visible());
        let values: Vec<&str> = a.items().iter().map(|i| i.value.as_str()).collect();
        assert_eq!(values, vec!["Shanghai", "Shenzhen"]);
        assert_eq!(a.state(), AutocompleteState::Opened);
        assert_eq!(a.highlighted_index(), Some(0));
    }

    #[test]
    fn test_enter_and_choose() {
        let mut a = sample();
        a.handle(AutocompleteMessage::Char('b'));
        a.handle(AutocompleteMessage::Enter);
        assert_eq!(a.value(), "Beijing");
        assert!(!a.visible());
        // 再来一轮：清空后 MoveDown + Enter
        a.handle(AutocompleteMessage::Clear);
        for c in "sh".chars() {
            a.handle(AutocompleteMessage::Char(c));
        }
        assert!(a.visible(), "Clear 后输入应重新拉起面板");
        a.handle(AutocompleteMessage::MoveDown); // -> 1
        a.handle(AutocompleteMessage::Enter);
        assert_eq!(a.value(), "Shenzhen");
    }

    #[test]
    fn test_move_wraps_around() {
        let mut a = sample();
        a.handle(AutocompleteMessage::Char('s'));
        assert_eq!(a.items().len(), 2);
        a.handle(AutocompleteMessage::MoveDown);
        a.handle(AutocompleteMessage::MoveDown);
        assert_eq!(a.highlighted_index(), Some(0)); // 环绕
        a.handle(AutocompleteMessage::MoveUp);
        assert_eq!(a.highlighted_index(), Some(1)); // 反向环绕
    }

    #[test]
    fn test_choose_by_index_and_clear() {
        let mut a = sample();
        a.handle(AutocompleteMessage::Char('s'));
        a.handle(AutocompleteMessage::Choose(0));
        assert_eq!(a.value(), "Shanghai");
        a.handle(AutocompleteMessage::Clear);
        assert_eq!(a.value(), "");
        assert!(!a.visible());
        assert!(a.items().is_empty());
    }

    #[test]
    fn test_no_match_hides_panel() {
        let mut a = sample();
        for c in "zzz".chars() {
            a.handle(AutocompleteMessage::Char(c));
        }
        assert_eq!(a.value(), "zzz");
        assert!(!a.visible());
        assert!(a.items().is_empty());
        assert_eq!(a.state(), AutocompleteState::Idle);
    }

    #[test]
    fn test_disabled_blocks_input() {
        let mut a = sample().with_disabled(true);
        a.handle(AutocompleteMessage::Char('x'));
        a.handle(AutocompleteMessage::Enter);
        assert_eq!(a.value(), "");
        assert!(!a.visible());
    }

    #[test]
    fn test_provider_takes_priority() {
        fn provider(q: &str) -> Vec<String> {
            vec![format!("{}-custom", q.to_uppercase())]
        }
        let mut a = Autocomplete::new()
            .with_suggestions(vec!["local".to_string()])
            .with_provider(provider);
        a.handle(AutocompleteMessage::Char('a'));
        assert_eq!(a.items().len(), 1);
        assert_eq!(a.items()[0].value, "A-custom");
        assert!(!a.loading());
    }

    #[test]
    fn test_set_suggestions_app_driven() {
        let mut a = Autocomplete::new();
        a.handle(AutocompleteMessage::SetSuggestions(vec![
            "x".to_string(),
            "y".to_string(),
        ]));
        assert!(a.visible());
        assert_eq!(a.highlighted_index(), Some(0));
        // 空结果收起面板
        a.handle(AutocompleteMessage::SetSuggestions(vec![]));
        assert!(!a.visible());
    }
}
