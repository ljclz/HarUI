//! Transfer 穿梭框 — 参考 Element Plus `<el-transfer>`
//!
//! 支持：左右双面板、勾选后经按钮整体搬移、逐项勾选/全选、禁用项不可搬移、
//! 搬移后对应侧勾选自动清空。
//!
//! ## 数据模型
//! 单一 `items` 列表，每项带 `side`（Left/Right）标记归属；勾选集分列维护。

use har_ui_core::theme::Theme;
use iced::widget::{container, text};
use iced::{Color, Element, Length, Padding};

/// 面板方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

/// 穿梭项
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransferItem {
    pub key: String,
    pub label: String,
    pub side: Side,
    pub disabled: bool,
}

impl TransferItem {
    pub fn new(key: impl Into<String>, label: impl Into<String>, side: Side) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            side,
            disabled: false,
        }
    }

    pub fn with_disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }
}

/// Transfer 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransferMessage {
    /// 勾选/取消左面板某项
    ToggleCheckLeft(String),
    /// 勾选/取消右面板某项
    ToggleCheckRight(String),
    /// 左面板全选/全不选
    CheckAllLeft(bool),
    /// 右面板全选/全不选
    CheckAllRight(bool),
    /// 把左面板勾选项搬到右面板（禁用项跳过）
    ToRight,
    /// 把右面板勾选项搬回左面板（禁用项跳过）
    ToLeft,
}

/// Transfer 组件
#[derive(Debug, Clone)]
pub struct Transfer {
    items: Vec<TransferItem>,
    checked_left: Vec<String>,
    checked_right: Vec<String>,
    titles: (String, String),
    disabled: bool,
}

impl Default for Transfer {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

impl Transfer {
    pub fn new(items: Vec<TransferItem>) -> Self {
        Self {
            items,
            checked_left: Vec::new(),
            checked_right: Vec::new(),
            titles: ("列表 1".to_string(), "列表 2".to_string()),
            disabled: false,
        }
    }

    pub fn with_titles(mut self, left: impl Into<String>, right: impl Into<String>) -> Self {
        self.titles = (left.into(), right.into());
        self
    }

    pub fn with_disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }

    /// 左面板项（保持插入顺序）
    pub fn left(&self) -> Vec<&TransferItem> {
        self.items.iter().filter(|i| i.side == Side::Left).collect()
    }

    /// 右面板项
    pub fn right(&self) -> Vec<&TransferItem> {
        self.items
            .iter()
            .filter(|i| i.side == Side::Right)
            .collect()
    }

    pub fn checked_left(&self) -> &[String] {
        &self.checked_left
    }

