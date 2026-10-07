//! Tooltip 文字提示组件 — 参考 Element Plus `<el-tooltip>`。
//! 支持：4 种 trigger、12 种 placement、dark/light 主题、hide_after 自动关闭、enterable 进入气泡、disabled。

use har_ui_core::theme::Theme;
use iced::widget::{container, text};
use iced::{Color, Element, Length, Padding};

/// Tooltip 触发方式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TooltipTrigger {
    #[default]
    Hover,
    Focus,
    Click,
    Manual,
}

/// Tooltip 弹出位置（12 种，与 Popover 共用语义）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TooltipPlacement {
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

/// 气泡与锚点的默认间距（Element Plus el-tooltip offset 规范值）
pub const TOOLTIP_GAP: f32 = 12.0;

impl From<TooltipPlacement> for har_ui_core::behavior::overlay::Placement {
    fn from(p: TooltipPlacement) -> Self {
        use har_ui_core::behavior::overlay::Placement as P;
        match p {
            TooltipPlacement::Top => P::Top,
            TooltipPlacement::TopStart => P::TopStart,
            TooltipPlacement::TopEnd => P::TopEnd,
            TooltipPlacement::Bottom => P::Bottom,
            TooltipPlacement::BottomStart => P::BottomStart,
            TooltipPlacement::BottomEnd => P::BottomEnd,
            TooltipPlacement::Left => P::Left,
            TooltipPlacement::LeftStart => P::LeftStart,
            TooltipPlacement::LeftEnd => P::LeftEnd,
            TooltipPlacement::Right => P::Right,
            TooltipPlacement::RightStart => P::RightStart,
            TooltipPlacement::RightEnd => P::RightEnd,
        }
    }
}

/// Tooltip 主题效果
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TooltipEffect {
    #[default]
    Dark,
    Light,
}

/// Tooltip 消息
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TooltipMessage {
    Click,
    MouseEnter,
    MouseLeave,
    Focus,
    Blur,
    Show,
    Hide,
    /// 鼠标进入气泡本体（用于 enterable）
    EnterTooltip,
    /// 鼠标离开气泡本体
    LeaveTooltip,
}

/// Tooltip 组件
#[derive(Debug, Clone)]
pub struct Tooltip {
    /// 提示内容
    content: String,
    /// 是否可见
    visible: bool,
    /// 触发方式
    trigger: TooltipTrigger,
    /// 弹出位置
    placement: TooltipPlacement,
    /// 主题
    effect: TooltipEffect,
    /// 是否显示箭头
    show_arrow: bool,
    /// 鼠标是否可进入气泡
    enterable: bool,
    /// 是否禁用
    disabled: bool,
    /// 自动隐藏延迟（毫秒），None 表示不自动关闭
    hide_after: Option<u64>,
    /// 距上次显示以来的累计毫秒（用于 hide_after 计时）
    elapsed_since_show: u64,
    /// 鼠标当前是否在气泡内
    inside_tooltip: bool,
}

