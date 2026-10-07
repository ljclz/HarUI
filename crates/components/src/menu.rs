//! Menu 组件 — 导航菜单
//!
//! 参考 Element Plus `<el-menu>`。
//! 支持：horizontal/vertical 模式、collapse 折叠、多级嵌套、选中态、disabled、unique_opened。

use har_ui_core::theme::Theme;
use iced::widget::{button, container, text};
use iced::{Color, Element, Length, Padding};

/// 菜单模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MenuMode {
    #[default]
    Vertical,
    Horizontal,
}

/// 菜单项
#[derive(Debug, Clone)]
pub struct MenuItem {
    pub id: String,
    pub label: String,
    pub disabled: bool,
    pub children: Vec<MenuItem>,
}

impl MenuItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            disabled: false,
            children: Vec::new(),
        }
    }

    pub fn disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }

    pub fn with_child(mut self, child: MenuItem) -> Self {
        self.children.push(child);
        self
    }

    /// 是否为子菜单（有子项）
    pub fn is_submenu(&self) -> bool {
        !self.children.is_empty()
    }
}

/// 菜单消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MenuMessage {
    /// 选中项
    Select(String),
    /// 切换子菜单展开/收起
    ToggleSubmenu(String),
    /// 关闭其他子菜单
    CloseOthers(String),
    /// 折叠/展开整个菜单（vertical 模式）
    Collapse(bool),
}

/// 菜单状态
#[derive(Debug, Clone)]
pub struct MenuState {
    mode: MenuMode,
    collapsed: bool,
    unique_opened: bool,
    active: Option<String>,
    opened_submenus: Vec<String>,
    /// 已注册的菜单项（用于查 disabled）
    items: Vec<MenuItem>,
}

impl Default for MenuState {
    fn default() -> Self {
        Self::new()
    }
}

impl MenuState {
    pub fn new() -> Self {
        Self {
            mode: MenuMode::Vertical,
            collapsed: false,
            unique_opened: false,
            active: None,
            opened_submenus: Vec::new(),
            items: Vec::new(),
        }
    }

    pub fn with_mode(mut self, m: MenuMode) -> Self {
        self.mode = m;
        self
    }

    pub fn with_collapse(mut self, v: bool) -> Self {
        // horizontal 模式下 collapse 无效
        if self.mode == MenuMode::Horizontal {
            return self;
        }
        self.collapsed = v;
        if v {
            self.opened_submenus.clear();
        }
        self
    }

    pub fn with_unique_opened(mut self, v: bool) -> Self {
        self.unique_opened = v;
        self
    }

    pub fn mode(&self) -> MenuMode {
        self.mode
    }

    pub fn collapsed(&self) -> bool {
        self.collapsed
    }

    pub fn unique_opened(&self) -> bool {
        self.unique_opened
    }

    pub fn active(&self) -> Option<&String> {
        self.active.as_ref()
    }

    pub fn opened_submenus(&self) -> &[String] {
        &self.opened_submenus
    }

    /// 注册菜单项（用于查 disabled）
    pub fn register_item(&mut self, item: MenuItem) {
        self.items.push(item);
    }

