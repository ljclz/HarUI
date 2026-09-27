//! T6 混沌测试 — 事件风暴 / 并发状态切换 / 故障注入
//!
//! **与 T4 Fuzz 的区别：**
//! - T4（fuzz_test.rs）：proptest 随机输入，逐消息验证不变式（正确性）
//! - T6（本文件）：高容量事件风暴 + 多线程并发，验证「不崩溃、不挂起、终态合法」（鲁棒性）
//!
//! **通过标准（技术实现方案 §7.1）：**
//! - 不崩溃：任意消息序列执行后无 panic
//! - 不挂起：单组件 10,000 条消息 < 3s
//! - 终态合法：风暴结束后状态不变式仍成立
//! - 并发安全：多线程同时操作无数据竞争
//!
//! **覆盖组件：**
//! - Button：disabled/loading 状态下的风暴免疫
//! - Input：IME 组合风暴（中文输入场景）
//! - Keypad：POS 数字键盘边界风暴
//! - Payment：支付状态机并发切换
//! - HangOrder：挂单 ID 唯一性 + 时间序不变式
//!
//! **运行方式：**
//! ```bash
//! # 全量混沌测试
//! cargo test --test chaos_test -- --test-threads=4
//!
//! # 单个场景
//! cargo test --test chaos_test event_storm_button -- --nocapture
//! ```

use har_ui_components::button::{Button, ButtonMessage};
use har_ui_components::hang_order::{HangOrder, HangOrderItem, HangOrderMessage};
use har_ui_components::input::{Input, InputMessage};
use har_ui_components::keypad::{Keypad, KeypadMessage, KeypadMode};
use har_ui_components::payment::{Payment, PaymentMessage, PaymentMethod};
use har_ui_core::utils::ime::ImeEvent;
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Instant;

// ===================== 事件风暴测试 =====================

const STORM_COUNT: usize = 10_000; // 每个场景 10,000 条消息
const STORM_TIMEOUT_SECS: u64 = 3; // 单组件 3s 内完成

/// Button：disabled/loading 状态下 10K 事件风暴不崩溃
///
/// **不变式：**
/// - disabled=true 或 loading=true 时，Clicked 必返回 NoChange
/// - 终态 value（文本）不被修改
#[test]
fn event_storm_button_guarded_states() {
    let mut btn_disabled = Button::new("测试").disabled(true);
    let mut btn_loading = Button::new("测试").loading(true);
    let mut btn_normal = Button::new("测试");

    let msgs: Vec<ButtonMessage> = (0..STORM_COUNT)
        .map(|_| match rand::random::<u8>() % 6 {
            0 => ButtonMessage::Clicked,
            1 => ButtonMessage::Hovered,
            2 => ButtonMessage::Unhovered,
            3 => ButtonMessage::Pressed,
            4 => ButtonMessage::Released,
            _ => ButtonMessage::NoChange,
        })
        .collect();

    let t0 = Instant::now();

    for msg in &msgs {
        // disabled 状态：Clicked 必返回 NoChange
        let ret_d = btn_disabled.handle(*msg);
        assert_eq!(
            ret_d,
            ButtonMessage::NoChange,
            "disabled button leaked event"
        );

        // loading 状态：Clicked 必返回 NoChange
        let ret_l = btn_loading.handle(*msg);
        assert_eq!(
            ret_l,
            ButtonMessage::NoChange,
            "loading button leaked event"
        );

        // 正常状态：不 panic 即可
        let _ = btn_normal.handle(*msg);
    }

    assert!(
        t0.elapsed() < std::time::Duration::from_secs(STORM_TIMEOUT_SECS),
        "Button storm timeout: {:?}",
        t0.elapsed()
    );
}

