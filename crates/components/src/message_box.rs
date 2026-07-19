//! MessageBox 消息框组件 — 参考 Element Plus `ElMessageBox`。
//! 支持：alert/confirm/prompt 三种模式、4 种类型、按钮配置、close_on_click_modal、center、prompt 输入。

/// MessageBox 类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MessageBoxType {
    Success,
    Warning,
    #[default]
    Info,
    Error,
}

impl MessageBoxType {
    pub fn as_str(self) -> &'static str {
        match self {
            MessageBoxType::Success => "success",
            MessageBoxType::Warning => "warning",
            MessageBoxType::Info => "info",
            MessageBoxType::Error => "error",
        }
    }
}

/// MessageBox 用户动作
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageBoxAction {
    Confirm,
    Cancel,
    Close,
}

/// MessageBox 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageBoxMessage {
    /// 打开对话框
    Open,
    /// 关闭对话框（点击 X 或 Esc）
    Close,
    /// 点击确认
    Confirm,
    /// 点击取消
    Cancel,
    /// 点击遮罩层
    ClickModal,
    /// prompt 模式输入值
    Input(String),
}

/// MessageBox 组件
#[derive(Debug, Clone)]
pub struct MessageBox {
    /// 标题
    title: String,
    /// 消息内容
    message: String,
    /// 类型
    msg_type: MessageBoxType,
    /// 是否可见
    visible: bool,
    /// 是否显示确认按钮
    show_confirm: bool,
    /// 是否显示取消按钮
    show_cancel: bool,
    /// 是否显示关闭按钮
    show_close: bool,
    /// 是否居中显示
    center: bool,
    /// 是否点击遮罩关闭
    close_on_click_modal: bool,
    /// 确认按钮文本
    confirm_text: String,
    /// 取消按钮文本
    cancel_text: String,
    /// 是否为 prompt 模式
    is_prompt: bool,
    /// prompt 输入值
    input_value: String,
    /// 上次用户动作（用于回调判断）
    last_action: Option<MessageBoxAction>,
    /// 确认时携带的输入值（仅 prompt 模式有效）
    confirmed_input: Option<String>,
}

impl MessageBox {
    pub fn new(title: &str, message: &str) -> Self {
        Self {
            title: title.to_string(),
            message: message.to_string(),
            msg_type: MessageBoxType::Info,
            visible: false,
            show_confirm: true,
            show_cancel: false,
            show_close: true,
            center: false,
            close_on_click_modal: false,
            confirm_text: "确认".to_string(),
            cancel_text: "取消".to_string(),
            is_prompt: false,
            input_value: String::new(),
            last_action: None,
            confirmed_input: None,
        }
    }

    /// 创建 prompt 模式对话框
    pub fn prompt(title: &str, message: &str) -> Self {
        let mut mb = Self::new(title, message);
        mb.is_prompt = true;
        mb.show_cancel = true;
        mb
    }

    // ---------- Builder ----------

    pub fn with_type(mut self, t: MessageBoxType) -> Self {
        self.msg_type = t;
        self
    }

    pub fn with_show_confirm(mut self, s: bool) -> Self {
        self.show_confirm = s;
        self
    }

    pub fn with_show_cancel(mut self, s: bool) -> Self {
        self.show_cancel = s;
        self
    }

    pub fn with_show_close(mut self, s: bool) -> Self {
        self.show_close = s;
        self
    }

    pub fn with_center(mut self, c: bool) -> Self {
        self.center = c;
        self
    }

    pub fn with_close_on_click_modal(mut self, c: bool) -> Self {
        self.close_on_click_modal = c;
        self
    }

    pub fn with_confirm_text(mut self, t: &str) -> Self {
        self.confirm_text = t.to_string();
        self
    }

    pub fn with_cancel_text(mut self, t: &str) -> Self {
        self.cancel_text = t.to_string();
        self
    }

    // ---------- Getter ----------

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn msg_type(&self) -> MessageBoxType {
        self.msg_type
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn show_confirm_button(&self) -> bool {
        self.show_confirm
    }

    pub fn show_cancel_button(&self) -> bool {
        self.show_cancel
    }

    pub fn show_close(&self) -> bool {
        self.show_close
    }

    pub fn center(&self) -> bool {
        self.center
    }

    pub fn close_on_click_modal(&self) -> bool {
        self.close_on_click_modal
    }

    pub fn confirm_button_text(&self) -> &str {
        &self.confirm_text
    }

    pub fn cancel_button_text(&self) -> &str {
        &self.cancel_text
    }

    pub fn is_prompt(&self) -> bool {
        self.is_prompt
    }

    pub fn input_value(&self) -> &str {
        &self.input_value
    }

    pub fn last_action(&self) -> Option<MessageBoxAction> {
        self.last_action
    }

    /// 确认动作携带的输入值（仅 prompt 模式 + Confirm 后有效）
    pub fn confirmed_input(&self) -> Option<&str> {
        self.confirmed_input.as_deref()
    }

    // ---------- 消息处理 ----------

    pub fn handle(&mut self, msg: MessageBoxMessage) {
        match msg {
            MessageBoxMessage::Open => {
                self.visible = true;
                self.last_action = None;
                // 重新打开时清空 prompt 输入
                if self.is_prompt {
                    self.input_value.clear();
                    self.confirmed_input = None;
                }
            }
            MessageBoxMessage::Close => {
                self.visible = false;
                self.last_action = Some(MessageBoxAction::Close);
            }
            MessageBoxMessage::Confirm => {
                if self.is_prompt {
                    self.confirmed_input = Some(self.input_value.clone());
                }
                self.visible = false;
                self.last_action = Some(MessageBoxAction::Confirm);
            }
            MessageBoxMessage::Cancel => {
                self.confirmed_input = None;
                self.visible = false;
                self.last_action = Some(MessageBoxAction::Cancel);
            }
            MessageBoxMessage::ClickModal => {
                if self.close_on_click_modal {
                    self.visible = false;
                    self.last_action = Some(MessageBoxAction::Close);
                }
            }
            MessageBoxMessage::Input(v) => {
                if self.is_prompt {
                    self.input_value = v;
                }
            }
        }
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_mb_internal_default() {
        let mb = MessageBox::new("t", "m");
        assert_eq!(mb.title, "t");
        assert_eq!(mb.message, "m");
        assert_eq!(mb.msg_type, MessageBoxType::Info);
        assert!(mb.show_confirm);
        assert!(!mb.show_cancel);
        assert!(!mb.is_prompt);
    }

    #[test]
    fn test_mb_internal_prompt_constructor() {
        let mb = MessageBox::prompt("t", "m");
        assert!(mb.is_prompt);
        assert!(mb.show_cancel);
    }

    #[test]
    fn test_mb_internal_open_resets_state() {
        let mut mb = MessageBox::prompt("t", "m");
        mb.handle(MessageBoxMessage::Open);
        mb.handle(MessageBoxMessage::Input("测试".to_string()));
        mb.handle(MessageBoxMessage::Cancel);
        mb.handle(MessageBoxMessage::Open);
        assert!(mb.input_value.is_empty());
        assert!(mb.confirmed_input.is_none());
        assert!(mb.last_action.is_none());
    }
}
