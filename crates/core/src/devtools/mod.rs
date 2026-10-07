//! HarUI 开发辅助工具 — 性能自证（对标 GPUI Kit 的 gpui-fps，路线图 W5）
//!
//! 本模块允许依赖 iced（与 theme 适配层同级别），
//! 但与 `behavior` 行为层的"零 iced"硬约束互不干涉。

pub mod fps_meter;
