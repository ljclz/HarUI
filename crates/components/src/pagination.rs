//! Pagination 分页组件 — 参考 Element Plus `<el-pagination>`。
//! 支持：total/page-size 控制、page-sizes 切换、页码按钮折叠（含省略号）、上一页/下一页边界。

use har_ui_core::theme::Theme;
use har_ui_core::theme::style_sheets::{self, ButtonKind};
use iced::widget::{button, container, text};
use iced::{Color, Element, Padding};

/// Pagination 消息
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaginationMessage {
    Next,
    Prev,
    JumpTo(i64),
    ChangePageSize(u32),
}

/// Pagination 组件
#[derive(Debug, Clone)]
pub struct Pagination {
    total: u64,
    page_size: u32,
    current_page: u64,
    page_sizes: Vec<u32>,
}

impl Pagination {
    pub fn new(total: u64, page_size: u32) -> Self {
        Self {
            total,
            page_size: page_size.max(1),
            current_page: 1,
            page_sizes: vec![],
        }
    }

    pub fn with_page_sizes(mut self, sizes: Vec<u32>) -> Self {
        self.page_sizes = sizes;
        self
    }

    pub fn total(&self) -> u64 {
        self.total
    }

    pub fn page_size(&self) -> u32 {
        self.page_size
    }

    pub fn current_page(&self) -> u64 {
        self.current_page
    }

    pub fn page_sizes(&self) -> &[u32] {
        &self.page_sizes
    }

    /// 总页数（至少 1）
    pub fn total_pages(&self) -> u64 {
        if self.page_size == 0 || self.total == 0 {
            return 1;
        }
        self.total.div_ceil(self.page_size as u64).max(1)
    }

    /// 计算页码按钮（0 表示省略号占位）
    pub fn page_buttons(&self) -> Vec<u64> {
        let total_pages = self.total_pages();
        let cur = self.current_page;
        // 小于等于 7 页时直接显示全部
        if total_pages <= 7 {
            return (1..=total_pages).collect();
        }
        let mut btns: Vec<u64> = Vec::new();
        // 始终显示第 1 页
        btns.push(1);
        // 当前页接近开头：1 2 3 4 5 ... N
        if cur <= 4 {
            btns.extend([2, 3, 4, 5]);
            btns.push(0); // 省略号
            btns.push(total_pages);
        } else if cur >= total_pages - 3 {
            // 当前页接近末尾：1 ... N-4 N-3 N-2 N-1 N
            btns.push(0);
            btns.extend([
                total_pages - 4,
                total_pages - 3,
                total_pages - 2,
                total_pages - 1,
                total_pages,
            ]);
        } else {
            // 中间：1 ... cur-1 cur cur+1 ... N
            btns.push(0);
            btns.extend([cur - 1, cur, cur + 1]);
            btns.push(0);
            btns.push(total_pages);
        }
        btns
    }

    /// 处理消息
    pub fn handle(&mut self, msg: PaginationMessage) {
        match msg {
            PaginationMessage::Next => {
                if self.current_page < self.total_pages() {
                    self.current_page += 1;
                }
            }
            PaginationMessage::Prev => {
                if self.current_page > 1 {
                    self.current_page -= 1;
                }
            }
            PaginationMessage::JumpTo(page) => {
                if page < 1 {
                    self.current_page = 1;
                } else if page as u64 > self.total_pages() {
                    self.current_page = self.total_pages();
                } else {
                    self.current_page = page as u64;
                }
            }
            PaginationMessage::ChangePageSize(size) => {
                if size > 0 {
                    self.page_size = size;
                    self.current_page = 1; // 重置到第一页
                }
            }
        }
    }

