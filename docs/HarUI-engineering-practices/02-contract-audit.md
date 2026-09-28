# HarUI 项目行为变更契约审计规范

> 适用范围：所有基于 iced 0.13 + HarUI 的 Rust GUI 项目
> 版本：v1.0  日期：2026-07-20
> 来源：菜场收银台 v0.1.0 工程实践提炼

---

## 一、为什么需要契约审计

`cargo test` 只能验证"代码做了什么"，无法守护"调用方期望它做什么"。在 GUI 项目中，**行为契约**尤为重要：

1. **State 转移契约**：`AppState` 在 `update()` 中经过一系列 `AppMessage` 处理后，必须达到预期的路由/数据状态；
2. **Message 处理契约**：`AppMessage::LoginSubmit` 必须触发 `UserApi::login` 并最终产生 `LoginResult` 消息；
3. **Component 渲染契约**：`LoginView::view` 在任意 State 下必须返回有效 `Element`，不得 panic；
4. **API 加密契约**：`UserApi::login` 必须用 AES 加密请求体、RSA 加密 aesKey、解密 `res.data.data`。

破坏这些契约的典型场景：
- 重构 `LoginSubmit` 时忘记触发 `Task::perform`，导致登录按钮"点了没反应"
- 修改 `AppMessage` 变体名称，导致序列化/反序列化失败
- 修改 `CartStore::push_product` 时忘记更新 `total_fee`，导致购物车金额错误

---

## 二、契约文档结构

`docs/api-contracts.md` 按章节组织，每章描述一个公共 API 的行为契约：

```markdown
## 第 N 章：<API 名称>

**签名**：
```rust
pub fn view<'a>(state: &'a AppState, theme: &'a Theme) -> Element<'a, AppMessage>
```

**前置条件**：
- `state.login_view.username` 非空（默认 "18851052481"）
- `state.login_view.password` 非空（默认 "123456"）
- `state.app_store.aes_key.is_some()`（AppStore::new 已初始化）

**后置条件**：
- 返回的 Element 包含：系统名称 + Logo + 用户名输入框 + 密码输入框 + 登录按钮
- 用户名输入框 `.on_submit(LoginSubmit)`
- 密码输入框 `.secure(true)` 且 `.on_submit(LoginSubmit)`
- 登录按钮 `.on_press(LoginSubmit)`

**不变量（Invariant）**：
- 任意 State 下 view 不 panic（包括 error_msg 非空 / is_logging_in=true）
- 表单容器宽度固定 420px（1366×768 与 1920×1080 下保持一致）

**示例**：
```rust
let state = AppState::new();
let theme = Theme::element_light();
let el = view(&state, &theme);  // 不 panic
drop(el);
```

**破坏性变更判定**：
- 修改 view 返回类型 → MAJOR
- 修改前置条件（更严格）→ MAJOR
- 修改后置条件（移除 on_submit / secure）→ MAJOR
- 修改不变量（如改为可变宽度）→ MAJOR
- 仅扩展（新增错误提示样式）→ MINOR
```

---

## 三、契约测试映射

每章契约对应 `tests/contract/<chapter>.rs` 一个测试模块，至少包含：

| 测试类型 | 数量 | 验证目标 |
|---------|------|---------|
| 前置条件违反测试 | ≥1 | 违反前置条件时返回预期错误或 panic |
| 后置条件验证测试 | ≥1 | 成功路径下后置条件成立 |
| 不变量保持测试 | ≥1 | 多次操作后不变量仍成立 |
| 边界/极端测试 | ≥1 | error_msg 非空 / is_logging_in=true 等极端 State |

### 示例：LoginView 契约测试

```rust
#[test]
fn contract_login_view_renders_without_panic_default() {
    let state = AppState::new();
    let theme = Theme::element_light();
    let _el = crate::pages::login::view(&state, &theme);
}

#[test]
fn contract_login_view_renders_with_error_msg() {
    let mut state = AppState::new();
    state.login_view.set_error("密码错误");
    let theme = Theme::element_light();
    let _el = crate::pages::login::view(&state, &theme);
}

#[test]
fn contract_login_view_renders_with_logging_in() {
    let mut state = AppState::new();
    state.login_view.start_logging();
    let theme = Theme::element_light();
    let _el = crate::pages::login::view(&state, &theme);
}

#[test]
fn contract_login_view_default_credentials() {
    let state = AppState::new();
    assert_eq!(state.login_view.username, "18851052481");
    assert_eq!(state.login_view.password, "123456");
}
```

---

## 四、State 转移契约

GUI 项目的核心契约是 **State 转移契约**：给定初始 State + Message 序列，必须达到确定性的终态。

### 4.1 转移表格式

