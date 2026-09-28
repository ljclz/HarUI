# HarUI 项目测试体系规范

> 适用范围：所有基于 iced 0.13 + HarUI 的 Rust GUI 项目
> 版本：v1.0  日期：2026-07-20
> 来源：菜场收银台 v0.1.0 四线验证体系（1873 测试通过）

---

## 一、GUI 测试金字塔

```
                ▲
                │   ┌─────────────┐
                │   │  像素验证    │  截图 + 颜色分布断言（~10 项）
                │   └─────────────┘
                │ ┌──────────────────┐
                │ │   真机集成        │  真实 API + 硬件 DLL（#[ignore]）
                │ └──────────────────┘
              ┌─┴──────────────────────┐
              │     视图渲染测试         │  view() 不 panic（~200 项）
              └─┬──────────────────────┘
            ┌───┴──────────────────────────┐
            │         单元测试               │  State 转移 / 纯函数（~1600 项）
            └──────────────────────────────┘
```

---

## 二、四线验证体系

| 线 | 名称 | 目标 | 工具 | 触发时机 |
|----|------|------|------|---------|
| 1 | 单元 | State 转移 + 纯函数 + 消息处理 | `cargo test` | 每次提交 |
| 2 | 视图 | `view()` 在任意 State 下不 panic | `cargo test` | 每次提交 |
| 3 | 像素 | 截图 + 颜色分布断言 | PowerShell + .NET | 每次提交 |
| 4 | 集成 | 真实 API + 硬件 DLL | `#[ignore]` 标记 | 手动 / 发版前 |

---

## 三、测试目录结构

```
菜场收银台/
├── src/
│   ├── pages/
│   │   ├── login.rs          # 模块内 #[cfg(test)] mod tests
│   │   ├── main.rs           # 主页视图测试
│   │   └── ...
│   ├── store/
│   │   ├── cart_state.rs     # CartStore 单元测试
│   │   ├── login_state.rs    # LoginView 单元测试
│   │   └── ...
│   ├── api/
│   │   ├── user.rs           # UserApi 单元测试（mock）
│   │   └── ...
│   └── components/
│       └── *.rs              # 组件测试
└── tests/
    ├── contract/             # 契约测试（每章一模块）
    │   ├── mod.rs
    │   ├── state_transfer.rs
    │   └── message_handle.rs
    ├── real_api/             # 真实 API 集成测试（#[ignore]）
    │   ├── mod.rs
    │   └── login.rs
    └── pixel/                # 像素验证测试
        └── mod.rs
```

---

## 四、单元测试（线 1）

### 4.1 State 转移测试

```rust
#[test]
fn test_login_submit_sets_logging_in_flag() {
    let mut state = AppState::new();
    assert!(!state.login_view.is_logging_in);
    let _ = state.update(AppMessage::LoginSubmit);
    assert!(state.login_view.is_logging_in);
}

#[test]
fn test_login_submit_empty_username_sets_error() {
    let mut state = AppState::new();
    state.login_view.username.clear();
    let _ = state.update(AppMessage::LoginSubmit);
    assert_eq!(state.login_view.error_msg, "请输入登录账号");
    assert!(!state.login_view.is_logging_in);
}

#[test]
fn test_login_result_success_extracts_token_and_store() {
    let mut state = AppState::new();
    state.current_route = Route::Login;
    let data = serde_json::json!({
        "token": "abc123",
        "data": {
            "store_id": 100,
            "cashier_name": "李四",
            "version": "1.0.0"
        }
    });
    let _ = state.update(AppMessage::LoginResult(Ok(data)));
    assert_eq!(state.current_route, Route::Main);
    assert_eq!(state.user_store.token, "abc123");
    assert_eq!(state.user_store.store.store_id, 100);
    assert_eq!(state.user_store.cashier.cashier_name, "李四");
}
```

### 4.2 纯函数测试

```rust
#[test]
fn test_cart_total_fee_calculation() {
    let mut cart = CartStore::new();
    cart.push_product(CartItem {
        good_id: 1,
        good_name: "苹果".to_string(),
        weight: 0.5,
        price: 10.0,
        total_fee: 5.0,
        ..
    });
    cart.push_product(CartItem {
        good_id: 2,
        good_name: "香蕉".to_string(),
        weight: 1.0,
        price: 6.0,
        total_fee: 6.0,
        ..
    });
    assert!((cart.cart_total_fee() - 11.0).abs() < f64::EPSILON);
}
```

### 4.3 加密函数测试

