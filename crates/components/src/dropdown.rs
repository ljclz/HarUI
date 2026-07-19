//! Dropdown 下拉菜单 — 参考 Element Plus `<el-dropdown>`。
//!
//! 支持：3 种 trigger（hover/click/contextmenu）、菜单项点击、disabled、divided、hide_on_click、手动显示隐藏。

/// 触发方式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DropdownTrigger {
    #[default]
    Hover,
    Click,
    ContextMenu,
}

/// 菜单项
#[derive(Debug, Clone)]
pub struct DropdownItem {
    command: String,
    label: String,
    disabled: bool,
    divided: bool,
}

impl DropdownItem {
    pub fn new(command: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            command: command.into(),
            label: label.into(),
            disabled: false,
            divided: false,
        }
    }

    pub fn with_disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }

    pub fn with_divided(mut self, v: bool) -> Self {
        self.divided = v;
        self
    }

    pub fn command(&self) -> &str {
        &self.command
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn divided(&self) -> bool {
        self.divided
    }
}

/// Dropdown 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DropdownMessage {
    Show,
    Hide,
    Click,
    ClickOutside,
    ContextMenu,
    MouseEnter,
    MouseLeave,
    Select(usize),
    ClearCommand,
}

/// Dropdown 组件
#[derive(Debug, Clone)]
pub struct Dropdown {
    items: Vec<DropdownItem>,
    trigger: DropdownTrigger,
    visible: bool,
    hide_on_click: bool,
    last_command: Option<String>,
}

impl Default for Dropdown {
    fn default() -> Self {
        Self::new()
    }
}

impl Dropdown {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            trigger: DropdownTrigger::Hover,
            visible: false,
            hide_on_click: true,
            last_command: None,
        }
    }

    pub fn with_item(mut self, item: DropdownItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn with_trigger(mut self, t: DropdownTrigger) -> Self {
        self.trigger = t;
        self
    }

    pub fn with_hide_on_click(mut self, v: bool) -> Self {
        self.hide_on_click = v;
        self
    }

    pub fn items(&self) -> &[DropdownItem] {
        &self.items
    }

    pub fn trigger(&self) -> DropdownTrigger {
        self.trigger
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn last_command(&self) -> Option<&str> {
        self.last_command.as_deref()
    }

    pub fn handle(&mut self, msg: DropdownMessage) {
        match msg {
            DropdownMessage::Show => self.visible = true,
            DropdownMessage::Hide => self.visible = false,
            DropdownMessage::Click => {
                if self.trigger == DropdownTrigger::Click {
                    self.visible = !self.visible;
                }
            }
            DropdownMessage::ClickOutside => {
                if self.trigger == DropdownTrigger::Click {
                    self.visible = false;
                }
            }
            DropdownMessage::ContextMenu => {
                if self.trigger == DropdownTrigger::ContextMenu {
                    self.visible = !self.visible;
                }
            }
            DropdownMessage::MouseEnter => {
                if self.trigger == DropdownTrigger::Hover {
                    self.visible = true;
                }
            }
            DropdownMessage::MouseLeave => {
                if self.trigger == DropdownTrigger::Hover {
                    self.visible = false;
                }
            }
            DropdownMessage::Select(idx) => {
                if let Some(item) = self.items.get(idx) {
                    if !item.disabled {
                        self.last_command = Some(item.command.clone());
                        if self.hide_on_click {
                            self.visible = false;
                        }
                    }
                }
            }
            DropdownMessage::ClearCommand => {
                self.last_command = None;
            }
        }
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_dropdown_default_hide_on_click_true() {
        let d = Dropdown::new();
        assert!(d.hide_on_click);
        assert_eq!(d.trigger(), DropdownTrigger::Hover);
    }
}
