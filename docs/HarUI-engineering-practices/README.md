# HarUI 工程化实践规范（菜场收银台提炼）

> 本目录汇总自菜场收银台（caichang-shouyintai）v0.1.0 项目工程实践，
> 适用于所有基于 iced 0.13 + HarUI 的 Rust GUI 项目。
> 版本：v1.0  日期：2026-07-20

---

## 适用范围

本规范针对 **iced 0.13 + HarUI 组件库** 的 Rust GUI 应用项目，特别关注：

- Elm 架构（State + Message + Update + View）的状态管理
- GUI 主线程与异步任务（iced::Task）的并发安全
- iced 0.13 API 约束（Padding/scrollable/button::Style 闭包等）
- 硬件驱动集成（DLL/HTTP/WebSocket）的 spawn_blocking 模式
- 加密通信（AES-256-CBC + RSA-PKCS1v15）的安全实践
- UI 自适应分辨率（1366×768 / 1920×1080）

---

## 目录

| 序号 | 文档 | 核心内容 |
|------|------|---------|
| 1 | [01-integration-gates.md](01-integration-gates.md) | 集成层强制门禁（7 关）|
| 2 | [02-contract-audit.md](02-contract-audit.md) | 行为变更契约审计（State/Message/Component）|
| 3 | [03-concurrency-safety.md](03-concurrency-safety.md) | GUI 并发安全模式（主线程 + Task::perform）|
| 4 | [04-test-pyramid.md](04-test-pyramid.md) | GUI 测试体系（单元/快照/像素/集成）|
| 5 | [05-engineering-practices.md](05-engineering-practices.md) | HarUI 工程化全局规范 |

---

## 应用建议

### 新项目接入顺序

1. 先读 [05-engineering-practices.md](05-engineering-practices.md) → 建立 Cargo.toml/CI/文档骨架
2. 再读 [04-test-pyramid.md](04-test-pyramid.md) → 搭建四线测试体系（单元/视图/像素/集成）
3. 再读 [03-concurrency-safety.md](03-concurrency-safety.md) → 异步任务并发审查 checklist
4. 再读 [02-contract-audit.md](02-contract-audit.md) → 编写 State/Message/Component 契约
5. 最后读 [01-integration-gates.md](01-integration-gates.md) → 落地 7 关门禁

### 已有项目接入顺序

1. 先跑 [01-integration-gates.md](01-integration-gates.md) 的前 5 关（fmt/check/clippy/test/doc）→ 立即提升质量基线
2. 再补 [02-contract-audit.md](02-contract-audit.md) → 锁定 State 转移与 Message 处理行为
3. 再补 [03-concurrency-safety.md](03-concurrency-safety.md) → 修复 Task::perform 跨 await 持锁
4. 再补 [04-test-pyramid.md](04-test-pyramid.md) → 扩展像素验证与快照测试
5. 最后按 [05-engineering-practices.md](05-engineering-practices.md) → 完善工程化

---

## 与其他规范的关系

本目录的规范是对菜场收银台项目实施进度表的**横向抽取**：

- `菜场收银台迁移进度.md` 记录"做了什么"（M1.S1-S11）
- 本目录记录"如何复用到其他 iced + HarUI 项目"

---

## 演进原则

- 本规范**不是**一成不变的教条，而是**最佳实践起点**
- 项目可基于自身特点裁剪/扩展（如纯展示型 GUI 可弱化并发规范）
- 重大决策应记录为 ADR（架构决策记录）
- 规范本身应随 iced 生态演进而迭代（如 iced 0.14/master 分支 API 变化后调整）

---

## 验证清单

接入本项目规范后，可用以下清单自检：

- [ ] Cargo.toml 集中管理 features（api/hardware/comm）与 optional 依赖
- [ ] rustfmt.toml + clippy.toml 配置完成
- [ ] 7 关门禁脚本可执行（PowerShell）
- [ ] State/Message/Component 行为契约文档已建立
- [ ] 契约测试目录已建立（tests/contract/）
- [ ] Task::perform 跨 await 持锁 checklist 已纳入代码审查
- [ ] 四线测试体系至少覆盖前 3 线（单元/视图/像素）
- [ ] CHANGELOG.md 持续维护
- [ ] ADR 目录已建立（docs/adr/）
- [ ] iced 0.13 API 约束 checklist 已纳入代码审查（见 03-concurrency-safety.md 第七章）
- [ ] UI 自适应分辨率验证（至少 1366×768 + 1920×1080）
