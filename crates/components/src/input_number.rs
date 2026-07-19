//! InputNumber 组件 — 数字输入框
//!
//! 参考 Element Plus `<el-input-number>` 组件。
//! 支持 min/max/step/precision 以及 +/- 按钮和直接输入。

/// 控件位置
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ControlsPosition {
    /// 默认：两侧
    #[default]
    Default,
    /// 右侧：上下箭头
    Right,
}

/// InputNumber 消息
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputNumberMessage {
    /// + 按钮
    Increment,
    /// - 按钮
    Decrement,
    /// 直接设置值
    SetValue(f64),
}

/// InputNumber 组件
#[derive(Debug, Clone)]
pub struct InputNumber {
    value: f64,
    min: Option<f64>,
    max: Option<f64>,
    step: f64,
    precision: Option<u8>,
    disabled: bool,
    controls_position: ControlsPosition,
}

impl InputNumber {
    pub fn new() -> Self {
        Self {
            value: 0.0,
            min: None,
            max: None,
            step: 1.0,
            precision: None,
            disabled: false,
            controls_position: ControlsPosition::Default,
        }
    }

    pub fn with_min(mut self, v: f64) -> Self {
        self.min = Some(v);
        self
    }

    pub fn with_max(mut self, v: f64) -> Self {
        self.max = Some(v);
        self
    }

    pub fn with_step(mut self, v: f64) -> Self {
        self.step = v;
        self
    }

    pub fn with_precision(mut self, p: u8) -> Self {
        self.precision = Some(p);
        self
    }

    pub fn disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }

    pub fn with_controls_position(mut self, p: ControlsPosition) -> Self {
        self.controls_position = p;
        self
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn min(&self) -> Option<f64> {
        self.min
    }

    pub fn max(&self) -> Option<f64> {
        self.max
    }

    pub fn step(&self) -> f64 {
        self.step
    }

    pub fn precision(&self) -> Option<u8> {
        self.precision
    }

    pub fn is_disabled(&self) -> bool {
        self.disabled
    }

    pub fn controls_position(&self) -> ControlsPosition {
        self.controls_position
    }

    /// 处理消息
    pub fn handle(&mut self, msg: InputNumberMessage) {
        if self.disabled {
            return;
        }

        match msg {
            InputNumberMessage::Increment => {
                let new_val = self.apply_precision(self.value + self.step);
                self.value = self.clamp(new_val);
            }
            InputNumberMessage::Decrement => {
                let new_val = self.apply_precision(self.value - self.step);
                self.value = self.clamp(new_val);
            }
            InputNumberMessage::SetValue(v) => {
                let v = self.apply_precision(v);
                self.value = self.clamp(v);
            }
        }
    }

    /// 应用 precision 四舍五入
    fn apply_precision(&self, v: f64) -> f64 {
        if let Some(p) = self.precision {
            let factor = 10f64.powi(p as i32);
            (v * factor).round() / factor
        } else {
            v
        }
    }

    /// 限制值在 [min, max] 范围内
    fn clamp(&self, v: f64) -> f64 {
        let v = self.min.map_or(v, |min| v.max(min));
        let v = self.max.map_or(v, |max| v.min(max));
        v
    }
}

impl Default for InputNumber {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_default_props() {
        let inp = InputNumber::new();
        assert_eq!(inp.value(), 0.0);
        assert_eq!(inp.step(), 1.0);
        assert_eq!(inp.min(), None);
        assert_eq!(inp.max(), None);
        assert_eq!(inp.precision(), None);
        assert!(!inp.is_disabled());
        assert_eq!(inp.controls_position(), ControlsPosition::Default);
    }

    #[test]
    fn test_apply_precision_with_no_precision() {
        let inp = InputNumber::new();
        assert_eq!(inp.apply_precision(3.14159), 3.14159);
    }

    #[test]
    fn test_apply_precision_with_two_decimal() {
        let inp = InputNumber::new().with_precision(2);
        assert!((inp.apply_precision(3.14159) - 3.14).abs() < 0.001);
    }
}
