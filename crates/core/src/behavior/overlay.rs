//! 弹层定位引擎 — 无样式、纯 f32 几何、零 iced 依赖（ADR-009）
//!
//! 提供 Element Plus 语义的 12 方位弹出定位：给定锚点矩形、内容尺寸与视窗，
//! 计算弹层矩形；空间不足时按 [`CollisionPolicy`] 翻转/钳制。
//! 所有类型与函数均为纯计算，可被 T1 单测与 T4 proptest 全空间覆盖。

/// 矩形（行为层自有几何类型，禁止 iced 类型渗入本模块）
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width: width.max(0.0),
            height: height.max(0.0),
        }
    }

    pub fn right(&self) -> f32 {
        self.x + self.width
    }

    pub fn bottom(&self) -> f32 {
        self.y + self.height
    }
}

/// 二维尺寸
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width: width.max(0.0),
            height: height.max(0.0),
        }
    }
}

/// 弹出方位（12 种，与 Element Plus tooltip/popover 的 placement 语义一致）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Placement {
    /// 锚点上方，水平居中
    #[default]
    Top,
    /// 锚点上方，左对齐锚点左缘
    TopStart,
    /// 锚点上方，右对齐锚点右缘
    TopEnd,
    /// 锚点下方，水平居中
    Bottom,
    BottomStart,
    BottomEnd,
    /// 锚点左侧，垂直居中
    Left,
    LeftStart,
    LeftEnd,
    /// 锚点右侧，垂直居中
    Right,
    RightStart,
    RightEnd,
}

impl Placement {
    /// 翻转到对侧（Top↔Bottom / Left↔Right，Start/End 随轴映射）
    pub fn flipped(self) -> Placement {
        match self {
            Placement::Top => Placement::Bottom,
            Placement::TopStart => Placement::BottomStart,
            Placement::TopEnd => Placement::BottomEnd,
            Placement::Bottom => Placement::Top,
            Placement::BottomStart => Placement::TopStart,
            Placement::BottomEnd => Placement::TopEnd,
            Placement::Left => Placement::Right,
            Placement::LeftStart => Placement::RightStart,
            Placement::LeftEnd => Placement::RightEnd,
            Placement::Right => Placement::Left,
            Placement::RightStart => Placement::LeftStart,
            Placement::RightEnd => Placement::LeftEnd,
        }
    }

    fn is_vertical(self) -> bool {
        matches!(
            self,
            Placement::Top
                | Placement::TopStart
                | Placement::TopEnd
                | Placement::Bottom
                | Placement::BottomStart
                | Placement::BottomEnd
        )
    }
}

/// 空间不足时的碰撞策略
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CollisionPolicy {
    /// 不调整，按声明方位放置
    None,
    /// 首选侧空间不足且对侧更充裕时翻转到对侧（不钳制）
    #[default]
    Flip,
    /// 翻转优先；翻转后仍越界则钳回视窗内（等价 Element Plus 的 flip+shift）
    FlipThenShift,
}

/// 定位参数
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlacementOptions {
    pub placement: Placement,
    /// 弹层与锚点的间距（像素）
    pub offset: f32,
    pub collision: CollisionPolicy,
}

impl Default for PlacementOptions {
    fn default() -> Self {
        Self {
            placement: Placement::Top,
            offset: 4.0,
            collision: CollisionPolicy::Flip,
        }
    }
}

/// 定位结果：最终矩形 + 实际生效的方位（翻转后可能不同于声明值）
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ResolvedRect {
    pub rect: Rect,
    pub effective_placement: Placement,
}

/// 主轴剩余空间：返回 (声明方向空间, 对侧空间)
fn axis_space(anchor: &Rect, viewport: &Rect, vertical: bool) -> (f32, f32) {
    if vertical {
        let above = anchor.y - viewport.y;
        let below = viewport.bottom() - anchor.bottom();
        (above, below)
    } else {
        let left = anchor.x - viewport.x;
        let right = viewport.right() - anchor.right();
        (left, right)
    }
}

/// 沿主轴放置（不含对齐偏移）：返回内容矩形在主轴上的起点坐标
fn place_along_axis(anchor: &Rect, content: &Size, opts: &PlacementOptions, vertical: bool) -> f32 {
    if vertical {
        match opts.placement {
            Placement::Top | Placement::TopStart | Placement::TopEnd => {
                anchor.y - content.height - opts.offset
            }
            _ => anchor.bottom() + opts.offset,
        }
    } else {
        match opts.placement {
            Placement::Left | Placement::LeftStart | Placement::LeftEnd => {
                anchor.x - content.width - opts.offset
            }
            _ => anchor.right() + opts.offset,
        }
    }
}

