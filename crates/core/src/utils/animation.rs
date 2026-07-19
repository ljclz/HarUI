//! 动画工具函数
//!
//! 提供缓动曲线（easing）计算和数值插值。

/// 动画曲线类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationCurve {
    /// 线性
    Linear,
    /// 缓入（先慢后快）
    EaseIn,
    /// 缓出（先快后慢）
    EaseOut,
    /// 缓入缓出（两端慢中间快）
    EaseInOut,
}

/// 计算缓动曲线在 t (0..=1) 处的值
pub fn easing(curve: AnimationCurve, t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    match curve {
        AnimationCurve::Linear => t,
        AnimationCurve::EaseIn => t * t,
        AnimationCurve::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
        AnimationCurve::EaseInOut => {
            if t < 0.5 {
                2.0 * t * t
            } else {
                1.0 - 2.0 * (1.0 - t) * (1.0 - t)
            }
        }
    }
}

/// 在 start 和 end 之间按进度 t (0..=1) 线性插值
pub fn interpolate(start: f32, end: f32, t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    start + (end - start) * t
}

/// 在 start 和 end 之间按进度 t (0..=1) 用指定曲线插值
pub fn interpolate_with_curve(start: f32, end: f32, t: f32, curve: AnimationCurve) -> f32 {
    let eased_t = easing(curve, t);
    start + (end - start) * eased_t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpolate_clamps_t() {
        // t < 0 应被 clamp 到 0
        assert_eq!(interpolate(10.0, 20.0, -0.5), 10.0);
        // t > 1 应被 clamp 到 1
        assert_eq!(interpolate(10.0, 20.0, 1.5), 20.0);
    }

    #[test]
    fn test_interpolate_with_curve_linear() {
        let v = interpolate_with_curve(0.0, 100.0, 0.5, AnimationCurve::Linear);
        assert!((v - 50.0).abs() < 0.001);
    }

    #[test]
    fn test_easing_clamps_t() {
        assert!((easing(AnimationCurve::Linear, -1.0) - 0.0).abs() < 0.001);
        assert!((easing(AnimationCurve::Linear, 2.0) - 1.0).abs() < 0.001);
    }
}
