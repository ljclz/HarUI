//! MessageBox 消息框组件 — 参考 Element Plus `ElMessageBox`。

use har_ui_components::message_box::{
    MessageBox, MessageBoxAction, MessageBoxMessage, MessageBoxType,
};

// ---------- 基础构造 ----------

#[test]
fn test_message_box_default() {
    let mb = MessageBox::new("提示", "确认删除？");
    assert!(!mb.visible());
    assert_eq!(mb.title(), "提示");
    assert_eq!(mb.message(), "确认删除？");
    assert_eq!(mb.msg_type(), MessageBoxType::Info);
    assert!(mb.show_confirm_button());
    assert!(!mb.show_cancel_button());
    assert!(mb.show_close());
    assert!(!mb.center());
    assert!(!mb.close_on_click_modal());
    assert_eq!(mb.confirm_button_text(), "确认");
    assert_eq!(mb.cancel_button_text(), "取消");
}

#[test]
fn test_message_box_with_title() {
    let mb = MessageBox::new("警告", "x");
    assert_eq!(mb.title(), "警告");
}

#[test]
fn test_message_box_with_message() {
    let mb = MessageBox::new("x", "确定要继续吗？");
    assert_eq!(mb.message(), "确定要继续吗？");
}

#[test]
fn test_message_box_with_empty_title_message() {
    let mb = MessageBox::new("", "");
    assert_eq!(mb.title(), "");
    assert_eq!(mb.message(), "");
}

// ---------- 4 种类型 ----------

#[test]
fn test_message_box_with_type() {
    use MessageBoxType::*;
    let cases = [Success, Warning, Info, Error];
    for t in cases.iter() {
        let mb = MessageBox::new("x", "y").with_type(*t);
        assert_eq!(mb.msg_type(), *t);
    }
}

#[test]
fn test_message_box_type_as_str() {
    use MessageBoxType::*;
    assert_eq!(Success.as_str(), "success");
    assert_eq!(Warning.as_str(), "warning");
    assert_eq!(Info.as_str(), "info");
    assert_eq!(Error.as_str(), "error");
}

// ---------- 按钮配置 ----------

#[test]
fn test_message_box_with_confirm_text() {
    let mb = MessageBox::new("x", "y").with_confirm_text("好的");
    assert_eq!(mb.confirm_button_text(), "好的");
}

#[test]
fn test_message_box_with_cancel_text() {
    let mb = MessageBox::new("x", "y").with_cancel_text("算了");
    assert_eq!(mb.cancel_button_text(), "算了");
}

#[test]
fn test_message_box_with_show_cancel() {
    let mb = MessageBox::new("x", "y").with_show_cancel(true);
    assert!(mb.show_cancel_button());
}

#[test]
fn test_message_box_with_show_confirm_false() {
    let mb = MessageBox::new("x", "y").with_show_confirm(false);
    assert!(!mb.show_confirm_button());
}

// ---------- 显示/隐藏 ----------

#[test]
fn test_message_box_open_close() {
    let mut mb = MessageBox::new("x", "y");
    assert!(!mb.visible());
    mb.handle(MessageBoxMessage::Open);
    assert!(mb.visible());
    mb.handle(MessageBoxMessage::Close);
    assert!(!mb.visible());
}

#[test]
fn test_message_box_open_resets_action() {
    let mut mb = MessageBox::new("x", "y");
    mb.handle(MessageBoxMessage::Open);
    mb.handle(MessageBoxMessage::Confirm);
    assert_eq!(mb.last_action(), Some(MessageBoxAction::Confirm));
    // 再次打开应重置上次动作
    mb.handle(MessageBoxMessage::Close);
    mb.handle(MessageBoxMessage::Open);
    assert_eq!(mb.last_action(), None);
}

// ---------- Confirm / Cancel 行为 ----------

#[test]
fn test_message_box_confirm_action() {
    let mut mb = MessageBox::new("x", "y");
    mb.handle(MessageBoxMessage::Open);
    mb.handle(MessageBoxMessage::Confirm);
    assert_eq!(mb.last_action(), Some(MessageBoxAction::Confirm));
    assert!(!mb.visible(), "确认后应自动关闭");
}

