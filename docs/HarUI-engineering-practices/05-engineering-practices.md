# HarUI 项目工程化实践规范

> 适用范围：所有基于 iced 0.13 + HarUI 的 Rust GUI 项目
> 版本：v1.0  日期：2026-07-20
> 来源：菜场收银台 v0.1.0 工程化实践提炼（1873 测试 / 0 panic / 单 binary crate）

---

## 一、项目结构规范

### 1.1 单 Binary Crate 布局

```
菜场收银台/                       # 项目根
├── Cargo.toml                   # [package] + [features] + [dependencies]
├── build.rs                     # Windows 资源嵌入（图标）
├── rustfmt.toml                 # 格式化规则
├── clippy.toml                  # lint 规则
├── .github/
│   └── workflows/
│       ├── ci.yml               # 7 关门禁
│       └── release.yml          # 打包发布
├── scripts/
│   ├── gate.ps1                 # 门禁脚本
│   ├── audit-api-changes.ps1    # 契约审计
│   ├── pub-api-baseline.txt     # API 基线
│   └── pixel_verify.ps1         # 像素验证
├── docs/
│   ├── api-contracts.md         # 公共 API 行为契约
│   └── adr/                     # 架构决策记录
├── resources/
│   └── favicon.ico              # 应用图标
├── src/
│   ├── main.rs                  # 程序入口
│   ├── app.rs                   # AppState + AppMessage + update()
│   ├── router.rs                # 路由定义
│   ├── pages/                   # 页面视图（login/main/home/...）
│   ├── components/              # 可复用组件（print_setting/cart/...）
│   ├── store/                   # 状态管理（app_state/cart_state/...）
│   ├── api/                     # API 客户端（user/order/goods/...）
│   ├── hardware/                # 硬件驱动（scale/scanner/rfid/...）
│   └── utils/                   # 工具（encryption/audio/mqtt/...)
└── tests/
    ├── contract/                # 契约测试
    ├── real_api/                # 真实 API 集成测试（#[ignore]）
    └── pixel/                   # 像素验证
```

### 1.2 Cargo.toml 规范

```toml
[package]
name = "caichang-shouyintai"
version = "0.1.0"
edition = "2021"
description = "菜场收银台 — Rust + iced + HarUI 实现"
authors = ["盛庄集团"]
license = "Proprietary"

# 打包配置（cargo-bundle 读取）
[package.metadata.bundle]
name = "盛庄收银台"
identifier = "cashier"
version = "1.0.0"
icon = ["resources/favicon.ico"]

[dependencies]
# UI 框架
iced = { version = "0.13", features = ["advanced", "tokio"] }
har-ui-core = { path = "../har-ui/crates/core" }
har-ui-components = { path = "../har-ui/crates/components" }

# API 层（optional，按需启用）
reqwest = { version = "0.12", features = ["json", "rustls-tls"], default-features = false, optional = true }
tokio = { version = "1", features = ["rt-multi-thread", "macros", "fs", "sync", "time"], optional = true }

# 加密
md-5 = "0.10"
aes = "0.8"
cbc = { version = "0.1", features = ["alloc"] }
rsa = { version = "0.9", default-features = false, features = ["pem"] }

[features]
# 默认启用所有层
default = ["api", "hardware", "comm"]
api = ["dep:reqwest", "dep:tokio"]
hardware = ["dep:tokio-tungstenite", "dep:futures-util", "api"]
comm = ["dep:rumqttc", "api"]

[profile.release]
opt-level = 3
lto = false
codegen-units = 16
```

**关键原则**：
- 版本号**集中管理**在 `[package]`
- 可选依赖用 `optional = true` + `[features]` 控制
- `default` feature 包含所有生产环境需要的层
- 硬件/通信层依赖 API 层（`hardware = ["...", "api"]`）

---

## 二、代码质量规范

### 2.1 禁止清单

| 编号 | 禁止项 | 替代方案 |
|------|--------|---------|
| Q1 | `unimplemented!()` / `todo!()` 在生产代码 | 返回 `Err` 或占位 UI |
| Q2 | `panic!()` 在 view() / update() | 返回占位 Element / 忽略消息 |
| Q3 | `unwrap()` / `expect()` 在生产代码 | `match` 或 `?` |
| Q4 | `unwrap_or_default()` 掩盖错误 | 显式处理 |
| Q5 | 硬编码字符串/数字 | `const` 常量 |
| Q6 | `console.log` / `println!` 留在代码 | 删除（用户规则）|
| Q7 | `render_skeleton` 调用残留 | 删除函数定义 |
| Q8 | `PowerShell` 重定向替换文件 | 用 Edit 工具 |
| Q9 | `sshpass` 登录服务器 | 用 Node.js ssh2 包 |

