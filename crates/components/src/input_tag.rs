//! InputTag 标签输入框 — 参考 Element Plus `<el-input-tag>`（2.9+）
//!
//! 支持：字符流输入 + Enter 提交为标签、退格双语义（有文字改文字 /
//! 空文字删末尾标签）、最大标签数、去重与空值拒绝、清空、禁用。

use iced::Element;

/// InputTag 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputTagMessage {
    /// 输入字符
    Char(char),
    /// 退格：有文字删文字，无文字删末尾标签（EP 语义）
    Backspace,
    /// 提交当前输入为标签（Enter）
    Commit,
    /// 直接设置标签全集（应用侧回填）
    SetTags(Vec<String>),
    /// 移除指定索引的标签
    Remove(usize),
    /// 清空全部标签与输入
    ClearAll,
}

/// InputTag 组件
#[derive(Debug, Clone)]
pub struct InputTag {
    tags: Vec<String>,
    input: String,
    max: Option<usize>,
    placeholder: String,
    disabled: bool,
}

impl Default for InputTag {
    fn default() -> Self {
        Self::new()
    }
}

impl InputTag {
    pub fn new() -> Self {
        Self {
            tags: Vec::new(),
            input: String::new(),
            max: None,
            placeholder: String::new(),
            disabled: false,
        }
    }

    /// 最大标签数（None 不限；达到后 Char 输入与 Commit 均拒绝）
    pub fn with_max(mut self, max: usize) -> Self {
        self.max = Some(max);
        self
    }

    pub fn with_placeholder(mut self, p: impl Into<String>) -> Self {
        self.placeholder = p.into();
        self
    }

