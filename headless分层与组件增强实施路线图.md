# HarUI 组件增强实施路线图（GPUI Kit 启发）

> 制定日期：2026-09-30  
> 依据：长桥 GPUI Kit 调研（头条/微信两篇文章 + GitHub 实测 `longbridge/gpui-kit`，16.4k stars，纯 Rust）  
> 版本锚点：HarUI v1.0.0（commit `9781254`，61 组件，2047 测试，12 关门禁全通过）  
> 状态标记：⬜ 未开始 / 🔶 进行中 / ✅ 已完成  
> **本文档为后续开发的唯一参照入口**，各工作项启动前必须先读本节对应的"现状证据"，避免重复调研。

---

## 一、背景与吸收边界

### 1.1 GPUI Kit 是什么

长桥证券开源的 Rust 桌面组件库，构建在 Zed 编辑器的 GPUI 图形引擎之上（macOS Metal / Linux Vulkan / Windows DirectX 纯 GPU 渲染）。核心事实：

- 从 2019 年起承载 Longbridge Pro（券商交易台）真实业务，"GPUI 提供渲染基础，Longbridge 提供生产基础"
- v0.6.0 大重构为四层架构：`gpui-base`（无样式行为层）+ `gpui-component`（75+ 带样式控件）+ `gpui-shell`（JS 运行时热扩展）+ `gpui-kit`（总入口）
- 硬指标：虚拟表格扛数十万行（固定列/拖拽列宽/排序/单元格选择）、代码编辑器扛 20 万行（rope + Tree-sitter + LSP）、120 FPS、AccessKit 无障碍、headless 窗口模拟键鼠跑 UI 单测
- 分发创新：`npx skills add longbridge/gpui-kit` 向 Cursor/Claude Code 提供官方技能包

### 1.2 对 HarUI 的定位验证

微信文的选型对比表将"原生 egui/iced"评价为「偏向基础表单，缺少专业布局」——**HarUI 在 iced 生态中扮演的正是 gpui-kit 在 GPUI 生态中的角色**（给引擎补企业级组件层）。这验证了 HarUI 定位的正确性；其分层、测试、分发实践可直接借鉴。

### 1.3 吸收决策表

| GPUI Kit 能力 | 决策 | 理由 |
|---|---|---|
| styled / headless 分层（gpui-base 模式） | ✅ **吸收**（W1） | 五段式中 State/handle 天然是行为层，抽离成本低；换肤/测试双收益 |
| 虚拟表格（固定列/拖拽列宽/排序） | ✅ **吸收**（W2） | 收银流水/库存表刚需；`FixedSide` 已声明未渲染，补完即得 |
| criterion 性能基线 | ✅ **吸收**（W3） | bench 现无 Table 覆盖（实测），性能承诺需变成可回归数字 |
| headless 键鼠模拟 UI 单测 | ✅ **吸收**（W4） | 与 chaos（随机风暴）互补：固定序列回放断言业务正确性 |
| 实时 FPS 监控组件（gpui-fps） | ✅ **吸收**（W5） | 60 FPS 承诺变成现场可见 |
| 画廊即文档 | ✅ **吸收**（W5） | showcase 已存在，升级为可索引画廊 |
| AI 技能包（npx skills add） | ✅ **吸收**（W6） | 菜场收银台迁移时 AI 编码准确率直接受益 |
| 小步发布节奏 | ✅ **吸收**（W7） | v1.0.x patch 流，建立业务方依赖信心 |
| 换 GPUI 引擎 | ❌ **不吸收** | IME workaround、无头 CI、Element Plus 复刻资产全在 iced；DirectX 路线中文输入成熟度未知 |
| gpui-shell JS 运行时热扩展 | ❌ **不吸收** | 与"零 Web 依赖、确定性"定位相反，扩大审计面 |
| 代码编辑器 / LSP | ❌ **不吸收** | 收银台无代码编辑场景 |
| Dock 多面板停靠 | ⏸ **远期**（W8） | 收银台窗口固定；仅当后台管理端需求出现再启动 |
| WASM 画廊 | ⏸ **远期**（W8） | iced 0.13 生态限制，升级后再评估 |
| AccessKit 无障碍 | ⏸ **远期**（W8） | POS 场景优先级低；W1 的焦点管理顺带部分满足 |

---

## 二、版本规划总览

