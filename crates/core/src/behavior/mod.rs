//! HarUI 行为层（headless）— 无样式纯计算，零 iced 依赖（ADR-009）
//!
//! 与渲染层（组件 view）分离的几何/数值逻辑：
//! - `overlay`: 弹层 12 方位定位引擎（碰撞翻转/钳制）
//! - `virtual_list`: 等高行虚拟滚动度量
//! - `stack`: 浮层同侧堆叠偏移
//!
//! 硬约束：本模块树禁止 `use iced`，坐标一律用自有 `f32` 类型，
//! 保证可被 proptest 全空间覆盖并可在任何 Rust GUI 栈中复用。

pub mod overlay;
pub mod stack;
pub mod virtual_list;
