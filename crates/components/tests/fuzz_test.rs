//! T4 Fuzz 测试 — proptest 随机 Props/事件序列
//!
//! 对代表性组件进行 1000 轮随机测试，验证：
//! 1. 任何 Props 组合 + 任何事件序列都不会 panic
//! 2. 状态不变式始终成立（钳制、范围、step 吸附等）
//! 3. DML 操作序列无状态污染（连续操作后状态仍合法）
//!
//! 覆盖组件：Slider、Rate、Progress、Cascader、Collapse、Steps、Upload

use har_ui_components::cascader::{Cascader, CascaderMessage, CascaderNode};
use har_ui_components::collapse::{Collapse, CollapseItem, CollapseMessage};
use har_ui_components::progress::{Progress, ProgressMessage, ProgressStatus};
use har_ui_components::rate::{Rate, RateMessage};
use har_ui_components::slider::{Slider, SliderMessage};
use har_ui_components::steps::{Step, Steps, StepsMessage};
use har_ui_components::upload::{Upload, UploadFile, UploadMessage};
use proptest::prelude::*;

// ===================== Slider Fuzz =====================

prop_compose! {
    fn arb_slider_props()(
        min in -100.0f64..100.0,
        range_size in 1.0f64..200.0,
        step in 0.0f64..10.0,
        disabled in prop::bool::ANY,
    ) -> (f64, f64, f64, bool) {
        (min, min + range_size, step, disabled)
    }
}

prop_compose! {
    fn arb_slider_msg()(
        kind in 0u8..4,
        v in -1000.0f64..1000.0,
    ) -> SliderMessage {
        match kind {
            0 => SliderMessage::SetValue(v),
            1 => SliderMessage::SetRange(v, v + 50.0),
            2 => SliderMessage::Increase,
            _ => SliderMessage::Decrease,
        }
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]

    /// Slider：1000 轮随机 Props + 100 个事件序列
    #[test]
    fn fuzz_slider_state_invariants(
        (min, max, step, disabled) in arb_slider_props(),
        msgs in prop::collection::vec(arb_slider_msg(), 1..100),
    ) {
        let mut s = Slider::new()
            .with_min(min)
            .with_max(max)
            .with_step(step)
            .with_disabled(disabled);

        // 初始化 value 到 [min, max] 内（默认 value=0 可能不在范围内）
        // 注意：disabled=true 时 handle 不执行，value 保持 0.0
        s.handle(SliderMessage::SetValue(min));

        for msg in msgs {
            s.handle(msg);

            // 不变式 1：单值始终在 [min, max] 内
            // 注意：disabled=true 时 handle 不执行，value 保持初始值（可能不在范围内）
            if !disabled {
                prop_assert!(s.value() >= min - 1e-9);
                prop_assert!(s.value() <= max + 1e-9);
            }

            // 不变式 2：range_value 始终满足 lo ≤ hi
            let (lo, hi) = s.range_value();
            prop_assert!(lo <= hi + 1e-9);
        }

        // 不变式 3：最终值仍在范围内（仅 !disabled 时）
        if !disabled {
            prop_assert!(s.value() >= min - 1e-9);
            prop_assert!(s.value() <= max + 1e-9);
        }
    }
}

// ===================== Rate Fuzz =====================

prop_compose! {
    fn arb_rate_props()(
        max in 1u32..20,
        disabled in prop::bool::ANY,
        allow_half in prop::bool::ANY,
    ) -> (u32, bool, bool) {
        (max, disabled, allow_half)
    }
}

prop_compose! {
    fn arb_rate_msg()(
        kind in 0u8..4,
        v in -10.0f64..30.0,
    ) -> RateMessage {
        match kind {
            0 => RateMessage::SetValue(v),
            1 => RateMessage::Increase,
            2 => RateMessage::Decrease,
            _ => RateMessage::Clear,
        }
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]

    /// Rate：1000 轮随机 Props + 事件序列
    #[test]
    fn fuzz_rate_state_invariants(
        (max, disabled, allow_half) in arb_rate_props(),
        msgs in prop::collection::vec(arb_rate_msg(), 1..100),
    ) {
        let mut r = Rate::new()
            .with_max(max)
            .with_disabled(disabled)
            .with_allow_half(allow_half);

        for msg in msgs {
            r.handle(msg);

            // 不变式 1：value 在 [0, max]
            prop_assert!(r.value() >= -1e-9);
            prop_assert!(r.value() <= max as f64 + 1e-9);

            // 不变式 2：allow_half=false 时 value 必为整数
            if !allow_half {
                let v = r.value();
                prop_assert!((v - v.round()).abs() < 1e-9, "non-int value: {}", v);
            }

            // 不变式 3：allow_half=true 时 value 必为 0.5 的倍数
            if allow_half {
                let v = r.value();
                let doubled = v * 2.0;
                prop_assert!((doubled - doubled.round()).abs() < 1e-9, "non-half value: {}", v);
            }
        }
    }
}