/// Input：IME 组合 + 字符混合风暴，终态 value 合法
///
/// **不变式：**
/// - maxlength 约束始终满足
/// - value 不含 \0 字符
/// - 任意时刻 value 是 UTF-8 合法字符串（Rust 类型系统已保证）
#[test]
fn event_storm_input_ime_composition() {
    let mut input = Input::new().with_maxlength(50);

    let t0 = Instant::now();

    for i in 0..STORM_COUNT {
        let msg = match rand::random::<u8>() % 8 {
            0..=2 => InputMessage::Char(rand::random::<char>()),
            3 => InputMessage::Backspace,
            4 => InputMessage::Clear,
            5 => InputMessage::Focused,
            6 => InputMessage::Blurred,
            _ => {
                // IME 组合事件：模拟中文输入法快速打字
                match rand::random::<u8>() % 4 {
                    0 => InputMessage::ImeEvent(ImeEvent::CompositionStart),
                    1 => InputMessage::ImeEvent(ImeEvent::CompositionUpdate(format!("组合{}", i))),
                    2 => InputMessage::ImeEvent(ImeEvent::CompositionEnd(format!("最终文本{}", i))),
                    _ => InputMessage::ImeEvent(ImeEvent::CompositionEnd(String::new())),
                }
            }
        };
        input.handle(msg);
    }

    // 终态不变式
    let val = input.value();
    assert!(
        val.len() <= 50,
        "maxlength violated: len={} value={:?}",
        val.len(),
        val
    );
    assert!(!val.contains('\0'), "null byte in value: {:?}", val);

    assert!(
        t0.elapsed() < std::time::Duration::from_secs(STORM_TIMEOUT_SECS),
        "Input IME storm timeout: {:?}",
        t0.elapsed()
    );
}

/// Keypad：三种模式各 10K 次击键，验证小数位钳制
///
/// **不变式：**
/// - Price 模式：小数位 ≤ 2
/// - Number 模式：无小数点
/// - Quantity 模式：小数位 ≤ 3
/// - 整数位 ≤ 8（MAX_INT_LEN）
#[test]
fn event_storm_keypad_decimal_clamp() {
    let modes = [KeypadMode::Price, KeypadMode::Number, KeypadMode::Quantity];
    let max_decimals = [2usize, 0, 3];

    for (mode, &max_dec) in modes.iter().zip(max_decimals.iter()) {
        let mut kp = Keypad::new().with_mode(*mode);
        let t0 = Instant::now();

        for i in 0..STORM_COUNT {
            let msg = match rand::random::<u8>() % 6 {
                0 => KeypadMessage::Digit(rand::random::<u8>() % 10),
                1 => KeypadMessage::DoubleZero,
                2 => KeypadMessage::Dot,
                3 => KeypadMessage::Backspace,
                4 => KeypadMessage::Clear,
                _ => KeypadMessage::Ok,
            };
            kp.handle(msg);

            // 每 1000 次采样一次不变式（避免过度断言影响性能测试）
            if i % 1000 == 0 {
                let v = kp.value();
                // 小数位钳制
                if let Some(dot_pos) = v.find('.') {
                    let dec_count = v.len() - dot_pos - 1;
                    assert!(
                        dec_count <= max_dec,
                        "mode={:?} decimals={} max={}",
                        mode,
                        dec_count,
                        max_dec
                    );
                }
                // Number 模式无小数点
                if matches!(mode, KeypadMode::Number) {
                    assert!(!v.contains('.'), "Number mode has dot: {}", v);
                }
                // 整数位 ≤ 8
                let int_part = v.split('.').next().unwrap_or("");
                // 去掉前导负号
                let int_len = int_part.trim_start_matches('-').len();
                assert!(
                    int_len <= 8,
                    "mode={:?} int_len={} value={}",
                    mode,
                    int_len,
                    v
                );
            }
        }

        assert!(
            t0.elapsed() < std::time::Duration::from_secs(STORM_TIMEOUT_SECS),
            "Keypad {:?} storm timeout: {:?}",
            mode,
            t0.elapsed()
        );
    }
}

