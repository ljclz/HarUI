# HarUI 项目并发安全模式规范

> 适用范围：所有使用 iced::Task + tokio + Mutex/RwLock 的 Rust GUI 项目
> 版本：v1.0  日期：2026-07-20
> 来源：菜场收银台 v0.1.0 工程实践提炼

---

## 一、为什么需要这份规范

iced 0.13 采用 Elm 架构：`update(state, message) -> (state, task)`。表面上看，`update` 是同步函数，似乎没有并发问题。但实际上：

- `Task::perform` 启动的异步任务可能在后台执行数秒（API 调用、硬件 I/O）
- 硬件驱动（DLL/HTTP/WebSocket）需要 `spawn_blocking` 包装同步阻塞操作
- `Arc<RwLock<AsrPlugin>>` 等可变插件状态需要在异步任务中访问
- MQTT/WebSocket 长连接的后台任务会持续产生 `AppMessage`

这些场景下，Rust 的 `Send + Sync` 只保证**内存安全**，不保证**逻辑正确**。本规范汇总 GUI 项目特有的并发陷阱。

---

## 二、并发安全 Checklist

### 2.1 iced::Task 与 State 共享

| 编号 | 规则 | 违反后果 |
|------|------|---------|
| T1 | **Task::perform 闭包不能借用 &mut State** | 编译错误 |
| T2 | **Task::perform 闭包只能 clone 所需数据** | 闭包不 Send |
| T3 | **Task::perform 完成后通过 AppMessage 回写 State** | State 不一致 |
| T4 | **禁止在 Task 闭包中直接修改 AppState** | 破坏 Elm 架构 |

**违反示例**：

```rust
// ❌ 错误：闭包借用 &mut state，编译失败
let task = iced::Task::perform(
    async move {
        UserApi::login(&state.api_client, ...)  // state 是 &mut 引用
    },
    |result| AppMessage::LoginResult(result),
);
```

**正确写法**：

```rust
// ✅ 闭包前 clone 所需数据，通过消息回写
let client = state.api_client.clone()?;
let aes_key = state.app_store.aes_key.clone()?;
let public_key = state.app_store.public_key.clone();
let params = state.login_view.build_login_params("");
let task = iced::Task::perform(
    async move {
        UserApi::login(&client, &params, "", &aes_key, &public_key).await
    },
    |result| match result {
        Ok(resp) => AppMessage::LoginResult(Ok(resp.data.unwrap_or(Value::Null))),
        Err(e) => AppMessage::LoginResult(Err(e.to_string())),
    },
);
// 回写通过 AppMessage::LoginResult 在 update 中处理
```

### 2.2 spawn_blocking 与 GUI 主线程

| 编号 | 规则 |
|------|------|
| B1 | **DLL 调用、串口 I/O、文件读取必须用 spawn_blocking** |
| B2 | **spawn_blocking 闭包不能持有 GUI 线程锁** |
| B3 | **spawn_blocking 结果必须通过 Task::perform 回传** |
| B4 | **禁止在 spawn_blocking 中调用 iced API** |

**正确示例**（对应菜场收银台 `tts_announce` 插件）：

```rust
// ✅ 同步 DLL 调用包在 spawn_blocking 中
let dll_path = self.dll_path.clone();
let text = text.to_string();
iced::Task::perform(
    async move {
        tokio::task::spawn_blocking(move || {
            // 同步调用 DLL：TTS 播报
            call_dll_speak(&dll_path, &text)
        }).await
    },
    |result| match result {
        Ok(Ok(())) => AppMessage::AnnounceDone,
        Ok(Err(e)) => AppMessage::AnnounceError(e.to_string()),
        Err(e) => AppMessage::AnnounceError(format!("JoinError: {}", e)),
    },
)
```

### 2.3 Arc<RwLock<T>> 插件状态

| 编号 | 规则 |
|------|------|
| P1 | **可变插件状态必须用 `Arc<RwLock<T>>` 包装** |
| P2 | **持写锁期间禁止 `await`** |
| P3 | **持读锁期间禁止调用用户回调** |
| P4 | **锁内只做内存操作（read/write 字段）** |

**违反示例**：