/// 交叉轴对齐：Start 对齐锚点起始缘，End 对齐终止缘，缺省居中
fn align_cross_axis(anchor: &Rect, content: &Size, opts: &PlacementOptions, vertical: bool) -> f32 {
    let start_align = match opts.placement {
        Placement::TopStart
        | Placement::BottomStart
        | Placement::LeftStart
        | Placement::RightStart => -1,
        Placement::TopEnd | Placement::BottomEnd | Placement::LeftEnd | Placement::RightEnd => 1,
        _ => 0,
    };
    if vertical {
        match start_align {
            -1 => anchor.x,
            1 => anchor.right() - content.width,
            _ => anchor.x + (anchor.width - content.width) / 2.0,
        }
    } else {
        match start_align {
            -1 => anchor.y,
            1 => anchor.bottom() - content.height,
            _ => anchor.y + (anchor.height - content.height) / 2.0,
        }
    }
}

/// 计算弹层矩形（ADR-009）
///
/// - `anchor`: 触发元素在视窗坐标系下的矩形
/// - `content`: 弹层内容尺寸
/// - `viewport`: 可用视窗矩形
pub fn compute_placement(
    anchor: Rect,
    content: Size,
    viewport: Rect,
    opts: PlacementOptions,
) -> ResolvedRect {
    let vertical = opts.placement.is_vertical();
    let mut placement = opts.placement;

    // 碰撞处理：翻转判定（主轴剩余空间比较，与 Element Plus flip 语义一致）
    if opts.collision != CollisionPolicy::None {
        let (backward, forward) = axis_space(&anchor, &viewport, vertical);
        let need = content.needed_axis_size(vertical) + opts.offset;
        // Top/Left 系朝负向（声明侧 = backward），Bottom/Right 系朝正向
        let (declared_space, opposite_space) = if is_forward_direction(placement) {
            (forward, backward)
        } else {
            (backward, forward)
        };
        if declared_space < need && opposite_space > declared_space {
            placement = placement.flipped();
        }
    }

    // 翻转后的方位必须同步用于几何计算（否则出现"判定翻转、位置仍在原侧"的错位）
    let effective = PlacementOptions { placement, ..opts };

    let mut x = if vertical {
        align_cross_axis(&anchor, &content, &effective, true)
    } else {
        place_along_axis(&anchor, &content, &effective, false)
    };
    let mut y = if vertical {
        place_along_axis(&anchor, &content, &effective, true)
    } else {
        align_cross_axis(&anchor, &content, &effective, false)
    };

    // FlipThenShift：翻转后仍越界则钳回视窗
    if opts.collision == CollisionPolicy::FlipThenShift {
        let max_x = (viewport.x + viewport.width - content.width).max(viewport.x);
        let max_y = (viewport.y + viewport.height - content.height).max(viewport.y);
        x = x.clamp(viewport.x, max_x);
        y = y.clamp(viewport.y, max_y);
    }

    ResolvedRect {
        rect: Rect::new(x, y, content.width, content.height),
        effective_placement: placement,
    }
}

/// 方位是否朝坐标轴正向（Bottom/Right 系）
fn is_forward_direction(p: Placement) -> bool {
    matches!(
        p,
        Placement::Bottom
            | Placement::BottomStart
            | Placement::BottomEnd
            | Placement::Right
            | Placement::RightStart
            | Placement::RightEnd
    )
}

