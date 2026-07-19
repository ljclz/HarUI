//! Slider 滑块 — 参考 Element Plus `<el-slider>`。
//!
//! 支持：min/max/step、value、disabled、vertical、show_input、show_stops、
//! range、Increase/Decrease、step 吸附、范围钳制与顺序交换。

/// Slider 消息
#[derive(Debug, Clone, PartialEq)]
pub enum SliderMessage {
    /// 设置单值
    SetValue(f64),
    /// 设置范围值（range 模式）
    SetRange(f64, f64),
    /// 增加 step
    Increase,
    /// 减少 step
    Decrease,
}

/// Slider 组件
#[derive(Debug, Clone)]
pub struct Slider {
    min: f64,
    max: f64,
    step: f64,
    value: f64,
    range_value: (f64, f64),
    disabled: bool,
    vertical: bool,
    show_input: bool,
    show_stops: bool,
    show_tooltip: bool,
    range: bool,
}

impl Default for Slider {
    fn default() -> Self {
        Self::new()
    }
}

impl Slider {
    pub fn new() -> Self {
        Self {
            min: 0.0,
            max: 100.0,
            step: 1.0,
            value: 0.0,
            range_value: (0.0, 0.0),
            disabled: false,
            vertical: false,
            show_input: false,
            show_stops: false,
            show_tooltip: true,
            range: false,
        }
    }

    pub fn with_min(mut self, v: f64) -> Self {
        self.min = v;
        self
    }

    pub fn with_max(mut self, v: f64) -> Self {
        self.max = v;
        self
    }

    pub fn with_step(mut self, v: f64) -> Self {
        self.step = v;
        self
    }

    pub fn with_disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }

    pub fn with_vertical(mut self, v: bool) -> Self {
        self.vertical = v;
        self
    }

    pub fn with_show_input(mut self, v: bool) -> Self {
        self.show_input = v;
        self
    }

    pub fn with_show_stops(mut self, v: bool) -> Self {
        self.show_stops = v;
        self
    }

    pub fn with_show_tooltip(mut self, v: bool) -> Self {
        self.show_tooltip = v;
        self
    }

    pub fn with_range(mut self, v: bool) -> Self {
        self.range = v;
        self
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn max(&self) -> f64 {
        self.max
    }

    pub fn step(&self) -> f64 {
        self.step
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn range_value(&self) -> (f64, f64) {
        self.range_value
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn vertical(&self) -> bool {
        self.vertical
    }

    pub fn show_input(&self) -> bool {
        self.show_input
    }

    pub fn show_stops(&self) -> bool {
        self.show_stops
    }

    pub fn show_tooltip(&self) -> bool {
        self.show_tooltip
    }

    pub fn range(&self) -> bool {
        self.range
    }

    /// 将原始值钳制到 [min, max] 并吸附到 step 网格
    fn snap(&self, raw: f64) -> f64 {
        let clamped = raw.clamp(self.min, self.max);
        if self.step <= 0.0 {
            return clamped;
        }
        let n = ((clamped - self.min) / self.step).round();
        let snapped = self.min + n * self.step;
        snapped.clamp(self.min, self.max)
    }

    pub fn handle(&mut self, msg: SliderMessage) {
        if self.disabled {
            return;
        }
        match msg {
            SliderMessage::SetValue(v) => {
                self.value = self.snap(v);
            }
            SliderMessage::SetRange(a, b) => {
                let lo = self.snap(a.min(b));
                let hi = self.snap(a.max(b));
                self.range_value = (lo, hi);
            }
            SliderMessage::Increase => {
                self.value = self.snap(self.value + self.step);
            }
            SliderMessage::Decrease => {
                self.value = self.snap(self.value - self.step);
            }
        }
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_snap_basic() {
        let s = Slider::new().with_min(0.0).with_max(100.0).with_step(10.0);
        assert_eq!(s.snap(23.0), 20.0);
        assert_eq!(s.snap(27.0), 30.0);
    }

    #[test]
    fn test_snap_clamp() {
        let s = Slider::new().with_min(10.0).with_max(50.0).with_step(5.0);
        assert_eq!(s.snap(0.0), 10.0);
        assert_eq!(s.snap(100.0), 50.0);
    }
}