### 2.2 推荐清单

| 编号 | 推荐项 |
|------|--------|
| R1 | 用 `thiserror` 定义错误类型 |
| R2 | 用 `tracing` 替代 `println!`（生产代码）|
| R3 | 用 `Arc<T>` 替代 `Rc<T>`（异步场景）|
| R4 | 用 `Cow<'_, T>` 减少不必要的 clone |
| R5 | 用 `iced::Task::perform` 处理异步副作用 |
| R6 | 用 `iced::Task::done` 处理同步副作用消息 |
| R7 | view() 函数保持纯函数，无副作用 |
| R8 | update() 函数返回 Task 而非直接执行异步操作 |

### 2.3 clippy 严格规则

```toml
# clippy.toml
msrv = "1.75"
too-many-arguments-threshold = 10
```

```rust
// main.rs
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
)]
#![allow(
    clippy::module_name_repetitions,
    clippy::missing_errors_doc,
    clippy::must_use_candidate,  // GUI 项目返回值多为 Element，不必标注 must_use
)]
```

---

## 三、错误处理规范

### 3.1 错误分层

```
ApiError (api/mod.rs)
├── NetworkError(reqwest::Error)
├── Timeout
├── DecodeError(serde_json::Error)
├── EncryptError(String)
├── ServerError { code: i32, msg: String }
└── Offline

HardwareError (hardware/mod.rs)
├── ConnectionFailed(String)
├── DeviceNotFound(String)
├── IoError(std::io::Error)
└── ProtocolError(String)

AppState 层用 String 存储错误消息（对应 login_view.error_msg）
```

### 3.2 view() 中的错误处理

```rust
// ❌ 错误：view() 中 panic
pub fn view<'a>(state: &'a AppState, _theme: &'a Theme) -> Element<'a, AppMessage> {
    let item = state.cart_store.goods_list.first().unwrap(); // panic!
    text(item.good_name.clone()).into()
}

// ✅ 正确：view() 中降级处理
pub fn view<'a>(state: &'a AppState, _theme: &'a Theme) -> Element<'a, AppMessage> {
    let item_name = state.cart_store.goods_list.first()
        .map(|i| i.good_name.clone())
        .unwrap_or_else(|| "待收银".to_string());
    text(item_name).into()
}
```

### 3.3 update() 中的错误处理

```rust
// ❌ 错误：update() 中 panic
AppMessage::LoginSubmit => {
    let client = state.api_client.as_ref().unwrap(); // panic!
}

// ✅ 正确：update() 中降级 + 错误消息
AppMessage::LoginSubmit => {
    let client = match state.api_client.as_ref() {
        Some(c) => c.clone(),
        None => {
            state.login_view.set_error("API 客户端未初始化");
            return iced::Task::none();
        }
    };
    // ...
}
```

---

## 四、安全规范

### 4.1 加密原语

| 用途 | 推荐库 | 禁止 |
|------|--------|------|
| AES-256-CBC | `aes` + `cbc` | 手写 AES |
| RSA-PKCS1v15 | `rsa` | 手写 RSA |
| MD5（仅密钥派生）| `md-5` | 用于密码哈希 |
| 随机数 | `rand`（OsRng）| `Math.random()` |
| HMAC | `hmac` | 手写 HMAC |

### 4.2 AES-256-CBC 固定 IV 的注意事项

菜场收银台使用固定 IV `"NJSZJT_LIUJIECLZ"`（对应原项目）。这是**不安全**的做法，但为了与后端兼容必须保留。

```rust
// 固定 IV（对应原项目，不安全但兼容后端）
const AES_IV: &str = "NJSZJT_LIUJIECLZ"; // 16 字节

// AES key 为 32 字节 hex 字符串
pub fn init_aes_key() -> String {
    let timestamp = chrono::Local::now().timestamp_millis();
    let random: String = (0..32).map(|_| {
        let c = rand::random::<u8>() % 62;
        if c < 10 { (b'0' + c) as char }
        else if c < 36 { (b'a' + c - 10) as char }
        else { (b'A' + c - 36) as char }
    }).collect();
    format!("{:x}", md5::compute(format!("{}{}", timestamp, random).as_bytes()))
}
```