| 版本 | 主题 | 工作项 | 原则 |
|------|------|--------|------|
| **v1.1.0**（特性版，原拟 v1.0.1） | Table 增强 + 性能基线 | W2、W3(基础) | W2 新增公开 API（view_msg / 新消息变体 / TableColumn.pub 字段），按 SemVer 属 minor 而非 patch —— 实施时（2026-09-30）修正版本号 |
| **v1.1.0**（特性版） | headless 分层 + 测试/分发体系 | W1、W4、W5、W6、W3(完整) | 架构级变更，需 ADR |
| **v1.2.0+ / 远期** | 触发式 | W7 持续、W8 按需 | 不承诺时间表 |

依赖关系：W1 独立可先行；W2 与 W1 可并行；W3(完整) 的 overlay/table bench 依赖 W1/W2 完成；W6 依赖 W1/W2 API 稳定后编写（避免返工）。

---

## 三、工作项明细

### W1：core 行为层（headless）抽离 ✅ **P0，架构级（两批 2026-09-30 全部完成，ADR-009）**

**现状证据**：
- `crates/core/src/utils/keyboard.rs` 已有 `Shortcut/ShortcutRegistry/KeyEvent/subscription()`（行为层的雏形，仅键盘）
- 弹层定位逻辑散落各组件自算：`dialog.rs:139`（居中）、`tooltip.rs`、`popover.rs`、`select.rs`、`date_picker.rs` 各自处理位置，无共享算法
- `crates/components/src/table.rs:136-193` 的 `ViewportState`（`visible_range`/`clamp_offset`）是优秀的虚拟列表度量实现，但被 Table 独占

**改造内容**：
1. 新增 `crates/core/src/behavior/` 模块：
   - `overlay.rs`：`Placement` 12 方位枚举 + `compute_placement(anchor, content_size, viewport, placement, offset, collision) -> ResolvedRect`。**硬约束：纯 f32 计算，禁止引入 iced 类型**（用自有 `Rect/Size` 结构），保证可 proptest
   - `virtual_list.rs`：将 `ViewportState` 泛化为通用虚拟列表度量；`table.rs` 改为薄封装调用（行为不变）
   - `focus.rs`：焦点环 / Tab 序管理（扩展 keyboard.rs）
   - `stack.rs`：notification/message 同侧堆叠偏移计算
2. 组件接入顺序（由简到繁，每接入一个跑一次全量测试）：tooltip → popconfirm → popover → select → dropdown → date_picker → dialog → notification/message
3. `lib.rs` 导出 `har_ui_core::behavior::*`

**验收标准**：
- [ ] overlay 定位 proptest（T4）：随机 anchor/content/viewport，结果必完整落在 viewport 内，碰撞翻转策略正确
- [ ] 焦点环 T1 单测；虚拟列表泛化后 table 现有 11 个测试全绿
- [ ] 8 个弹层类组件全部走 `behavior::overlay`，组件内不再有定位常量
- [ ] `cargo clippy -D warnings` / fmt / 2047+ 测试全通过

**规模**：L（分 2-3 个会话）｜ **ADR 要求**：ADR-009（headless 行为层与渲染层分离）

**实施记录（第一批，2026-09-30）**：
- `behavior/` 三模块落地：`overlay.rs`（12 方位 + CollisionPolicy{None,Flip,FlipThenShift} +
  compute_placement，纯 f32 零 iced 类型）、`virtual_list.rs`（VirtualList 上收，table.rs 以
  `pub type VirtualScroll` 别名保持 API，23 测试原样通过）、`stack.rs`（stacked_offsets 泛化，
  notification 委托）。
- tooltip 首个接入：`From<TooltipPlacement> for overlay::Placement` + `resolved_rect(anchor,
  content, viewport)`（FlipThenShift），新增 3 个定位单测；notification 堆叠委托 stack。
- 开发中修复两处自引入缺陷：① 翻转判定与几何计算脱节（翻转后仍按原方位算坐标）；
  ② axis_space 主轴映射方向颠倒。proptest `overlay_fuzz`（1000 轮 × 4 不变式）守住。
- 核心测试：core lib 27 + overlay_fuzz 1000 cases；组件侧 tooltip 6/6、table 23/23 不回归。
- 余下（下一批）：focus.rs 焦点环、popconfirm→popover→select→dropdown→date_picker→
  dialog→notification/message 定位全量接入 overlay。

