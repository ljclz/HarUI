//! 同侧堆叠偏移 — 无样式、纯 u32 数值、零渲染依赖（ADR-009）
//!
//! 自 `NotificationList::stacked_offsets` 上收：任意"纵向/横向依次堆叠的浮层"
//! （通知、消息、Toast）均可复用。第 i 项偏移 = 前序各项(自身附加偏移 + gap) 累加。

/// 计算同侧堆叠时各项的偏移
///
/// - `extras`: 各项自身的附加偏移（如通知的 `offset` 属性）
/// - `gap`: 相邻两项的间距
///
/// 返回与 `extras` 等长的偏移数组；空输入返回空数组。
pub fn stacked_offsets(extras: &[u32], gap: u32) -> Vec<u32> {
    let mut out = Vec::with_capacity(extras.len());
    let mut acc = 0u32;
    for &e in extras {
        let cur = acc.saturating_add(e);
        out.push(cur);
        acc = cur.saturating_add(gap);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stacked_offsets_basic() {
        // 三项，无附加偏移，gap 8：0 / 8 / 16
        assert_eq!(stacked_offsets(&[0, 0, 0], 8), vec![0, 8, 16]);
    }

    #[test]
    fn test_stacked_offsets_with_extras() {
        // 第二项自带 offset 4：0 / (8+4)=12 / (12+8+0)=20 —— 附加偏移并入累加位
        assert_eq!(stacked_offsets(&[0, 4, 0], 8), vec![0, 12, 20]);
    }

    #[test]
    fn test_stacked_offsets_empty_and_zero_gap() {
        assert!(stacked_offsets(&[], 8).is_empty());
        // gap=0 时全部等于 extras 前缀和
        assert_eq!(stacked_offsets(&[3, 5, 2], 0), vec![3, 8, 10]);
    }

    #[test]
    fn test_stacked_offsets_saturating_no_overflow() {
        // 极大值不溢出 u32（饱和运算）
        let out = stacked_offsets(&[u32::MAX, u32::MAX], 10);
        assert_eq!(out[0], u32::MAX);
        assert_eq!(out[1], u32::MAX);
    }
}
