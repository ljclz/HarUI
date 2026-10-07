//! W4 业务流回放测试 — 固定操作序列 + 中间不变式与终态断言
//!
//! 与 chaos_test（随机事件风暴，管健壮性）互补：本文件用**真实业务顺序**回放
//! 典型收银操作流，验证组件组合下的业务正确性（路线图 W4 / M-B）。
//! 全部 headless（纯 handle 序列，无渲染）。

use har_ui_components::dialog::{Dialog, DialogMessage, DialogState};
use har_ui_components::form::{Form, FormItem, FormMessage, FormState};
use har_ui_components::hang_order::{HangOrder, HangOrderItem, HangOrderMessage};
use har_ui_components::keypad::{Keypad, KeypadMessage, KeypadMode, KeypadState};
use har_ui_components::payment::{Payment, PaymentMessage, PaymentState};

use std::collections::BTreeSet;

/// 不变式辅助：挂单列表内 ID 全程唯一
fn assert_ids_unique(items: &[HangOrderItem]) {
    let ids: BTreeSet<&str> = items.iter().map(|i| i.id.as_str()).collect();
    assert_eq!(ids.len(), items.len(), "挂单 ID 重复: {:?}", ids);
}

/// 流 1：扫码收银 — Price 键盘输入金额 → 确认 → 现金收款 → 找零 → 终态
#[test]
fn replay_scan_and_cash_checkout() {
    // 步骤 1：键盘逐位输入 12.50（Price 模式）
    let mut keypad = Keypad::new().with_mode(KeypadMode::Price);
    let keys = [
        KeypadMessage::Digit(1),
        KeypadMessage::Digit(2),
        KeypadMessage::Dot,
        KeypadMessage::Digit(5),
        KeypadMessage::Digit(0),
    ];
    for k in keys {
        keypad.handle(k);
        // 中间不变式：输入过程中始终处于编辑态
        assert_eq!(keypad.state(), KeypadState::Editing);
    }
    assert_eq!(keypad.value(), "12.50");

    // 步骤 2：确认金额
    keypad.handle(KeypadMessage::Ok);
    assert_eq!(keypad.state(), KeypadState::Confirmed);
    let total: f64 = keypad.value().parse().expect("确认后的金额可解析");

    // 步骤 3：现金收款 20 元 → 找零 7.50
    let mut pay = Payment::new(total);
    assert_eq!(pay.state(), PaymentState::Pending);
    pay.handle(PaymentMessage::Received(20.0));
    assert!(
        (pay.change() - 7.5).abs() < 1e-9,
        "找零应为 7.50，实际 {}",
        pay.change()
    );

    // 步骤 4：确认收款 → 终态不变式（现金实收 ≥ 总额）
    pay.handle(PaymentMessage::Confirm);
    assert_eq!(pay.state(), PaymentState::Confirmed);
    assert!(pay.received() >= pay.total());
}

/// 流 2：挂单生命周期 — 挂两单 → 取单一单 → 重挂 → 删除 → 清空（ID 全程唯一）
#[test]
fn replay_hang_order_lifecycle() {
    let item = |id: &str, ts: i64| HangOrderItem {
        id: id.to_string(),
        customer_name: "散客".to_string(),
        total: 25.0,
        item_count: 3,
        timestamp: ts,
    };
    let mut hang = HangOrder::new();

    // 步骤 1：挂起两单
    hang.handle(HangOrderMessage::Hang(item("H1", 100)));
    hang.handle(HangOrderMessage::Hang(item("H2", 200)));
    assert_eq!(hang.count(), 2);
    assert_ids_unique(hang.items());

    // 步骤 2：取单 H1 → 从列表移除并返回内容
    let taken = hang.handle(HangOrderMessage::Take("H1".to_string()));
    assert_eq!(taken.map(|t| t.id), Some("H1".to_string()));
    assert_eq!(hang.count(), 1);

    // 步骤 3：同 ID 重新挂单（时间戳更新语义）
    hang.handle(HangOrderMessage::Hang(item("H1", 300)));
    assert_eq!(hang.count(), 2);
    assert_ids_unique(hang.items());

    // 步骤 4：删除 H2 → 清空
    hang.handle(HangOrderMessage::Delete("H2".to_string()));
    assert_eq!(hang.count(), 1);
    hang.handle(HangOrderMessage::ClearAll);
    assert_eq!(hang.count(), 0);
}

/// 流 3：表单提交 — 空值校验失败 → 修正 → 通过 → 重置回初始态
#[test]
fn replay_form_submit_flow() {
    let mut form = Form::new().with_item(FormItem::new("member", "会员卡号").set_required(true));

    // 步骤 1：空值校验 → Failed 且有错误项
    form.handle(FormMessage::SetValue("member".to_string(), String::new()));
    form.handle(FormMessage::Validate);
    assert_eq!(form.state(), FormState::Failed);
    assert_eq!(form.errors().len(), 1);

    // 步骤 2：修正输入 → 重新校验通过
    form.handle(FormMessage::SetValue(
        "member".to_string(),
        "VIP8888".to_string(),
    ));
    assert_eq!(form.value_of("member").map(String::as_str), Some("VIP8888"));
    form.handle(FormMessage::Validate);
    assert_eq!(form.state(), FormState::Passed);
    assert!(form.errors().is_empty());

    // 步骤 3：重置 → Idle + 值清空
    form.handle(FormMessage::Reset);
    assert_eq!(form.state(), FormState::Idle);
    assert_eq!(form.value_of("member"), None);
}

/// 流 4：弹层流 — 打开对话框 → 内部表单输入 → 取消关闭 → 数据保留可重试
#[test]
fn replay_dialog_form_flow() {
    let mut dlg = Dialog::new("修改数量", "输入新的数量");
    let mut form = Form::new().with_item(FormItem::new("qty", "数量").set_required(true));

    // 步骤 1：打开对话框（Open → Opening → 动画完成 → Open）
    dlg.handle(DialogMessage::Open);
    assert!(dlg.is_visible(), "Opening 态即视为可见");
    dlg.handle(DialogMessage::AnimationFinished);
    assert_eq!(dlg.state(), DialogState::Open);

    // 步骤 2：对话框内表单输入
    form.handle(FormMessage::SetValue("qty".to_string(), "3".to_string()));
    assert_eq!(form.value_of("qty").map(String::as_str), Some("3"));

    // 步骤 3：取消关闭（Escape 仅在 Open 态生效 → Closing → 动画完成 → Closed）
    dlg.handle(DialogMessage::EscapePressed);
    dlg.handle(DialogMessage::AnimationFinished);
    assert_eq!(dlg.state(), DialogState::Closed);
    assert!(!dlg.is_visible());

    // 步骤 4：状态还原 — 表单数据保留，可直接重开重试
    assert_eq!(form.value_of("qty").map(String::as_str), Some("3"));
    form.handle(FormMessage::Validate);
    assert_eq!(form.state(), FormState::Passed);
    dlg.handle(DialogMessage::Open);
    dlg.handle(DialogMessage::AnimationFinished);
    assert!(dlg.is_visible(), "关闭后可重新打开");
}