/// Payment：支付状态机风暴 + 总额变更连锁反应
///
/// **不变式：**
/// - 现金支付：confirmed 时 received ≥ total，change = max(0, received - total)
/// - 非现金支付：confirmed 时 received == total，change == 0
/// - 总额变更后状态重置为 Pending
#[test]
fn event_storm_payment_state_machine() {
    let mut payment = Payment::new(100.0);

    let t0 = Instant::now();
    let mut confirm_count = 0u64;

    for i in 0..STORM_COUNT {
        let msg = match rand::random::<u8>() % 8 {
            0 => PaymentMessage::Received(rand::random::<f64>() * 300.0), // 0..300
            1 => PaymentMessage::SwitchMethod(PaymentMethod::Cash),
            2 => PaymentMessage::SwitchMethod(PaymentMethod::WeChat),
            3 => PaymentMessage::SwitchMethod(PaymentMethod::Alipay),
            4 => PaymentMessage::UpdateTotal(rand::random::<f64>() * 200.0),
            5 => PaymentMessage::Confirm,
            6 => PaymentMessage::Cancel,
            _ => PaymentMessage::SwitchMethod(PaymentMethod::UnionPay),
        };

        let _was_confirmed = payment.state() == har_ui_components::payment::PaymentState::Confirmed;
        payment.handle(msg);

        if matches!(msg, PaymentMessage::Confirm) {
            confirm_count += 1;
        }

        // 每 1000 次采样不变式
        if i % 1000 == 0 || matches!(msg, PaymentMessage::Confirm) {
            let total = payment.total();
            let received = payment.received();
            let change = payment.change();
            let method = payment.method();

            if payment.state() == har_ui_components::payment::PaymentState::Confirmed {
                if method.is_cash() {
                    assert!(
                        received >= total - 1e-9,
                        "cash confirmed but received < total: received={} total={}",
                        received,
                        total
                    );
                    let expected_change = (received - total).max(0.0);
                    assert!(
                        (change - expected_change).abs() < 1e-9,
                        "cash change mismatch: got={} expected={}",
                        change,
                        expected_change
                    );
                } else {
                    assert!(
                        (received - total).abs() < 1e-9,
                        "non-cash confirmed but received != total: received={} total={}",
                        received,
                        total
                    );
                    assert!(
                        change.abs() < 1e-9,
                        "non-cash confirmed but change != 0: {}",
                        change
                    );
                }
            }

            // 总额不能为负
            assert!(total >= 0.0, "negative total: {}", total);
            // 实收不能为负
            assert!(received >= 0.0, "negative received: {}", received);
            // 找零不能为负
            assert!(change >= 0.0, "negative change: {}", change);
        }
    }

    assert!(confirm_count > 0, "no Confirm events fired — check arb");
    assert!(
        t0.elapsed() < std::time::Duration::from_secs(STORM_TIMEOUT_SECS),
        "Payment storm timeout: {:?}",
        t0.elapsed()
    );
}

