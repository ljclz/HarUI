//! Carousel 走马灯 — 参考 Element Plus `<el-carousel>`。
//!
//! 支持：自动播放、间隔、循环、方向、指示器、箭头、hover 暂停、tick 时间推进。

/// 方向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CarouselDirection {
    #[default]
    Horizontal,
    Vertical,
}

/// Carousel 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CarouselMessage {
    Next,
    Prev,
    JumpTo(usize),
}

/// Carousel 组件
#[derive(Debug, Clone)]
pub struct Carousel {
    slides: Vec<String>,
    current_index: usize,
    autoplay: bool,
    interval: u64,
    loop_enabled: bool,
    direction: CarouselDirection,
    show_indicator: bool,
    show_arrow: bool,
    pause_on_hover: bool,
    hovered: bool,
    /// 累计时间（用于自动播放推进）
    elapsed: u64,
}

impl Default for Carousel {
    fn default() -> Self {
        Self::new()
    }
}

impl Carousel {
    pub fn new() -> Self {
        Self {
            slides: Vec::new(),
            current_index: 0,
            autoplay: false,
            interval: 3000,
            loop_enabled: true,
            direction: CarouselDirection::Horizontal,
            show_indicator: true,
            show_arrow: true,
            pause_on_hover: false,
            hovered: false,
            elapsed: 0,
        }
    }

    pub fn with_slide(mut self, s: impl Into<String>) -> Self {
        self.slides.push(s.into());
        self
    }

    pub fn with_autoplay(mut self, v: bool) -> Self {
        self.autoplay = v;
        self
    }

    pub fn with_interval(mut self, ms: u64) -> Self {
        self.interval = ms.max(1);
        self
    }

    pub fn with_loop(mut self, v: bool) -> Self {
        self.loop_enabled = v;
        self
    }

    pub fn with_direction(mut self, d: CarouselDirection) -> Self {
        self.direction = d;
        self
    }

    pub fn with_show_indicator(mut self, v: bool) -> Self {
        self.show_indicator = v;
        self
    }

    pub fn with_show_arrow(mut self, v: bool) -> Self {
        self.show_arrow = v;
        self
    }

    pub fn with_pause_on_hover(mut self, v: bool) -> Self {
        self.pause_on_hover = v;
        self
    }

    pub fn set_hovered(&mut self, v: bool) {
        self.hovered = v;
    }

    pub fn slides(&self) -> &[String] {
        &self.slides
    }

    pub fn current_index(&self) -> usize {
        self.current_index
    }

    pub fn autoplay(&self) -> bool {
        self.autoplay
    }

    pub fn interval(&self) -> u64 {
        self.interval
    }

    pub fn direction(&self) -> CarouselDirection {
        self.direction
    }

    pub fn show_indicator(&self) -> bool {
        self.show_indicator
    }

    pub fn show_arrow(&self) -> bool {
        self.show_arrow
    }

    pub fn handle(&mut self, msg: CarouselMessage) {
        if self.slides.is_empty() {
            return;
        }
        match msg {
            CarouselMessage::Next => self.advance(),
            CarouselMessage::Prev => self.retreat(),
            CarouselMessage::JumpTo(idx) => {
                let max = self.slides.len() - 1;
                self.current_index = idx.min(max);
            }
        }
    }

    fn advance(&mut self) {
        if self.slides.is_empty() {
            return;
        }
        let max = self.slides.len() - 1;
        if self.current_index < max {
            self.current_index += 1;
        } else if self.loop_enabled {
            self.current_index = 0;
        }
    }

    fn retreat(&mut self) {
        if self.slides.is_empty() {
            return;
        }
        if self.current_index > 0 {
            self.current_index -= 1;
        } else if self.loop_enabled {
            self.current_index = self.slides.len() - 1;
        }
    }

    /// 模拟时间推进（用于自动播放测试）
    pub fn tick(&mut self, ms: u64) {
        if !self.autoplay || self.slides.is_empty() {
            return;
        }
        if self.pause_on_hover && self.hovered {
            return;
        }
        self.elapsed += ms;
        while self.elapsed >= self.interval {
            self.elapsed -= self.interval;
            self.advance();
        }
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_carousel_default_interval_3000() {
        let c = Carousel::new();
        assert_eq!(c.interval(), 3000);
        assert!(c.show_indicator());
        assert!(c.show_arrow());
        assert!(c.loop_enabled);
    }
}
