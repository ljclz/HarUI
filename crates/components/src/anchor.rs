//! Anchor 锚点 — 参考 Element Plus `<el-anchor>`
//!
//! 支持：注册链接（href + 标题）、点击高亮当前项、滚动联动
//! （应用层从滚动位置回发 SetCurrent）、_affix 固定由外层容器承担。
//!
//! ## 语义说明
//! 点击仅切换高亮并把 href 回传给应用（应用负责真实滚动——iced 无 DOM 滚动锚点）；
//! `SetCurrent` 用于滚动位置联动，未知 href 忽略（高亮只落在已注册链接上）。

use har_ui_core::theme::Theme;
use iced::widget::{container, text};
use iced::{Color, Element, Length, Padding};

/// 锚点链接
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnchorLink {
    pub href: String,
    pub title: String,
}

impl AnchorLink {
    pub fn new(href: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            href: href.into(),
            title: title.into(),
        }
    }
}

/// Anchor 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnchorMessage {
    /// 点击链接（高亮切换 + href 回传应用滚动）
    Click(String),
    /// 滚动联动：设置当前高亮（未知 href 忽略）
    SetCurrent(String),
}

/// Anchor 组件
#[derive(Debug, Clone, Default)]
pub struct Anchor {
    links: Vec<AnchorLink>,
    current: Option<String>,
}

impl Anchor {
    pub fn new() -> Self {
        Self::default()
    }

    /// 注册链接（追加）
    pub fn with_link(mut self, href: impl Into<String>, title: impl Into<String>) -> Self {
        self.links.push(AnchorLink::new(href, title));
        self
    }

    pub fn links(&self) -> &[AnchorLink] {
        &self.links
    }

    /// 当前高亮 href
    pub fn current(&self) -> Option<&str> {
        self.current.as_deref()
    }

    /// 处理消息
    pub fn handle(&mut self, msg: AnchorMessage) {
        match msg {
            AnchorMessage::Click(href) => {
                if self.links.iter().any(|l| l.href == href) {
                    self.current = Some(href);
                }
            }
            AnchorMessage::SetCurrent(href) => {
                if self.links.iter().any(|l| l.href == href) {
                    self.current = Some(href);
                }
            }
        }
    }

    /// 渲染：竖向链接列表，当前项高亮 + 左侧指示条
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        on_msg: impl Fn(AnchorMessage) -> Message + 'a,
    ) -> Element<'a, Message> {
        let text_regular = Color::from(theme.neutral.text_regular);
        let primary = Color::from(theme.primary.base);
        let border = Color::from(theme.neutral.border_lighter);

        // 把链接行包上点击（需要 per-link on_press）
        let mut clickable = iced::widget::Column::new()
            .spacing(4)
            .padding(Padding::from([4u16, 8u16]));
        for link in &self.links {
            let active = self.current.as_deref() == Some(link.href.as_str());
            let color = if active { primary } else { text_regular };
            let marker = if active { "▍" } else { " " };
            let row = container(
                text(format!("{} {}", marker, link.title))
                    .size(13)
                    .color(color),
            )
            .width(Length::Fill)
            .padding(Padding::from([4u16, 4u16]));
            clickable = clickable.push(
                iced::widget::mouse_area(row)
                    .on_press(on_msg(AnchorMessage::Click(link.href.clone()))),
            );
        }
        container(clickable)
            .width(Length::Fixed(160.0))
            .style(move |_t| iced::widget::container::Style {
                text_color: None,
                background: None,
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

    fn sample() -> Anchor {
        Anchor::new()
            .with_link("section-1", "第一章")
            .with_link("section-2", "第二章")
            .with_link("section-3", "第三章")
    }

    #[test]
    fn test_initial_no_current() {
        let a = sample();
        assert_eq!(a.current(), None);
        assert_eq!(a.links().len(), 3);
    }

    #[test]
    fn test_click_sets_current() {
        let mut a = sample();
        a.handle(AnchorMessage::Click("section-2".to_string()));
        assert_eq!(a.current(), Some("section-2"));
        // 已知 href 重复点击切换
        a.handle(AnchorMessage::Click("section-1".to_string()));
        assert_eq!(a.current(), Some("section-1"));
    }

    #[test]
    fn test_unknown_href_ignored() {
        let mut a = sample();
        a.handle(AnchorMessage::Click("nope".to_string()));
        assert_eq!(a.current(), None);
        a.handle(AnchorMessage::Click("section-1".to_string()));
        a.handle(AnchorMessage::SetCurrent("nope".to_string()));
        assert_eq!(a.current(), Some("section-1"));
    }

    #[test]
    fn test_set_current_scroll_link() {
        let mut a = sample();
        a.handle(AnchorMessage::SetCurrent("section-3".to_string()));
        assert_eq!(a.current(), Some("section-3"));
    }

    #[test]
    fn test_empty_anchor_click_ignored() {
        let mut a = Anchor::new();
        a.handle(AnchorMessage::Click("x".to_string()));
        assert_eq!(a.current(), None);
    }
}