    /// 查找某 id 是否 disabled
    fn is_disabled(&self, id: &str) -> bool {
        fn find<'a>(items: &'a [MenuItem], id: &str) -> Option<&'a MenuItem> {
            for i in items {
                if i.id == id {
                    return Some(i);
                }
                if let Some(found) = find(&i.children, id) {
                    return Some(found);
                }
            }
            None
        }
        find(&self.items, id).is_some_and(|i| i.disabled)
    }

    /// 处理消息
    pub fn handle(&mut self, msg: MenuMessage) {
        match msg {
            MenuMessage::Select(id) => {
                // 折叠状态或 disabled 不能选中
                if self.collapsed {
                    return;
                }
                if self.is_disabled(&id) {
                    return;
                }
                self.active = Some(id);
            }
            MenuMessage::ToggleSubmenu(id) => {
                if self.collapsed {
                    return;
                }
                if let Some(pos) = self.opened_submenus.iter().position(|s| s == &id) {
                    self.opened_submenus.remove(pos);
                } else {
                    if self.unique_opened {
                        self.opened_submenus.clear();
                    }
                    self.opened_submenus.push(id);
                }
            }
            MenuMessage::CloseOthers(keep) => {
                self.opened_submenus.retain(|s| s == &keep);
            }
            MenuMessage::Collapse(v) => {
                if self.mode == MenuMode::Horizontal {
                    return;
                }
                self.collapsed = v;
                if v {
                    self.opened_submenus.clear();
                }
            }
        }
    }

    /// 渲染 MenuState 为 iced::Element
    ///
    /// - items 为空时返回空容器
    /// - 折叠状态下渲染 "[menu collapsed]" 占位
    /// - 否则按 mode 排列所有 items（递归渲染子菜单）
    /// - on_select 参数为叶子节点的 id
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        on_select: impl Fn(String) -> Message + 'a + Clone,
    ) -> Element<'a, Message> {
        if self.items.is_empty() {
            return container(text("")).into();
        }

        if self.collapsed {
            let indicator = container(
                text("[collapsed]")
                    .color(Color::from(theme.neutral.text_secondary))
                    .size(12.0),
            )
            .padding(Padding::from([8u16, 12u16]));
            return indicator.into();
        }

        let children: Vec<Element<'a, Message>> = self
            .items
            .iter()
            .map(|item| Self::render_item(item, self, theme, &on_select, 0))
            .collect();

        match self.mode {
            MenuMode::Horizontal => iced::widget::Row::with_children(children).spacing(0).into(),
            MenuMode::Vertical => iced::widget::Column::with_children(children)
                .spacing(0)
                .into(),
        }
    }

    /// 递归渲染单个 MenuItem
    fn render_item<'a, Message: Clone + 'a>(
        item: &'a MenuItem,
        state: &'a MenuState,
        theme: &'a Theme,
        on_select: &(impl Fn(String) -> Message + 'a + Clone),
        depth: u32,
    ) -> Element<'a, Message> {
        let text_regular = Color::from(theme.neutral.text_regular);
        let text_disabled = Color::from(theme.neutral.text_disabled);
        let primary = Color::from(theme.primary.base);
        let border_lighter = Color::from(theme.neutral.border_lighter);

        let is_active = state.active.as_deref() == Some(item.id.as_str());
        let is_disabled = item.disabled;
        let label_color = if is_disabled {
            text_disabled
        } else if is_active {
            primary
        } else {
            text_regular
        };

        // 子菜单指示器
        let indicator_str = if item.is_submenu() {
            if state.opened_submenus.iter().any(|s| s == &item.id) {
                "▼"
            } else {
                "▶"
            }
        } else {
            ""
        };

        let mut row_children: Vec<Element<'a, Message>> = Vec::new();
        // 缩进
        if depth > 0 {
            row_children.push(
                iced::widget::Space::new()
                    .width(Length::Fixed(depth as f32 * 16.0))
                    .into(),
            );
        }
        if !indicator_str.is_empty() {
            row_children.push(text(indicator_str).color(label_color).size(12.0).into());
            row_children.push(iced::widget::Space::new().width(Length::Fixed(6.0)).into());
        }
        row_children.push(
            text(item.label.clone())
                .color(label_color)
                .size(14.0)
                .into(),
        );

        let content = iced::widget::Row::with_children(row_children)
            .align_y(iced::Alignment::Center)
            .spacing(0);

        let mut btn = button(content)
            .padding(Padding::from([10u16, 16u16]))
            .width(Length::Fill)
            .style(move |_t, _status| iced::widget::button::Style {
                background: if is_active {
                    Some(iced::Background::Color(Color { a: 0.08, ..primary }))
                } else {
                    None
                },
                text_color: label_color,
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
                snap: false,
            });

        if !is_disabled {
            btn = btn.on_press((on_select)(item.id.clone()));
        }

        let mut col_children: Vec<Element<'a, Message>> = Vec::new();
        col_children.push(btn.into());

        // 子菜单展开
        if item.is_submenu() && state.opened_submenus.iter().any(|s| s == &item.id) {
            for child in &item.children {
                col_children.push(Self::render_item(child, state, theme, on_select, depth + 1));
            }
        }

        let col = iced::widget::Column::with_children(col_children).spacing(0);

        container(col)
            .width(Length::Fill)
            .style(move |_t| iced::widget::container::Style {
                text_color: None,
                background: None,
                border: iced::Border {
                    color: border_lighter,
                    width: 0.0,
                    radius: iced::border::radius(0.0),
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
    fn test_menu_item_is_submenu() {
        let leaf = MenuItem::new("leaf", "Leaf");
        assert!(!leaf.is_submenu());

        let parent = MenuItem::new("parent", "Parent").with_child(leaf);
        assert!(parent.is_submenu());
    }

    #[test]
    fn test_menu_state_disabled_lookup() {
        let mut s = MenuState::new();
        s.register_item(MenuItem::new("m1", "M1").disabled(true));
        s.register_item(MenuItem::new("m2", "M2"));
        assert!(s.is_disabled("m1"));
        assert!(!s.is_disabled("m2"));
        assert!(!s.is_disabled("nonexistent"));
    }

    #[test]
    fn test_menu_state_collapse_clears_submenus() {
        let mut s = MenuState::new();
        s.handle(MenuMessage::ToggleSubmenu("sub-1".to_string()));
        assert!(!s.opened_submenus().is_empty());
        s.handle(MenuMessage::Collapse(true));
        assert!(s.opened_submenus().is_empty());
    }
}