**实施记录（第二批，2026-09-30）**：
- `focus.rs` 焦点环落地：Tab 序注册 / `focus_next`·`focus_prev` 环绕跳过禁用项 /
  动态增删与焦点让出；命名避开 `Iterator::next` 混淆（clippy should_implement_trait）。
- overlay 引擎补两个便利构造：`dropdown_placement`（Bottom + Flip，select/dropdown/
  date_picker 面板语义）、`centered`（dialog 居中语义，内容超视窗钳到起点）。
- 弹层组件定位全量接入（8/8）：popconfirm / popover（From 映射 + resolved_rect，
  FlipThenShift）；select / dropdown / date_picker（resolved_rect → dropdown_placement）；
  dialog（centered_rect）；notification（上批）与 message（offsets 委托 stack，
  首条 extra=DEFAULT_OFFSET 口径逐值等价）。
- 组件测试 +10：popconfirm/popover/select/dropdown/dialog 各 1-2 个定位单测；
  core lib 36 个（+focus 7、+便利构造 2）。

---

### W2：Table 固定列渲染 + 列宽拖拽 ✅ **P0，v1.0.1（2026-09-30 完成，ADR-008）**

**现状证据**：
- `table.rs:37` `FixedSide` 枚举、`:49` `fixed: Option<FixedSide>` 字段、`:73` `with_fixed()` 构建器、`:552-557` 仅测试字段存储
- **渲染段（280-540 行）对 `fixed` 零引用** —— 声明了但没渲染
- 列宽仅 `width: Option<f32>` → `Length::Fixed`（`:395/:445`），无拖拽

**改造内容**：
1. 渲染分组：表体横向拆为 `left_fixed | scrollable | right_fixed` 三段；冻结列随垂直滚动同步、横向滚动时保持原位
2. 列宽拖拽：`ColWidth::Fixed/Auto/Resizable(min,max)`，表头分隔条产生 `TableMessage::ResizeColumn(idx, delta)`，拖拽中实时 clamp 到 [min, max]
3. 横向/纵向滚动解耦（`scroll_x` 与 `scroll_offset` 独立，各自动 clamp）
4. `table_demo` / showcase 增加"冻结列 + 拖拽列宽"演示页

**验收标准**：
- [ ] T1：冻结列偏移计算单测（横向滚动量 → 冻结列位移 = 0）
- [ ] T4：Resize 边界 proptest（min ≤ w ≤ max，总宽不塌缩为负）
- [ ] T3：截图测试——横向滚动前后冻结列像素一致
- [ ] 虚拟滚动与固定列组合下 1 万行数据滚动流畅（配合 W3 bench）

**规模**：M ｜ **ADR 要求**：ADR-008（Table 渲染分层与冻结列布局）

**实施记录（2026-09-30）**：
- 三处设计细化（详见 ADR-008）：① 不引入 `ColWidth` 枚举破坏 `width` 字段类型，改用
  `width`（默认宽）+ `resizable: Option<(min,max)>`（拖拽边界）两字段表达同语义，v1.0.1 零 API 破坏；
  ② 现状核查修正：垂直虚拟滚动的"消息驱动"是设计而非缺陷（demo 按钮驱动即既定模式），
  渲染层仅渲染 `visible_rows()`；③ iced 0.13 scrollable 有"内容不得填充滚动轴"构造期断言，
  拖拽条须用固定高（32px = 表头文本 16 + 上下 padding 8×2），不可 Fill。
- 新增交互入口 `view_msg(on_msg)`（横向滚动 ScrollX / 列宽拖拽 ResizeStart·Move·End /
  行点击 RowClicked 统一接线），`view()` 签名与行为保持 v1.0.0 完全一致。
- Fuzzer 抓到并修复 1 个真缺陷：resize 收缩列宽后 `scroll_x` 未回钳（`resize_column`
  末尾补 `clamp_scroll_x()`）。
- 交付测试：T1 内部 12 个新增（累计 23）+ T4 `table_fuzz.rs` 2 个 proptest（各 1000 cases）
  + 渲染 smoke 6 个新增（table_view.rs 累计 17）。
- W3 基线数字落 `docs/perf_baseline.md`：10 万行滚动计算 ~10.5ns/次（与 1 千行同价），
  拖拽步进 ~386ns，均低于预算 3 个数量级以上。

---

### W3：T5 性能基线扩展 🔶 **P0(基础 ✅ 2026-09-30) / P1(完整)**