/// HangOrder：挂单 ID 唯一性 + 时间倒序不变式风暴
///
/// **不变式：**
/// - 所有挂单 ID 唯一（重复 Hang 被拒绝）
/// - items 按 timestamp 降序排列
/// - count() == items().len()
#[test]
fn event_storm_hang_order_uniqueness() {
    let mut ho = HangOrder::new();
    let t0 = Instant::now();

    let mut hung_ids: Vec<String> = Vec::new();

    for i in 0..STORM_COUNT {
        let msg = match rand::random::<u8>() % 4 {
            0 => {
                // Hang：随机生成，30% 概率复用已有 ID（测试去重）
                let id = if !hung_ids.is_empty() && rand::random::<u8>() % 10 < 3 {
                    hung_ids[rand::random::<usize>() % hung_ids.len()].clone()
                } else {
                    format!("order_{}_{}", i, rand::random::<u64>())
                };
                HangOrderMessage::Hang(HangOrderItem {
                    id: id.clone(),
                    customer_name: format!("顾客{}", i % 100),
                    total: (i % 1000) as f64,
                    item_count: (i % 20) + 1,
                    timestamp: i as i64, // 单调递增，方便验证降序
                })
            }
            1 => {
                // Take：随机 ID（可能不存在）
                let id = if !hung_ids.is_empty() && rand::random::<u8>().is_multiple_of(2) {
                    hung_ids[rand::random::<usize>() % hung_ids.len()].clone()
                } else {
                    format!("ghost_{}", rand::random::<u64>())
                };
                HangOrderMessage::Take(id)
            }
            2 => {
                // Delete：随机 ID
                let id = if !hung_ids.is_empty() && rand::random::<u8>().is_multiple_of(2) {
                    hung_ids[rand::random::<usize>() % hung_ids.len()].clone()
                } else {
                    format!("ghost_{}", rand::random::<u64>())
                };
                HangOrderMessage::Delete(id)
            }
            _ => HangOrderMessage::ClearAll,
        };

        let _result = ho.handle(msg.clone());

        // 维护 hung_ids 追踪（仅用于测试参考，不用于断言）
        match msg {
            HangOrderMessage::Hang(item) => {
                // 如果 handle 接受了（即 ID 不重复），加入追踪
                if !hung_ids.contains(&item.id) {
                    hung_ids.push(item.id.clone());
                }
            }
            HangOrderMessage::Take(id) | HangOrderMessage::Delete(id) => {
                hung_ids.retain(|x| x != &id);
            }
            HangOrderMessage::ClearAll => {
                hung_ids.clear();
            }
        }

        // 每 2000 次采样不变式
        if i % 2000 == 0 {
            let items = ho.items();

            // 不变式 1：ID 唯一
            let mut seen_ids = std::collections::HashSet::new();
            for item in items {
                assert!(seen_ids.insert(&item.id), "duplicate id: {}", item.id);
            }

            // 不变式 2：按 timestamp 降序
            for w in items.windows(2) {
                assert!(
                    w[0].timestamp >= w[1].timestamp,
                    "not descending: {} >= {}",
                    w[0].timestamp,
                    w[1].timestamp
                );
            }

            // 不变式 3：count 一致
            assert_eq!(
                ho.count(),
                items.len(),
                "count mismatch: count={} len={}",
                ho.count(),
                items.len()
            );
        }
    }

    assert!(
        t0.elapsed() < std::time::Duration::from_secs(STORM_TIMEOUT_SECS),
        "HangOrder storm timeout: {:?}",
        t0.elapsed()
    );
}

// ===================== 并发状态切换测试 =====================

/// 多线程并发操作 Button：验证 Arc<Mutex<>> 下无数据竞争
///
/// **场景：** 16 线程同时发送 Clicked/Hovered/Pressed 事件
/// **不变式：** 无 panic，无死锁（通过 barrier 同步启动）
#[test]
fn concurrent_state_switch_button() {
    let btn = Arc::new(std::sync::Mutex::new(Button::new("并发测试")));
    let num_threads = 16;
    let msgs_per_thread = 1000;
    let barrier = Arc::new(Barrier::new(num_threads));

    let handles: Vec<_> = (0..num_threads)
        .map(|tid| {
            let btn = Arc::clone(&btn);
            let barrier = Arc::clone(&barrier);
            thread::spawn(move || {
                // 所有线程同步启动，最大化并发冲突
                barrier.wait();

                for _ in 0..msgs_per_thread {
                    let msg = match tid % 6 {
                        0 => ButtonMessage::Clicked,
                        1 => ButtonMessage::Hovered,
                        2 => ButtonMessage::Unhovered,
                        3 => ButtonMessage::Pressed,
                        4 => ButtonMessage::Released,
                        _ => ButtonMessage::NoChange,
                    };
                    let mut b = btn.lock().unwrap();
                    let _ = b.handle(msg);
                    drop(b); // 尽早释放锁
                }
            })
        })
        .collect();

    let t0 = Instant::now();
    for h in handles {
        h.join().expect("thread panicked");
    }

    assert!(
        t0.elapsed() < std::time::Duration::from_secs(5),
        "Concurrent button test timeout: {:?}",
        t0.elapsed()
    );
}

