# ADR-009: core 行为层（headless）与渲染层分离

- **状态**: Accepted
- **日期**: 2026-09-30
- **相关代码**: `crates/core/src/behavior/`（新增）、`crates/components/src/{table,tooltip,notification}.rs`（接入）
- **关联**: 路线图 W1（headless分层与组件增强实施路线图.md）；前置 ADR-008

## 背景

GPUI Kit 的 v0.6.0 重构将体系拆为 gpui-base（无样式行为层：焦点管理、浮层定位、
虚拟列表、状态机）与 gpui-component（带样式控件），行为层可独立测试与换肤。
HarUI 的五段式（Props/State/Message/view/handle）中 State/handle 天然是行为层，
但当前三类纯几何/数值逻辑耦合在组件内部：

1. 弹层定位：`TooltipPlacement` 等 12 方位枚举只存语义，无几何计算，各弹层组件各写一份
2. 虚拟滚动：`Table` 独占 `VirtualScroll`（visible_range/clamp_offset），其他列表场景无法复用
3. 堆叠偏移：`NotificationList::stacked_offsets` 私有实现，message 无法复用

## 决策

### 1. 新增 `har_ui_core::behavior` 模块，硬约束"纯 f32 计算，零 iced 类型"

- `overlay.rs`：`Rect/Size`（自有几何类型）+ `Placement`（12 方位，Element Plus 语义）+
  `CollisionPolicy { None, Flip, FlipThenShift }` + `compute_placement(anchor, content,
  viewport, opts) -> ResolvedRect { rect, effective_placement }`
- `virtual_list.rs`：`VirtualList`（自 `Table` 的 VirtualScroll 上收，数学逐行等价）
- `stack.rs`：`stacked_offsets(extras: &[u32], gap: u32) -> Vec<u32>`（同侧堆叠累加）
- 模块 lint 约束：不 `use iced`；坐标计算可被 proptest 全空间覆盖

### 2. 组件侧零破坏接入

- `Table`：`pub type VirtualScroll = har_ui_core::behavior::virtual_list::VirtualList;`
  类型别名保持原路径、原名、原方法；既有 23 个 table 测试即回归底线
- `Tooltip`：新增 `resolved_rect(anchor, content, viewport) -> ResolvedRect`，12 方位经
  `From<TooltipPlacement> for overlay::Placement` 委托行为层 —— iced 0.13 无绝对定位 widget，
  该 API 供应用层手工定位/动画方向判定/测试断言使用
- `NotificationList::stacked_offsets`：实现体委托 `behavior::stack`，签名不变

### 3. 接入节奏（本 ADR 覆盖第一批）

第一批：overlay 引擎 + tooltip 接入 + virtual_list 上收 + stack 上收。
后续（按路线图 W1 顺序）：popconfirm → popover → select → dropdown → date_picker →
dialog → notification/message 定位全量走 overlay；focus.rs（焦点环/Tab 序）另立小节补充。

## 后果

**正面**：
- 行为层可被 proptest 全空间覆盖（组件渲染测试只能采样），W4 回放测试与 W2 拖拽已验证此模式 ROI
- 换肤/主题切换只动样式层；第二套视觉（HarPOS 品牌色）不需要重写行为
- 虚拟滚动/堆叠逻辑跨组件复用（流水列表、多通知、多消息）

**负面**：
- `Tooltip::resolved_rect` 在 iced 0.13 下是"可选消费"API（框架本身按布局定位），
  存在"暂时无人调用"的过渡期 —— 换取 iced 0.14+ overlay 能力就绪时零迁移
- 行为层上收使 core 体积增加；对仅用主题的下游无影响（tree-shaking 生效）

## 注意事项

- **Bug 定位提示**：如果 ① 弹层出现在视窗外 —— 检查 `CollisionPolicy` 分支与 viewport 传参；
  ② 虚拟滚动行区间异常 —— table 的 `VirtualScroll` 现为 core 别名，优先查
  `behavior::virtual_list`；③ 堆叠偏移跳变 —— 检查 extras 顺序与 gap 折算口径
- 性能预算沿用 `docs/perf_baseline.md`；overlay 计算属同量级纯算术，接入 bench 时追加行
