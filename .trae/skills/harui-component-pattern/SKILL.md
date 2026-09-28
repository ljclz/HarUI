---
name: harui-component-pattern
description: HarUI 组件模式检查 — 确保新组件遵循 Props/State/Message/view()/handle() 五段式模式、样式走 style_sheets、状态机有穷举转换。新增或修改组件时触发。
tools: [cargo, cargo-test]
agentMode: auto
---

# HarUI 组件模式检查

## 触发条件

- 新增 `crates/components/src/<name>.rs`
- 修改现有组件（Props/State/Message/view/handle）
- 修改 `crates/core/src/theme/style_sheets.rs`

## 组件五段式模式（必遵循）

每个组件必须包含：

```rust
// 1. Props struct — 配置属性，with_* builder
#[derive(Debug, Clone)]
pub struct ButtonProps {
    pub text: String,
    pub button_type: ButtonType,
    pub disabled: bool,
    // ...
}
impl ButtonProps {
    pub fn with_type(mut self, t: ButtonType) -> Self { self.button_type = t; self }
    // ...
}

// 2. State enum — 交互状态机（显式转换图注释）
pub enum ButtonState { Normal, Hover, Active }
// 转换图：Normal --hover--> Hover --press--> Active --release--> Normal

// 3. Message enum — 组件局部事件
pub enum ButtonMessage { Clicked, Hovered, Pressed }

// 4. view() — 纯渲染（无副作用）
pub fn view<'a, Message>(&'a self, theme: &'a Theme, on_click: impl Fn() -> Message + 'a) -> Element<'a, Message>

// 5. handle() — 状态变更（返回局部输出）
pub fn handle(&mut self, msg: ButtonMessage) -> ButtonOutput
```

## 检查步骤

1. 新组件是否遵循五段式模式（Props/State/Message/view/handle）
2. State enum 是否有文档注释的转换图
3. 样式是否走 `core/src/theme/style_sheets.rs`（不在 view 内联样式）
4. `view()` 是否纯函数（无 IO、无状态写入）
5. `handle()` 是否穷举 Message 变体
6. 是否使用 builder 模式（with_* 方法）
7. 是否配套 `*_test.rs`（逻辑测试）和 `*_view.rs`（渲染测试）

## 通过标准

- 新组件 5 段式齐全
- State 转换图穷举（无不可达状态）
- `view()` 无副作用（render test 可稳定断言）
- 样式全部走 style_sheets（无魔法数字散落组件内）
- `cargo test --workspace` 全部通过（当前基线 2030+）
- 新增组件注册到 `crates/components/src/lib.rs` 的模块声明

## 常见 Bug 定位提示

| 现象 | 排查点 |
|------|--------|
| 组件状态不更新 | handle() 是否穷举 Message？应用 update() 是否路由到 handle |
| 样式与主题不符 | 是否直接写死颜色而非走 style_sheets(theme) |
| 组件在暗色主题下错乱 | 是否依赖硬编码颜色而非 ThemeColor 中性色反转 |
| view() 有延迟 | 是否在 view 中做了计算/IO（应在 handle 或外部状态） |

## 参考

- `HarUI/docs/技术实现方案.md` — 组件库技术实现
- `HarUI/docs/设计规范.md` — Element Plus 1:1 token 映射
- `HarUI/crates/components/src/button.rs` — 参考实现
