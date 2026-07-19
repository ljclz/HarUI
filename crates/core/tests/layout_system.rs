//! 布局系统测试 — Row/Col/Container 24 列网格
//!
//! 参考 Element Plus Layout 组件：
//! - Row: gutter / justify / align / tag
//! - Col: span(0-24) / offset / push / pull / xs/sm/md/lg/xl/xxl 响应式断点

use har_ui_core::layout::{
    row::{Row, RowAlign, RowJustify},
    col::{Col, ColSpan, ColOffset},
    container::{Container, ContainerWidth},
};

#[test]
fn test_row_default() {
    let row = Row::new();
    assert_eq!(row.gutter(), 0.0);
    assert_eq!(row.justify(), RowJustify::Start);
    assert_eq!(row.align(), RowAlign::Top);
}

#[test]
fn test_row_with_gutter() {
    let row = Row::new().with_gutter(20.0);
    assert_eq!(row.gutter(), 20.0);
}

#[test]
fn test_row_justify_variants() {
    assert_eq!(Row::new().with_justify(RowJustify::Center).justify(), RowJustify::Center);
    assert_eq!(Row::new().with_justify(RowJustify::End).justify(), RowJustify::End);
    assert_eq!(Row::new().with_justify(RowJustify::SpaceBetween).justify(), RowJustify::SpaceBetween);
    assert_eq!(Row::new().with_justify(RowJustify::SpaceAround).justify(), RowJustify::SpaceAround);
    assert_eq!(Row::new().with_justify(RowJustify::SpaceEvenly).justify(), RowJustify::SpaceEvenly);
}

#[test]
fn test_row_align_variants() {
    assert_eq!(Row::new().with_align(RowAlign::Middle).align(), RowAlign::Middle);
    assert_eq!(Row::new().with_align(RowAlign::Bottom).align(), RowAlign::Bottom);
}

#[test]
fn test_col_default() {
    let col = Col::new();
    assert_eq!(col.span(), ColSpan::Full); // 默认 24 占满
    assert_eq!(col.offset(), ColOffset::None);
    assert_eq!(col.push(), 0);
    assert_eq!(col.pull(), 0);
}

#[test]
fn test_col_with_span() {
    let col = Col::new().with_span(ColSpan::new(12));
    assert_eq!(col.span().value(), 12);
}

#[test]
fn test_col_span_boundary_values() {
    assert_eq!(ColSpan::new(0).value(), 0);
    assert_eq!(ColSpan::new(24).value(), 24);
    // 超出范围应被 clamp
    assert_eq!(ColSpan::new(25).value(), 24);
    assert_eq!(ColSpan::new(100).value(), 24);
}

#[test]
fn test_col_offset_boundary() {
    assert_eq!(ColOffset::new(0).value(), 0);
    assert_eq!(ColOffset::new(12).value(), 12);
    assert_eq!(ColOffset::new(24).value(), 24);
    assert_eq!(ColOffset::new(30).value(), 24);
}

#[test]
fn test_col_push_pull() {
    let col = Col::new().with_push(3).with_pull(2);
    assert_eq!(col.push(), 3);
    assert_eq!(col.pull(), 2);
}

#[test]
fn test_col_responsive_breakpoints() {
    use har_ui_core::layout::col::ResponsiveSpan;
    let col = Col::new()
        .with_xs(ResponsiveSpan::Span(24))
        .with_sm(ResponsiveSpan::Span(12))
        .with_md(ResponsiveSpan::Span(8))
        .with_lg(ResponsiveSpan::Span(6))
        .with_xl(ResponsiveSpan::Span(4))
        .with_xxl(ResponsiveSpan::Span(3));
    assert_eq!(col.xs(), Some(&ResponsiveSpan::Span(24)));
    assert_eq!(col.sm(), Some(&ResponsiveSpan::Span(12)));
    assert_eq!(col.md(), Some(&ResponsiveSpan::Span(8)));
    assert_eq!(col.lg(), Some(&ResponsiveSpan::Span(6)));
    assert_eq!(col.xl(), Some(&ResponsiveSpan::Span(4)));
    assert_eq!(col.xxl(), Some(&ResponsiveSpan::Span(3)));
}

#[test]
fn test_col_span_predefined() {
    assert_eq!(ColSpan::Full.value(), 24);
    assert_eq!(ColSpan::Half.value(), 12);
    assert_eq!(ColSpan::Third.value(), 8);
    assert_eq!(ColSpan::Quarter.value(), 6);
    assert_eq!(ColSpan::None.value(), 0);
}

#[test]
fn test_container_default() {
    let c = Container::new();
    assert_eq!(c.width(), ContainerWidth::Fluid);
}

#[test]
fn test_container_fixed_width() {
    let c = Container::new().with_width(ContainerWidth::Fixed(1200.0));
    assert_eq!(c.width(), ContainerWidth::Fixed(1200.0));
}

#[test]
fn test_container_centered() {
    let c = Container::new().with_centered(true);
    assert!(c.is_centered());
}

#[test]
fn test_container_padding() {
    let c = Container::new().with_padding(20.0);
    assert_eq!(c.padding(), 20.0);
}

#[test]
fn test_row_builder_pattern_chains() {
    let row = Row::new()
        .with_gutter(16.0)
        .with_justify(RowJustify::Center)
        .with_align(RowAlign::Middle);
    assert_eq!(row.gutter(), 16.0);
    assert_eq!(row.justify(), RowJustify::Center);
    assert_eq!(row.align(), RowAlign::Middle);
}

#[test]
fn test_col_width_percentage_calculation() {
    // span=12 of 24 → 50%
    let col = Col::new().with_span(ColSpan::new(12));
    assert_eq!(col.width_percentage(), 50.0);
    // span=6 of 24 → 25%
    let col = Col::new().with_span(ColSpan::new(6));
    assert_eq!(col.width_percentage(), 25.0);
    // span=24 → 100%
    let col = Col::new();
    assert_eq!(col.width_percentage(), 100.0);
    // span=0 → 0%
    let col = Col::new().with_span(ColSpan::new(0));
    assert_eq!(col.width_percentage(), 0.0);
}
