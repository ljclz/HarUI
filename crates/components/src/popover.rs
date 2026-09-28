//! Popover 气泡组件 — 参考 Element Plus `<el-popover>`。
//! 支持：4 种 trigger（click/hover/focus/manual）、12 种 placement、title/content/width/show-arrow/disabled、外部点击关闭。

use har_ui_core::theme::Theme;
use iced::widget::{button, container, text};
use iced::{Color, Element, Length, Padding};

/// Popover 触发方式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PopoverTrigger {
    #[default]
    Click,
    Hover,
    Focus,
    Manual,
}

/// Popover 弹出位置（12 种）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PopoverPlacement {
    Top,
    TopStart,
    TopEnd,
    #[default]
    Bottom,
    BottomStart,
    BottomEnd,
    Left,
    LeftStart,
    LeftEnd,
    Right,
    RightStart,
    RightEnd,
}

impl PopoverPlacement {
    /// 转为 Element Plus 风格字符串
    pub fn as_str(self) -> &'static str {
        match self {
            PopoverPlacement::Top => "top",
            PopoverPlacement::TopStart => "top-start",
            PopoverPlacement::TopEnd => "top-end",
            PopoverPlacement::Bottom => "bottom",
            PopoverPlacement::BottomStart => "bottom-start",
            PopoverPlacement::BottomEnd => "bottom-end",
            PopoverPlacement::Left => "left",
            PopoverPlacement::LeftStart => "left-start",
            PopoverPlacement::LeftEnd => "left-end",
            PopoverPlacement::Right => "right",
            PopoverPlacement::RightStart => "right-start",
            PopoverPlacement::RightEnd => "right-end",
        }
    }
}

/// Popover 消息
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopoverMessage {
    /// 鼠标点击触发元素（trigger=click 时切换显示）
    Click,
    /// 鼠标移入（trigger=hover 时显示）
    MouseEnter,
    /// 鼠标移出（trigger=hover 时隐藏）
    MouseLeave,
    /// 获得焦点（trigger=focus 时显示）
    Focus,
    /// 失去焦点（trigger=focus 时隐藏）
    Blur,
    /// 显式显示（trigger=manual 时使用）
    Show,
    /// 显式隐藏（trigger=manual 时使用）
    Hide,
    /// 点击外部（trigger=click 时关闭气泡）
    ClickOutside,
}

/// Popover 组件
#[derive(Debug, Clone)]
pub struct Popover {
    /// 是否可见
    visible: bool,
    /// 触发方式
    trigger: PopoverTrigger,
    /// 弹出位置
    placement: PopoverPlacement,
    /// 标题
    title: Option<String>,
    /// 内容
    content: Option<String>,
    /// 宽度
    width: Option<u32>,
    /// 是否显示箭头
    show_arrow: bool,
    /// 是否禁用
    disabled: bool,
}

impl Default for Popover {
    fn default() -> Self {
        Self::new()
    }
}

impl Popover {
    pub fn new() -> Self {
        Self {
            visible: false,
            trigger: PopoverTrigger::Click,
            placement: PopoverPlacement::Bottom,
            title: None,
            content: None,
            width: None,
            show_arrow: true,
            disabled: false,
        }
    }

    // ---------- Builder ----------

    pub fn with_visible(mut self, v: bool) -> Self {
        self.visible = v;
        self
    }

    pub fn with_trigger(mut self, t: PopoverTrigger) -> Self {
        self.trigger = t;
        self
    }

    pub fn with_placement(mut self, p: PopoverPlacement) -> Self {
        self.placement = p;
        self
    }

    pub fn with_title(mut self, t: &str) -> Self {
        self.title = Some(t.to_string());
        self
    }

    pub fn with_content(mut self, c: &str) -> Self {
        self.content = Some(c.to_string());
        self
    }

    pub fn with_width(mut self, w: u32) -> Self {
        self.width = Some(w);
        self
    }

    pub fn with_show_arrow(mut self, s: bool) -> Self {
        self.show_arrow = s;
        self
    }

