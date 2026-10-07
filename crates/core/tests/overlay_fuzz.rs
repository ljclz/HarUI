//! T4 Fuzz 测试 — behavior::overlay 定位引擎（ADR-009 / 路线图 W1）
//!
//! 对弹层定位进行 1000 轮随机锚点/内容/视窗组合，验证不变式：
//! 1. 任何输入组合都不 panic、不产生 NaN
//! 2. FlipThenShift 下弹层矩形完整落在视窗内
//! 3. 生效方位只能是声明方位或其对侧翻转（12 方位封闭）
//! 4. 内容尺寸守恒（不缩放输入）

use har_ui_core::behavior::overlay::{
    CollisionPolicy, Placement, PlacementOptions, Rect, Size, compute_placement,
};
use proptest::prelude::*;

fn arb_rect(limit: f64) -> impl Strategy<Value = Rect> {
    (
        -200.0f64..limit,
        -200.0f64..limit,
        1.0f64..600.0,
        1.0f64..400.0,
    )
        .prop_map(|(x, y, w, h)| Rect::new(x as f32, y as f32, w as f32, h as f32))
}

fn arb_size() -> impl Strategy<Value = Size> {
    (1.0f64..500.0, 1.0f64..300.0).prop_map(|(w, h)| Size::new(w as f32, h as f32))
}

fn arb_policy() -> impl Strategy<Value = CollisionPolicy> {
    prop_oneof![
        Just(CollisionPolicy::None),
        Just(CollisionPolicy::Flip),
        Just(CollisionPolicy::FlipThenShift),
    ]
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]

    #[test]
    fn fuzz_overlay_placement_invariants(
        anchor in arb_rect(900.0),
        content in arb_size(),
        viewport in arb_rect(1000.0),
        placement in prop_oneof![Just(Placement::Top), Just(Placement::TopStart), Just(Placement::TopEnd),
                                 Just(Placement::Bottom), Just(Placement::BottomStart), Just(Placement::BottomEnd),
                                 Just(Placement::Left), Just(Placement::LeftStart), Just(Placement::LeftEnd),
                                 Just(Placement::Right), Just(Placement::RightStart), Just(Placement::RightEnd)],
        offset in 0.0f64..40.0,
        collision in arb_policy(),
    ) {
        let opts = PlacementOptions { placement, offset: offset as f32, collision };
        let r = compute_placement(anchor, content, viewport, opts);

        // 不变式 1：无 NaN
        prop_assert!(r.rect.x.is_finite() && r.rect.y.is_finite());
        prop_assert!(r.rect.width.is_finite() && r.rect.height.is_finite());

        // 不变式 4：内容尺寸守恒
        prop_assert_eq!(r.rect.width, content.width);
        prop_assert_eq!(r.rect.height, content.height);

        // 不变式 3：生效方位 ∈ {声明, 对侧}
        prop_assert!(
            r.effective_placement == placement || r.effective_placement == placement.flipped(),
            "effective {:?} not in {{ {:?}, {:?} }}",
            r.effective_placement,
            placement,
            placement.flipped()
        );
        if collision == CollisionPolicy::None {
            prop_assert_eq!(r.effective_placement, placement);
        }

        // 不变式 2：FlipThenShift 下，宽度可容纳的轴必落在视窗内
        // （内容超出视窗时该轴只能钳到视窗起点，溢出属调用方约束，与 Element 语义一致）
        if collision == CollisionPolicy::FlipThenShift {
            if content.width <= viewport.width {
                prop_assert!(r.rect.x >= viewport.x - 1e-4, "x {} < vp.x {}", r.rect.x, viewport.x);
                prop_assert!(
                    r.rect.right() <= viewport.right() + 1e-4,
                    "right {} > vp.right {}",
                    r.rect.right(),
                    viewport.right()
                );
            }
            if content.height <= viewport.height {
                prop_assert!(r.rect.y >= viewport.y - 1e-4, "y {} < vp.y {}", r.rect.y, viewport.y);
                prop_assert!(
                    r.rect.bottom() <= viewport.bottom() + 1e-4,
                    "bottom {} > vp.bottom {}",
                    r.rect.bottom(),
                    viewport.bottom()
                );
            }
        }
    }
}