    pub fn with_disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }

    pub fn tags(&self) -> &[String] {
        &self.tags
    }

    pub fn input(&self) -> &str {
        &self.input
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    /// 处理消息
    pub fn handle(&mut self, msg: InputTagMessage) {
        match msg {
            InputTagMessage::Char(c) => {
                if self.disabled || self.max_reached() {
                    return;
                }
                self.input.push(c);
            }
            InputTagMessage::Backspace => {
                if self.disabled {
                    return;
                }
                if self.input.is_empty() {
                    // 空输入：删末尾标签
                    self.tags.pop();
                } else {
                    self.input.pop();
                }
            }
            InputTagMessage::Commit => {
                if self.disabled || self.max_reached() {
                    return;
                }
                let v = self.input.trim().to_string();
                if v.is_empty() || self.tags.contains(&v) {
                    return;
                }
                self.tags.push(v);
                self.input.clear();
            }
            InputTagMessage::SetTags(values) => {
                if self.disabled {
                    return;
                }
                self.tags = values;
                if let Some(max) = self.max {
                    self.tags.truncate(max);
                }
            }
            InputTagMessage::Remove(i) => {
                if self.disabled {
                    return;
                }
                if i < self.tags.len() {
                    self.tags.remove(i);
                }
            }
            InputTagMessage::ClearAll => {
                self.tags.clear();
                self.input.clear();
            }
        }
    }

    fn max_reached(&self) -> bool {
        self.max.map(|m| self.tags.len() >= m).unwrap_or(false)
    }

    /// 渲染：标签芯片行 + 输入区（占位符在无标签且无输入时显示）
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a har_ui_core::theme::Theme,
        on_msg: impl Fn(InputTagMessage) -> Message + 'a,
    ) -> Element<'a, Message> {
        use iced::widget::{container, mouse_area, row, text};
        use iced::{Color, Length, Padding};
        let text_regular = Color::from(theme.neutral.text_regular);
        let text_placeholder = Color::from(theme.neutral.text_placeholder);
        let border = Color::from(theme.neutral.border_lighter);
        let tag_bg = Color::from(theme.neutral.bg_base);
        let danger = Color::from(theme.danger.base);

        let mut chips = row![].spacing(4);
        for (i, tag) in self.tags.iter().enumerate() {
            let chip = container(
                row![
                    text(tag.clone()).size(12).color(text_regular),
                    // 移除叉：点击映射 Remove(i)
                    mouse_area(text("×").size(12).color(danger))
                        .on_press(on_msg(InputTagMessage::Remove(i))),
                ]
                .spacing(4),
            )
            .padding(Padding::from([2u16, 6u16]))
            .style(move |_t| iced::widget::container::Style {
                text_color: None,
                background: Some(iced::Background::Color(tag_bg)),
                border: iced::Border {
                    color: border,
                    width: 1.0,
                    radius: iced::border::radius(4.0),
                },
                shadow: iced::Shadow::default(),
                snap: false,
            });
            chips = chips.push(chip);
        }

        let input_display: Element<'a, Message> = if self.input.is_empty() && self.tags.is_empty() {
            text(self.placeholder.clone())
                .color(text_placeholder)
                .into()
        } else if self.input.is_empty() {
            text("").into()
        } else {
            text(self.input.clone()).color(text_regular).into()
        };

        container(chips.push(input_display))
            .width(Length::Fill)
            .padding(Padding::from([4u16, 8u16]))
            .style(move |_t| iced::widget::container::Style {
                text_color: None,
                background: Some(iced::Background::Color(Color::from(
                    theme.neutral.bg_overlay,
                ))),
                border: iced::Border {
                    color: border,
                    width: 1.0,
                    radius: iced::border::radius(4.0),
                },
                shadow: iced::Shadow::default(),
                snap: false,
            })
            .into()
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_commit_creates_tag_and_clears_input() {
        let mut it = InputTag::new();
        for c in "Rust".chars() {
            it.handle(InputTagMessage::Char(c));
        }
        it.handle(InputTagMessage::Commit);
        assert_eq!(it.tags(), ["Rust"]);
        assert_eq!(it.input(), "");
    }

    #[test]
    fn test_empty_and_duplicate_rejected() {
        let mut it = InputTag::new();
        it.handle(InputTagMessage::Commit); // 空输入
        assert!(it.tags().is_empty());
        for c in "Rust".chars() {
            it.handle(InputTagMessage::Char(c));
        }
        it.handle(InputTagMessage::Commit);
        // 重复拒绝
        for c in "Rust".chars() {
            it.handle(InputTagMessage::Char(c));
        }
        it.handle(InputTagMessage::Commit);
        assert_eq!(it.tags().len(), 1);
    }

    #[test]
    fn test_backspace_dual_semantics() {
        let mut it = InputTag::new();
        for c in "ab".chars() {
            it.handle(InputTagMessage::Char(c));
        }
        it.handle(InputTagMessage::Commit);
        // 无文字 Backspace → 删末尾标签
        it.handle(InputTagMessage::Backspace);
        assert!(it.tags().is_empty());
        // 有文字 Backspace → 删文字
        it.handle(InputTagMessage::Char('x'));
        it.handle(InputTagMessage::Backspace);
        assert_eq!(it.input(), "");
        assert!(it.tags().is_empty());
    }

    #[test]
    fn test_max_tags_guard() {
        let mut it = InputTag::new().with_max(2);
        for v in ["a", "b", "c"] {
            for c in v.chars() {
                it.handle(InputTagMessage::Char(c));
            }
            it.handle(InputTagMessage::Commit);
        }
        assert_eq!(it.tags(), ["a", "b"], "达到上限后 Commit 拒绝");
        // 上限后字符输入也拒绝
        it.handle(InputTagMessage::Char('z'));
        assert_eq!(it.input(), "");
    }

    #[test]
    fn test_remove_by_index() {
        let mut it = InputTag::new();
        it.handle(InputTagMessage::SetTags(vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
        ]));
        it.handle(InputTagMessage::Remove(1));
        assert_eq!(it.tags(), ["a", "c"]);
        // 越界忽略
        it.handle(InputTagMessage::Remove(9));
        assert_eq!(it.tags().len(), 2);
    }

    #[test]
    fn test_set_tags_truncates_to_max() {
        let mut it = InputTag::new().with_max(2);
        it.handle(InputTagMessage::SetTags(vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
        ]));
        assert_eq!(it.tags(), ["a", "b"]);
    }

    #[test]
    fn test_clear_all() {
        let mut it = InputTag::new();
        it.handle(InputTagMessage::SetTags(vec!["a".to_string()]));
        it.handle(InputTagMessage::Char('x'));
        it.handle(InputTagMessage::ClearAll);
        assert!(it.tags().is_empty());
        assert_eq!(it.input(), "");
    }

    #[test]
    fn test_disabled_blocks_all() {
        let mut it = InputTag::new().with_disabled(true);
        it.handle(InputTagMessage::Char('x'));
        it.handle(InputTagMessage::Commit);
        it.handle(InputTagMessage::SetTags(vec!["a".to_string()]));
        it.handle(InputTagMessage::Remove(0));
        assert!(it.tags().is_empty());
        assert_eq!(it.input(), "");
    }

    #[test]
    fn test_view_renders() {
        let theme = har_ui_core::theme::Theme::element_light();
        let mut it = InputTag::new().with_placeholder("输入后回车");
        it.handle(InputTagMessage::SetTags(vec!["Rust".to_string()]));
        let _e = it.view::<()>(&theme, |_| ());
    }
}
