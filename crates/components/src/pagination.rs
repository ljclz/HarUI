//! Pagination 分页组件 — 参考 Element Plus `<el-pagination>`。
//! 支持：total/page-size 控制、page-sizes 切换、页码按钮折叠（含省略号）、上一页/下一页边界。

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
        ((self.total + self.page_size as u64 - 1) / self.page_size as u64).max(1)
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
            btns.extend([total_pages - 4, total_pages - 3, total_pages - 2, total_pages - 1, total_pages]);
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