```markdown
## 第 N 章：LoginSubmit 消息处理契约

| 初始 State | Message | 前置条件 | 终态 State | 副作用 |
|-----------|---------|---------|-----------|--------|
| Route::Login, is_logging_in=false | LoginSubmit | username/password 非空 | is_logging_in=true | Task::perform(UserApi::login) |
| Route::Login, is_logging_in=true | LoginSubmit | （忽略重复提交）| 不变 | 无 |
| Route::Login, error_msg="..." | LoginSubmit | username/password 非空 | is_logging_in=true, error_msg="" | start_logging 清空错误 |
| Route::Login | LoginSubmit | username="" | error_msg="请输入登录账号" | 无 Task::perform |
| Route::Login | LoginResult(Ok(data)) | token 字段存在 | Route::Main, user_store 更新 | ToastShow("登录成功") |
| Route::Login | LoginResult(Err(msg)) | — | error_msg=msg, is_logging_in=false | ToastShow(msg, Error) |
```

### 4.2 转移测试骨架

```rust
#[test]
fn contract_login_submit_triggers_api_call() {
    let mut state = AppState::new();
    assert!(!state.login_view.is_logging_in);
    
    let task = state.update(AppMessage::LoginSubmit);
    assert!(state.login_view.is_logging_in, "is_logging_in must be true after LoginSubmit");
    // task 应该是 Task::perform(UserApi::login, ...)
    // （具体验证方式取决于 iced::Task 是否可 introspect）
}

#[test]
fn contract_login_submit_validates_empty_username() {
    let mut state = AppState::new();
    state.login_view.username.clear();
    let _ = state.update(AppMessage::LoginSubmit);
    assert_eq!(state.login_view.error_msg, "请输入登录账号");
    assert!(!state.login_view.is_logging_in, "不应启动 logging_in");
}

#[test]
fn contract_login_result_success_navigates_to_main() {
    let mut state = AppState::new();
    state.current_route = Route::Login;
    let data = serde_json::json!({
        "token": "test_token_123",
        "data": {
            "supplier_id": 100,
            "store_id": 200,
            "cashier_name": "张三",
            "skin": "{}",
            "version": "1.0.0"
        }
    });
    let _ = state.update(AppMessage::LoginResult(Ok(data)));
    assert_eq!(state.current_route, Route::Main);
    assert_eq!(state.user_store.token, "test_token_123");
    assert_eq!(state.user_store.store.store_id, 200);
    assert_eq!(state.user_store.cashier.cashier_name, "张三");
}
```

---

## 五、变更分级与版本策略

| 变更类型 | 示例 | 版本 bump | 处理流程 |
|---------|------|----------|---------|
| **破坏性（MAJOR）** | 删除/重命名 AppMessage 变体、修改 view 签名、收紧前置条件、改变后置条件 | major（0.x 时为 minor） | 1. 更新 `api-contracts.md`<br>2. 更新契约测试<br>3. PR 描述附迁移指南<br>4. 更新 baseline |
| **兼容新增（MINOR）** | 新增 AppMessage 变体、新增 Component、放宽前置条件 | minor | 1. 可选新增契约章节<br>2. 更新 baseline |
| **修复（PATCH）** | 内部实现修复，签名/行为不变 | patch | 无需契约改动 |

---

## 六、契约审计与代码审查的关系

| 维度 | 代码审查 | 契约审计 |
|------|---------|---------|
| 关注点 | 实现质量、可读性、iced API 闭坑 | State/Message 行为一致性 |
| 执行时机 | PR 评审 | CI 自动 + PR 评审 |
| 失败结果 | 人工阻断 | CI 阻断 |
| 覆盖范围 | 主观判断 | 客观可执行 |

二者**互补**：代码审查防"写得差"，契约审计防"行为变了没人知道"。

---

## 七、落地清单

- [ ] 建立 `docs/api-contracts.md`，至少覆盖核心 API（AppState::update / LoginView::view / CartStore::push_product / UserApi::login）
- [ ] 建立 `tests/contract/` 目录，每章对应一个测试模块
- [ ] 建立 `scripts/audit-api-changes.ps1` + `scripts/pub-api-baseline.txt`
- [ ] 在 `gate.ps1` 中加入 `api-audit` 和 `contract` 两道关卡
- [ ] 在 `CONTRIBUTING.md` 中说明：破坏性变更必须更新契约文档和测试

---

## 八、与其他规范的关系

- 本规范定义**契约如何编写与审计**；
- [01-integration-gates.md](01-integration-gates.md) 定义**契约测试如何作为门禁执行**；
- [03-concurrency-safety.md](03-concurrency-safety.md) 提供**契约背后的并发约束清单**。