```rust
// ❌ 错误：持写锁期间 await
pub async fn reload(&self) -> Result<(), Error> {
    let mut plugin = self.asr_plugin.write().await;
    plugin.config = load_config_from_disk().await?;  // ← 持锁跨 await！
    Ok(())
}
```

**正确写法**：

```rust
// ✅ 锁外 await，锁内只赋值
pub async fn reload(&self) -> Result<(), Error> {
    let new_config = load_config_from_disk().await?;
    let mut plugin = self.asr_plugin.write().await;
    plugin.config = new_config;
    Ok(())
}
```

### 2.4 MQTT / WebSocket 后台任务

| 编号 | 规则 |
|------|------|
| M1 | **长连接后台任务必须能被取消（CancellationToken）** |
| M2 | **后台任务通过 channel 发送 AppMessage，不直接修改 State** |
| M3 | **连接断开必须产生 AppMessage 通知 UI** |
| M4 | **重连逻辑必须有退避（指数退避 + 抖动）** |

**正确示例**（对应菜场收银台 MQTT 客户端）：

```rust
// ✅ 后台任务通过 channel 发送事件
pub fn spawn_mqtt_loop(
    client: Arc<MqttClient>,
    tx: mpsc::UnboundedSender<AppMessage>,
    cancel: CancellationToken,
) {
    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = cancel.cancelled() => break,
                event = client.next() => {
                    if let Some(Ok(msg)) = event {
                        let _ = tx.send(AppMessage::MqttMessage(msg));
                    } else {
                        // 连接断开，触发重连
                        let _ = tx.send(AppMessage::MqttDisconnected);
                        tokio::time::sleep(Duration::from_secs(5)).await;
                    }
                }
            }
        }
    });
}
```

### 2.5 Atomic vs Mutex

| 编号 | 规则 |
|------|------|
| A1 | **单字段计数器用 `AtomicU32/AtomicU64`，不要用 `Mutex<u32>`** |
| A2 | **多字段协同更新必须用 Mutex（Atomic 无法保证原子性跨字段）** |
| A3 | **CAS（compare_exchange）循环优于加锁**，但需有重试上限 |

### 2.6 集合类无上限

| 编号 | 规则 |
|------|------|
| C1 | **Toast 队列 / 挂单板列表 / 抽奖记录必须有上限** |
| C2 | **超过上限必须 LRU 淘汰，而非 panic** |
| C3 | **API 响应缓存的 key 必须有上限（防 OOM）** |

---

## 三、iced 0.13 API 闭坑清单

这些不是并发问题，但属于"代码审查必须检查的 iced API 约束"。

### 7.1 Padding 类型

| 编号 | 规则 | 违反后果 |
|------|------|---------|
| IP1 | **`Padding::from([u16; 4])` 不存在** | 编译错误 |
| IP2 | **`Padding::from([u16; 2])` 是 `[vertical, horizontal]`** | 布局错乱 |
| IP3 | **4 元素 padding 必须用 `Padding::from([16u16, 16u16])` 等价转换** | 编译错误 |

```rust
// ❌ 错误：4 元素数组不支持
.padding(Padding::from([0u16, 16u16, 16u16, 16u16]))

// ✅ 正确：2 元素数组 [vertical, horizontal]
.padding(Padding::from([16u16, 16u16]))

// ✅ 正确：单值（四边相同）
.padding(Padding::from(16u16))
```

### 7.2 scrollable

| 编号 | 规则 | 违反后果 |
|------|------|---------|
| IS1 | **scrollable 没有 `.padding()` 方法** | 编译错误 |
| IS2 | **scrollable 内容不能 `Fill` 垂直滚动轴** | panic |
| IS3 | **scrollable 必须用 container 包裹施加 padding** | 布局错乱 |

```rust
// ❌ 错误：scrollable 无 padding 方法
let scroll = scrollable(grid).padding(Padding::from(16u16));

// ❌ 错误：内容 Fill 垂直轴导致 panic
let scroll = scrollable(column.width(Length::Fill).height(Length::Fill));

// ✅ 正确：用 container 包裹施加 padding，内容用 Shrink
let scroll = scrollable(column.width(Length::Fill).height(Length::Shrink));
let wrap = container(scroll).padding(Padding::from([0u16, 16u16]));
```

