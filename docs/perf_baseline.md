# HarUI 性能基线（T5 criterion）

> 建立日期：2026-09-30（路线图 W3）  
> 环境：Windows 10 x64 / 12th Gen Intel Core i9-12900H / rustc 1.90.0 / criterion 0.5.1（100 samples）  
> 复现命令：`cargo bench -p har-ui-components --bench component_bench -- table/`

## 性能预算

| 预算项 | 阈值 | 依据 |
|---|---|---|
| 滚动帧内虚拟滚动计算（visible_range + clamp_offset） | **< 1ms / 帧** | 60 FPS 帧预算 16.6ms 的 6% |
| 冻结列三段划分 + 8 列宽度解析 | **< 100µs / 帧** | 同上 |
| 单次列宽拖拽步进（ResizeMove → resize_column） | **< 10µs** | 拖拽流畅性（每帧可多次触发） |

## 基线数字（2026-09-30 实测）

### table/virtual_scroll — 100 次滚动推进 + 钳制 + 可见区间计算

| 行数量级 | mean | 单次成本 |
|---|---|---|
| 1,000 | 1,079.7 ns | ~10.8 ns |
| 10,000 | 1,046.1 ns | ~10.5 ns |
| 100,000 | 1,045.4 ns | ~10.5 ns |

**结论**：纯算术实现与行数量级无关（10 万行与 1 千行同价），预算余量 ~1000 倍。

### table/frozen_state — 冻结列状态层（8 列，首尾冻结，ADR-008）

| 基准 | mean | 说明 |
|---|---|---|
| partition_widths_8cols | 402.2 ps | 三段划分 + 8 列 resolved_width + 冻结总宽 |
| resize_1000x | 386.4 µs | 1000 次 ResizeColumn（含 handle 分发 + scroll_x 重钳制），单次 ~386 ns |

**结论**：单次拖拽步进 ~386 ns，预算余量 ~25000 倍；`resize_column` 内含 `clamp_scroll_x`
（列宽收缩后横向偏移自动回钳，见 ADR-008 与 table_fuzz 不变式 4）。

## 历史预算对照

| 预算 | 来源 | 状态 |
|---|---|---|
| 500 行布局 < 500ms | 实施进度 M2 | ✅ 单测 `test_table_virtual_scroll_500_rows_layout_under_500ms` 持续把守 |
| Table 1000+ 行虚拟滚动 60 FPS | README / RELEASE_NOTES | ✅ 本基线证明状态层远未成为瓶颈 |

## 维护规则

1. 任何触碰 `VirtualScroll` / `Table` 状态层的改动，跑 `cargo bench -p har-ui-components --bench component_bench -- table/` 对照本表；mean 回归 > 2× 时门禁驳回。
2. 新增状态层基准时在本表追加行并注明日期与环境。
3. 与其他项目共用 `CARGO_TARGET_DIR` 时，criterion 报告目录会混入外部项目的 `hot_path_*` 结果，本表数字以 `table/` 前缀目录为准。