> **ADR 建议**：新项目应使用随机 IV 并在请求中传输，而非固定 IV。

### 4.3 RSA 公钥硬编码

```rust
// 公钥硬编码在 AppStore::new() 中（对应原项目 store/modules/app.js state.publicKey）
const PUBLIC_KEY: &str = "-----BEGIN PUBLIC KEY-----\n\
    MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA...\n\
    -----END PUBLIC KEY-----";
```

### 4.4 Token 存储

```rust
// ❌ 错误：token 留在内存明文
pub token: String,

// ✅ 正确：登录后存入 UserStore，退出时清空
impl UserStore {
    pub fn set_token(&mut self, token: String) {
        self.token = token;
    }
    pub fn clear_token(&mut self) {
        self.token.clear();
    }
}
```

---

## 五、UI 自适应分辨率规范

### 5.1 目标分辨率

| 分辨率 | 优先级 | 说明 |
|--------|--------|------|
| 1366×768 | P0 | 收银台常见分辨率 |
| 1920×1080 | P0 | 标准全高清 |
| 1280×720 | P1 | 低分辨率兼容 |
| 2560×1440 | P2 | 2K 屏 |

### 5.2 自适应实现原则

```rust
// ✅ 正确：用 Fill + FillPortion 自适应
let layout = row![
    sidebar.width(Length::Fixed(80.0)),     // 固定宽度侧边栏
    content.width(Length::Fill),            // 主内容自适应
    cart.width(Length::FillPortion(3)),     // 购物车占 3/10
];

// ✅ 正确：字号用固定值（不随分辨率缩放）
let title = text("收银台").size(24);

// ❌ 错误：字号随窗口尺寸变化（导致 1366×768 下文字溢出）
let title = text("收银台").size(window_width / 50.0);
```

### 5.3 表单容器固定宽度

登录表单等居中布局用固定宽度，不随窗口缩放：

```rust
// ✅ 正确：表单固定 420px，居中
let form = container(form_content)
    .width(Length::Fixed(420.0))
    .center_x(Length::Fill);

// ❌ 错误：表单 Fill 导致 1920×1080 下过宽
let form = container(form_content).width(Length::Fill);
```

### 5.4 像素验证

每次 UI 变更后，必须在 1366×768 和 1920×1080 下分别截图验证（见 [04-test-pyramid.md](04-test-pyramid.md) 第六章）。

---

## 六、CI/CD 规范

### 6.1 CI 流水线（GitHub Actions）

```yaml
name: CI
on:
  push:
    branches: [main, master, develop]
  pull_request:

jobs:
  gate:
    runs-on: windows-latest  # GUI 项目必须在 Windows 上测试
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - name: Run 7 Gates
        run: pwsh ./scripts/gate.ps1

  matrix:
    strategy:
      matrix:
        os: [windows-latest]  # iced GUI 主要支持 Windows
        rust: [stable, beta]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@${{ matrix.rust }}
      - run: cargo test --all-features
```

### 6.2 发布流程

1. 更新 `Cargo.toml` 版本号 + `[package.metadata.bundle]` version
2. 更新 `CHANGELOG.md`
3. 运行完整 7 关门禁
4. 创建 git tag `v0.1.0`
5. `cargo build --release --all-features` 生成 .exe
6. 可选：`cargo bundle --release` 生成安装包
7. GitHub Release 创建

---

## 七、文档规范

### 7.1 必备文档

| 文档 | 位置 | 内容 |
|------|------|------|
| README.md | 根 | 项目概览、快速入门 |
| CONTRIBUTING.md | 根 | 贡献流程、代码规范 |
| CHANGELOG.md | 根 | 版本变更记录 |
| api-contracts.md | docs/ | State/Message 行为契约 |
| ADR | docs/adr/ | 架构决策记录 |
| 迁移进度.md | docs/ | 迁移任务进度（如适用）|

### 7.2 文档注释规范