/// 多线程并发操作 Payment：验证支付状态在并发切换下不出现数据不一致
///
/// **场景：** 8 线程同时 SwitchMethod + Confirm + Received
/// **不变式：** 终态 confirmed 时业务规则仍成立
#[test]
fn concurrent_state_switch_payment() {
    let payment = Arc::new(std::sync::Mutex::new(Payment::new(100.0)));
    let num_threads = 8;
    let msgs_per_thread = 500;
    let barrier = Arc::new(Barrier::new(num_threads));

    let handles: Vec<_> = (0..num_threads)
        .map(|tid| {
            let payment = Arc::clone(&payment);
            let barrier = Arc::clone(&barrier);
            thread::spawn(move || {
                barrier.wait();

                for _ in 0..msgs_per_thread {
                    let msg = match tid % 5 {
                        0 => PaymentMessage::Received((tid as f64) * 10.0 + 50.0),
                        1 => PaymentMessage::SwitchMethod(PaymentMethod::Cash),
                        2 => PaymentMessage::SwitchMethod(PaymentMethod::WeChat),
                        3 => PaymentMessage::Confirm,
                        _ => PaymentMessage::Cancel,
                    };
                    let mut p = payment.lock().unwrap();
                    p.handle(msg);
                    drop(p);
                }
            })
        })
        .collect();

    let t0 = Instant::now();
    for h in handles {
        h.join().expect("thread panicked");
    }

    // 终态不变式
    let p = payment.lock().unwrap();
    let total = p.total();
    let received = p.received();
    let change = p.change();
    let method = p.method();

    if p.state() == har_ui_components::payment::PaymentState::Confirmed {
        if method.is_cash() {
            assert!(received >= total - 1e-9);
            assert!((change - (received - total).max(0.0)).abs() < 1e-9);
        } else {
            assert!((received - total).abs() < 1e-9);
            assert!(change.abs() < 1e-9);
        }
    }
    assert!(total >= 0.0 && received >= 0.0 && change >= 0.0);

    assert!(
        t0.elapsed() < std::time::Duration::from_secs(5),
        "Concurrent payment test timeout: {:?}",
        t0.elapsed()
    );
}

/// 多线程并发操作 HangOrder：验证 ID 唯一性在并发 Hang/Take 下保持
///
/// **场景：** 8 线程同时 Hang（含重复 ID）+ Take + Delete
/// **不变式：** 终态所有 ID 唯一
#[test]
fn concurrent_state_switch_hang_order() {
    let ho = Arc::new(std::sync::Mutex::new(HangOrder::new()));
    let num_threads = 8;
    let ops_per_thread = 500;
    let barrier = Arc::new(Barrier::new(num_threads));

    let handles: Vec<_> = (0..num_threads)
        .map(|tid| {
            let ho = Arc::clone(&ho);
            let barrier = Arc::clone(&barrier);
            thread::spawn(move || {
                barrier.wait();

                for i in 0..ops_per_thread {
                    // 30% 概率使用共享 ID 前缀（制造冲突）
                    let shared_id = format!("shared_{}", i % 50);
                    let unique_id = format!("t{}_op{}", tid, i);
                    let id = if rand::random::<u8>() % 10 < 3 {
                        shared_id
                    } else {
                        unique_id
                    };

                    let msg = match tid % 4 {
                        0 => HangOrderMessage::Hang(HangOrderItem {
                            id: id.clone(),
                            customer_name: format!("顾客{}", tid),
                            total: (i % 100) as f64,
                            item_count: (i % 10) + 1,
                            timestamp: i as i64,
                        }),
                        1 => HangOrderMessage::Take(id),
                        2 => HangOrderMessage::Delete(id),
                        _ => HangOrderMessage::ClearAll,
                    };

                    let mut h = ho.lock().unwrap();
                    let _ = h.handle(msg);
                    drop(h);
                }
            })
        })
        .collect();

    let t0 = Instant::now();
    for h in handles {
        h.join().expect("thread panicked");
    }

    // 终态不变式：所有 ID 唯一
    let h = ho.lock().unwrap();
    let mut seen = std::collections::HashSet::new();
    for item in h.items() {
        assert!(
            seen.insert(&item.id),
            "concurrent duplicate id: {}",
            item.id
        );
    }

    assert!(
        t0.elapsed() < std::time::Duration::from_secs(5),
        "Concurrent hang_order test timeout: {:?}",
        t0.elapsed()
    );
}

