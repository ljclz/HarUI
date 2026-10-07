# iced 0.14.0 升级评估（W8 触发器已命中）

> 评估日期：2026-09-30 ｜ 触发条件：iced 0.14.0 正式发布（[Release Notes](https://github.com/iced-rs/iced/releases/tag/0.14.0)）
> 结论先行：**值得升级，独立立项为 v1.2.0 候选**；收益大、成本中等、非紧急。
>
> **✅ 升级已实施（2026-09-30，分支 feat/iced-0.14）**：全 workspace 编译通过、
> 2113 测试 0 失败、门禁 12 关 PASS、audit 重审 **9 条豁免清零 6 条**（quick-xml ×2 /
> instant / rustybuzz / event-listener / lru 旧档），保留 3 条（paste / ttf-parser /
> lru unsound——0.14 仍传递依赖）。实际迁移面：snap 字段 122+6 处（脚本自动补）、
> Space::with_width/height → new().width/height（44 处）、application 构造改 boot 闭包 +
> .title()/.run()（15 示例）、keyboard::on_key_press → listen（1 处）、
> Status::Focused 结构体变体（3 处）、Palette.warning（1 处）。
> 本文档是评估记录，不是迁移指南；实际升级前需重读官方迁移说明。

## 一、升级收益（按 HarUI 痛点对齐）

| 收益 | 对应 HarUI 现状 | 证据 |
|---|---|---|
| **IME 原生支持**（#2777，`input_method` 模块公开） | `core/utils/ime.rs` 基于 `event::listen_with` 的 workaround（技术债务 #2 的根源） | 0.14 Release Notes Added |
| **headless 模式 + 端到端测试**（#2698 / #3059） | T3 截图测试 CI 依赖 xvfb-run 技巧；W4 回放为纯 handle 序列 | 0.14 可原生无头渲染断言 |
| **audit 豁免清零**：quick-xml/lru/instant/paste/rustybuzz/ttf-parser/event-listener **均不在 0.14 直接依赖中**（实测 sparse index 依赖表） | `audit.toml` 9 条豁免（iced 0.13 生态钉死） | 升级后 `cargo audit` 全量重审，预期可清零 |
| **原生 table/grid widget**（#3018 / #2885） | 自研 Table（冻结列/拖拽/虚拟滚动）——需对比评估：保留自研或底层换原生 | 特性对比待升级后做 |
| **devtools 基建 + comet**（#2879/#2891） | 自研 `devtools::fps_meter` | 可能被原生性能指标替代或互补 |
| **Animation API**（#2757） | Dialog/Drawer 手写 tick 动画状态机 | 可简化 |
| 布局/渲染性能（primitive culling #2611、SDF quad #2967、lazy Compositor #2722） | POS 场景高频重绘受益 | — |

## 二、迁移风险（成本所在）

1. **布局语义变化**（#3045 "Prioritized `Shrink` over `Fill` in layout logic"）：
   直接影响 61 组件的布局结果——冻结列表格/弹层定位 overlay 与布局强耦合，
   **必须全组件视觉回归**（T3）+ table_virtual_scroll 基准对照。
2. **Widget API 变更**（#2781 update 按 Event 引用、#3038 Mutable Widget Methods）：
   HarUI 仅组合内建 widget、未实现裸 Widget trait，预计影响小；但 `container::Style`
   闭包签名等若变，61 组件 × StyleSheet 全部过一遍编译器即可暴露。
3. **主题 Palette 生成 overhaul（Oklch）**（#3028）：`theme::element_light/dark`
   是手工复刻 Element Plus 令牌，不依赖 Palette 生成——预计零影响，需验证。
4. **Reactive rendering（#2662）**：0.14 的新渲染模型——初判为**可选路径**，
   经典 Elm update/view 模型保留（官方示例仍为主模式），HarUI 无需跟随重构。
   若后续证实强制迁移，则升级成本升一档（这是"独立立项、不急于在本仓库顺手做"的主因）。
5. 依赖迁移面：iced 0.14 直接依赖 14 个（较 0.13 精简），workspace 其余依赖
   （criterion/proptest 等）与 iced 无耦合。

## 三、建议实施路径（立项后执行）

1. 开分支 `feat/iced-0.14`：`Cargo.toml` 升 iced → `cargo check` 收敛编译错误（预计集中在
   Style 闭包与少数 API 改名）
2. 全量测试 + 12 关门禁；重点：table_demo/showcase 人工视检布局（#3045 语义）
3. `cargo audit` 不带豁免跑一次 → 预期 0 命中后删除 `audit.toml` 9 条豁免
4. IME workaround 移除评估：`core/utils/ime.rs` 与 Input/Textarea 的 Ime 状态机改接
   原生 `input_method` API（原 4 个 IME 测试为回归底线）
5. FPS meter 与 comet devtools 功能对比，决定保留/替换
6. T5 基准全量重跑，更新 `perf_baseline.md`（跨版本对比表）
7. 发版：按 `docs/release-checklist.md`，major/minor 按 API 实际变化判定

## 四、不升级的代价

- 9 条 RUSTSEC 豁免长期挂账（其中 quick-xml DoS 类 2 条为漏洞级）
- IME workaround 持续维护；E2E/headless 测试能力缺席
- iced 0.13 进入维护末期，社区问题反馈减少

## 五、结论

触发器命中、收益明确、风险可控且已识别主战场（布局语义回归）。
**建议下个迭代立项 v1.2.0 = iced 0.14 升级专项**，预算 2-3 个会话；
本仓库当前 v1.1.0 状态稳定，可先行发布（推送待凭据）。
