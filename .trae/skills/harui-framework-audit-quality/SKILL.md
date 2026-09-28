---
name: harui-framework-audit-quality
description: 审计质量规范（HarUI）。每次审计必须遵循：每项修复必须实际运行测试验证，不更新文档状态除非测试真实通过，所有结论附 file:line 证据。修改任何审计相关文档时触发。
tools: [cargo, cargo-test, cargo-clippy]
agentMode: auto
---

# 审计质量规范（HarUI）

## 🚫 审计前强制清单（未全部完成则禁止开始审计）

- [ ] **运行基线测试**：执行 `cargo test --workspace` 并记录 `test result: ok. N passed; 0 failed`（基线 2035）
- [ ] **确认编译通过**：执行 `cargo check --workspace` 并确认无 error
- [ ] **记录基线测试数 N**：写入审计报告开头
- [ ] **声明审计范围**：如 `crates/components/src/`、`crates/core/src/theme/`

> ⚠️ 违反上述任意一项，审计结果视为无效。

## 核心原则

> **每项修复后必须实际运行相关测试验证，不更新文档状态除非测试真实通过，主动扫描同类问题，所有结论附 file:line 证据。**

## 🔒 单一审计流程（禁止并行 Agent）

- ✅ 正确：单一 Agent 顺序执行每个检查项，每步输出 file:line 证据
- ❌ 错误：多个并行 Agent 扫描同一代码库（结论无法交叉验证）

## 审计流程

### 1. 审计前准备
- [ ] `cargo test --workspace` 记录基线 N（2035）
- [ ] `cargo check --workspace` 确认编译通过

### 2. 审计执行
- [ ] **每项修复附带测试证据**：修复后立即运行相关测试（`cargo test -p har-ui-components <module>::`），输出 `test result: ok. N passed; 0 failed`
  - ⚠️ 修复后测试数 ≥ 基线 N
- [ ] **所有结论附 file:line 证据**：`[crates/components/src/button.rs:87](file:///.../button.rs#L87)`
- [ ] **主动扫描同类问题**：修复一处组件模式违规后扫描其他组件
- [ ] **禁止结论互相引用**

### 3. 文档更新
- [ ] **测试未通过前禁止更新文档状态**
- [ ] 文档更新必须附测试证据

### 4. 审计后验证（硬闸门）
- [ ] 最终测试数 ≥ 基线（2035）
- [ ] 新组件确认五段式模式合规（Props/State/Message/view/handle）
- [ ] 填写审计质量自查表

## 审计证据清单

| 证据类型 | 格式 | 示例 |
|---------|------|------|
| 测试通过 | cargo test 输出 | `test result: ok. 2035 passed; 0 failed` |
| 编译通过 | cargo check 输出 | `Finished dev [unoptimized + debuginfo] target(s)` |
| 代码位置 | file:line 链接 | `[crates/components/src/button.rs:87](file:///.../button.rs#L87)` |
| 渲染测试 | render test | `test button_view ... ok` |

### 禁止的证据形式

| 形式 | 原因 |
|------|------|
| `应该没问题` | 未验证 |
| `参见其他文档` | 互相引用 |
| `大概/可能` | 不确定 |
| `已修复`（无测试） | 无证据 |

## 审计质量自查表

```markdown
## 审计质量自查
- [ ] 所有修复是否附带测试证据？
- [ ] 所有结论是否附带 file:line 证据？
- [ ] 是否主动扫描了同类问题？
- [ ] 文档更新是否在测试通过之后？
- [ ] 是否有未验证的结论？

### 发现的问题
| 问题 | 位置 | 证据 | 状态 |

### 遗留问题
| 问题 | 位置 | 影响 | 建议 |
```

## 违规处理（立即停止审计并报告）

1. 测试未通过但文档标记 ✅
2. 结论无 file:line 证据
3. 未运行测试即声称"已修复"
4. 并行 Agent 扫描同一代码库
5. 审计前强制清单未完成
6. file:line 行号编造

## 参考

- `docs/软件项目审计清单.md` — HarUI 审计维度（组件模式审计）
- `docs/ADR与生产Bug定位规范.md` — Bug 定位方法论
