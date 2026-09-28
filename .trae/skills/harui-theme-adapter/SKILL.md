---
name: harui-theme-adapter
description: HarUI 主题适配检查 — 确保设计令牌（Color/Typography/Spacing/Radius/Shadow/ZIndex）完整、明暗主题切换正确、iced_adapter 映射无遗漏。修改 theme/ 或 iced_adapter.rs 时触发。
tools: [cargo, cargo-test]
agentMode: auto
---

# HarUI 主题适配检查

## 触发条件

- 修改 `crates/core/src/theme/*.rs`
- 修改 `crates/core/src/theme/iced_adapter.rs`
- 修改组件中使用主题令牌的代码

## 设计令牌系统（6 个子系统）

| 子系统 | 文件 | 检查点 |
|--------|------|--------|
| Color | `color.rs` | `ThemeColor` RGBA 结构、`ColorPalette` 9 浅 + 2 深变体、`mix()` 混合函数 |
| Typography | `typography.rs` | `FontSize` 10 档、`FontWeight`、`LineHeight` 完整 |
| Spacing | `spacing.rs` | 4px 网格（xxs=2 → xxxl=40） |
| Radius | `radius.rs` | none → circle=9999 |
| Shadow | `shadow.rs` | 4 档（base/light/lighter/dark） |
| ZIndex | `zindex.rs` | normal=1 / dropdown=1000 / modal=2001 / tooltip=3100 |

## 检查步骤

1. 新组件是否使用令牌而非魔法数字（颜色/间距/圆角）
2. 明暗主题切换：`Theme::element_light()` / `Theme::element_dark()`
3. `iced_adapter::to_iced_theme()` 映射完整（5 字段 Palette：primary/success/danger/text/background）
4. 中性色反转正确（`is_dark` 标志控制，accent 色保持稳定）
5. 组件样式函数（`button_style` / `input_style` 等）从 `&Theme` 取令牌

## 通过标准

- 新组件无硬编码颜色（grep 检查 `Color::from_rgb` 直接调用）
- 明暗主题下组件均可读（对比度 ≥ 4.5:1）
- `to_iced_theme()` 映射 5 字段完整
- 令牌值符合 Element Plus 规范（对照 `docs/设计规范.md`）
- showcase 示例明暗切换无布局错乱

## 常见 Bug 定位提示

| 现象 | 排查点 |
|------|--------|
| 暗色主题下文字不可读 | 是否用了硬编码深色文字而非 `NeutralColor` 反转 |
| 主题切换后部分组件不变 | 组件样式闭包是否捕获了旧 theme（未重新求值） |
| Z 轴层级错乱（弹窗被遮挡） | ZIndex 是否用令牌（modal=2001 vs dropdown=1000） |
| 自定义主题不生效 | `to_iced_theme()` 是否只处理默认主题 |

## 参考

- `HarUI/crates/core/src/theme/theme.rs` — Theme 聚合结构
- `HarUI/docs/设计规范.md` — Element Plus token 映射表
- `HarUI/crates/core/src/theme/iced_adapter.rs` — 映射实现