impl Size {
    /// 该尺寸在指定轴上（true=垂直）的占用量
    fn needed_axis_size(self, vertical: bool) -> f32 {
        if vertical { self.height } else { self.width }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VIEW: Rect = Rect {
        x: 0.0,
        y: 0.0,
        width: 800.0,
        height: 600.0,
    };

    fn opts(p: Placement) -> PlacementOptions {
        PlacementOptions {
            placement: p,
            offset: 4.0,
            collision: CollisionPolicy::None,
        }
    }

    #[test]
    fn test_top_centered_above_anchor() {
        let anchor = Rect::new(380.0, 300.0, 40.0, 20.0);
        let r = compute_placement(anchor, Size::new(100.0, 30.0), VIEW, opts(Placement::Top));
        // 水平居中：380 + (40-100)/2 = 350；上方：300 - 30 - 4 = 266
        assert_eq!(r.rect.x, 350.0);
        assert_eq!(r.rect.y, 266.0);
        assert_eq!(r.effective_placement, Placement::Top);
    }

    #[test]
    fn test_top_start_end_alignment() {
        let anchor = Rect::new(380.0, 300.0, 40.0, 20.0);
        let c = Size::new(100.0, 30.0);
        let s = compute_placement(anchor, c, VIEW, opts(Placement::TopStart));
        assert_eq!(s.rect.x, 380.0);
        let e = compute_placement(anchor, c, VIEW, opts(Placement::TopEnd));
        assert_eq!(e.rect.x, 380.0 + 40.0 - 100.0);
    }

    #[test]
    fn test_bottom_family_below_anchor() {
        let anchor = Rect::new(380.0, 300.0, 40.0, 20.0);
        let r = compute_placement(
            anchor,
            Size::new(100.0, 30.0),
            VIEW,
            opts(Placement::Bottom),
        );
        assert_eq!(r.rect.y, 320.0 + 4.0);
        assert_eq!(r.effective_placement, Placement::Bottom);
    }

    #[test]
    fn test_left_right_vertical_centered() {
        let anchor = Rect::new(380.0, 300.0, 40.0, 20.0);
        let c = Size::new(80.0, 40.0);
        let l = compute_placement(anchor, c, VIEW, opts(Placement::Left));
        // 右缘贴锚点左缘 - offset：380 - 80 - 4 = 296；垂直居中 300 + (20-40)/2 = 290
        assert_eq!(l.rect.x, 296.0);
        assert_eq!(l.rect.y, 290.0);
        let rr = compute_placement(anchor, c, VIEW, opts(Placement::Right));
        assert_eq!(rr.rect.x, 420.0 + 4.0);
        // LeftStart/RightStart 顶对齐锚点
        let rs = compute_placement(anchor, c, VIEW, opts(Placement::RightStart));
        assert_eq!(rs.rect.y, 300.0);
        let re = compute_placement(anchor, c, VIEW, opts(Placement::LeftEnd));
        assert_eq!(re.rect.y, 300.0 + 20.0 - 40.0);
    }

    #[test]
    fn test_flip_when_no_space_and_opposite_richer() {
        // 锚点贴近视窗顶部：上方空间 10 < 需求 34 → 翻转到下方
        let anchor = Rect::new(380.0, 10.0, 40.0, 20.0);
        let r = compute_placement(
            anchor,
            Size::new(100.0, 30.0),
            VIEW,
            PlacementOptions {
                placement: Placement::Top,
                offset: 4.0,
                collision: CollisionPolicy::Flip,
            },
        );
        assert_eq!(r.effective_placement, Placement::Bottom);
        assert_eq!(r.rect.y, 30.0 + 4.0);
    }

    #[test]
    fn test_no_flip_when_opposite_not_richer() {
        // 上下空间相等（各 10）：对侧不比声明侧充裕 → 即使空间不足也不翻转
        let tight = Rect::new(0.0, 0.0, 800.0, 40.0);
        let anchor = Rect::new(380.0, 10.0, 40.0, 20.0);
        let r = compute_placement(
            anchor,
            Size::new(100.0, 30.0),
            tight,
            PlacementOptions {
                placement: Placement::Top,
                offset: 4.0,
                collision: CollisionPolicy::Flip,
            },
        );
        assert_eq!(r.effective_placement, Placement::Top);
        // 对照：collision=None 永不翻转
        let r2 = compute_placement(
            anchor,
            Size::new(100.0, 30.0),
            tight,
            PlacementOptions {
                placement: Placement::Top,
                offset: 4.0,
                collision: CollisionPolicy::None,
            },
        );
        assert_eq!(r2.effective_placement, Placement::Top);
    }

    #[test]
    fn test_flip_then_shift_clamps_into_viewport() {
        // 锚点在顶部，内容超宽：翻转后 x 居中会越出左缘 → 钳回视窗
        let anchor = Rect::new(10.0, 10.0, 40.0, 20.0);
        let r = compute_placement(
            anchor,
            Size::new(600.0, 30.0),
            VIEW,
            PlacementOptions {
                placement: Placement::Top,
                offset: 4.0,
                collision: CollisionPolicy::FlipThenShift,
            },
        );
        assert_eq!(r.effective_placement, Placement::Bottom);
        assert_eq!(r.rect.x, 0.0); // 钳回左缘
        assert!(r.rect.right() <= VIEW.right());
        assert!(r.rect.bottom() <= VIEW.bottom());
    }

    #[test]
    fn test_horizontal_flip_insufficient_side_space() {
        // 锚点贴近左缘：左侧空间 20 < 需求 84 → 翻到右侧
        let anchor = Rect::new(20.0, 300.0, 40.0, 20.0);
        let r = compute_placement(
            anchor,
            Size::new(80.0, 40.0),
            VIEW,
            PlacementOptions {
                placement: Placement::Left,
                offset: 4.0,
                collision: CollisionPolicy::Flip,
            },
        );
        assert_eq!(r.effective_placement, Placement::Right);
        assert_eq!(r.rect.x, 60.0 + 4.0);
    }

    #[test]
    fn test_zero_content_size_is_safe() {
        let anchor = Rect::new(100.0, 100.0, 40.0, 20.0);
        let r = compute_placement(anchor, Size::new(0.0, 0.0), VIEW, opts(Placement::Top));
        assert_eq!(r.rect.width, 0.0);
        assert_eq!(r.rect.height, 0.0);
    }
}