```rust
#[test]
fn test_aes_encrypt_decrypt_roundtrip() {
    let key = "0123456789abcdef0123456789abcdef"; // 32 字节 hex
    let iv = "NJSZJT_LIUJIECLZ"; // 16 字节
    let plaintext = r#"{"username":"18851052481","password":"123456"}"#;
    let ciphertext = aes_encrypt(plaintext, key, iv).unwrap();
    let decrypted = aes_decrypt(&ciphertext, key, iv).unwrap();
    assert_eq!(decrypted, plaintext);
}
```

---

## 五、视图渲染测试（线 2）

### 5.1 标准 view 测试骨架

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::AppState;
    use har_ui_core::Theme;

    #[test]
    fn test_view_renders_without_panic() {
        let state = AppState::new();
        let theme = Theme::element_light();
        let _el = view(&state, &theme);
    }

    #[test]
    fn test_view_with_error_message() {
        let mut state = AppState::new();
        state.login_view.set_error("用户名或密码错误");
        let theme = Theme::element_light();
        let _el = view(&state, &theme);
    }

    #[test]
    fn test_view_with_logging_in_state() {
        let mut state = AppState::new();
        state.login_view.start_logging();
        let theme = Theme::element_light();
        let _el = view(&state, &theme);
    }

    #[test]
    fn test_view_with_empty_cart() {
        let state = AppState::new();
        let theme = Theme::element_light();
        let _el = view(&state, &theme);  // 空购物车不应 panic
    }

    #[test]
    fn test_view_with_filled_cart() {
        let mut state = AppState::new();
        state.cart_store.push_product(CartItem {
            good_id: 1,
            good_name: "测试商品".to_string(),
            weight: 0.5,
            price: 10.0,
            total_fee: 5.0,
            ..Default::default()
        });
        let theme = Theme::element_light();
        let _el = view(&state, &theme);
    }
}
```

### 5.2 视图测试覆盖维度

每个 `view()` 函数至少覆盖以下 State 组合：

| State 维度 | 测试值 |
|-----------|--------|
| 默认 State | `AppState::new()` |
| 错误状态 | `error_msg` 非空 |
| 加载状态 | `is_logging_in` / `is_loading` 为 true |
| 空数据 | 空列表 / 空购物车 |
| 满数据 | 列表有多项 / 购物车有多项 |
| 极端数据 | 超长文本 / 超大数字 |

---

## 六、像素验证测试（线 3）

### 6.1 为什么需要像素验证

视图渲染测试只能证明 `view()` 不 panic，无法证明 UI 真正显示在屏幕上。iced 的 winit 后端在某些情况下可能：
- 窗口创建失败但 `view()` 返回正常
- GPU 渲染异常导致白屏
- 布局计算错误导致元素不可见

像素验证通过截图 + 颜色分布分析，独立验证 UI 真实渲染。

### 6.2 PowerShell 像素验证脚本

```powershell
# scripts/pixel_verify.ps1
param(
    [string]$WindowTitle = "菜场收银台",
    [string]$ExpectedBgColor = "64/158/255",  # 蓝色背景
    [float]$MinBgPercent = 60.0
)

Add-Type -AssemblyName System.Drawing
Add-Type -AssemblyName UIAutomationClient

# 1. 查找窗口
$root = [System.Windows.Automation.AutomationElement]::RootElement
$cond = New-Object System.Windows.Automation.PropertyCondition(
    [System.Windows.Automation.AutomationElement]::NameProperty, $WindowTitle)
$win = $root.FindFirst([System.Windows.Automation.TreeScope]::Children, $cond)
if (-not $win) { Write-Error "Window not found"; exit 1 }

# 2. 截图
$hwnd = $win.Current.NativeWindowHandle
$shotPath = "$env:TEMP\pixel_verify_$(Get-Date -Format 'yyyyMMdd_HHmmss').png"
& "$PSScriptRoot\take_screenshot.ps1" -WindowHandle $hwnd -Path $shotPath

# 3. 分析颜色分布
$img = [System.Drawing.Image]::FromFile($shotPath)
$bmp = New-Object System.Drawing.Bitmap($img)
$colors = @{}
$samples = 0
for ($y = 0; $y -lt $bmp.Height; $y += 8) {
    for ($x = 0; $x -lt $bmp.Width; $x += 8) {
        $c = $bmp.GetPixel($x, $y)
        $key = "$($c.R)/$($c.G)/$($c.B)"
        if (-not $colors.ContainsKey($key)) { $colors[$key] = 0 }
        $colors[$key]++
        $samples++
    }
}

