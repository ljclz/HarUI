//! Tour 漫游式引导 — 参考 Element Plus `<el-tour>`
//!
//! 支持：步骤序列（标题 + 描述 + 可选目标标识）、上一步/下一步/跳过/完成、
//! 遮罩点击关闭（可配）、最后一步 Next 即完成关闭、空步骤防护。
//!
//! ## 语义说明
//! 目标定位（target 元素高亮框）需要应用层提供元素矩形——本组件管理引导的
//! 流程状态机与文案渲染，`current_target()` 返回当前步骤的目标标识供
//! 应用层绘制高亮（与 iced 无 DOM 查询的现实一致）。

use har_ui_core::theme::Theme;
use iced::Element;

/// 引导步骤
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TourStep {
    pub title: String,
    pub description: String,
    /// 目标标识（应用层元素；None 表示居中通用步骤）
    pub target: Option<String>,
}

impl TourStep {
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            target: None,
        }
    }

    pub fn with_target(mut self, target: impl Into<String>) -> Self {
        self.target = Some(target.into());
        self
    }
}

/// Tour 消息
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TourMessage {
    /// 打开引导（从第一步开始）
    Open,
    /// 关闭引导（跳过）
    Close,
    /// 遮罩点击（mask_click_close 时等效 Close）
    MaskClicked,
    /// 下一步（最后一步时关闭并标记完成）
    Next,
    /// 上一步
    Prev,
    /// 跳转到指定步（越界钳制）
    Goto(usize),
}

/// Tour 组件
#[derive(Debug, Clone)]
pub struct Tour {
    steps: Vec<TourStep>,
    current: usize,
    open: bool,
    mask_click_close: bool,
    /// 本次引导是否完整走完（Next 到最后一步）
    completed: bool,
}

impl Default for Tour {
    fn default() -> Self {
        Self::new()
    }
}

impl Tour {
    pub fn new() -> Self {
        Self {
            steps: Vec::new(),
            current: 0,
            open: false,
            mask_click_close: true,
            completed: false,
        }
    }

    pub fn with_step(mut self, step: TourStep) -> Self {
        self.steps.push(step);
        self
    }

    /// 遮罩点击是否关闭（默认 true）
    pub fn with_mask_click_close(mut self, v: bool) -> Self {
        self.mask_click_close = v;
        self
    }

    pub fn steps(&self) -> &[TourStep] {
        &self.steps
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn completed(&self) -> bool {
        self.completed
    }

    /// 当前步索引（未打开时为 0）
    pub fn current(&self) -> usize {
        self.current
    }

    /// 进度文案（如 "2 / 5"）；空步骤返回 "-"
    pub fn progress(&self) -> String {
        if self.steps.is_empty() {
            return "-".to_string();
        }
        format!("{} / {}", self.current + 1, self.steps.len())
    }

    /// 当前步骤的目标标识（应用层据此绘制高亮）
    pub fn current_target(&self) -> Option<&str> {
        self.steps
            .get(self.current)
            .and_then(|s| s.target.as_deref())
    }

    /// 处理消息
    pub fn handle(&mut self, msg: TourMessage) {
        match msg {
            TourMessage::Open => {
                if !self.steps.is_empty() {
                    self.open = true;
                    self.current = 0;
                    self.completed = false;
                }
            }
            TourMessage::Close => {
                self.open = false;
            }
            TourMessage::MaskClicked => {
                if self.mask_click_close {
                    self.open = false;
                }
            }
            TourMessage::Next => {
                if !self.open || self.steps.is_empty() {
                    return;
                }
                if self.current + 1 >= self.steps.len() {
                    self.open = false;
                    self.completed = true;
                } else {
                    self.current += 1;
                }
            }
            TourMessage::Prev => {
                if self.open {
                    self.current = self.current.saturating_sub(1);
                }
            }
            TourMessage::Goto(i) => {
                if self.open && !self.steps.is_empty() {
                    self.current = i.min(self.steps.len() - 1);
                }
            }
        }
    }

    /// 渲染引导卡片（标题/描述/进度/上一步/下一步/跳过）
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        on_msg: impl Fn(TourMessage) -> Message + Clone + 'a,
    ) -> Element<'a, Message> {
        use iced::widget::{button, column, container, row, text};
        use iced::{Color, Length, Padding};
        if !self.open || self.steps.is_empty() {
            return iced::widget::Column::new().into();
        }
        let step = &self.steps[self.current];
        let title_color = Color::from(theme.neutral.text_primary);
        let desc_color = Color::from(theme.neutral.text_regular);
        let secondary = Color::from(theme.neutral.text_secondary);
        let border = Color::from(theme.neutral.border_lighter);
        let bg = Color::from(theme.neutral.bg_overlay);
        let primary = Color::from(theme.primary.base);

