# HarUI

> Rust native UI component library, Element Plus equivalent in Rust ecosystem.

[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![iced](https://img.shields.io/badge/iced-0.14.0-blue.svg)](https://github.com/iced-rs/iced)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-1.3.0-brightgreen.svg)](#)

## Status: v1.3.0 — EP 对齐补齐 + iced 0.14 + headless 行为层，可被业务项目引用

HarUI 是一个对标 [Element Plus](https://element-plus.org/) 的 Rust 原生 UI 组件库，基于 [iced 0.14.0](https://github.com/iced-rs/iced) 构建，专为 POS（Point of Sale）收银场景设计，同时适用于桌面端管理后台、表单应用、数据展示等场景。

## Features

- **68 组件**：覆盖 Element Plus 全部核心组件 + 333 图标（EP 全集），包含 POS 专用组件（Keypad / Payment / HangOrder / CustomerDisplay / StatusBar）
- **主题系统**：light / dark 双主题，CSS 变量级别的颜色 token 系统，完全复刻 Element Plus 设计规范
- **虚拟滚动**：Table 组件支持 1000+ 行虚拟滚动，保持 60 FPS
- **IME 支持**：Input / Textarea 内置中文输入法支持，无闪烁、候选框正常
- **零依赖绘制**：纯 Rust 原生渲染（iced + winit + wgpu），无需 Web View / Node.js
- **2100+ 测试**：单元 + 集成 + 模糊(proptest) + 业务流回放 + 快照测试
- **headless 行为层**：弹层 12 方位定位引擎（碰撞翻转/钳制）、虚拟列表、焦点环——纯计算可全空间 fuzz
- **Table 冻结列**：左/右冻结 + 列宽拖拽 + 横向滚动，与虚拟滚动正交
- **FPS 自证**：`window::frames` 逐帧统计 overlay（showcase / table_demo 可切换）
- **AI 技能包**：`docs/skill/har-ui-skills.md`，API 速查由脚本从源码生成
- **零警告编译**：`cargo build --workspace` 0 warning 0 error

## Component Categories

| Category | Components | Count |
|----------|-----------|-------|
| 基础 | Button / Input / InputNumber / Select / Radio / Checkbox / Switch / Tag / Badge / Avatar | 10 |
| 表单 | Form / DatePicker / TimePicker / Cascader / Slider / Rate / ColorPicker / Upload / Autocomplete / Image / TreeSelect / Transfer | 12 |
| 数据展示 | Table / Card / Pagination / Descriptions / Timeline / Collapse / Statistic / Empty / Result / Skeleton / Progress / Tree / Countdown | 13 |
| 导航 | Tabs / Steps / Breadcrumb / Dropdown / Backtop / Affix / Menu / PageHeader / Anchor | 9 |
| 反馈 | Dialog / Drawer / MessageBox / Notification / Message / Loading / Tooltip / Popover / Popconfirm / Alert | 10 |
| 其他 | Text / Link / Divider / Space / Scrollbar / Calendar / Carousel / Grid / Watermark | 9 |
| POS 专用 | Keypad / Payment / HangOrder / CustomerDisplay / StatusBar | 5 |
| **合计** | | **68** |

## WASM 画廊（Web 部署）

68 组件全量支持 `wasm32-unknown-unknown` 编译（iced 0.14）。本地体验：

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli   # wasm-bindgen-cli 版本需与 Cargo.lock 一致
cd examples/showcase
trunk serve                            # http://localhost:8080
trunk build --release                  # 产物 dist/ 可直接部署静态站点
```

## Quick Start

### 引用方式

```toml
[dependencies]
har-ui-core = { git = "https://github.com/harui/har-ui.git", tag = "v1.0.0" }
har-ui-components = { git = "https://github.com/harui/har-ui.git", tag = "v1.0.0" }
iced = "0.14"
```

### 最简示例

```rust
use har_ui_components::button::{Button, ButtonType};
use har_ui_core::theme::Theme;
use iced::widget::{column, container};
use iced::{Element, Length, Task};

fn main() -> iced::Result {
    iced::application(
        || {
            (
                State {
                    theme: Theme::element_light(),
                    button: Button::new("Click me").with_type(ButtonType::Primary),
                },
                Task::none(),
            )
        },
        update,
        view,
    )
    .title(|_state: &State| String::from("HarUI App"))
    .window_size(iced::Size::new(640.0, 480.0))
    .run()
}

struct State { theme: Theme, button: Button }

#[derive(Debug, Clone)]
enum Message { ButtonClick }

fn update(state: &mut State, msg: Message) -> Task<Message> {
    match msg { Message::ButtonClick => Task::none() }
}

fn view(state: &State) -> Element<'_, Message> {
    container(
        column!(state.button.view(&state.theme, Message::ButtonClick))
            .padding(40)
    )
    .width(Length::Fill).height(Length::Fill).center_x(Length::Fill).center_y(Length::Fill)
    .into()
}
```

## Examples

所有 demo 可独立运行：

```bash
cargo run -p button_demo
cargo run -p input_demo
cargo run -p table_demo        # 1000 行虚拟滚动
cargo run -p dialog_demo
cargo run -p form_demo
cargo run -p select_demo
cargo run -p pagination_demo
cargo run -p card_demo
cargo run -p tabs_demo
cargo run -p date_picker_demo  # 自绘日历
cargo run -p cascader_demo
cargo run -p upload_demo
cargo run -p showcase          # 12 组件综合演示
cargo run -p screenshot_test   # 截图测试框架
```

## Testing

```bash
# 全量测试（2030+ tests, 0 failure）
cargo test --workspace

# 单 crate 测试
cargo test -p har-ui-core
cargo test -p har-ui-components

# 渲染层验收
cargo build --workspace        # 0 warning 0 error
```

## Project Structure

```
har-ui/
├── crates/
│   ├── core/                  # 主题系统 + 工具层 + 布局层
│   │   ├── src/
│   │   │   ├── theme/         # Theme + Color tokens（复刻 Element Plus）
│   │   │   ├── layout/        # col / row / container / grid 布局
│   │   │   └── utils/         # IME / keyboard / virtual_scroll
│   │   └── tests/
│   └── components/            # 60 组件实现
│       ├── src/               # 组件源码（view() + handle() + getter）
│       └── tests/             # 单元测试 + view 渲染测试
├── examples/                  # 14 个 demo（含 showcase 综合演示）
└── docs/                      # 文档
```

## License

MIT