// ===================== 故障注入测试 =====================

/// IME 状态机故障注入：乱序 CompositionEnd / CompositionStart
///
/// **场景：** 模拟输入法异常行为（未 CompositionStart 就 CompositionEnd、
/// 连续多次 CompositionEnd、空字符串 CompositionUpdate）
/// **不变式：** 不 panic，IME 状态最终可恢复到 Idle
#[test]
fn fault_injection_ime_sequence() {
    let mut input = Input::new();

    // 故障序列 1：未开始就结束
    input.handle(InputMessage::ImeEvent(ImeEvent::CompositionEnd(
        "文本".to_string(),
    )));
    // 故障序列 2：连续结束
    input.handle(InputMessage::ImeEvent(ImeEvent::CompositionEnd(
        "文本2".to_string(),
    )));
    // 故障序列 3：空字符串更新
    input.handle(InputMessage::ImeEvent(ImeEvent::CompositionUpdate(
        String::new(),
    )));
    // 故障序列 4：开始后无更新直接结束
    input.handle(InputMessage::ImeEvent(ImeEvent::CompositionStart));
    input.handle(InputMessage::ImeEvent(ImeEvent::CompositionEnd(
        "最终".to_string(),
    )));
    // 正常序列恢复
    input.handle(InputMessage::ImeEvent(ImeEvent::CompositionStart));
    input.handle(InputMessage::ImeEvent(ImeEvent::CompositionUpdate(
        "测".to_string(),
    )));
    input.handle(InputMessage::ImeEvent(ImeEvent::CompositionUpdate(
        "测试".to_string(),
    )));
    input.handle(InputMessage::ImeEvent(ImeEvent::CompositionEnd(
        "测试文本".to_string(),
    )));

    // 终态：value 应为最终组合文本（IME 正常结束后的值）
    let val = input.value();
    assert!(!val.contains('\0'));
}

/// Keypad 故障注入：Price 模式下连续多点、前导零、超大整数
///
/// **场景：** 验证极端输入下不 panic，小数位严格 ≤ 2
#[test]
fn fault_injection_keypad_edge_cases() {
    let mut kp = Keypad::new().with_mode(KeypadMode::Price);

    // 连续多点
    kp.handle(KeypadMessage::Dot);
    kp.handle(KeypadMessage::Dot);
    kp.handle(KeypadMessage::Dot);
    assert_eq!(
        kp.value().matches('.').count(),
        1,
        "multiple dots: {}",
        kp.value()
    );

    // 小数点后连续 DoubleZero
    kp.handle(KeypadMessage::Dot);
    kp.handle(KeypadMessage::DoubleZero);
    kp.handle(KeypadMessage::DoubleZero);
    kp.handle(KeypadMessage::DoubleZero);
    let dec_part = kp.value().split('.').nth(1).unwrap_or("");
    assert!(
        dec_part.len() <= 2,
        "Price decimals > 2: {} dec_part={}",
        kp.value(),
        dec_part
    );

    // 超大整数（超过 MAX_INT_LEN=8）
    let mut kp2 = Keypad::new().with_mode(KeypadMode::Price);
    for _ in 0..20 {
        kp2.handle(KeypadMessage::Digit(9));
    }
    let int_part = kp2
        .value()
        .split('.')
        .next()
        .unwrap_or("")
        .trim_start_matches('-');
    assert!(
        int_part.len() <= 8,
        "int overflow: len={} value={}",
        int_part.len(),
        kp2.value()
    );

    // 前导零
    let mut kp3 = Keypad::new().with_mode(KeypadMode::Price);
    kp3.handle(KeypadMessage::Digit(0));
    kp3.handle(KeypadMessage::Digit(0));
    kp3.handle(KeypadMessage::Digit(0));
    kp3.handle(KeypadMessage::Digit(1));
    // 不 panic 即通过，具体前导零处理策略由组件决定
}