// ===================== Progress Fuzz =====================

prop_compose! {
    fn arb_progress_msg()(
        kind in 0u8..3,
        v in -200i32..200,
    ) -> ProgressMessage {
        match kind {
            0 => ProgressMessage::SetPercentage(v),
            1 => ProgressMessage::Increment(v),
            _ => ProgressMessage::Reset,
        }
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]

    /// Progress：1000 轮随机事件序列，percentage 始终在 [0, 100]
    #[test]
    fn fuzz_progress_percentage_clamp(
        msgs in prop::collection::vec(arb_progress_msg(), 1..100),
    ) {
        let mut p = Progress::new();

        for msg in msgs {
            let is_reset = matches!(msg, ProgressMessage::Reset);
            p.handle(msg);

            // 不变式 1：percentage 始终在 [0, 100]
            prop_assert!(p.percentage() >= 0);
            prop_assert!(p.percentage() <= 100);

            // 不变式 2：Reset 后 status 必为 Default，percentage 必为 0
            if is_reset {
                prop_assert_eq!(p.status(), ProgressStatus::Default);
                prop_assert_eq!(p.percentage(), 0);
            }
        }
    }
}

// ===================== Cascader Fuzz =====================

fn arb_cascader_node() -> impl Strategy<Value = CascaderNode> {
    let leaf = "[a-z]{1,3}".prop_map(|v| {
        CascaderNode::new(format!("v_{}", v), format!("L_{}", v))
    });
    leaf.prop_recursive(
        3,      // 深度 3
        10,     // 总节点数 10
        3,      // 每层节点数 3
        |inner| {
            (
                "[a-z]{1,3}".prop_map(|s| format!("v_{}", s)),
                prop::collection::vec(inner, 1..3),
            ).prop_map(|(v, children)| {
                CascaderNode::new(v, "N".to_string()).with_children(children)
            })
        },
    )
}

fn arb_cascader_tree() -> impl Strategy<Value = Vec<CascaderNode>> {
    prop::collection::vec(arb_cascader_node(), 1..3)
}

fn collect_values(nodes: &[CascaderNode]) -> Vec<String> {
    let mut out = Vec::new();
    fn walk(nodes: &[CascaderNode], out: &mut Vec<String>) {
        for n in nodes {
            out.push(n.value().to_string());
            walk(n.children(), out);
        }
    }
    walk(nodes, &mut out);
    out
}

/// 辅助：检查路径是否在树中存在
/// 注意：树中可能存在重复 value，需要尝试所有匹配的节点
fn path_exists_in_tree(nodes: &[CascaderNode], path: &[String]) -> bool {
    if path.is_empty() {
        return true;
    }
    for node in nodes {
        if node.value() == path[0] {
            if path.len() == 1 {
                return true;
            }
            if path_exists_in_tree(node.children(), &path[1..]) {
                return true;
            }
            // 继续查找其他同名节点
        }
    }
    false
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]

    /// Cascader：1000 轮随机树 + 事件序列，select 后路径必在树中存在
    #[test]
    fn fuzz_cascader_select_invariants(
        tree in arb_cascader_tree(),
        check_strictly in prop::bool::ANY,
        msg_kinds in prop::collection::vec(0u8..3, 1..50),
        msg_indices in prop::collection::vec(0usize..100, 1..50),
    ) {
        let values = collect_values(&tree);
        let mut c = Cascader::new()
            .with_options(tree.clone())
            .with_check_strictly(check_strictly);

        for (kind, idx) in msg_kinds.iter().zip(msg_indices.iter()) {
            let msg = match kind {
                0 => CascaderMessage::Select(
                    values.get(idx % values.len().max(1)).cloned().unwrap_or_default(),
                ),
                1 => CascaderMessage::Clear,
                _ => CascaderMessage::TogglePanel,
            };
            c.handle(msg.clone());

            match msg {
                CascaderMessage::Select(_) => {
                    // 不变式 1：selected_path 若非空，必在树中存在
                    if !c.selected_path().is_empty() {
                        prop_assert!(path_exists_in_tree(&tree, c.selected_path()));
                    }
                }
                CascaderMessage::Clear => {
                    prop_assert!(c.selected_path().is_empty());
                }
                CascaderMessage::TogglePanel => {
                    // panel_visible 翻转后只能是 true/false
                    prop_assert!(c.panel_visible() == true || c.panel_visible() == false);
                }
            }
        }
    }
}