    pub fn checked_right(&self) -> &[String] {
        &self.checked_right
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    /// 处理消息
    pub fn handle(&mut self, msg: TransferMessage) {
        match msg {
            TransferMessage::ToggleCheckLeft(key) => {
                if self.disabled {
                    return;
                }
                Self::toggle_check(&self.items, &mut self.checked_left, &key, Side::Left);
            }
            TransferMessage::ToggleCheckRight(key) => {
                if self.disabled {
                    return;
                }
                Self::toggle_check(&self.items, &mut self.checked_right, &key, Side::Right);
            }
            TransferMessage::CheckAllLeft(v) => {
                if self.disabled {
                    return;
                }
                self.checked_left = if v {
                    self.items
                        .iter()
                        .filter(|i| i.side == Side::Left && !i.disabled)
                        .map(|i| i.key.clone())
                        .collect()
                } else {
                    Vec::new()
                };
            }
            TransferMessage::CheckAllRight(v) => {
                if self.disabled {
                    return;
                }
                self.checked_right = if v {
                    self.items
                        .iter()
                        .filter(|i| i.side == Side::Right && !i.disabled)
                        .map(|i| i.key.clone())
                        .collect()
                } else {
                    Vec::new()
                };
            }
            TransferMessage::ToRight => {
                if self.disabled {
                    return;
                }
                let moving: Vec<String> = self
                    .checked_left
                    .iter()
                    .filter(|k| !self.is_disabled(k))
                    .cloned()
                    .collect();
                for key in &moving {
                    if let Some(item) = self.items.iter_mut().find(|i| &i.key == key) {
                        item.side = Side::Right;
                    }
                }
                self.checked_left.retain(|k| !moving.contains(k));
            }
            TransferMessage::ToLeft => {
                if self.disabled {
                    return;
                }
                let moving: Vec<String> = self
                    .checked_right
                    .iter()
                    .filter(|k| !self.is_disabled(k))
                    .cloned()
                    .collect();
                for key in &moving {
                    if let Some(item) = self.items.iter_mut().find(|i| &i.key == key) {
                        item.side = Side::Left;
                    }
                }
                self.checked_right.retain(|k| !moving.contains(k));
            }
        }
    }

    fn is_disabled(&self, key: &str) -> bool {
        self.items
            .iter()
            .find(|i| i.key == key)
            .map(|i| i.disabled)
            .unwrap_or(false)
    }

    fn toggle_check(items: &[TransferItem], checked: &mut Vec<String>, key: &str, side: Side) {
        // 只允许勾选本侧且未禁用的项
        let ok = items
            .iter()
            .any(|i| i.key == key && i.side == side && !i.disabled);
        if !ok {
            return;
        }
        match checked.iter().position(|k| k == key) {
            Some(pos) => {
                checked.remove(pos);
            }
            None => checked.push(key.to_string()),
        }
    }

    /// 渲染双面板（按钮搬移经 on_msg；勾选列表为文本行 + ☑/☐ 标记）
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        on_msg: impl Fn(TransferMessage) -> Message + Clone + 'a,
    ) -> Element<'a, Message> {
        let _ = &on_msg;
        let text_regular = Color::from(theme.neutral.text_regular);
        let text_disabled = Color::from(theme.neutral.text_placeholder);
        let border = Color::from(theme.neutral.border_lighter);
        let bg = Color::from(theme.neutral.bg_overlay);
        let primary = Color::from(theme.primary.base);