    /// 渲染 Pagination 为 iced::Element
    ///
    /// # 参数
    /// - `theme`: HarUI 主题引用
    /// - `on_jump`: 跳转页码时发出消息，参数为目标页码（i64，负数表示 prev/next 也可用 -1/-2）
    ///   约定：`-1` = Prev, `-2` = Next
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        on_jump: impl Fn(i64) -> Message + 'a,
    ) -> Element<'a, Message> {
        let text_color = Color::from(theme.neutral.text_regular);
        let primary = Color::from(theme.primary.base);
        let text_disabled = Color::from(theme.neutral.text_disabled);
        let cur = self.current_page;
        let total = self.total_pages();
        let buttons = self.page_buttons();

        let mut children: Vec<Element<'a, Message>> = Vec::new();

        // 上一页按钮
        let prev_disabled = cur <= 1;
        let mut prev_btn = button(text("〈").color(if prev_disabled {
            text_disabled
        } else {
            text_color
        }))
        .padding(Padding::from([6u16, 10u16]))
        .style(move |_t, status| {
            style_sheets::button_style(theme, ButtonKind::Default, false, status)
        });
        if !prev_disabled {
            prev_btn = prev_btn.on_press(on_jump(-1));
        }
        children.push(prev_btn.into());

        // 页码按钮
        for btn_page in &buttons {
            if *btn_page == 0 {
                // 省略号
                children.push(
                    container(text("...").color(text_disabled))
                        .padding(Padding::from([6u16, 8u16]))
                        .into(),
                );
            } else {
                let is_current = *btn_page == cur;
                let page_text = btn_page.to_string();
                let page_color = if is_current { Color::WHITE } else { text_color };
                let page_i64 = *btn_page as i64;
                let mut page_btn = button(text(page_text).color(page_color))
                    .padding(Padding::from([6u16, 10u16]))
                    .style(move |_t, status| {
                        if is_current {
                            // 当前页：实心 primary
                            iced::widget::button::Style {
                                background: Some(iced::Background::Color(primary)),
                                text_color: Color::WHITE,
                                border: iced::Border::default(),
                                shadow: iced::Shadow::default(),
                                snap: false,
                            }
                        } else {
                            style_sheets::button_style(theme, ButtonKind::Default, false, status)
                        }
                    });
                if !is_current {
                    page_btn = page_btn.on_press(on_jump(page_i64));
                }
                children.push(page_btn.into());
            }
        }

        // 下一页按钮
        let next_disabled = cur >= total;
        let mut next_btn = button(text("〉").color(if next_disabled {
            text_disabled
        } else {
            text_color
        }))
        .padding(Padding::from([6u16, 10u16]))
        .style(move |_t, status| {
            style_sheets::button_style(theme, ButtonKind::Default, false, status)
        });
        if !next_disabled {
            next_btn = next_btn.on_press(on_jump(-2));
        }
        children.push(next_btn.into());

        iced::widget::Row::with_children(children)
            .spacing(4)
            .align_y(iced::Alignment::Center)
            .into()
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_pagination_default_page_size_clamped() {
        let p = Pagination::new(100, 0);
        assert_eq!(p.page_size(), 1); // 0 自动变 1
    }

    #[test]
    fn test_pagination_total_pages_calc() {
        assert_eq!(Pagination::new(100, 10).total_pages(), 10);
        assert_eq!(Pagination::new(105, 10).total_pages(), 11);
        assert_eq!(Pagination::new(0, 10).total_pages(), 1);
        // page_size=0 在 new() 中已被 clamp 为 1，所以 100 条 / 1 = 100 页
        assert_eq!(Pagination::new(100, 0).total_pages(), 100);
    }

    #[test]
    fn test_pagination_buttons_layout_middle() {
        let mut p = Pagination::new(200, 10);
        p.handle(PaginationMessage::JumpTo(10));
        let btns = p.page_buttons();
        // 应包含 1 ... 9 10 11 ... 20
        assert!(btns.contains(&1));
        assert!(btns.contains(&9));
        assert!(btns.contains(&10));
        assert!(btns.contains(&11));
        assert!(btns.contains(&20));
        assert!(btns.contains(&0)); // 省略号
    }
}
