//! T4 Fuzz 测试 — Table 冻结列 / 列宽拖拽 / 横向滚动（ADR-008 / 路线图 W2）
//!
//! 对 Table 状态层进行 1000 轮随机操作，验证不变式：
//! 1. 任何列配置 + 任何消息序列都不会 panic
//! 2. 被 resize 过的列宽必在其拖拽边界内（无显式边界时 ≥ 40）
//! 3. 未被 resize 的列保持列默认宽（运行时覆盖不出现）
//! 4. scroll_x 始终在 [0, max_scroll_x] 内
//! 5. 三段划分并集恰好覆盖全部列且无重复

use har_ui_components::table::{FixedSide, Table, TableColumn, TableMessage};
use proptest::prelude::*;

/// 随机列配置：宽度 / 可拖拽边界 / 冻结方向
fn arb_column() -> impl Strategy<Value = TableColumn> {
    (
        40.0f64..400.0,
        prop::option::of((20.0f64..60.0, 80.0f64..600.0)),
        prop::option::of(prop::bool::ANY), // None | Some(Left) | Some(Right)
    )
        .prop_map(|(w, bounds, fixed)| {
            let mut col = TableColumn::new("col", "列").with_width(w as f32);
            if let Some((min, max)) = bounds {
                col = col.with_resize_bounds(min as f32, max as f32);
            }
            if let Some(is_left) = fixed {
                col = col.with_fixed(if is_left {
                    FixedSide::Left
                } else {
                    FixedSide::Right
                });
            }
            col
        })
}

fn arb_columns() -> impl Strategy<Value = Vec<TableColumn>> {
    prop::collection::vec(arb_column(), 1..8).prop_map(|cols| {
        // 保证 prop 唯一，便于追踪
        cols.into_iter()
            .enumerate()
            .map(|(i, mut c)| {
                c.prop = format!("col{}", i);
                c
            })
            .collect()
    })
}