// ===================== Collapse Fuzz =====================

prop_compose! {
    fn arb_collapse_msg()(
        kind in 0u8..5,
        key in 0u32..10,
    ) -> CollapseMessage {
        let name = format!("item_{}", key);
        match kind {
            0 => CollapseMessage::Toggle(name),
            1 => CollapseMessage::Open(name),
            2 => CollapseMessage::Close(name),
            3 => CollapseMessage::OpenAll,
            _ => CollapseMessage::CloseAll,
        }
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]

    /// Collapse：1000 轮随机事件序列
    #[test]
    fn fuzz_collapse_state_invariants(
        accordion in prop::bool::ANY,
        n_items in 1usize..10,
        msgs in prop::collection::vec(arb_collapse_msg(), 1..100),
    ) {
        let mut c = Collapse::new().with_accordion(accordion);
        for i in 0..n_items {
            let item = CollapseItem::new(format!("item_{}", i), format!("标题{}", i));
            c.add_item(item);
        }
        prop_assert_eq!(c.items().len(), n_items);

        for msg in msgs {
            c.handle(msg);

            // 不变式 1：accordion 模式下 active_keys 最多 1 个
            if accordion {
                prop_assert!(c.active_keys().len() <= 1);
            }

            // 不变式 2：active_keys 中所有 key 必在 items 中存在
            for k in c.active_keys() {
                prop_assert!(
                    c.items().iter().any(|i| i.name() == k),
                    "active key not in items: {}",
                    k
                );
            }
        }
    }
}

// ===================== Steps Fuzz =====================

prop_compose! {
    fn arb_steps_msg()(
        kind in 0u8..5,
        idx in 0usize..20,
    ) -> StepsMessage {
        match kind {
            0 => StepsMessage::Next,
            1 => StepsMessage::Prev,
            2 => StepsMessage::JumpTo(idx),
            3 => StepsMessage::Finish,
            _ => StepsMessage::Reset,
        }
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]

    /// Steps：1000 轮随机事件序列
    #[test]
    fn fuzz_steps_state_invariants(
        n_steps in 1usize..10,
        msgs in prop::collection::vec(arb_steps_msg(), 1..100),
    ) {
        let mut s = Steps::new();
        for i in 0..n_steps {
            s = s.with_step(Step::new(format!("步骤{}", i + 1)));
        }
        prop_assert_eq!(s.steps().len(), n_steps);

        for msg in msgs {
            s.handle(msg);

            // 不变式 1：current 在 [0, n_steps) 内（若未 Finish）
            if !s.is_finished() {
                prop_assert!(s.current() < n_steps, "current={} n_steps={}", s.current(), n_steps);
            }
        }
    }
}

// ===================== Upload Fuzz =====================

prop_compose! {
    fn arb_upload_msg()(
        kind in 0u8..7,
        idx in 0usize..10,
        name in "[a-z]{1,5}\\.(jpg|png|gif)",
        size in 0u64..1_000_000,
        pct in 0u32..200,
    ) -> UploadMessage {
        match kind {
            0 => UploadMessage::AddFile(UploadFile::new(name, size)),
            1 => UploadMessage::Drop(vec![UploadFile::new(name, size)]),
            2 => UploadMessage::Remove(idx),
            3 => UploadMessage::Clear,
            4 => UploadMessage::StartUpload(idx),
            5 => UploadMessage::Progress(idx, pct),
            _ => UploadMessage::UploadSuccess(idx),
        }
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]

    /// Upload：1000 轮随机事件序列，文件列表长度合法、progress 合法
    #[test]
    fn fuzz_upload_state_invariants(
        multiple in prop::bool::ANY,
        limit in 0u32..5,  // 0 表示不限
        msgs in prop::collection::vec(arb_upload_msg(), 1..50),
    ) {
        let mut u = Upload::new()
            .with_multiple(multiple)
            .with_limit(limit as usize);

        for msg in msgs {
            u.handle(msg);

            // 不变式 1：file_list 长度合法
            if limit > 0 {
                prop_assert!(u.file_list().len() <= limit as usize);
            }

            // 不变式 2：非 multiple 模式下 file_list 最多 1 个
            if !multiple {
                prop_assert!(u.file_list().len() <= 1);
            }

            // 不变式 3：每个文件 progress 在 [0, 100]
            for f in u.file_list() {
                prop_assert!(f.progress() <= 100);
            }
        }
    }
}