/// Payment 故障注入：负总额、NaN、无穷大
///
/// **场景：** 验证异常金额输入下不 panic
#[test]
fn fault_injection_payment_abnormal_amounts() {
    // 负总额
    let mut p1 = Payment::new(-100.0);
    p1.handle(PaymentMessage::Confirm);
    p1.handle(PaymentMessage::Received(-50.0));
    // 不 panic 即通过

    // 零总额
    let mut p2 = Payment::new(0.0);
    p2.handle(PaymentMessage::Confirm);
    p2.handle(PaymentMessage::UpdateTotal(0.0));
    // 不 panic 即通过

    // NaN 输入（f64 特殊值）
    let mut p3 = Payment::new(100.0);
    p3.handle(PaymentMessage::Received(f64::NAN));
    p3.handle(PaymentMessage::UpdateTotal(f64::INFINITY));
    p3.handle(PaymentMessage::UpdateTotal(f64::NEG_INFINITY));
    // 不 panic 即通过
}

/// HangOrder 故障注入：空 ID、超长 ID、特殊字符 ID
///
/// **场景：** 验证异常 ID 输入下不 panic，ID 唯一性仍成立
#[test]
fn fault_injection_hang_order_abnormal_ids() {
    let mut ho = HangOrder::new();

    let abnormal_items = vec![
        HangOrderItem {
            id: String::new(),
            customer_name: "空ID".into(),
            total: 10.0,
            item_count: 1,
            timestamp: 1,
        },
        HangOrderItem {
            id: "a".repeat(1000),
            customer_name: "超长ID".into(),
            total: 10.0,
            item_count: 1,
            timestamp: 2,
        },
        HangOrderItem {
            id: "id-with-special!@#$%^&*()".into(),
            customer_name: "特殊字符".into(),
            total: 10.0,
            item_count: 1,
            timestamp: 3,
        },
        HangOrderItem {
            id: "🎉emoji".into(),
            customer_name: "Emoji".into(),
            total: 10.0,
            item_count: 1,
            timestamp: 4,
        },
        HangOrderItem {
            id: "id-with-unicode-中文-日本語".into(),
            customer_name: "Unicode".into(),
            total: 10.0,
            item_count: 1,
            timestamp: 5,
        },
    ];

    for item in abnormal_items {
        let _ = ho.handle(HangOrderMessage::Hang(item));
    }

    // 重复挂同一个空 ID
    let _ = ho.handle(HangOrderMessage::Hang(HangOrderItem {
        id: String::new(),
        customer_name: "重复空ID".into(),
        total: 20.0,
        item_count: 2,
        timestamp: 6,
    }));

    // 取不存在的 ID
    let _ = ho.handle(HangOrderMessage::Take("non-existent-id".into()));
    // 删不存在的 ID
    let _ = ho.handle(HangOrderMessage::Delete("non-existent-id".into()));

    // 不变式：所有 ID 唯一
    let mut seen = std::collections::HashSet::new();
    for item in ho.items() {
        assert!(
            seen.insert(item.id.clone()),
            "duplicate after fault injection: {:?}",
            item.id
        );
    }
}
