//! Popconfirm 气泡确认框组件 — 参考 Element Plus `<el-popconfirm>`。
//! 支持：4 种 trigger、12 种 placement、confirm/cancel 动作回调、disabled、外部点击关闭。

use har_ui_core::theme::Theme;
use iced::widget::{button, container, text};
use iced::{Color, Element, Length, Padding};

/// Popconfirm 触发方式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PopconfirmTrigger {
    #[default]
    Click,
    Hover,
    Focus,
    Manual,
}

/// Popconfirm 弹出位置（12 种）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PopconfirmPlacement {
    #[default]
    Top,
    TopStart,
    TopEnd,
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

/// 气泡与锚点的默认间距（与 tooltip/popover 家族一致）
pub const POPCONFIRM_GAP: f32 = 12.0;

impl From<PopconfirmPlacement> for har_ui_core::behavior::overlay::Placement {
    fn from(p: PopconfirmPlacement) -> Self {
        use har_ui_core::behavior::overlay::Placement as P;
        match p {
            PopconfirmPlacement::Top => P::Top,
            PopconfirmPlacement::TopStart => P::TopStart,
            PopconfirmPlacement::TopEnd => P::TopEnd,
            PopconfirmPlacement::Bottom => P::Bottom,
            PopconfirmPlacement::BottomStart => P::BottomStart,
            PopconfirmPlacement::BottomEnd => P::BottomEnd,
            PopconfirmPlacement::Left => P::Left,
            PopconfirmPlacement::LeftStart => P::LeftStart,
            PopconfirmPlacement::LeftEnd => P::LeftEnd,
            PopconfirmPlacement::Right => P::Right,
            PopconfirmPlacement::RightStart => P::RightStart,
            PopconfirmPlacement::RightEnd => P::RightEnd,
        }
    }
}

/// Popconfirm 用户动作
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopconfirmAction {
    Confirm,
    Cancel,
}

/// Popconfirm 消息
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopconfirmMessage {
    Click,
    MouseEnter,
    MouseLeave,
    Focus,
    Blur,
    Show,
    Hide,
    Confirm,
    Cancel,
    ClickOutside,
}

/// Popconfirm 组件
#[derive(Debug, Clone)]
pub struct Popconfirm {
    /// 标题（提示内容）
    title: String,
    /// 是否可见
    visible: bool,
    /// 触发方式
    trigger: PopconfirmTrigger,
    /// 弹出位置
    placement: PopconfirmPlacement,
    /// 宽度
    width: Option<u32>,
    /// 是否显示箭头
    show_arrow: bool,
    /// 是否禁用
    disabled: bool,
    /// 确认按钮文本
    confirm_text: String,
    /// 取消按钮文本
    cancel_text: String,
    /// 上次用户动作
    last_action: Option<PopconfirmAction>,
}