```rust
/// 登录页视图入口
///
/// 对应原项目 `views/Login.vue`（252 行）。
///
/// # 参数
/// - `state`: 应用状态（含 login_view 子状态）
/// - `theme`: HarUI 主题
///
/// # 返回
/// 返回登录页 Element，包含系统名称 + Logo + 用户名/密码输入 + 登录按钮
///
/// # 前置条件
/// - `state.app_store.aes_key.is_some()`（AppStore::new 已初始化）
/// - `state.app_store.public_key` 非空
///
/// # 示例
/// ```
/// let state = AppState::new();
/// let theme = Theme::element_light();
/// let el = view(&state, &theme);
/// ```
///
/// # 安全性
/// 此函数不执行网络请求，仅渲染 UI。登录请求在 `update(LoginSubmit)` 中触发。
pub fn view<'a>(state: &'a AppState, theme: &'a Theme) -> Element<'a, AppMessage> {
    // ...
}
```

### 7.3 ADR 模板

```markdown
# ADR-NN: <决策标题>

日期：YYYY-MM-DD
状态：提议 / 接受 / 废弃

## 背景
<为什么需要这个决策？>

## 决策
<做了什么决策？>

## 备选方案
- 方案 A：...
- 方案 B：...

## 后果
- 正面：...
- 负面：...
- 中性：...
```

---

## 八、版本管理规范

### 8.1 Semantic Versioning

- MAJOR：破坏性 API 变更（修改 AppMessage 变体、view 签名）
- MINOR：兼容新增功能（新增页面、新增 Component）
- PATCH：bug 修复

### 8.2 0.x 阶段

- 0.x.y：兼容新增功能（按 MINOR 处理）
- 0.x.0 之后的破坏性变更可以不升 MAJOR

### 8.3 版本集中管理

GUI 单 binary 项目版本集中在 `[package]` version 和 `[package.metadata.bundle]` version 两处。发布时**两处必须同步更新**。

```toml
[package]
version = "0.1.0"           # ← 此处

[package.metadata.bundle]
version = "1.0.0"           # ← 此处（产品对外版本，可与 cargo 版本不同）
```

---

## 九、依赖管理规范

### 9.1 依赖审查

- 新增依赖必须 PR 评审
- `cargo tree -d` 检查重复依赖
- iced 0.13 依赖冲突是最常见的编译问题

### 9.2 optional 依赖

所有非核心依赖（reqwest/tokio/tokio-tungstenite/rumqttc）必须 `optional = true`，通过 `[features]` 启用：

```toml
[dependencies]
reqwest = { version = "0.12", features = ["json", "rustls-tls"], default-features = false, optional = true }

[features]
api = ["dep:reqwest"]
```

---

## 十、可观测性规范

### 10.1 tracing 使用

GUI 项目建议在 debug build 中启用 `tracing`，release build 中移除：

```rust
// 仅 debug 输出日志
#[cfg(debug_assertions)]
{
    tracing::info!(username = %state.login_view.username, "Login attempt");
}
```

### 10.2 错误采集

所有 `Task::perform` 的错误必须通过 `AppMessage::XxxResult(Err(...))` 回传，并在 UI 中显示 Toast：

```rust
AppMessage::LoginResult(Err(msg)) => {
    state.login_view.set_error(msg.clone());
    return iced::Task::done(AppMessage::ToastShow {
        text: msg,
        kind: ToastKind::Error,
    });
}
```

---

## 十一、成熟度评估维度

每季度对项目进行多维度评估：

| 维度 | 权重 | 评估项 |
|------|------|--------|
| 工程化 | 20% | CI/CD、文档、版本管理、依赖管理 |
| 可靠性 | 25% | 测试覆盖、错误处理、并发安全、容错 |
| UI 质量 | 20% | 自适应分辨率、像素验证、无 panic |
| 安全 | 15% | 加密、Token 管理、输入校验 |
| 可维护性 | 10% | 代码质量、模块化、注释 |
| 性能 | 10% | 启动时间、页面切换延迟 |

**评分标准**：
- 5.0：行业领先
- 4.5：生产可用
- 4.0：成熟
- 3.5：可用但需改进
- <3.5：未达生产标准

---

## 十二、与其他规范的关系

- 本规范定义**HarUI 工程化的全局规范**；
- [01-integration-gates.md](01-integration-gates.md) 定义**门禁如何执行**；
- [02-contract-audit.md](02-contract-audit.md) 定义**State/Message 行为契约**；
- [03-concurrency-safety.md](03-concurrency-safety.md) 定义**Task::perform 并发约束**；
- [04-test-pyramid.md](04-test-pyramid.md) 定义**GUI 测试体系**。
