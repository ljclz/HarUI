//! Tooltip 文字提示组件 — 参考 Element Plus `<el-tooltip>`。
//! 支持：4 种 trigger、12 种 placement、dark/light 主题、hide_after 自动关闭、enterable 进入气泡、disabled。

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
        if let Some(limit) = self.hide_after {
            if self.elapsed_since_show >= limit {
                self.visible = false;
            }
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
}