**现状证据**：`crates/components/benches/component_bench.rs` 覆盖 slider/rate/progress/cascader/collapse 的 handle 类 bench，**完全没有 Table**。

**改造内容**：
1. 基础（v1.0.1）：新增 `bench_table_visible_range`（1k / 1w / 10w 行 `visible_range` + `clamp_offset`）、`bench_table_layout`（行布局计算）
2. 完整（W1/W2 后）：`bench_overlay_compute_placement`、`bench_theme_resolve`
3. 建立预算文档：滚动帧内布局计算 **< 1ms**（60 FPS 帧预算 16.6ms 的 6%），写入 `docs/perf_baseline.md`，criterion 基线数字入库

**验收标准**：
- [ ] criterion 基线建立并有数字记录
- [ ] 10 万行 `visible_range` 计算耗时 < 1μs 量级（纯算术，应轻松达标并记录实测）

**规模**：S ｜ **ADR**：无需

---

### W4：交互回放测试层（业务场景回放）⬜ **P1**

**现状证据**：`tests/chaos_test.rs`（随机事件风暴，管健壮性）与 `tests/screenshot_test.rs`（截图比对，管视觉）已存在；**缺"固定业务序列 + 状态断言"层**。

**改造内容**：新增 `crates/components/tests/replay_test.rs`，脚本化回放典型收银操作流并断言中间不变式与终态：
1. 扫码收银流：keypad 输入 → payment 现金 → 找零断言
2. 挂单流：hang_order 挂单 → 列表 → 取单 → 清空（ID 唯一性全程断言）
3. 表单提交流：form 校验失败 → 修正 → 成功
4. 弹层流：dialog 开 → 表单输入 → 取消/确认 → 状态还原

**验收标准**：
- [ ] 回放测试全部 headless（无渲染，纯 handle 序列）
- [ ] 纳入 gate.yml 的 `cargo test --workspace` 自动覆盖（无需改 CI）

**规模**：S-M ｜ **ADR**：无需

---

### W5：showcase 画廊化 + FPS overlay ⬜ **P1**

**现状证据**：`examples/showcase/src/main.rs` 单文件；无帧率可见性。

**改造内容**：
1. showcase 拆分：`gallery/mod.rs`（组件索引页，61 组件导航）+ `pages/`（每组件一页示例）+ `shared/code_block.rs`（源码片段展示，等宽文本即可）
2. 新增 `fps_meter.rs`（core 或 showcase 内）：iced subscription 每帧 tick，统计当前/平均/最差帧耗，overlay 显示——先在 showcase 与 table_demo 使用，与 W3 的预算数字互相印证

**验收标准**：
- [ ] `cargo run -p showcase` 可索引浏览全部 61 组件
- [ ] FPS overlay 在 table_demo 万行滚动下实时显示帧耗

**规模**：M ｜ **ADR**：无需

---

### W6：LLM/AI 技能包文档 ⬜ **P1（依赖 W1/W2 API 稳定）**

**改造内容**：输出 `docs/skill/har-ui-skills.md`（LLM 友好格式）：
1. 61 组件 API 速查表：Props / Message / 最常用 `with_*` 构建器（从现有源码抽取，脚本生成防手工漂移）
2. 8-10 个常用模式完整可编译片段：表单校验、表格+分页、弹层 CRUD、keypad+payment、hang_order、CustomerDisplay 双屏、主题切换、IME 输入
3. AI 编码规约：五段式模板、禁止绕过门禁的写法（如硬编码颜色、unwrap）

**验收标准**：
- [ ] 新开的 LLM 会话仅凭该文档零上下文写出可编译示例（人工抽查 3 例）

**规模**：S ｜ **ADR**：无需

---

### W7：发布节奏与 CHANGELOG ⬜ **P2（持续）**

**改造内容**：建立 `CHANGELOG.md`（Keep a Changelog 格式，回填 v0.1.0→v1.0.0 关键节点）；patch 版只收 bugfix；tag 规范 `vX.Y.Z` + 发布 checklist（对应 6.4 节 G 阶段流程）。

**规模**：S（持续）｜ **ADR**：无需

---

### W8：远期评估项（触发式，不承诺）⬜