        let first = self.current == 0;
        let last = self.current + 1 >= self.steps.len();
        let prev_btn = button(text("上一步").size(13).color(secondary))
            .on_press_maybe((!first).then(|| on_msg(TourMessage::Prev)))
            .padding(Padding::from([4u16, 10u16]));
        let next_label = if last { "完成" } else { "下一步" };
        let next_btn = button(text(next_label).size(13).color(Color::WHITE))
            .on_press(on_msg(TourMessage::Next))
            .padding(Padding::from([4u16, 10u16]))
            .style(move |_t, _s| button::Style {
                background: Some(iced::Background::Color(primary)),
                border: iced::Border::default(),
                ..button::Style::default()
            });
        let skip_btn = button(text("跳过").size(13).color(secondary))
            .on_press(on_msg(TourMessage::Close))
            .padding(Padding::from([4u16, 10u16]));

        let card = column![
            text(step.title.clone()).size(16).color(title_color),
            text(step.description.clone()).size(13).color(desc_color),
            row![
                text(self.progress()).size(12).color(secondary),
                iced::widget::Space::new().width(iced::Length::Fill),
                skip_btn,
                prev_btn,
                next_btn,
            ]
            .spacing(8)
            .align_y(iced::Alignment::Center),
        ]
        .spacing(10)
        .padding(Padding::from(16u16));

        container(card)
            .width(Length::Fixed(320.0))
            .style(move |_t| iced::widget::container::Style {
                text_color: None,
                background: Some(iced::Background::Color(bg)),
                border: iced::Border {
                    color: border,
                    width: 1.0,
                    radius: iced::border::radius(8.0),
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

    fn sample() -> Tour {
        Tour::new()
            .with_step(TourStep::new("欢迎", "这是第一步").with_target("sidebar"))
            .with_step(TourStep::new("结算", "点击结算按钮").with_target("checkout-btn"))
            .with_step(TourStep::new("完成", "引导结束"))
    }

    #[test]
    fn test_open_starts_at_first() {
        let mut t = sample();
        assert!(!t.is_open());
        t.handle(TourMessage::Open);
        assert!(t.is_open());
        assert_eq!(t.current(), 0);
        assert_eq!(t.current_target(), Some("sidebar"));
        assert_eq!(t.progress(), "1 / 3");
    }

    #[test]
    fn test_open_empty_steps_noop() {
        let mut t = Tour::new();
        t.handle(TourMessage::Open);
        assert!(!t.is_open(), "空步骤不可打开");
        t.handle(TourMessage::Next);
        assert!(!t.is_open());
    }

    #[test]
    fn test_next_advances_and_finishes() {
        let mut t = sample();
        t.handle(TourMessage::Open);
        t.handle(TourMessage::Next);
        assert_eq!(t.current(), 1);
        assert_eq!(t.current_target(), Some("checkout-btn"));
        t.handle(TourMessage::Next);
        assert_eq!(t.current(), 2);
        // 最后一步 Next → 关闭且标记完成
        t.handle(TourMessage::Next);
        assert!(!t.is_open());
        assert!(t.completed());
    }

    #[test]
    fn test_prev_clamped_at_first() {
        let mut t = sample();
        t.handle(TourMessage::Open);
        t.handle(TourMessage::Prev);
        assert_eq!(t.current(), 0, "第一步 Prev 钳制");
        t.handle(TourMessage::Next);
        t.handle(TourMessage::Prev);
        assert_eq!(t.current(), 0);
    }

    #[test]
    fn test_goto_clamped() {
        let mut t = sample();
        t.handle(TourMessage::Open);
        t.handle(TourMessage::Goto(1));
        assert_eq!(t.current(), 1);
        t.handle(TourMessage::Goto(99));
        assert_eq!(t.current(), 2, "越界钳制到最后一步");
    }

    #[test]
    fn test_mask_click_close_config() {
        let mut t = sample().with_mask_click_close(true);
        t.handle(TourMessage::Open);
        t.handle(TourMessage::MaskClicked);
        assert!(!t.is_open());
        // mask_click_close=false 时遮罩点击不关
        let mut t2 = sample().with_mask_click_close(false);
        t2.handle(TourMessage::Open);
        t2.handle(TourMessage::MaskClicked);
        assert!(t2.is_open());
    }

    #[test]
    fn test_skip_vs_complete() {
        let mut t = sample();
        t.handle(TourMessage::Open);
        t.handle(TourMessage::Close); // 跳过
        assert!(!t.is_open());
        assert!(!t.completed(), "跳过不算完成");
        t.handle(TourMessage::Open);
        t.handle(TourMessage::Goto(2));
        t.handle(TourMessage::Next); // 完整走完
        assert!(t.completed());
        // 重新 Open 重置 completed
        t.handle(TourMessage::Open);
        assert!(!t.completed());
        assert_eq!(t.current(), 0);
    }

    #[test]
    fn test_view_renders_closed_and_open() {
        let theme = Theme::element_light();
        let t = sample();
        let _closed = t.view::<()>(&theme, |_| ());
        let mut o = sample();
        o.handle(TourMessage::Open);
        let _open = o.view::<()>(&theme, |_| ());
    }
}
