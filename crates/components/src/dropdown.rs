//! Dropdown 下拉菜单 — 参考 Element Plus `<el-dropdown>`。
//!
//! 支持：3 种 trigger（hover/click/contextmenu）、菜单项点击、disabled、divided、hide_on_click、手动显示隐藏。

use har_ui_core::theme::Theme;
use iced::widget::{button, container, text};
use iced::{Color, Element, Length, Padding};

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

    /// 计算下拉菜单在给定锚点/视窗下的解析矩形（委托 core 行为层，ADR-009）：
    /// 默认锚点下方展开，下方空间不足自动翻转到上方（Element Plus dropdown 语义）
    pub fn resolved_rect(
        &self,
        anchor: har_ui_core::behavior::overlay::Rect,
        menu: har_ui_core::behavior::overlay::Size,
        viewport: har_ui_core::behavior::overlay::Rect,
    ) -> har_ui_core::behavior::overlay::ResolvedRect {
        har_ui_core::behavior::overlay::dropdown_placement(anchor, menu, viewport, 4.0)
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
                if let Some(item) = self.items.get(idx)
                    && !item.disabled
                {
                    self.last_command = Some(item.command.clone());
                    if self.hide_on_click {
                        self.visible = false;
                    }
                }
            }
            DropdownMessage::ClearCommand => {
                self.last_command = None;
            }
        }
    }

    /// 渲染 Dropdown 为 iced::Element
    ///
    /// # 参数
    /// - `theme`: HarUI 主题引用
    /// - `on_trigger`: 点击触发器（click/contextmenu 模式）时发出消息
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        on_trigger: impl Fn() -> Message + 'a,
    ) -> Element<'a, Message> {
        let text_primary = Color::from(theme.neutral.text_primary);
        let text_regular = Color::from(theme.neutral.text_regular);
        let text_disabled = Color::from(theme.neutral.text_disabled);
        let border_light = Color::from(theme.neutral.border_light);

        // 触发器：显示当前 last_command 或默认 "Dropdown"
        let trigger_label = self
            .last_command
            .clone()
            .unwrap_or_else(|| "Dropdown".to_string());
        let arrow = if self.visible { "▲" } else { "▼" };

        let trigger_content = iced::widget::Row::new()
            .push(text(trigger_label).color(text_primary).size(14.0))
            .push(iced::widget::Space::with_width(Length::Fixed(6.0)))
            .push(text(arrow).color(text_regular).size(12.0))
            .align_y(iced::Alignment::Center);

        let mut trigger_btn = button(trigger_content)
            .padding(Padding::from([8u16, 12u16]))
            .style(move |_t, _status| iced::widget::button::Style {
                background: None,
                text_color: text_primary,
                border: iced::Border {
                    color: border_light,
                    width: 1.0,
                    radius: iced::border::radius(4.0),
                },
                shadow: iced::Shadow::default(),
            });
        // hover 模式不需要点击触发
        if !matches!(self.trigger, DropdownTrigger::Hover) {
            trigger_btn = trigger_btn.on_press(on_trigger());
        }

        let mut outer_children: Vec<Element<'a, Message>> = Vec::new();
        outer_children.push(trigger_btn.into());

        // 下拉菜单（visible 时显示）
        if self.visible && !self.items.is_empty() {
            let mut menu_children: Vec<Element<'a, Message>> = Vec::new();
            for item in &self.items {
                if item.divided {
                    let divider = container(text(""))
                        .width(Length::Fill)
                        .height(Length::Fixed(1.0))
                        .style(move |_t| iced::widget::container::Style {
                            text_color: None,
                            background: Some(iced::Background::Color(border_light)),
                            border: iced::Border::default(),
                            shadow: iced::Shadow::default(),
                        });
                    menu_children.push(divider.into());
                }

                let item_color = if item.disabled {
                    text_disabled
                } else {
                    text_regular
                };
                let item_text = text(item.label().to_string()).color(item_color).size(14.0);
                let item_wrap = container(item_text)
                    .width(Length::Fill)
                    .padding(Padding::from([8u16, 16u16]))
                    .style(move |_t| iced::widget::container::Style {
                        text_color: Some(item_color),
                        background: None,
                        border: iced::Border::default(),
                        shadow: iced::Shadow::default(),
                    });
                menu_children.push(item_wrap.into());
            }

            let menu = iced::widget::Column::with_children(menu_children).spacing(0);

            let menu_wrap = container(menu)
                .width(Length::Fixed(180.0))
                .padding(Padding::from(4u16))
                .style(move |_t| iced::widget::container::Style {
                    text_color: None,
                    background: Some(iced::Background::Color(Color::WHITE)),
                    border: iced::Border {
                        color: border_light,
                        width: 1.0,
                        radius: iced::border::radius(4.0),
                    },
                    shadow: iced::Shadow::default(),
                });

            outer_children.push(iced::widget::Space::with_height(Length::Fixed(4.0)).into());
            outer_children.push(menu_wrap.into());
        }

        iced::widget::Column::with_children(outer_children)
            .spacing(0)
            .into()
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

    // ============ 定位引擎接入（ADR-009） ============

    use har_ui_core::behavior::overlay::{Placement, Rect, Size};

    const VP: Rect = Rect {
        x: 0.0,
        y: 0.0,
        width: 800.0,
        height: 600.0,
    };

    #[test]
    fn test_dropdown_resolved_rect_flip_up_near_bottom() {
        let d = Dropdown::new();
        let up = d.resolved_rect(
            Rect::new(380.0, 540.0, 100.0, 32.0),
            Size::new(160.0, 120.0),
            VP,
        );
        // 锚点贴底：下方空间不足 → 翻转到上方，菜单底缘距锚点顶缘 4px
        assert_eq!(up.effective_placement, Placement::Top);
        assert_eq!(up.rect.bottom(), 540.0 - 4.0);
    }
}