| 项 | 触发条件 | 内容 |
|---|---|---|
| iced 0.14+ 升级 | iced 发布稳定版 | audit 豁免 9 条清零重审；IME workaround 是否可移除；profile 性能对比 |
| WASM 画廊 | iced 支持 WASM 后 | 画廊部署到网页，上手门槛归零 |
| Dock 多面板 | 后台管理端需求出现 | 参考_gpui-kit 的 dock 基础设施做 iced 版 |
| AccessKit 无障碍 | 收银机无障碍合规要求 | 对标 gpui-kit 的 AccessKit 集成 |

---

## 四、工程体系对接

1. **ADR 前置**：架构级工作项（W1→ADR-009、W2→ADR-008）启动前先写 ADR（模板见 `docs/ADR与生产Bug定位规范.md`，编号顺延现有 ADR-007）。
2. **门禁自动生效**：所有新代码过 12 关门禁 + pre-commit 钩子。headless 层不产生颜色（关卡 10 天然合规）、不建 Message 枚举的纯计算模块不受关卡 9 约束。
3. **测试金字塔映射**：W1→T1+T4；W2→T1+T4+T3；W3→T5；W4→T2（业务集成层）；W5→人工验收 + T3。
4. **执行环境提醒**：本机存在多会话共用 `CARGO_HOME`/`CARGO_TARGET_DIR` 的情况（2026-09-30 实测教训）。**跑门禁/测试前必须确认无其他 cargo 进程**（`ps -W | grep cargo`），否则出现锁等待与门禁误报。

---

## 五、风险与边界

| 风险 | 缓解 |
|---|---|
| W1 行为层泄漏 iced 类型，破坏"纯计算可 fuzz" | behavior 模块 lint 禁用 iced 导入（CI 可加 grep 检查） |
| W2 渲染重构引入虚拟滚动回归 | 现有 11 个 table 测试为底线 + 新增冻结列截图测试 |
| showcase 拆分过度拖慢编译 | 每页保持轻量，仅复用已有组件，不新增依赖 |
| 文档与代码漂移（W6 速查表失真） | 速查表用脚本从源码生成而非手写 |
| 范围蔓延（想顺手做 Dock/编辑器） | 严格按 1.3 决策表执行，⏸ 项一律不启动 |

---

## 六、里程碑验收清单

### M-A：v1.1.0（Table 增强 + 基线；原拟 v1.0.1，SemVer 修正见上表）
- [x] ADR-008 完成（2026-09-30）
- [x] W2：固定列渲染 + 列宽拖拽 + 全部验收项（T1 23/23、T4 2×1000 cases、渲染 smoke 17/17）
- [x] W3(基础)：Table bench 建立 + 预算文档（docs/perf_baseline.md）
- [x] 全量门禁 12 关通过（2026-09-30，2063 测试 0 失败）
- [ ] tag `v1.1.0` 并推送（推送待 GitHub 凭据）

### M-B：v1.1.0（headless + 测试分发体系）
- [x] ADR-009 完成（2026-09-30）
- [x] W1：behavior 模块 + 8 个弹层组件接入 + proptest（overlay_fuzz 1000 cases）
- [x] W4：replay_test 四条业务流（扫码收银 / 挂单生命周期 / 表单提交 / 弹层表单流）——
  并抓出并修复 Form `set_required` 标记未参与校验的缺陷（2026-09-30）
- [x] W3(完整)：overlay/focus bench 追加（定位 ~6.4ns/次、Tab 步进 ~6.2ns），基线落 perf_baseline.md
- [x] W5：画廊化 + FPS overlay（2026-09-30：catalog.rs 61 组件目录索引页 + 12 演示页用法
  片段；core `devtools::fps_meter`（对标 gpui-fps，window::frames 订阅 + 健康度配色 overlay），
  showcase 与 table_demo 均可切换显示）
- [x] W6：AI 技能包文档（2026-09-30：docs/skill/har-ui-skills.md — API 速查由
  scripts/gen_skill_doc.py 从源码自动生成（61 组件），10 个可编译模式片段逐一与源码核对
  （修正 Button/Pagination/Card 3 处 API 漂移，README 同步修正），AI 编码规约 5 条硬约束）
- [ ] 全量门禁 12 关通过；tag `v1.1.0` 并推送

### 远期
- [ ] W7 持续执行；W8 各项按触发条件评估后另立文档

---

*本路线图由 GPUI Kit 调研（2026-09-30）推导，所有"现状证据"均经本机代码实测核对；执行中发现与现状不符时，先更新本文档再动代码。*
