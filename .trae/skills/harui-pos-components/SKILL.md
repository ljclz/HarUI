---
name: harui-pos-components
description: HarUI POS 专属组件检查 — 确保 Keypad/Payment/HangOrder/CustomerDisplay/StatusBar 的计价与状态逻辑正确、金额精度处理安全。修改 POS 组件时触发。
tools: [cargo, cargo-test]
agentMode: auto
---

# HarUI POS 专属组件检查

## 触发条件

- 修改 `crates/components/src/keypad.rs` / `payment.rs` / `hang_order.rs` / `customer_display.rs` / `status_bar.rs`
- 修改 POS 组件相关 store（菜场收银台计价逻辑）

## POS 组件清单与检查要点

### 1. Keypad（数字键盘）

```rust
pub enum KeypadMode { Price, Number, Quantity }
```

**检查点**：
- [ ] Price 模式最多 2 位小数
- [ ] Quantity 模式最多 3 位小数
- [ ] 金额无浮点误差累积（内部用分/厘整数或 Decimal）
- [ ] 状态机：Editing → Confirmed / Invalid 穷举
- [ ] 60×60 触控按钮点击区域完整

### 2. Payment（支付面板）

**检查点**：
- [ ] 现金支付：`received >= total` 校验，找零 = `(received - total).max(0)`
- [ ] 非现金支付：`received = total` 自动设置
- [ ] 5 种支付方式（Cash/WeChat/Alipay/UnionPay/MemberBalance）穷举
- [ ] 金额比较用整数分（`f64` 直接比较有精度风险）
- [ ] 支付成功/失败状态转换正确

### 3. HangOrder（挂单管理）

**检查点**：
- [ ] Hang：按时间戳降序插入
- [ ] Take：移除并返回挂单
- [ ] Delete / ClearAll：正确清理
- [ ] 挂单金额与取单后金额一致（无丢失折扣/优惠状态）

### 4. CustomerDisplay（客显屏）

**检查点**：
- [ ] 6 状态（Idle/Welcome/ShowingAmount/ShowingQrCode/ShowingMessage/Success）穷举
- [ ] 只读渲染（`Element<'a, ()>` 无消息）
- [ ] 金额显示格式化（¥ + 2 位小数）

### 5. StatusBar（硬件状态栏）

**检查点**：
- [ ] 4 指示灯（scale/scanner/printer/network）与 HarRT 设备状态同步
- [ ] `StatusLevel`（Info/Success/Warning/Error）颜色映射正确
- [ ] 设备离线时显示 Warning/Error 并自动轮询恢复

## 金额精度规范（POS 核心）

```rust
// ❌ 错误：f64 直接比较（0.1 + 0.2 != 0.3）
if received >= total { /* 可能误判 */ }

// ✅ 正确：整数分比较
let received_cents = (received * 100.0).round() as i64;
let total_cents = (total * 100.0).round() as i64;
if received_cents >= total_cents { /* 精确比较 */ }
```

**检查点**：
- [ ] 所有金额比较用整数分/厘
- [ ] 折扣/优惠/活动叠加计算有边界测试（0 折扣、100% 折扣、负数）
- [ ] 与菜场收银台 `CartStore` 计价方法结果一致（`cart_receivable()` 等）

## 通过标准

- 5 个 POS 组件各有 `*_test.rs` + `*_view.rs`
- 金额精度测试覆盖：小数进位、大金额（> 1 亿）、0 金额
- 状态机穷举测试通过
- `cargo test -p har-ui-components` 全部通过

## 常见 Bug 定位提示

| 现象 | 排查点 |
|------|--------|
| 找零金额差 0.01 元 | f64 精度问题，改为整数分比较 |
| 挂单后取单金额不对 | 挂单快照是否保存了完整业务字段（折扣/优惠券/会员） |
| 客显屏不更新 | CustomerDisplay 状态是否被应用 update() 驱动 |
| 状态栏显示在线但设备已离线 | 轮询间隔是否过长？HarRT 事件订阅是否断开 |

## 参考

- `HarUI/docs/技术实现方案.md` — POS 组件设计
- `菜场收银台/src/store/cart_state.rs` — 计价引擎（对照）