/// 随机消息序列：ScrollX / ResizeColumn / ResizeStart / ResizeMove / ResizeEnd
fn arb_msg() -> impl Strategy<Value = TableMessage> {
    prop_oneof![
        (-2000.0f64..2000.0).prop_map(|x| TableMessage::ScrollX(x as f32)),
        (0usize..8usize, -500.0f64..500.0)
            .prop_map(|(i, d)| TableMessage::ResizeColumn(i, d as f32)),
        (0usize..8usize).prop_map(TableMessage::ResizeStart),
        (-2000.0f64..2000.0).prop_map(|x| TableMessage::ResizeMove(x as f32)),
        Just(TableMessage::ResizeEnd),
    ]
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]

    /// 不变式 1-5：随机列配置 + 随机消息序列，状态始终合法
    #[test]
    fn fuzz_table_frozen_state_invariants(
        cols in arb_columns(),
        h_viewport in prop::option::of(200.0f64..2000.0),
        msgs in prop::collection::vec(arb_msg(), 1..80),
    ) {
        let n = cols.len();
        let mut table: Table<String> = Table::new().with_columns(cols.clone());
        if let Some(w) = h_viewport {
            table = table.with_horizontal_viewport(w as f32);
        }

        // 追踪每列是否被 resize 过（宽度实际发生变化）
        let mut ever_resized = vec![false; n];
        // 拖拽会话：(列索引, 本次会话内 move 次数) — 首次 move 仅锚定不改变宽度
        let mut session: Option<(usize, u32)> = None;

        for msg in msgs {
            match &msg {
                TableMessage::ResizeColumn(i, _) => {
                    if *i < n {
                        ever_resized[*i] = true;
                    }
                }
                // Start 仅对可拖拽列生效（组件会忽略不可拖拽列的 Start）
                TableMessage::ResizeStart(i) => {
                    if *i < n && cols[*i].resizable.is_some() {
                        session = Some((*i, 0));
                    }
                }
                TableMessage::ResizeMove(_) => {
                    if let Some((i, cnt)) = session.as_mut() {
                        *cnt += 1;
                        // 第二次 move 起宽度才真正变化（首次仅锚定起点）
                        if *cnt >= 2 {
                            ever_resized[*i] = true;
                        }
                    }
                }
                TableMessage::ResizeEnd => {
                    session = None;
                }
                _ => {}
            }

            table.handle(msg.clone());

            // 不变式 2/3：列宽合法性
            for i in 0..n {
                let resolved = table.resolved_width(i);
                if ever_resized[i] {
                    let w = resolved.unwrap_or_else(|| panic!("resized col{} lost width", i));
                    let (lo, hi) = cols[i].resizable.unwrap_or((40.0, f32::MAX));
                    prop_assert!(w >= lo - 1e-6, "col{} width {} < min {}", i, w, lo);
                    prop_assert!(w <= hi + 1e-6, "col{} width {} > max {}", i, w, hi);
                } else {
                    prop_assert_eq!(resolved, cols[i].width);
                }
            }

            // 不变式 4：scroll_x ∈ [0, max]
            let sx = table.scroll_x();
            prop_assert!(sx >= 0.0, "scroll_x {} < 0", sx);
            let max = (table.mid_content_width() - table.mid_available_width()).max(0.0);
            prop_assert!(sx <= max + 1e-6, "scroll_x {} > max {}", sx, max);

            // 不变式 5：三段划分覆盖全部列、无重复
            let (l, m, r) = table.fixed_partition();
            let mut all: Vec<usize> = l;
            all.extend(m);
            all.extend(r);
            prop_assert_eq!(all.len(), n);
            let mut sorted = all;
            sorted.sort_unstable();
            sorted.dedup();
            prop_assert_eq!(sorted.len(), n);
        }
    }

    /// 拖拽会话不变式：Start→Move×N→End 序列中，
    /// 宽度仅在 Move 阶段变化、单步变化量不超过单步位移（钳制只会收敛）、始终在边界内。
    /// 初始宽度生成于边界之内（拖拽边界约束的是 resize 结果，不约束初始宽）。
    #[test]
    fn fuzz_table_drag_session(
        setup in (20.0f64..60.0).prop_flat_map(|min| {
            (min + 40.0..800.0).prop_flat_map(move |max| {
                (min + 1.0..max - 1.0)
                    .prop_map(move |base| (min as f32, max as f32, base as f32))
            })
        }),
        moves in prop::collection::vec(-100.0f64..100.0, 1..50),
    ) {
        let (min, max, base_w) = setup;
        let mut table: Table<String> = Table::new().with_columns(vec![
            TableColumn::new("a", "A")
                .with_width(base_w)
                .with_resize_bounds(min, max),
        ]);

        table.handle(TableMessage::ResizeStart(0));
        let mut cursor = 0.0f32;
        let mut last_w = table.resolved_width(0).unwrap();

        for dx64 in moves {
            let dx = dx64 as f32;
            let prev_cursor = cursor;
            cursor += dx;
            table.handle(TableMessage::ResizeMove(cursor));
            let w = table.resolved_width(0).unwrap();
            prop_assert!(
                w >= min - 1e-6 && w <= max + 1e-6,
                "w {} out of [{}, {}]",
                w,
                min,
                max
            );
            // 钳制只会收敛变化量：|Δw| ≤ 组件实际收到的 |Δx|（f32 口径与组件一致，
            // 容差放宽到 1e-3 以吸收 f32 加减的 ulp 误差）
            let applied = (cursor - prev_cursor).abs() + 1e-3;
            prop_assert!(
                (w - last_w).abs() <= applied,
                "|{} - {}| > |{}|",
                w,
                last_w,
                cursor - prev_cursor
            );
            last_w = w;
        }
        table.handle(TableMessage::ResizeEnd);
        table.handle(TableMessage::ResizeMove(99999.0));
        prop_assert_eq!(table.resolved_width(0).unwrap(), last_w);
    }
}
