//! StatusBar 状态栏组件 — POS 专用
//!
//! 显示硬件设备状态：连接、称重、扫码、打印机、网络。
//! 支持左右两侧插槽、自定义状态项、批量更新硬件状态。

/// 状态级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StatusLevel {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

/// 单个状态项
#[derive(Debug, Clone)]
pub struct StatusItem {
    label: String,
    level: StatusLevel,
    detail: Option<String>,
}

impl StatusItem {
    pub fn new(label: impl Into<String>, level: StatusLevel) -> Self {
        Self {
            label: label.into(),
            level,
            detail: None,
        }
    }

    pub fn with_detail(mut self, d: impl Into<String>) -> Self {
        self.detail = Some(d.into());
        self
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn level(&self) -> StatusLevel {
        self.level
    }

    pub fn detail(&self) -> Option<&String> {
        self.detail.as_ref()
    }

    pub fn set_level(&mut self, l: StatusLevel) {
        self.level = l;
    }

    pub fn set_detail(&mut self, d: Option<String>) {
        self.detail = d;
    }
}

/// StatusBar 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatusBarMessage {
    /// 更新左侧某项的状态
    UpdateLeft(usize, StatusLevel, Option<String>),
    /// 更新右侧某项的状态
    UpdateRight(usize, StatusLevel, Option<String>),
    /// 批量设置硬件状态
    SetHardwareStatus {
        scale: StatusLevel,
        scanner: StatusLevel,
        printer: StatusLevel,
        network: StatusLevel,
    },
    /// 清空所有
    ClearAll,
}

/// StatusBar 组件
#[derive(Debug, Clone, Default)]
pub struct StatusBar {
    left_items: Vec<StatusItem>,
    right_items: Vec<StatusItem>,
}

impl StatusBar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_left_item(mut self, item: StatusItem) -> Self {
        self.left_items.push(item);
        self
    }

    pub fn with_right_item(mut self, item: StatusItem) -> Self {
        self.right_items.push(item);
        self
    }

    pub fn left_items(&self) -> &[StatusItem] {
        &self.left_items
    }

    pub fn right_items(&self) -> &[StatusItem] {
        &self.right_items
    }

    /// 处理消息
    pub fn handle(&mut self, msg: StatusBarMessage) {
        match msg {
            StatusBarMessage::UpdateLeft(idx, level, detail) => {
                if let Some(item) = self.left_items.get_mut(idx) {
                    item.set_level(level);
                    item.set_detail(detail);
                }
            }
            StatusBarMessage::UpdateRight(idx, level, detail) => {
                if let Some(item) = self.right_items.get_mut(idx) {
                    item.set_level(level);
                    item.set_detail(detail);
                }
            }
            StatusBarMessage::SetHardwareStatus {
                scale,
                scanner,
                printer,
                network,
            } => {
                // 覆盖左侧前 4 项（或重建）
                self.left_items = vec![
                    StatusItem::new("秤", scale),
                    StatusItem::new("扫码枪", scanner),
                    StatusItem::new("打印机", printer),
                    StatusItem::new("网络", network),
                ];
            }
            StatusBarMessage::ClearAll => {
                self.left_items.clear();
                self.right_items.clear();
            }
        }
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_status_item_builder() {
        let item = StatusItem::new("网络", StatusLevel::Success).with_detail("OK");
        assert_eq!(item.label(), "网络");
        assert_eq!(item.level(), StatusLevel::Success);
        assert_eq!(item.detail(), Some(&"OK".to_string()));
    }

    #[test]
    fn test_status_bar_left_right_independent() {
        let s = StatusBar::new()
            .with_left_item(StatusItem::new("L1", StatusLevel::Info))
            .with_right_item(StatusItem::new("R1", StatusLevel::Info));
        assert_eq!(s.left_items().len(), 1);
        assert_eq!(s.right_items().len(), 1);
    }

    #[test]
    fn test_status_bar_set_hardware_status_overwrites() {
        let mut s = StatusBar::new();
        s.handle(StatusBarMessage::SetHardwareStatus {
            scale: StatusLevel::Success,
            scanner: StatusLevel::Success,
            printer: StatusLevel::Success,
            network: StatusLevel::Success,
        });
        s.handle(StatusBarMessage::SetHardwareStatus {
            scale: StatusLevel::Error,
            scanner: StatusLevel::Error,
            printer: StatusLevel::Error,
            network: StatusLevel::Error,
        });
        assert_eq!(s.left_items().len(), 4);
        for item in s.left_items() {
            assert_eq!(item.level(), StatusLevel::Error);
        }
    }
}