#[test]
fn test_message_box_cancel_action() {
    let mut mb = MessageBox::new("x", "y").with_show_cancel(true);
    mb.handle(MessageBoxMessage::Open);
    mb.handle(MessageBoxMessage::Cancel);
    assert_eq!(mb.last_action(), Some(MessageBoxAction::Cancel));
    assert!(!mb.visible(), "取消后应自动关闭");
}

#[test]
fn test_message_box_close_action() {
    let mut mb = MessageBox::new("x", "y");
    mb.handle(MessageBoxMessage::Open);
    mb.handle(MessageBoxMessage::Close);
    assert_eq!(mb.last_action(), Some(MessageBoxAction::Close));
    assert!(!mb.visible());
}

// ---------- close_on_click_modal ----------

#[test]
fn test_message_box_close_on_click_modal_default_false() {
    let mut mb = MessageBox::new("x", "y");
    mb.handle(MessageBoxMessage::Open);
    // 默认不响应点击遮罩
    mb.handle(MessageBoxMessage::ClickModal);
    assert!(mb.visible());
}

#[test]
fn test_message_box_close_on_click_modal_true() {
    let mut mb = MessageBox::new("x", "y").with_close_on_click_modal(true);
    mb.handle(MessageBoxMessage::Open);
    mb.handle(MessageBoxMessage::ClickModal);
    assert!(!mb.visible());
}

// ---------- show_close ----------

#[test]
fn test_message_box_show_close_default_true() {
    let mb = MessageBox::new("x", "y");
    assert!(mb.show_close());
}

#[test]
fn test_message_box_with_show_close_false() {
    let mb = MessageBox::new("x", "y").with_show_close(false);
    assert!(!mb.show_close());
}

// ---------- center 居中 ----------

#[test]
fn test_message_box_with_center() {
    let mb = MessageBox::new("x", "y").with_center(true);
    assert!(mb.center());
}

// ---------- Prompt 输入模式 ----------

#[test]
fn test_message_box_prompt_input_default_empty() {
    let mb = MessageBox::prompt("x", "y");
    assert_eq!(mb.input_value(), "");
    assert!(mb.is_prompt());
}

#[test]
fn test_message_box_prompt_set_input() {
    let mut mb = MessageBox::prompt("x", "请输入名字");
    mb.handle(MessageBoxMessage::Input("张三".to_string()));
    assert_eq!(mb.input_value(), "张三");
}

#[test]
fn test_message_box_prompt_confirm_carries_input() {
    let mut mb = MessageBox::prompt("x", "请输入名字");
    mb.handle(MessageBoxMessage::Open);
    mb.handle(MessageBoxMessage::Input("李四".to_string()));
    mb.handle(MessageBoxMessage::Confirm);
    assert_eq!(mb.last_action(), Some(MessageBoxAction::Confirm));
    assert_eq!(mb.input_value(), "李四");
    assert_eq!(mb.confirmed_input(), Some("李四"));
}

#[test]
fn test_message_box_prompt_cancel_clears_input() {
    let mut mb = MessageBox::prompt("x", "y").with_show_cancel(true);
    mb.handle(MessageBoxMessage::Open);
    mb.handle(MessageBoxMessage::Input("测试".to_string()));
    mb.handle(MessageBoxMessage::Cancel);
    // 取消后输入值不保留
    assert_eq!(mb.confirmed_input(), None);
}

// ---------- 边界 ----------

#[test]
fn test_message_box_confirm_when_closed_no_op() {
    let mut mb = MessageBox::new("x", "y");
    // 未打开时 Confirm 不应改变状态
    mb.handle(MessageBoxMessage::Confirm);
    assert_eq!(mb.last_action(), Some(MessageBoxAction::Confirm));
    assert!(!mb.visible());
}

#[test]
fn test_message_box_open_resets_input() {
    let mut mb = MessageBox::prompt("x", "y");
    mb.handle(MessageBoxMessage::Open);
    mb.handle(MessageBoxMessage::Input("测试".to_string()));
    mb.handle(MessageBoxMessage::Cancel);
    // 重新打开应清空输入
    mb.handle(MessageBoxMessage::Open);
    assert_eq!(mb.input_value(), "");
}

#[test]
fn test_message_box_non_prompt_has_no_input() {
    let mb = MessageBox::new("x", "y");
    assert!(!mb.is_prompt());
    assert_eq!(mb.input_value(), "");
}
