//! HarUI 主题系统
//!
//! 与 Element Plus CSS 变量 1:1 映射。
//! 支持 light/dark 双主题。

pub mod color;
pub mod iced_adapter;
pub mod radius;
pub mod shadow;
pub mod spacing;
pub mod style_sheets;
// 允许 module_inception：theme::theme::Theme 是刻意的设计（与 Element Plus 的
// theme 命名空间对齐），重命名会破坏 har_ui_core::theme::theme 公开路径
#[allow(clippy::module_inception)]
pub mod theme;
pub mod typography;
pub mod zindex;

pub use color::ColorPalette;
pub use radius::Radius;
pub use shadow::Shadow;
pub use spacing::Spacing;
pub use theme::Theme;
pub use typography::Typography;
pub use zindex::ZIndex;