### 7.3 button::Style 闭包

| 编号 | 规则 |
|------|------|
| IB1 | **`button::Style` 闭包必须 2 参数 `\|_t, _status\|`** |
| IB2 | **`_status` 可用于 hover/pressed 状态切换** |

```rust
// ❌ 错误：1 参数闭包
.style(move |_t| button::Style { ... })

// ✅ 正确：2 参数闭包
.style(move |_t, _status| button::Style { ... })

// ✅ 正确：用 status 切换 hover 样式
.style(move |_t, status| button::Style {
    background: Some(Background::Color(match status {
        button::Status::Hovered => Color::from_rgb(0.30, 0.66, 0.30),
        _ => Color::from_rgb(0.36, 0.72, 0.36),
    })),
    ..button::Style::default()
})
```

### 7.4 text_input

| 编号 | 规则 |
|------|------|
| IT1 | **密码框用 `.secure(true)`，不是 `.password()`** |
| IT2 | **`.on_submit(msg)` 用于 Enter 触发** |
| IT3 | **`.on_input(\|s\| Msg::InputChanged(s))` 用于实时输入** |

```rust
// ❌ 错误：.password() 不存在
text_input("密码", &pwd).password()

// ✅ 正确：.secure(true)
text_input("密码", &pwd).secure(true)
```

### 7.5 border::radius

| 编号 | 规则 |
|------|------|
| IR1 | **`border::radius(4.0)` 接收 f32** |
| IR2 | **不要用 `4.0.into()`**（类型推断歧义）|

---

## 四、并发安全审查流程

### 4.1 静态扫描

```bash
# 找所有 Task::perform 调用点
grep -rn "Task::perform" --include="*.rs" src/

# 找所有 spawn_blocking
grep -rn "spawn_blocking" --include="*.rs" src/

# 找所有 RwLock/Mutex
grep -rn "RwLock\|Mutex" --include="*.rs" src/

# 找所有 lock().await
grep -rn "lock()\.await\|read()\.await\|write()\.await" --include="*.rs" src/
```

### 4.2 人工审查

对每个 `Task::perform` 点回答：
1. 闭包是否借用 `&mut state`？→ 必须重构为 clone
2. 闭包返回值是否通过 `AppMessage` 回写？→ 必须确认回写路径
3. 闭包内是否调用 iced API？→ 禁止

对每个 `lock().await` 点回答：
1. 持锁期间是否 `await`？→ 必须重构
2. 持锁期间是否调用 `fn` 参数？→ 必须重构
3. 持锁期间是否调用 `trait` 方法？→ 检查实现是否会反持锁

### 4.3 压力测试

```rust
#[tokio::test]
async fn stress_login_button_rapid_clicks() {
    let mut state = AppState::new();
    // 模拟快速连击登录按钮
    for _ in 0..10 {
        let _ = state.update(AppMessage::LoginSubmit);
    }
    // 只有第一次应该触发 Task::perform，后续应被 is_logging_in 标志拦截
    assert!(state.login_view.is_logging_in);
}
```

---

## 五、并发测试套件建议

| 类型 | 数量 | 验证目标 |
|------|------|---------|
| 快速连击防抖 | ≥3 | 同一按钮快速点击不触发多次 Task |
| 后台任务取消 | ≥2 | 应用退出时 CancellationToken 正确传播 |
| MQTT 重连退避 | ≥2 | 断线后按指数退避重连 |
| 插件状态并发读写 | ≥3 | Arc<RwLock> 不自我死锁 |
| 长稳态（5s+） | ≥1 | 后台任务无内存泄漏 |

---

## 六、与其他规范的关系

- 本规范定义**并发约束清单**；
- [01-integration-gates.md](01-integration-gates.md) 在第 4 关 `cargo test` 中执行并发测试；
- [02-contract-audit.md](02-contract-audit.md) 将并发不变量写入契约（如 `is_logging_in` 防抖）；
- [04-test-pyramid.md](04-test-pyramid.md) 提供并发测试编写模板。
