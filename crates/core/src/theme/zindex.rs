//! Z-index 层级系统
//!
//! 来源: Element Plus --el-index-* 变量
//!   --el-index-normal: 1
//!   --el-index-top: 1000
//!   --el-index-popper: 2000
//! 细分:
//!   dropdown: 1000
//!   sticky: 1100
//!   fixed: 1200
//!   modal-mask: 2000
//!   modal: 2001
//!   popover: 3000
//!   tooltip: 3100
//!   notification: 4000
//!   message-box: 4001

/// Z-index 层级系统
#[derive(Debug, Clone, Copy)]
pub struct ZIndex {
    pub normal: i32,       // 1
    pub dropdown: i32,     // 1000
    pub sticky: i32,       // 1100
    pub fixed: i32,        // 1200
    pub modal_mask: i32,   // 2000
    pub modal: i32,        // 2001
    pub popover: i32,      // 3000
    pub tooltip: i32,      // 3100
    pub notification: i32, // 4000
    pub message_box: i32,  // 4001
}

impl Default for ZIndex {
    fn default() -> Self {
        Self {
            normal: 1,
            dropdown: 1000,
            sticky: 1100,
            fixed: 1200,
            modal_mask: 2000,
            modal: 2001,
            popover: 3000,
            tooltip: 3100,
            notification: 4000,
            message_box: 4001,
        }
    }
}