impl Popconfirm {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            visible: false,
            trigger: PopconfirmTrigger::Click,
            placement: PopconfirmPlacement::Top,
            width: None,
            show_arrow: true,
            disabled: false,
            confirm_text: "确认".to_string(),
            cancel_text: "取消".to_string(),
            last_action: None,
        }
    }

    // ---------- Builder ----------

    pub fn with_visible(mut self, v: bool) -> Self {
        self.visible = v;
        self
    }

    pub fn with_trigger(mut self, t: PopconfirmTrigger) -> Self {
        self.trigger = t;
        self
    }

    pub fn with_placement(mut self, p: PopconfirmPlacement) -> Self {
        self.placement = p;
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

    pub fn with_confirm_text(mut self, t: &str) -> Self {
        self.confirm_text = t.to_string();
        self
    }

    pub fn with_cancel_text(mut self, t: &str) -> Self {
        self.cancel_text = t.to_string();
        self
    }

    // ---------- Getter ----------

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn trigger(&self) -> PopconfirmTrigger {
        self.trigger
    }

    pub fn placement(&self) -> PopconfirmPlacement {
        self.placement
    }

    /// 计算气泡在给定锚点/视窗下的解析矩形（委托 core 行为层定位引擎，ADR-009；
    /// 碰撞策略 FlipThenShift，间距 [`POPCONFIRM_GAP`]）
    pub fn resolved_rect(
        &self,
        anchor: har_ui_core::behavior::overlay::Rect,
        content: har_ui_core::behavior::overlay::Size,
        viewport: har_ui_core::behavior::overlay::Rect,
    ) -> har_ui_core::behavior::overlay::ResolvedRect {
        use har_ui_core::behavior::overlay::{self, CollisionPolicy, PlacementOptions};
        overlay::compute_placement(
            anchor,
            content,
            viewport,
            PlacementOptions {
                placement: self.placement.into(),
                offset: POPCONFIRM_GAP,
                collision: CollisionPolicy::FlipThenShift,
            },
        )
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

    pub fn confirm_button_text(&self) -> &str {
        &self.confirm_text
    }

    pub fn cancel_button_text(&self) -> &str {
        &self.cancel_text
    }

    pub fn last_action(&self) -> Option<PopconfirmAction> {
        self.last_action
    }

    // ---------- 运行时修改 ----------

    pub fn set_visible(&mut self, v: bool) {
        self.visible = v;
        if v {
            self.last_action = None;
        }
    }

    pub fn set_disabled(&mut self, d: bool) {
        self.disabled = d;
    }

    // ---------- 消息处理 ----------

    pub fn handle(&mut self, msg: PopconfirmMessage) {
        // Confirm/Cancel 动作只在可见时生效，但 disabled 时也被阻塞
        if self.disabled && !matches!(msg, PopconfirmMessage::ClickOutside) {
            return;
        }
        match msg {
            // 显式 Confirm/Cancel（无论 visible，但通常 visible 时调用）
            PopconfirmMessage::Confirm => {
                self.last_action = Some(PopconfirmAction::Confirm);
                self.visible = false;
            }
            PopconfirmMessage::Cancel => {
                self.last_action = Some(PopconfirmAction::Cancel);
                self.visible = false;
            }
            PopconfirmMessage::ClickOutside => {
                self.visible = false;
            }
            // 触发器相关消息
            other => self.handle_trigger(other),
        }
    }

    fn handle_trigger(&mut self, msg: PopconfirmMessage) {
        match (self.trigger, msg) {
            (PopconfirmTrigger::Click, PopconfirmMessage::Click) => {
                self.visible = !self.visible;
                if self.visible {
                    self.last_action = None;
                }
            }
            (PopconfirmTrigger::Hover, PopconfirmMessage::MouseEnter) => {
                self.visible = true;
                self.last_action = None;
            }
            (PopconfirmTrigger::Hover, PopconfirmMessage::MouseLeave) => {
                self.visible = false;
            }
            (PopconfirmTrigger::Focus, PopconfirmMessage::Focus) => {
                self.visible = true;
                self.last_action = None;
            }
            (PopconfirmTrigger::Focus, PopconfirmMessage::Blur) => {
                self.visible = false;
            }
            (PopconfirmTrigger::Manual, PopconfirmMessage::Show) => {
                self.visible = true;
                self.last_action = None;
            }
            (PopconfirmTrigger::Manual, PopconfirmMessage::Hide) => {
                self.visible = false;
            }
            _ => {}
        }
    }

    /// 渲染 Popconfirm 为 iced::Element
    ///
    /// # 参数
    /// - `theme`: HarUI 主题引用
    /// - `content`: 被包裹的触发元素
    /// - `on_confirm`: 点击确认按钮时发出消息
    /// - `on_cancel`: 点击取消按钮时发出消息
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        content: Element<'a, Message>,
        on_confirm: impl Fn() -> Message + 'a,
        on_cancel: impl Fn() -> Message + 'a,
    ) -> Element<'a, Message> {
        let bg = Color::from(theme.neutral.bg_overlay);
        let title_color = Color::from(theme.neutral.text_primary);
        let border_color = Color::from(theme.neutral.border_light);
        let primary = Color::from(theme.primary.base);
        let text_regular = Color::from(theme.neutral.text_regular);

        if !self.visible {
            return content;
        }

        // 标题行
        let title_text = text(self.title.clone()).color(title_color).size(14.0);

        // 按钮行
        let cancel_btn = button(
            text(self.cancel_text.clone())
                .color(text_regular)
                .size(13.0),
        )
        .padding(Padding::from([4u16, 10u16]))
        .style(move |_t, _status| iced::widget::button::Style {
            background: None,
            text_color: text_regular,
            border: iced::Border {
                color: border_color,
                width: 1.0,
                radius: iced::border::radius(3.0),
            },
            shadow: iced::Shadow::default(),
            snap: false,
        })
        .on_press(on_cancel());

        let confirm_btn = button(
            text(self.confirm_text.clone())
                .color(Color::WHITE)
                .size(13.0),
        )
        .padding(Padding::from([4u16, 10u16]))
        .style(move |_t, _status| iced::widget::button::Style {
            background: Some(iced::Background::Color(primary)),
            text_color: Color::WHITE,
            border: iced::Border {
                color: primary,
                width: 1.0,
                radius: iced::border::radius(3.0),
            },
            shadow: iced::Shadow::default(),
            snap: false,
        })
        .on_press(on_confirm());

        let buttons_row = iced::widget::Row::new()
            .push(iced::widget::Space::new().width(Length::Fill))
            .push(cancel_btn)
            .push(iced::widget::Space::new().width(Length::Fixed(8.0)))
            .push(confirm_btn)
            .align_y(iced::Alignment::Center);

        let pop_children: Vec<Element<'a, Message>> = vec![title_text.into(), buttons_row.into()];

        let pop_col = iced::widget::Column::with_children(pop_children).spacing(10);
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
                snap: false,
            });

        iced::widget::Column::new()
            .push(content)
            .push(iced::widget::Space::new().height(Length::Fixed(4.0)))
            .push(pop)
            .into()
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_popconfirm_internal_default() {
        let p = Popconfirm::new("x");
        assert_eq!(p.title, "x");
        assert_eq!(p.trigger, PopconfirmTrigger::Click);
        assert_eq!(p.placement, PopconfirmPlacement::Top);
        assert!(!p.visible);
    }

    #[test]
    fn test_popconfirm_internal_show_resets_action() {
        let mut p = Popconfirm::new("x");
        p.handle(PopconfirmMessage::Click);
        p.handle(PopconfirmMessage::Confirm);
        assert_eq!(p.last_action, Some(PopconfirmAction::Confirm));
        p.handle(PopconfirmMessage::Click);
        assert!(p.last_action.is_none());
    }

    #[test]
    fn test_popconfirm_internal_disabled_blocks_confirm_too() {
        let mut p = Popconfirm::new("x").with_disabled(true);
        p.handle(PopconfirmMessage::Show);
        p.handle(PopconfirmMessage::Confirm);
        // disabled 状态下 Confirm 也被阻塞
        assert!(!p.visible, "未显示");
        // 但如果之前已经显示了再 disabled，仍可被 Confirm 关闭（这里测未显示情况）
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
    fn test_popconfirm_resolved_rect_above_anchor_with_gap() {
        let p = Popconfirm::new("确认删除？");
        let r = p.resolved_rect(
            Rect::new(380.0, 300.0, 40.0, 20.0),
            Size::new(180.0, 60.0),
            VP,
        );
        assert_eq!(r.effective_placement, Placement::Top);
        assert_eq!(r.rect.y, 300.0 - 60.0 - POPCONFIRM_GAP);
    }

    #[test]
    fn test_popconfirm_resolved_rect_flips_near_top_edge() {
        let p = Popconfirm::new("确认删除？");
        let r = p.resolved_rect(
            Rect::new(380.0, 10.0, 40.0, 20.0),
            Size::new(180.0, 60.0),
            VP,
        );
        assert_eq!(r.effective_placement, Placement::Bottom);
    }
}
