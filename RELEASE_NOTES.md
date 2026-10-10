## v1.4.0 — 2026-10-11（EP 对齐清零版，W9 完结）

### Highlights
- **EP 缺口全部清零**：Tour / InputTag / Splitter 三个最后组件（71 组件总量）
- **Button 文字 14px**（EP 规格）：行高 46→35px，几何尺度首轮对齐
- **EP 像素对照管线**：visual_compare.py（AE+SSIM）+ ep-parity 复刻页 + 首轮数字
- **snap 试点**：技术验证通过（零错误），保守回退待 AE 定论

### Upgrade
```toml
har-ui-core = { git = "https://github.com/ljclz/HarUI", tag = "v1.4.0" }
har-ui-components = { git = "https://github.com/ljclz/HarUI", tag = "v1.4.0" }
iced = "0.14"
```

---

## v1.3.0 — 2026-10-10（EP 对齐补齐版，W9）

### Highlights
- **组件 68 个**：新增 Autocomplete / Image / TreeSelect（P1）+
  Transfer / Watermark / Anchor / Countdown（P2），EP 缺口 10 → 3 个低频项（Tour/InputTag/Splitter）
- **图标 333 个**：EP 官方全集 293（iconify path 数据）+ legacy 40，
  并首次挂上渲染路径（iced svg widget + Icon::view）
- 2163 测试 0 失败，12 关门禁 + GitHub CI 双绿

### Upgrade
```toml
har-ui-core = { git = "https://github.com/ljclz/HarUI", tag = "v1.3.0" }
har-ui-components = { git = "https://github.com/ljclz/HarUI", tag = "v1.3.0" }
iced = "0.14"
```

---

## v1.2.0 — 2026-09-30（iced 0.14 升级版）

### Highlights
- **升级 iced 0.13.1 → 0.14.0**：渲染性能（primitive culling / SDF quad / lazy Compositor）、
  Reactive 渲染可选路径、devtools 基建
- **audit 豁免 9 → 3**：6 条 RUSTSEC 条目随依赖树升级自然清零
- 2113 测试 0 失败，12 关门禁 PASS

### Upgrade
```toml
har-ui-core = { git = "https://github.com/ljclz/HarUI", tag = "v1.2.0" }
har-ui-components = { git = "https://github.com/ljclz/HarUI", tag = "v1.2.0" }
iced = "0.14"
```
v1.1.0 → v1.2.0 对下游零 API 破坏（HarUI 自身未改公开 API；下游需同步升 iced 0.14）。

---

## v1.1.0 — 2026-09-30（GPUI Kit 吸收特性版）

### Highlights
- **headless 行为层**（ADR-009）：`har_ui_core::behavior` — 弹层 12 方位定位引擎
  （碰撞翻转/钳制）、虚拟列表度量、浮层堆叠、焦点环；纯 f32 零 iced 依赖，可全空间 fuzz
- **Table 冻结列 + 列宽拖拽**（ADR-008）：左/右冻结三段式、`resizable(min,max)` 拖拽、
  横向滚动，与虚拟滚动正交；新增 `view_msg` 交互渲染入口
- **性能自证**：FPS overlay（showcase/table_demo 可切换）；基准：10 万行滚动计算
  ~10.5ns/次、定位引擎 ~6.4ns/次（docs/perf_baseline.md）
- **测试 +66**：2100+ 全通过 — 业务流回放 4 条、overlay_fuzz/table_fuzz 各 1000 轮
- **AI 技能包**：docs/skill/har-ui-skills.md（API 速查脚本自动生成）

### Upgrade
```toml
har-ui-core = { git = "https://github.com/ljclz/HarUI", tag = "v1.1.0" }
har-ui-components = { git = "https://github.com/ljclz/HarUI", tag = "v1.1.0" }
```
v1.0.0 → v1.1.0 零破坏性变更；新增 API 详见 CHANGELOG.md。

---

## HarUI v1.0.0 - Rust Native UI Component Library

### Features
- **60+ Components**: Full Element Plus equivalent in Rust ecosystem
- **Theme System**: Light/Dark dual theme, Element Plus design tokens
- **Virtual Scrolling**: Table supports 1000+ rows at 60 FPS
- **IME Support**: Input/Textarea with CJK IME, no flicker
- **Zero Web Deps**: Pure Rust rendering (iced + winit + wgpu)
- **2035 Tests**: Unit + integration + fuzz + snapshot tests
- **14 Demos**: Including showcase comprehensive demo
- **POS Specialized**: Keypad/Payment/HangOrder/CustomerDisplay/StatusBar

### Quick Start
```toml
[dependencies]
har-ui-core = { git = "https://github.com/ljclz/HarUI", tag = "v1.0.0" }
har-ui-components = { git = "https://github.com/ljclz/HarUI", tag = "v1.0.0" }
iced = "0.13"
```

### Stats
- 2035 tests, 0 failure, 0 warning, 0 error
- 60 components with view() rendering methods
- 14 demo applications + showcase
- Cargo build --workspace: 0 warning 0 error
