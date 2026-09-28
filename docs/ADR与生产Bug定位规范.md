# ADR 与生产 Bug 定位规范（HarUI 专属版）

> **来源**：HarPOS 生态方法论（源自 SZ-Rust 实测提炼），迁移至 HarUI 组件库场景。
> **适用**：HarUI（Rust 原生 UI 组件库）— 独立项目。HarRT 与菜场收银台各有独立规范（见各自仓库 docs/）。
> **版本**：1.0.0（2026-08-02）

---

## 1. 核心问题

HarUI 作为组件库，AI 驱动开发面临与 SZ-Rust 相同的三痛点，且有组件库特有的"视觉回归"盲区：

| 痛点 | HarUI 特殊表现 | 后果 |
|------|----------------|------|
| **无状态** | 组件为什么用 builder？为什么 State 是枚举？为什么样式走 style_sheets？ | 新组件不符合模式，库风格分裂 |
| **覆盖率盲区** | 62 个组件，ADR 只覆盖少量设计决策 | 状态机边界 bug 无法定位 |
| **运行时黑盒** | UI bug 无法远程复现，截图对比难自动化 | 只能靠人工肉眼检查 |
| **视觉回归**（特有） | 明暗主题切换、字体渲染、Z 轴层级问题 | 无法用逻辑测试覆盖 |

**本规范解决**：ADR 解决"无状态"，render test + screenshot test 解决"视觉回归"，四层定位流程解决"覆盖率盲区"。

---

## 2. 核心原则

1. **ADR 是决策记忆，不是 bug 定位工具**
2. **ADR + render test + screenshot + 源码四层组合**——HarUI 特有：用 `*_view.rs` render test（编译期断言）和 `screenshot_test` 示例（像素级对比）代替 tracing。
3. **ADR 必须含"Bug 定位提示"段**
4. **组件状态机必须有穷举测试**——没有穷举测试的状态机 = 未验证的 UI 行为。
5. **ADR 有效性必须实测**——零上下文子代理测试。

---

## 3. ADR 写作规范

### 3.1 文件命名

```
HarUI/docs/adr/
├── README.md                              # 索引 + 使用说明
├── 0001-短标题-kebab-case.md
└── template.md
```

### 3.2 结构（与 HarRT 版一致）

每个 ADR 必含：状态/日期/相关代码（带行号）/背景/决策/后果（正负面）/注意事项 + **Bug 定位提示**。

### 3.3 Bug 定位提示段（必填）

回答：现象是什么？该查哪里？哪些情况可排除？

**示例（Dialog 动画场景）**：

```markdown
- **Bug 定位提示**：如果 Dialog 打开/关闭动画异常：
  1. DialogState 是否卡在 Opening/Closing（动画回调未触发转换）
  2. `close_on_click_modal` 是否在 Opening 状态被误触发
  3. 动画时长（AnimationCurve）是否在无动画环境下异常
  4. 可排除：逻辑错误（若 state 穷举测试全通过）
```

---

## 4. ADR 覆盖率标准

### 4.1 必须覆盖的决策类型

| 决策类型 | HarUI 示例 |
|---------|-----------|
| **组件模式** | 五段式模式（Props/State/Message/view/handle）为什么这么设计 |
| **状态机设计** | Dialog 4 态动画、Select 开合、Keypad 编辑确认 |
| **样式架构** | style_sheets 纯函数分离（ButtonKind 放 core 避免循环依赖） |
| **主题系统** | 双主题（element_light/dark）、iced_adapter 映射 |
| **IME/键盘** | ImeProcessor 组合输入、ShortcutRegistry |
| **虚拟滚动** | Table 100 行阈值虚拟化 |
| **POS 组件金额** | 整数分比较（防浮点误差） |
| **测试策略** | render test vs 逻辑 test 分工、proptest |

### 4.2 覆盖率度量

- **ADR 密度** ≥ 0.15（每 1000 行至少 0.15 个 ADR）
- 定期盲区识别：62 个组件中无 ADR 覆盖的 = 盲区

---

## 5. Bug 定位流程（四层模型，HarUI 版）

```
UI bug 报告
   │
   ▼
第 1 层：决策层（ADR）       → 排除设计限制 / 确认 bug
   │
   ▼
第 2 层：render test 层      → 跑 *_view.rs + *_test.rs，确认状态机行为
   │
   ▼
第 3 层：screenshot 层       → screenshot_test 示例像素对比（明暗/字体/布局）
   │
   ▼
第 4 层：代码层              → 读源码 + 写复现测试
```

### 每层失败处理

- 第 1 层失败 → 进第 2 层，事后补 ADR
- 第 2 层失败 → 进第 3 层，事后补 render test
- 第 3 层失败 → 进第 4 层，事后补 screenshot 用例
- 第 4 层失败 → 加截图/日志，等复现

---

## 6. ADR 有效性验证

- **零上下文子代理测试**：每次新增 ADR 后执行，3/3 通过
- **Bug 定位命中率测试**：每季度，4 类 bug（设计限制/实现错误/运行时状态/未覆盖模块）
- **测试用例构造**：必须覆盖 4 类（与 HarRT 版一致）

---

## 7. 工程化门禁

### PR 检查清单

- [ ] 新设计决策是否写了 ADR（含 Bug 定位提示 + 行号）
- [ ] 新组件是否遵循五段式模式
- [ ] 是否配套 `*_test.rs` + `*_view.rs`
- [ ] 是否在 showcase 中演示
- [ ] 状态机是否有穷举测试

### 定期审计

- ADR 覆盖率盲区识别
- render test 覆盖率
- 失效 ADR 清理

---

## 8. 案例参考（HarUI 预设）

| Bug | 现象 | 第 1 层 | 第 2 层 | 根因 |
|-----|------|---------|---------|------|
| Bug 1 | Dialog 卡在打开动画 | ✅ ADR 负面后果吻合 | render test 断言失败 | Opening 状态无超时兜底 |
| Bug 2 | 暗色主题文字不可读 | ◐ ADR 提示中性色反转 | screenshot 对比 | 硬编码颜色未走令牌 |
| Bug 3 | Keypad 金额多一位小数 | ◐ ADR 提示 Price 模式 2 位 | 逻辑 test 失败 | 小数位未 clamp |
| Bug 4 | Table 虚拟滚动跳动 | ❌ 无 ADR | render test 通过 | viewport 高度计算错误 |

---

## 附录 A：ADR 模板

```markdown
# ADR-XXXX: 标题

- **状态**: Proposed | Accepted | Superseded | Deprecated
- **日期**: YYYY-MM-DD
- **相关代码**: `crates/components/src/xxx.rs (L123-L145)`

## 背景

## 决策

```rust
// 代码片段
```

## 后果

**正面**：...
**负面**：...

## 注意事项

- **Bug 定位提示**：如果 [现象]，检查：
  1. ...
  2. ...
```

---

## 附录 B：与其他文档的关系

- 本规范定义 **HarUI 的 ADR 与 Bug 定位方法论**；
- [`软件项目审计清单.md`](软件项目审计清单.md) 定义审计维度（组件模式审计 P0 是其落地）；
- [`docs/screenshot_test.md`](screenshot_test.md) 定义截图测试工作流（第 3 层依据）。

> **文档结束**
