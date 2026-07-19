//! Alert 警告 — 参考 Element Plus `<el-alert>`。
//!
//! 支持：4 种 type、title/description、closable、center、show-icon、effect（light/dark）、可见性切换。

/// 类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlertType {
    Success,
    Warning,
    #[default]
    Info,
    Error,
}

/// 视觉效果
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlertEffect {
    #[default]
    Light,
    Dark,
}

/// Alert 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlertMessage {
    Close,
    Open,
}

/// Alert 组件
#[derive(Debug, Clone)]
pub struct Alert {
    alert_type: AlertType,
    title: String,
    description: Option<String>,
    closable: bool,
    center: bool,
    show_icon: bool,
    effect: AlertEffect,
    visible: bool,
}

impl Default for Alert {
    fn default() -> Self {
        Self::new()
    }
}

impl Alert {
    pub fn new() -> Self {
        Self {
            alert_type: AlertType::Info,
            title: String::new(),
            description: None,
            closable: true,
            center: false,
            show_icon: true,
            effect: AlertEffect::Light,
            visible: true,
        }
    }

    pub fn with_type(mut self, t: AlertType) -> Self {
        self.alert_type = t;
        self
    }

    pub fn with_title(mut self, t: impl Into<String>) -> Self {
        self.title = t.into();
        self
    }

    pub fn with_description(mut self, d: impl Into<String>) -> Self {
        self.description = Some(d.into());
        self
    }

    pub fn with_closable(mut self, v: bool) -> Self {
        self.closable = v;
        self
    }

    pub fn with_center(mut self, v: bool) -> Self {
        self.center = v;
        self
    }

    pub fn with_show_icon(mut self, v: bool) -> Self {
        self.show_icon = v;
        self
    }

    pub fn with_effect(mut self, e: AlertEffect) -> Self {
        self.effect = e;
        self
    }

    pub fn alert_type(&self) -> AlertType {
        self.alert_type
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn closable(&self) -> bool {
        self.closable
    }

    pub fn center(&self) -> bool {
        self.center
    }

    pub fn show_icon(&self) -> bool {
        self.show_icon
    }

    pub fn effect(&self) -> AlertEffect {
        self.effect
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn handle(&mut self, msg: AlertMessage) {
        match msg {
            AlertMessage::Close => {
                if self.closable {
                    self.visible = false;
                }
            }
            AlertMessage::Open => {
                self.visible = true;
            }
        }
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_alert_default_visible() {
        let a = Alert::new();
        assert!(a.visible());
        assert!(a.closable());
        assert!(a.show_icon());
        assert!(!a.center());
    }
}
