# Changelog

本项目遵循 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/) 格式与
[SemVer](https://semver.org/lang/zh-CN/) 语义化版本。

## [v1.4.0] — 2026-10-11

### Added

- **Button 文字 14px**（EP 规格 FontSize::Sm）：按钮行高 46→35px，几何尺度首轮对齐
- **EP 像素对照管线工具**：scripts/visual_compare.py（AE+SSIM 双指标，PoC 三组通过）
- **EP 复刻对照页**：examples/ep-parity（首轮数字 AE=30.3 PASS / SSIM=0.36，修复路径明确）
- **snap 试点**：技术验证通过（零错误无回归），保守回退 false，结论入档

### Added (continued from W9)

- **Tour**（漫游式引导）：步骤状态机/上下步/遮罩配置/完成标记，target 标识由应用层绘制高亮
- **InputTag**（标签输入框）：Enter 提交/退格双语义/上限与去重/禁用
- **Splitter**（分栏面板）：水平垂直/相邻面板百分比转移/min 钳制/拖拽三元组/Reset

## [v1.3.0] — 2026-10-10

### Added

- **图标系统扩至 EP 全集**：`IconName` 333 个（EP 官方 293 + legacy 40），
  `scripts/gen_icons.py` 从 iconify 官方 path 数据生成；新增 `Icon::view()`
  渲染路径（iced "svg" feature，此前图标为纯数据零消费）
- **三个对齐缺口组件**：Autocomplete（本地过滤/自定义 provider/键盘环绕选择）、
  Image（加载状态机/失败重试/预览开关）、TreeSelect（勾选镜像单选/面板开合/
  树消息透传）；组件目录与 README 更新至 64
- **W9 专项**：docs/element-plus-alignment.md（EP 对齐评估：覆盖矩阵/缺口清单/
  补齐路线），剩余 7 个低频缺口挂起待业务触发

## [v1.2.0] — 2026-09-30

iced 0.14.0 升级专项（W8 触发器命中，评估见 docs/iced-0.14-upgrade-evaluation.md）。

### Changed

- **升级 iced 0.13.1 → 0.14.0**（rustc 要求 1.88，本项目 1.90.0 满足）
- Style 新增 `snap` 像素吸附字段（131 处字面量，保守取 `false` 保持既有渲染行为，
  可逐组件开启评估锐度）
- Application 构造改为 0.14 范式：boot 闭包（首参）+ `.title()` builder + `.run()` 收尾
  （15 个示例全部迁移）

### Added

- 依赖树收益兑现：audit 豁免 **9 → 3**（quick-xml ×2 / instant / rustybuzz /
  event-listener / lru 旧档共 6 条清零；保留 paste / ttf-parser / lru unsound，
  0.14 仍传递依赖）
- 迁移工具脚本入库：`scripts/migrate_014.py`（括号匹配迁移）、`fix_snap.py`
  （E0063 自动补齐）、`normalize_chain.py`（builder 链归一化）

### Fixed

- `keyboard::on_key_press` 在 0.14 移除 → `keyboard::listen` 过滤 KeyPressed
  （screenshot_test）
- `Space::with_width/height` 移除 → `Space::new().width/height`（44 处）
- `text_input::Status::Focused` 结构体变体（3 处）、`Palette.warning` 字段（1 处）、
  `Pixels: From<u16>` 移除（3 处）

[v1.4.0]: https://github.com/ljclz/HarUI/compare/v1.3.0...v1.4.0
[v1.3.0]: https://github.com/ljclz/HarUI/compare/v1.2.0...v1.3.0
[v1.2.0]: https://github.com/ljclz/HarUI/compare/v1.1.0...v1.2.0

## [v1.1.0] — 2026-09-30

GPUI Kit 调研吸收后的首个特性版本（路线图 M-A/M-B，ADR-008/009）。

### Added

- **headless 行为层**（`har_ui_core::behavior`，纯 f32 零 iced 依赖）：
  `overlay` 12 方位弹层定位引擎（碰撞翻转/钳制）+ `virtual_list` 虚拟滚动度量 +
  `stack` 浮层堆叠偏移 + `focus` 焦点环（Tab 序环绕/跳过禁用项）—— ADR-009
- **Table 增强**（ADR-008）：冻结列三段式布局（左/中/右，头体横向同步由同段共用
  scrollable 消解）、列宽拖拽（`resizable(min,max)` 边界钳制 + 增量拖拽会话）、
  横向滚动状态 `scroll_x`、交互渲染入口 `view_msg` 与 5 个新消息变体
- **弹层组件定位接线（8/8）**：tooltip/popconfirm/popover 的 `resolved_rect()`，
  select/dropdown/date_picker 面板自动翻转，dialog 居中，notification/message 堆叠统一
- **性能自证**：`devtools::fps_meter`（逐帧订阅 + 环形缓冲 + 健康度配色 overlay），
  showcase 与 table_demo 可切换显示
- **测试体系**：T4 新增 `table_fuzz`（2×1000 cases）与 `overlay_fuzz`（1000 cases）；
  W4 业务流回放 `replay_test`（扫码收银/挂单/表单/弹层四流）；T5 基准新增
  table/overlay/focus 三组（10 万行滚动 ~10.5ns/次、定位 ~6.4ns/次、Tab 步进 ~6.2ns）
- **工程化**：GitHub Actions 门禁工作流（fmt/check/clippy/test，xvfb 无头）、
  `scripts/gate.sh` 12 关全通过、pre-commit 钩子、`docs/perf_baseline.md`、
  发布清单 `docs/release-checklist.md`
- **AI 技能包**：`docs/skill/har-ui-skills.md`（API 速查由 `scripts/gen_skill_doc.py`
  从源码自动生成 + 10 个可编译模式片段 + AI 编码规约）
- **文档**：ADR-008/009、评测报告附录 C（门禁闭环复核）、实施路线图（GPUI Kit 启发 W1-W8）
- showcase 组件目录索引页（61 组件 × 7 分组）与 12 个演示页用法片段

### Fixed

- **Form**：项级 `set_required` 标记未参与校验（`validate_field` 只读 rules）——
  由回放测试发现并修复
- **Table**：列宽收缩后 `scroll_x` 未回钳导致横向偏移越界 —— 由 proptest 发现并修复
- 工程门禁违规清零：clippy 5 error + 32 warning、fmt 1 处差异
- Release 构建 LTO 内存溢出：`[profile.release]` 降级 LTO，新增 `release-lto`
  专用 crate 发布构建（ADR-007）
- gate.sh 四处失效引用（example 名/路径/正则/python 回退），12 关可日常执行

### Changed

- 主题令牌收敛：组件内硬编码颜色 16 → 0（阴影/遮罩/强调色全部走令牌，
  新增 `iced_shadow()` 统一映射），动态取色场景按 HARUI-EXCEPTION 机制登记
- 安全审计豁免登记：9 个 RUSTSEC 条目（iced 0.13 生态传递依赖）在 `audit.toml`
  与 gate.sh 双写登记，含暴露面评估与清理条件

## [v1.0.0] — 2026-07-19

渲染层就绪，可被业务项目引用的首个正式版。

### Added

- 60 组件全部具备 `view()` 渲染实现（Phase 6 阶段 R）
- 12 个 P0 交互 demo + showcase 综合演示（阶段 D）
- POS 专用组件：Keypad / Payment / HangOrder / CustomerDisplay / StatusBar
- 主题系统：Element Plus 令牌复刻，light/dark 双主题
- 六层验证金字塔（T1-T5 已落地，T6 于 v1.1.0 补齐）：2035 测试
- Table 1000+ 行虚拟滚动、Input/Textarea 中文 IME 支持

## [v0.1.0] — 2026-07-19

初始版本（Phase 5 + M5，代码层生产就绪）。

### Added

- 61 组件状态机与逻辑层（Elm 架构五段式）
- har-ui-core：主题令牌系统、40 内嵌 SVG 图标、布局系统、IME 工具
- T1-T5 验证管线：1243 测试通过
- 工程文档：技术实现方案、设计规范（2064 行）、实施进度、ADR 规范

[v1.1.0]: https://github.com/ljclz/HarUI/compare/v1.0.0...v1.1.0
[v1.0.0]: https://github.com/ljclz/HarUI/compare/v0.1.0...v1.0.0
[v0.1.0]: https://github.com/ljclz/HarUI/releases/tag/v0.1.0
