//! 虚拟列表度量 — 无样式、纯 f32 数值、零渲染依赖（ADR-009）
//!
//! 自 `Table` 组件的 VirtualScroll 上收：任意"等高行列表"均可复用。
//! 数学与 v1.1.0 的 Table 实现逐行等价，组件侧以类型别名保持 API 兼容。

/// 虚拟列表视窗配置与滚动状态
///
/// 视窗逻辑：visible_start = floor(scroll_offset / row_height)
///          visible_count = ceil(viewport_height / row_height) + 1（缓冲）
#[derive(Debug, Clone)]
pub struct VirtualList {
    /// 单行高度（像素）
    pub row_height: f32,
    /// 视窗高度（像素）
    pub viewport_height: f32,
    /// 当前滚动偏移（像素）
    pub scroll_offset: f32,
    /// 启用阈值：行数超过此值才启用虚拟滚动
    pub threshold: usize,
}

impl VirtualList {
    pub fn new(row_height: f32, viewport_height: f32) -> Self {
        Self {
            row_height: row_height.max(1.0),
            viewport_height: viewport_height.max(1.0),
            scroll_offset: 0.0,
            threshold: 100,
        }
    }

    pub fn with_threshold(mut self, t: usize) -> Self {
        self.threshold = t;
        self
    }

    /// 是否启用虚拟滚动
    pub fn should_enable(&self, total_rows: usize) -> bool {
        total_rows > self.threshold
    }

    /// 计算可见行索引区间 [start, end)
    pub fn visible_range(&self, total_rows: usize) -> (usize, usize) {
        if total_rows == 0 || self.row_height <= 0.0 {
            return (0, 0);
        }
        let start = (self.scroll_offset / self.row_height).floor() as usize;
        let visible_count = ((self.viewport_height / self.row_height).ceil() as usize) + 1;
        let end = (start.saturating_add(visible_count)).min(total_rows);
        (start.min(total_rows), end)
    }

    /// 总滚动高度
    pub fn total_height(&self, total_rows: usize) -> f32 {
        total_rows as f32 * self.row_height
    }

    /// 钳制 scroll_offset 到 [0, max_offset]
    pub fn clamp_offset(&mut self, total_rows: usize) {
        let max = (total_rows as f32 * self.row_height - self.viewport_height).max(0.0);
        if self.scroll_offset < 0.0 {
            self.scroll_offset = 0.0;
        }
        if self.scroll_offset > max {
            self.scroll_offset = max;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visible_range_and_clamp() {
        let mut vl = VirtualList::new(30.0, 300.0);
        let (s, e) = vl.visible_range(1000);
        assert_eq!((s, e), (0, 11));
        vl.scroll_offset = 150.0;
        assert_eq!(vl.visible_range(1000), (5, 16));
        // 越界钳制：1000×30-300 = 29700
        vl.scroll_offset = 99999.0;
        vl.clamp_offset(1000);
        assert_eq!(vl.scroll_offset, 29700.0);
        vl.scroll_offset = -100.0;
        vl.clamp_offset(1000);
        assert_eq!(vl.scroll_offset, 0.0);
    }

    #[test]
    fn test_threshold_and_empty() {
        let vl = VirtualList::new(30.0, 300.0).with_threshold(50);
        assert!(!vl.should_enable(50));
        assert!(vl.should_enable(51));
        assert_eq!(vl.visible_range(0), (0, 0));
    }

    #[test]
    fn test_total_height() {
        let vl = VirtualList::new(30.0, 300.0);
        assert_eq!(vl.total_height(1000), 30000.0);
    }
}