impl Tooltip {
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_string(),
            visible: false,
            trigger: TooltipTrigger::Hover,
            placement: TooltipPlacement::Top,
            effect: TooltipEffect::Dark,
            show_arrow: true,
            enterable: true,
            disabled: false,
            hide_after: None,
            elapsed_since_show: 0,
            inside_tooltip: false,
        }
    }

    // ---------- Builder ----------

    pub fn with_visible(mut self, v: bool) -> Self {
        self.visible = v;
        self
    }

    pub fn with_trigger(mut self, t: TooltipTrigger) -> Self {
        self.trigger = t;
        self
    }

    pub fn with_placement(mut self, p: TooltipPlacement) -> Self {
        self.placement = p;
        self
    }

    pub fn with_effect(mut self, e: TooltipEffect) -> Self {
        self.effect = e;
        self
    }

    pub fn with_show_arrow(mut self, s: bool) -> Self {
        self.show_arrow = s;
        self
    }

    pub fn with_enterable(mut self, e: bool) -> Self {
        self.enterable = e;
        self
    }

    pub fn with_disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }

    pub fn with_hide_after(mut self, ms: u64) -> Self {
        self.hide_after = Some(ms);
        self
    }

    // ---------- Getter ----------

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn trigger(&self) -> TooltipTrigger {
        self.trigger
    }

    pub fn placement(&self) -> TooltipPlacement {
        self.placement
    }

    /// 计算气泡在给定锚点/视窗下的解析矩形（委托 core 行为层定位引擎，ADR-009）
    ///
    /// iced 0.13 无绝对定位 widget，本 API 供应用层手工定位、弹出方向动画判定与测试断言；
    /// 碰撞策略 FlipThenShift（空间不足先翻转对侧，仍越界则钳回视窗）。
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
                offset: TOOLTIP_GAP,
                collision: CollisionPolicy::FlipThenShift,
            },
        )
    }

    pub fn effect(&self) -> TooltipEffect {
        self.effect
    }

    pub fn show_arrow(&self) -> bool {
        self.show_arrow
    }

    pub fn enterable(&self) -> bool {
        self.enterable
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn hide_after(&self) -> Option<u64> {
        self.hide_after
    }

    // ---------- 运行时修改 ----------

    pub fn set_visible(&mut self, v: bool) {
        self.visible = v;
        if v {
            self.elapsed_since_show = 0;
        }
    }

    pub fn set_disabled(&mut self, d: bool) {
        self.disabled = d;
    }

    /// 时间推进（用于 hide_after 自动关闭模拟）
    pub fn tick(&mut self, ms: u64) {
        if !self.visible {
            return;
        }
        self.elapsed_since_show += ms;
        if let Some(limit) = self.hide_after
            && self.elapsed_since_show >= limit
        {
            self.visible = false;
        }
    }

    // ---------- 消息处理 ----------

    pub fn handle(&mut self, msg: TooltipMessage) {
        if self.disabled {
            return;
        }
        match msg {
            TooltipMessage::EnterTooltip => {
                if self.enterable {
                    self.inside_tooltip = true;
                }
            }
            TooltipMessage::LeaveTooltip => {
                self.inside_tooltip = false;
                if self.enterable && !self.is_trigger_inside() {
                    self.visible = false;
                }
            }
            _ => self.handle_trigger(msg),
        }
    }

    /// 判断当前 trigger 状态下，"鼠标在内"是否意味着应保持显示
    fn is_trigger_inside(&self) -> bool {
        false
    }

    fn handle_trigger(&mut self, msg: TooltipMessage) {
        // enterable 模式下，鼠标在气泡内时 MouseLeave 不应隐藏
        if msg == TooltipMessage::MouseLeave && self.enterable && self.inside_tooltip {
            return;
        }
        match (self.trigger, msg) {
            (TooltipTrigger::Hover, TooltipMessage::MouseEnter) => {
                self.show();
            }
            (TooltipTrigger::Hover, TooltipMessage::MouseLeave) => {
                self.hide();
            }
            (TooltipTrigger::Focus, TooltipMessage::Focus) => {
                self.show();
            }
            (TooltipTrigger::Focus, TooltipMessage::Blur) => {
                self.hide();
            }
            (TooltipTrigger::Click, TooltipMessage::Click) => {
                if self.visible {
                    self.hide();
                } else {
                    self.show();
                }
            }
            (TooltipTrigger::Manual, TooltipMessage::Show) => {
                self.show();
            }
            (TooltipTrigger::Manual, TooltipMessage::Hide) => {
                self.hide();
            }
            _ => {}
        }
    }

    fn show(&mut self) {
        self.visible = true;
        self.elapsed_since_show = 0;
    }

    fn hide(&mut self) {
        self.visible = false;
    }

    /// 渲染 Tooltip 为 iced::Element
    ///
    /// # 参数
    /// - `theme`: HarUI 主题引用
    /// - `content`: 被包裹的触发元素
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        content: Element<'a, Message>,
    ) -> Element<'a, Message> {
        if !self.visible || self.disabled {
            return content;
        }

        let (bg, fg) = match self.effect {
            TooltipEffect::Dark => (
                Color::from(theme.neutral.bg_overlay),
                Color::from(theme.neutral.text_primary),
            ),
            TooltipEffect::Light => (
                Color::from(theme.neutral.text_primary),
                Color::from(theme.neutral.bg_overlay),
            ),
        };

        let tip_text = text(self.content.clone()).color(fg).size(12.0);
        let tip = container(tip_text)
            .padding(Padding::from([6u16, 10u16]))
            .style(move |_t| iced::widget::container::Style {
                text_color: Some(fg),
                background: Some(iced::Background::Color(bg)),
                border: iced::Border {
                    color: bg,
                    width: 0.0,
                    radius: iced::border::radius(4.0),
                },
                shadow: har_ui_core::theme::style_sheets::iced_shadow(&theme.shadow.lighter),
                snap: false,
            });

        let arrow = if self.show_arrow {
            Some(text("▲").color(bg).size(8.0))
        } else {
            None
        };

        let mut tip_col_children: Vec<Element<'a, Message>> = Vec::new();
        if let Some(arrow) = arrow {
            tip_col_children.push(arrow.into());
        }
        tip_col_children.push(tip.into());
        let tip_col = iced::widget::Column::with_children(tip_col_children).spacing(0);

        let col: Element<'a, Message> = iced::widget::Column::new()
            .push(content)
            .push(iced::widget::Space::new().height(Length::Fixed(4.0)))
            .push(tip_col)
            .into();
        col
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_tooltip_internal_default() {
        let t = Tooltip::new("x");
        assert_eq!(t.content, "x");
        assert_eq!(t.trigger, TooltipTrigger::Hover);
        assert_eq!(t.placement, TooltipPlacement::Top);
        assert_eq!(t.effect, TooltipEffect::Dark);
        assert!(t.show_arrow);
        assert!(t.enterable);
    }

    #[test]
    fn test_tooltip_internal_show_resets_timer() {
        let mut t = Tooltip::new("x").with_hide_after(1000);
        t.show();
        t.tick(500);
        assert_eq!(t.elapsed_since_show, 500);
        t.show();
        assert_eq!(t.elapsed_since_show, 0);
    }

    #[test]
    fn test_tooltip_internal_enterable_blocks_leave() {
        let mut t = Tooltip::new("x").with_enterable(true);
        t.show();
        t.handle(TooltipMessage::EnterTooltip);
        t.handle(TooltipMessage::MouseLeave);
        assert!(t.visible, "在气泡内时 MouseLeave 不应隐藏");
    }

    // ============ 定位引擎接入（ADR-009） ============

    use har_ui_core::behavior::overlay::{Rect, Size};

    const VP: Rect = Rect {
        x: 0.0,
        y: 0.0,
        width: 800.0,
        height: 600.0,
    };

    #[test]
    fn test_tooltip_resolved_rect_delegates_placement() {
        let t = Tooltip::new("x").with_placement(TooltipPlacement::Bottom);
        let r = t.resolved_rect(
            Rect::new(380.0, 300.0, 40.0, 20.0),
            Size::new(100.0, 30.0),
            VP,
        );
        // Bottom：锚点下方 + gap 12
        assert_eq!(
            r.effective_placement,
            har_ui_core::behavior::overlay::Placement::Bottom
        );
        assert_eq!(r.rect.y, 320.0 + TOOLTIP_GAP);
    }

    #[test]
    fn test_tooltip_resolved_rect_flips_near_top_edge() {
        // 锚点贴顶：上方空间 < 需求 → 翻转到下方
        let t = Tooltip::new("x"); // 默认 Top
        let r = t.resolved_rect(
            Rect::new(380.0, 10.0, 40.0, 20.0),
            Size::new(100.0, 30.0),
            VP,
        );
        assert_eq!(
            r.effective_placement,
            har_ui_core::behavior::overlay::Placement::Bottom
        );
    }

    #[test]
    fn test_tooltip_resolved_rect_clamped_into_viewport() {
        // 超宽气泡 + 角落锚点：FlipThenShift 钳回视窗
        let t = Tooltip::new("x");
        let r = t.resolved_rect(
            Rect::new(10.0, 10.0, 40.0, 20.0),
            Size::new(600.0, 30.0),
            VP,
        );
        assert!(r.rect.x >= 0.0 && r.rect.right() <= VP.right());
        assert!(r.rect.bottom() <= VP.bottom());
    }
}