# 4. 断言主色调
$bgPercent = ($colors[$ExpectedBgColor] / $samples) * 100
if ($bgPercent -lt $MinBgPercent) {
    Write-Error "Background color $ExpectedBgColor only $([math]::Round($bgPercent,1))%, expected >= $MinBgPercent%"
    exit 1
}
Write-Output "OK: Background $ExpectedBgColor = $([math]::Round($bgPercent,1))%"

$bmp.Dispose()
$img.Dispose()
```

### 6.3 像素验证覆盖矩阵

| 页面 | 期望主色调 | 最低占比 |
|------|-----------|---------|
| 登录页 | 64/158/255（蓝色背景）| 60% |
| 主页（收银台）| 240/240/240（浅灰背景）| 50% |
| 商品管理 | 250/250/250（白色）| 60% |
| 404 页 | 255/255/255（白色）| 80% |

---

## 七、真实集成测试（线 4）

### 7.1 真实 API 测试

```rust
// tests/real_api/login.rs
#![cfg(feature = "api")]
use caichang_shouyintai::api::{ApiClient, ApiConfig, user::UserApi};
use caichang_shouyintai::store::login_state::LoginState;

#[tokio::test]
#[ignore] // 需要网络，手动运行: cargo test --all-features -- --ignored
async fn real_login_with_default_credentials() {
    let client = ApiClient::new(ApiConfig {
        base_url: "https://wushe.ljclz.vip".to_string(),
        timeout_secs: 30,
        online: true,
    }).expect("API client init failed");

    let state = LoginState::new();
    let params = state.build_login_params("");
    let aes_key = caichang_shouyintai::utils::encryption::init_aes_key();
    let public_key = state.public_key(); // 或硬编码公钥

    let result = UserApi::login(&client, &params, "", &aes_key, &public_key).await;
    assert!(result.is_ok(), "Login API call failed: {:?}", result.err());

    let resp = result.unwrap();
    assert!(resp.code == 1 || resp.code == -1, "Unexpected code: {}", resp.code);
    // code=-1 表示密钥不准确（测试环境），code=1 表示登录成功
}
```

### 7.2 硬件 DLL 测试

```rust
// tests/real_hardware/scale.rs
#![cfg(feature = "hardware")]
#[tokio::test]
#[ignore]
async fn real_scale_read_weight() {
    let mut scale = ScaleDriver::new("COM3").expect("Scale init failed");
    let weight = scale.read_weight().await.expect("Read weight failed");
    assert!(weight >= 0.0 && weight < 100.0, "Weight out of range: {}", weight);
}
```

---

## 八、测试覆盖率要求

| 测试类型 | 覆盖率门槛 | 工具 |
|---------|-----------|------|
| 行覆盖 | ≥75% | `cargo-tarpaulin` |
| 分支覆盖 | ≥65% | `cargo-tarpaulin --branch` |
| view() 函数覆盖 | 100% | 人工标注 |
| State 转移覆盖 | ≥90% | 契约测试 |
| Critical 路径 | 100% | 人工标注 |

> GUI 项目行覆盖率门槛低于后端项目（75% vs 85%），因为 view() 渲染代码难以用单元测试覆盖，依赖像素验证。

---

## 九、测试编写规范

1. **测试名遵循 `<场景>_<预期>`**：`test_login_submit_empty_username_sets_error`
2. **每个测试 Arrange-Act-Assert 三段式**
3. **测试不依赖执行顺序**（并行安全）
4. **不依赖外部资源的测试默认运行**，依赖网络/硬件的用 `#[ignore]` 标记
5. **测试不能 `unwrap()` 生产代码**，但测试内部 `unwrap()` 可接受
6. **测试代码也要通过 clippy**（`--all-targets`）
7. **测试代码也要格式化**（`cargo fmt --all`）
8. **view() 测试必须覆盖空数据 + 满数据 + 错误状态**
9. **State 转移测试必须验证终态 + 副作用（Task 是否产生）**

---

## 十、与其他规范的关系

- 本规范定义**GUI 测试如何分层与编写**；
- [01-integration-gates.md](01-integration-gates.md) 定义**测试如何作为门禁执行**；
- [02-contract-audit.md](02-contract-audit.md) 定义**State 转移契约测试如何编写**；
- [03-concurrency-safety.md](03-concurrency-safety.md) 定义**并发测试要验证的不变量**。