    pub fn with_disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }

    // ---------- Getter ----------

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn trigger(&self) -> PopoverTrigger {
        self.trigger
    }

    pub fn placement(&self) -> PopoverPlacement {
        self.placement
    }

    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    pub fn content(&self) -> Option<&str> {
        self.content.as_deref()
    }

    pub fn width(&self) -> Option<u32> {
        self.width
    }

    pub fn show_arrow(&self) -> bool {
        self.show_arrow
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    // ---------- 运行时修改 ----------

    pub fn set_visible(&mut self, v: bool) {
        self.visible = v;
    }

    pub fn set_disabled(&mut self, d: bool) {
        self.disabled = d;
    }

    // ---------- 消息处理 ----------

    pub fn handle(&mut self, msg: PopoverMessage) {
        // disabled 状态下所有交互都被阻塞（除非是显式 Show 但 disabled 仍阻塞）
        if self.disabled {
            return;
        }
        match (self.trigger, msg) {
            // Click 触发：点击切换显示
            (PopoverTrigger::Click, PopoverMessage::Click) => {
                self.visible = !self.visible;
            }
            // Click 触发：外部点击关闭
            (PopoverTrigger::Click, PopoverMessage::ClickOutside) => {
                self.visible = false;
            }
            // Hover 触发：进入显示，离开隐藏
            (PopoverTrigger::Hover, PopoverMessage::MouseEnter) => {
                self.visible = true;
            }
            (PopoverTrigger::Hover, PopoverMessage::MouseLeave) => {
                self.visible = false;
            }
            // Focus 触发
            (PopoverTrigger::Focus, PopoverMessage::Focus) => {
                self.visible = true;
            }
            (PopoverTrigger::Focus, PopoverMessage::Blur) => {
                self.visible = false;
            }
            // Manual 触发：显式控制
            (PopoverTrigger::Manual, PopoverMessage::Show) => {
                self.visible = true;
            }
            (PopoverTrigger::Manual, PopoverMessage::Hide) => {
                self.visible = false;
            }
            _ => {}
        }
    }

    /// 渲染 Popover 为 iced::Element
    ///
    /// # 参数
    /// - `theme`: HarUI 主题引用
    /// - `content`: 被包裹的触发元素
    /// - `on_trigger`: 点击触发元素时发出消息
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        content: Element<'a, Message>,
        on_trigger: impl Fn() -> Message + 'a,
    ) -> Element<'a, Message> {
        let bg = Color::from(theme.neutral.bg_overlay);
        let title_color = Color::from(theme.neutral.text_primary);
        let content_color = Color::from(theme.neutral.text_regular);
        let border_color = Color::from(theme.neutral.border_light);

        let trigger_btn = button(content)
            .padding(Padding::from([2u16, 4u16]))
            .style(move |_t, _status| iced::widget::button::Style {
                background: None,
                text_color: title_color,
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
            })
            .on_press(on_trigger());

        if !self.visible {
            return trigger_btn.into();
        }

        // 构造气泡内容
        let mut pop_children: Vec<Element<'a, Message>> = Vec::new();
        if let Some(t) = &self.title {
            pop_children.push(text(t.clone()).color(title_color).size(15.0).into());
        }
        if let Some(c) = &self.content {
            pop_children.push(text(c.clone()).color(content_color).size(13.0).into());
        }
        if self.show_arrow {
            pop_children.push(text("▲").color(bg).size(8.0).into());
        }

        let pop_col = iced::widget::Column::with_children(pop_children).spacing(4);
        let pop_width = match self.width {
            Some(w) => Length::Fixed(w as f32),
            None => Length::Shrink,
        };
        let pop = container(pop_col)
            .width(pop_width)
            .padding(Padding::from([10u16, 14u16]))
            .style(move |_t| iced::widget::container::Style {
                text_color: None,
                background: Some(iced::Background::Color(bg)),
                border: iced::Border {
                    color: border_color,
                    width: 1.0,
                    radius: iced::border::radius(4.0),
                },
                shadow: har_ui_core::theme::style_sheets::iced_shadow(&theme.shadow.light),
            });

        iced::widget::Column::new()
            .push(trigger_btn)
            .push(iced::widget::Space::with_height(Length::Fixed(4.0)))
            .push(pop)
            .into()
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_popover_internal_default() {
        let p = Popover::new();
        assert!(!p.visible);
        assert_eq!(p.trigger, PopoverTrigger::Click);
        assert_eq!(p.placement, PopoverPlacement::Bottom);
        assert!(p.show_arrow);
    }

    #[test]
    fn test_popover_internal_disabled_blocks_all() {
        let mut p = Popover::new().with_disabled(true);
        p.handle(PopoverMessage::Click);
        assert!(!p.visible);
        p.handle(PopoverMessage::Show);
        assert!(!p.visible);
    }

    #[test]
    fn test_popover_internal_trigger_msg_matrix() {
        // Click trigger 只响应 Click/ClickOutside
        let mut p = Popover::new().with_trigger(PopoverTrigger::Click);
        p.handle(PopoverMessage::MouseEnter);
        assert!(!p.visible);
        p.handle(PopoverMessage::Focus);
        assert!(!p.visible);
        p.handle(PopoverMessage::Show);
        assert!(!p.visible);
    }
}