        let panel =
            |title: &str, items: Vec<&TransferItem>, checked: &[String]| -> Element<'a, Message> {
                let mut col: iced::widget::Column<'_, Message> =
                    iced::widget::Column::new().spacing(2);
                col = col.push(
                    text(format!("{}（{} / {}）", title, checked.len(), items.len()))
                        .size(13)
                        .color(text_regular),
                );
                for item in items {
                    let mark = if checked.contains(&item.key) {
                        "☑"
                    } else {
                        "☐"
                    };
                    let color = if item.disabled {
                        text_disabled
                    } else {
                        text_regular
                    };
                    col = col.push(
                        text(format!("{} {}", mark, item.label))
                            .size(13)
                            .color(color),
                    );
                }
                container(col)
                    .width(Length::Fill)
                    .padding(Padding::from(8u16))
                    .style(move |_t| iced::widget::container::Style {
                        text_color: None,
                        background: Some(iced::Background::Color(bg)),
                        border: iced::Border {
                            color: border,
                            width: 1.0,
                            radius: iced::border::radius(4.0),
                        },
                        shadow: iced::Shadow::default(),
                        snap: false,
                    })
                    .into()
            };

        let left_panel = panel(&self.titles.0, self.left(), &self.checked_left);
        let right_panel = panel(&self.titles.1, self.right(), &self.checked_right);

        let buttons = iced::widget::Column::new()
            .spacing(4)
            .push(
                iced::widget::button(text("→").size(13).color(primary))
                    .on_press_maybe((!self.disabled).then(|| on_msg(TransferMessage::ToRight))),
            )
            .push(
                iced::widget::button(text("←").size(13).color(primary))
                    .on_press_maybe((!self.disabled).then(|| on_msg(TransferMessage::ToLeft))),
            );

        iced::widget::Row::new()
            .push(left_panel)
            .push(container(buttons).padding(Padding::from(8u16)))
            .push(right_panel)
            .spacing(8)
            .into()
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    fn sample() -> Transfer {
        Transfer::new(vec![
            TransferItem::new("a", "选项A", Side::Left),
            TransferItem::new("b", "选项B", Side::Left),
            TransferItem::new("c", "选项C", Side::Left).with_disabled(true),
            TransferItem::new("d", "选项D", Side::Right),
        ])
    }

    #[test]
    fn test_initial_split() {
        let t = sample();
        assert_eq!(t.left().len(), 3);
        assert_eq!(t.right().len(), 1);
        assert!(t.checked_left().is_empty());
    }

    #[test]
    fn test_toggle_check_and_move_right() {
        let mut t = sample();
        t.handle(TransferMessage::ToggleCheckLeft("a".to_string()));
        t.handle(TransferMessage::ToggleCheckLeft("b".to_string()));
        assert_eq!(t.checked_left().len(), 2);
        t.handle(TransferMessage::ToRight);
        assert_eq!(t.left().len(), 1); // 只剩禁用的 c
        assert_eq!(t.right().len(), 3);
        // 搬移后勾选清空
        assert!(t.checked_left().is_empty());
    }

    #[test]
    fn test_disabled_item_not_moved_or_checked() {
        let mut t = sample();
        t.handle(TransferMessage::ToggleCheckLeft("c".to_string()));
        assert!(t.checked_left().is_empty(), "禁用项不可勾选");
        // 全选也跳过禁用项
        t.handle(TransferMessage::CheckAllLeft(true));
        assert_eq!(t.checked_left().len(), 2); // a、b，无 c
        // a、b 是合法勾选 → 搬移生效（d+c 之外新增 2 项）
        t.handle(TransferMessage::ToRight);
        assert_eq!(t.right().len(), 3);
        assert_eq!(t.left().len(), 1); // 只剩禁用的 c
    }

    #[test]
    fn test_only_disabled_checked_move_noop() {
        let mut t = sample();
        // 仅勾选禁用项 c → 搬移无效果
        t.handle(TransferMessage::ToggleCheckLeft("c".to_string()));
        assert!(t.checked_left().is_empty());
        t.handle(TransferMessage::ToRight);
        assert_eq!(t.right().len(), 1);
    }

    #[test]
    fn test_move_back_and_forth() {
        let mut t = sample();
        t.handle(TransferMessage::CheckAllLeft(true));
        t.handle(TransferMessage::ToRight);
        assert_eq!(t.right().len(), 3);
        t.handle(TransferMessage::ToggleCheckRight("a".to_string()));
        t.handle(TransferMessage::ToLeft);
        assert_eq!(t.left().len(), 2);
        assert_eq!(t.right().len(), 2);
    }

    #[test]
    fn test_check_all_off() {
        let mut t = sample();
        t.handle(TransferMessage::CheckAllLeft(true));
        assert_eq!(t.checked_left().len(), 2);
        t.handle(TransferMessage::CheckAllLeft(false));
        assert!(t.checked_left().is_empty());
    }

    #[test]
    fn test_cross_side_check_rejected() {
        let mut t = sample();
        // "d" 在右侧，不能勾到左侧
        t.handle(TransferMessage::ToggleCheckLeft("d".to_string()));
        assert!(t.checked_left().is_empty());
    }

    #[test]
    fn test_disabled_blocks_all() {
        let mut t = sample().with_disabled(true);
        t.handle(TransferMessage::ToggleCheckLeft("a".to_string()));
        t.handle(TransferMessage::CheckAllLeft(true));
        t.handle(TransferMessage::ToRight);
        assert_eq!(t.left().len(), 3);
        assert!(t.checked_left().is_empty());
    }
}
