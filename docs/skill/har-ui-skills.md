# HarUI AI 技能包（LLM-Friendly Reference）

> 面向 AI 编码助手（Cursor / Claude Code 等）与人类新人的单文件参考。
> 生成日期：2026-09-30 ｜ 对应版本：v1.2.0 分支 feat/iced-0.14（iced 0.14）
> API 速查段由 `python scripts/gen_skill_doc.py` 从源码自动生成——**发现与代码不符时重跑脚本，勿手改**。

## 一、项目定位与架构

- **HarUI**：Rust 生态中的 Element Plus，基于 iced 0.13.1 的纯 Rust 桌面组件库（零 Web 依赖），面向 POS 收银与中后台。
- Workspace：`har-ui-core`（主题令牌 + 行为层 `behavior` + devtools + 图标/布局）+ `har-ui-components`（61 个组件）。
- 引用方式：
  ```toml
  [dependencies]
  har-ui-core = { git = "https://github.com/szzmj1980/HarUI", tag = "v1.1.0" }
  har-ui-components = { git = "https://github.com/szzmj1980/HarUI", tag = "v1.1.0" }
  iced = "0.14"
  ```

## 二、组件五段式（所有组件统一遵循）

```rust
pub struct Button { /* State */ }            // 组件 = State 容器
Button::new("文本").with_type(ButtonType::Primary)   // Builder 构造
pub enum ButtonMessage { Clicked, ... }      // 事件
btn.handle(msg);                             // handle: 状态转移（含守卫）
btn.view(&theme, Message::Clicked)           // view: 渲染，泛型映射到应用消息
```

要点：
1. 构造一律 `Xxx::new()` + `with_*` Builder 链；不要直接字面量构造 struct（字段可能增加）。
2. 状态变更只经 `handle(msg)`；view 是纯函数（&self）。
3. `view` 的最后一个参数是把组件 Message 映射到应用 Message 的闭包/变体构造器；
   需要接线交互事件（滚动/拖拽/行点击）的组件（Table 等）用 `view_msg`，渲染-only 用 `view`。
4. 主题：`Theme::element_light()` / `Theme::element_dark()`；颜色一律走 `theme.xxx` 令牌。

## 三、AI 编码规约（硬约束，违反会被 12 关门禁拒绝）

1. 生产代码**禁止** `.unwrap()` / `.expect()` / `unsafe` / `todo!` / `unimplemented!` / `unreachable!`（测试代码允许）。
2. **禁止硬编码颜色**：`Color::from_rgb(...)` 只允许出现在 `theme/style_sheets.rs`；
   组件内一律 `Color::from(theme.xxx)`。确需字面量加 `// HARUI-EXCEPTION: 理由`。
3. clippy `-D warnings` 与 `cargo fmt` 强制通过；组件必须有 `view()`，含 Message 枚举必须有 `handle()`。
4. 架构改动先写 ADR（编号顺延 docs/ADR-009 之后）。
5. 新组件进 showcase 演示 + 补 T1 单测；行为类纯逻辑放 `core::behavior`（零 iced 依赖）。

## 四、常用模式（可编译片段，均已在 demo/测试中验证）

### 4.1 按钮

```rust
let btn = Button::new("结算")
    .with_type(ButtonType::Primary)
    .disabled(order.is_empty());
btn.view(&theme, Message::CheckoutClicked)
```

### 4.2 输入框（含 IME 中文输入）

```rust
let mut input = Input::new().with_placeholder("扫描商品条码");
input.handle(InputMessage::Char('A'));   // IME 组合态由组件内部状态机处理
input.handle(InputMessage::Backspace);
let text = input.value().to_string();
```

### 4.3 表格：虚拟滚动 + 冻结列 + 列宽拖拽（ADR-008）

```rust
let cols = vec![
    TableColumn::new("id", "ID").with_width(60.0).with_fixed(FixedSide::Left),
    TableColumn::new("name", "名称").with_width(200.0).with_resize_bounds(100.0, 400.0),
    TableColumn::new("op", "操作").with_width(100.0).with_fixed(FixedSide::Right),
];
let mut table = Table::new()
    .with_columns(cols)
    .with_rows(rows)                                  // Vec<实现 Identifiable 的行>
    .with_virtual_scroll(VirtualScroll::new(30.0, 240.0))
    .with_horizontal_viewport(680.0);                 // 启用冻结布局
table.handle(TableMessage::Scroll(1500.0));           // 垂直滚动 = 消息驱动
// 交互渲染（横向滚动/拖拽/行点击接线）：
let elem = table.view_msg(&theme, |row, prop| row.get(prop).cloned().unwrap_or_default(), Message::TableMsg);
```

### 4.4 表单校验与提交

```rust
let mut form = Form::new().with_item(
    FormItem::new("member", "会员卡号").set_required(true),  // 项级必填独立生效
);
form.handle(FormMessage::SetValue("member".to_string(), "VIP8888".to_string()));
form.handle(FormMessage::Validate);
let ok = form.state() == FormState::Passed && form.errors().is_empty();
```

### 4.5 对话框生命周期（注意动画确认步）

```rust
dialog.handle(DialogMessage::Open);            // → Opening（已可见）
dialog.handle(DialogMessage::AnimationFinished); // → Open（此时 Escape/遮罩才生效）
dialog.handle(DialogMessage::EscapePressed);   // → Closing
dialog.handle(DialogMessage::AnimationFinished); // → Closed
```

### 4.6 收银键盘 + 现金支付

```rust
let mut keypad = Keypad::new().with_mode(KeypadMode::Price); // ≤2 位小数
for k in [KeypadMessage::Digit(1), KeypadMessage::Digit(2), KeypadMessage::Dot,
          KeypadMessage::Digit(5), KeypadMessage::Digit(0)] { keypad.handle(k); }
let total: f64 = keypad.value().parse().unwrap_or_default();

let mut pay = Payment::new(total);
pay.handle(PaymentMessage::Received(20.0));   // 实收
let change = pay.change();                    // 找零
pay.handle(PaymentMessage::Confirm);
assert!(pay.state() == PaymentState::Confirmed);
```

### 4.7 挂单（HangOrder）

```rust
let mut hang = HangOrder::new();
hang.handle(HangOrderMessage::Hang(HangOrderItem {
    id: "H1".into(), customer_name: "散客".into(), total: 25.0, item_count: 3, timestamp: 100,
}));
let taken = hang.handle(HangOrderMessage::Take("H1".to_string())); // Option<HangOrderItem>
```

### 4.8 主题切换

```rust
state.theme = if state.theme.is_dark {
    Theme::element_light()
} else {
    Theme::element_dark()
};
```

### 4.9 弹层定位引擎（core 行为层，ADR-009）

```rust
use har_ui_core::behavior::overlay::{self, Rect, Size};
// 12 方位 + 空间不足自动翻转 + 钳回视窗
let r = overlay::compute_placement(
    Rect::new(380.0, 300.0, 120.0, 32.0),  // 锚点
    Size::new(200.0, 100.0),               // 气泡内容
    Rect::new(0.0, 0.0, 800.0, 600.0),     // 视窗
    overlay::PlacementOptions { placement: overlay::Placement::Top, offset: 12.0,
                                collision: overlay::CollisionPolicy::FlipThenShift },
);
let (x, y) = (r.rect.x, r.rect.y);
// 下拉面板（下方展开+翻转）与对话框居中有专用便利构造：
// overlay::dropdown_placement(...) / overlay::centered(...)
```

### 4.10 FPS overlay（性能自证）

```rust
// Message::Frame(std::time::Instant)（0.14：boot 闭包初始化，.run() 收尾）
iced::application(
    || (State::default(), Task::none()),
    update,
    view,
)
.title(|_state: &State| String::from("Demo"))
.subscription(|_s| iced::window::frames().map(Message::Frame))
.run()?;
// update 中：state.fps.record(instant);
// view 中（叠放右上角）：state.fps.overlay_view::<Message>(&theme)
```

## 五、组件 API 速查（自动生成）

| 模块 | 主结构体 | Message 变体 | 常用构建器（with_*） |
|------|---------|--------------|---------------------|
| affix | `Affix` | `AffixMessage`: `Scroll` | `with_position(mut self, p: AffixPosition)`<br>`with_offset(mut self, o: u32)`<br>`with_zindex(mut self, z: i32)`<br>`with_target(mut self, t: impl Into<String>)` |
| alert | `Alert` | `AlertMessage`: `Close` · `Open` | `with_type(mut self, t: AlertType)`<br>`with_title(mut self, t: impl Into<String>)`<br>`with_description(mut self, d: impl Into<String>)`<br>`with_closable(mut self, v: bool)`<br>`with_center(mut self, v: bool)`<br>`with_show_icon(mut self, v: bool)`<br>`with_effect(mut self, e: AlertEffect)` |
| avatar | `Avatar` | —（静态展示） | `with_text(mut self, t: impl Into<String>)`<br>`with_image_url(mut self, url: impl Into<String>)`<br>`with_icon(mut self, i: impl Into<String>)`<br>`with_size(mut self, s: AvatarSize)`<br>`with_pixel_size(mut self, s: u32)`<br>`with_shape(mut self, s: AvatarShape)`<br>`with_fit(mut self, f: AvatarFit)`<br>`with_fallback_text(mut self, t: impl Into<String>)`<br>…共 9 个 |
| backtop | `Backtop` | `BacktopMessage`: `Scroll` · `Click` · `SetVisibilityHeight` · `SetSmooth` | `with_visibility_height(mut self, v: u32)`<br>`with_right(mut self, v: u32)`<br>`with_bottom(mut self, v: u32)`<br>`with_smooth(mut self, v: bool)` |
| badge | `Badge` | `BadgeMessage`: `Update` · `SetHidden` | `with_max(mut self, m: u32)`<br>`with_is_dot(mut self, v: bool)`<br>`with_position(mut self, p: BadgePosition)` |
| breadcrumb | `BreadcrumbItem` | `BreadcrumbMessage`: `Click` · `Replace` · `ClearClick` | `with_to(mut self, to: impl Into<String>)`<br>`with_icon(mut self, icon: impl Into<String>)`<br>`with_separator(mut self, s: impl Into<String>)`<br>`with_item(mut self, item: BreadcrumbItem)` |
| button | `ButtonProps` | `ButtonMessage`: `Clicked` · `Hovered` · `Unhovered` · `Pressed` · `Released` · `NoChange` | `with_type(mut self, t: ButtonType)`<br>`with_size(mut self, s: ButtonSize)` |
| calendar | `SimpleDate` | `CalendarMessage`: `NextMonth` · `PrevMonth` · `Today` · `SelectDate` | `with_today(mut self, d: SimpleDate)`<br>`with_range(mut self, start: SimpleDate, end: SimpleDate)` |
| card | `Card` | `CardMessage`: `Hovered` · `Unhovered` · `UpdateBody` | `with_header(mut self, h: impl Into<String>)`<br>`with_footer(mut self, f: impl Into<String>)`<br>`with_image(mut self, src: impl Into<String>)`<br>`with_shadow(mut self, s: CardShadow)` |
| carousel | `Carousel` | `CarouselMessage`: `Next` · `Prev` · `JumpTo` | `with_slide(mut self, s: impl Into<String>)`<br>`with_autoplay(mut self, v: bool)`<br>`with_interval(mut self, ms: u64)`<br>`with_loop(mut self, v: bool)`<br>`with_direction(mut self, d: CarouselDirection)`<br>`with_show_indicator(mut self, v: bool)`<br>`with_show_arrow(mut self, v: bool)`<br>`with_pause_on_hover(mut self, v: bool)` |
| cascader | `CascaderNode` | `CascaderMessage`: `Select` · `Clear` · `TogglePanel` | `with_disabled(mut self, v: bool)`<br>`with_children(mut self, children: Vec<CascaderNode>)`<br>`with_options(mut self, opts: Vec<CascaderNode>)`<br>`with_emit_path(mut self, v: bool)`<br>`with_check_strictly(mut self, v: bool)`<br>`with_expand_trigger(mut self, t: ExpandTrigger)` |
| checkbox | `Checkbox` | `CheckboxMessage`: `Toggle` · `SetChecked` · `ToggleValue` · `SelectAll` · `ClearAll` | `with_checked(mut self, c: bool)`<br>`with_disabled(mut self, d: bool)`<br>`with_indeterminate(mut self, i: bool)`<br>`with_size(mut self, s: CheckboxSize)`<br>`with_border(mut self, b: bool)`<br>`with_true_value(mut self, v: &str)`<br>`with_false_value(mut self, v: &str)`<br>`with_value(mut self, vals: Vec<&str>)`<br>…共 11 个 |
| collapse | `CollapseItem` | `CollapseMessage`: `Toggle` · `Open` · `Close` · `OpenAll` · `CloseAll` | `with_disabled(mut self, v: bool)`<br>`with_default_active(mut self, v: bool)`<br>`with_accordion(mut self, v: bool)` |
| color_picker | `ColorPicker` | `ColorPickerMessage`: `SetColor` · `SetAlpha` · `TogglePanel` · `Clear` · `SelectPredefine` | `with_show_alpha(mut self, v: bool)`<br>`with_disabled(mut self, v: bool)`<br>`with_format(mut self, f: ColorFormat)`<br>`with_predefine(mut self, p: Vec<String>)` |
| customer_display | `CustomerDisplay` | `CustomerDisplayMessage`: `ShowAmount` · `ShowQrCode` · `ShowCustomMessage` · `ShowSuccess` · `ShowWelcome` · `Reset` | — |
| date_picker | `SimpleDate` | `DatePickerMessage`: `Open` · `Close` · `SelectDate` · `SelectDateTime` · `SelectRangeStart` · `SelectRangeEnd` · `Clear` · `PrevMonth` · `NextMonth` · `PrevYear` · `NextYear` · `JumpTo` | `with_type(mut self, t: DatePickerType)`<br>`with_disabled(mut self, v: bool)`<br>`with_clearable(mut self, v: bool)`<br>`with_placeholder(mut self, p: impl Into<String>)`<br>`with_today(mut self, d: SimpleDate)` |
| descriptions | `DescriptionsItem` | —（静态展示） | `with_span(mut self, s: u32)`<br>`with_title(mut self, t: impl Into<String>)`<br>`with_column(mut self, c: u32)`<br>`with_border(mut self, b: bool)`<br>`with_direction(mut self, d: DescriptionsDirection)`<br>`with_item(mut self, item: DescriptionsItem)`<br>`with_has_extra(mut self, v: bool)` |
| dialog | `DialogProps` | `DialogMessage`: `Open` · `Close` · `AnimationFinished` · `OverlayClicked` · `EscapePressed` · `Dragged` | `with_close_on_click_modal(mut self, v: bool)`<br>`with_close_on_press_escape(mut self, v: bool)`<br>`with_fullscreen(mut self, v: bool)`<br>`with_draggable(mut self, v: bool)`<br>`with_props(mut self, props: DialogProps)` |
| divider | `Divider` | —（静态展示） | `with_direction(mut self, d: DividerDirection)`<br>`with_content_position(mut self, p: DividerContentPosition)`<br>`with_text(mut self, t: impl Into<String>)`<br>`with_border_dashed(mut self, v: bool)` |
| drawer | `Drawer` | `DrawerMessage`: `Open` · `Close` · `AnimationEnd` · `ClickModal` · `PressEscape` · `ClickClose` | `with_title(mut self, t: &str)`<br>`with_direction(mut self, d: DrawerDirection)`<br>`with_size(mut self, s: &str)`<br>`with_show_close(mut self, s: bool)`<br>`with_modal(mut self, m: bool)`<br>`with_close_on_click_modal(mut self, c: bool)`<br>`with_close_on_press_escape(mut self, c: bool)`<br>`with_destroy_on_close(mut self, d: bool)` |
| dropdown | `DropdownItem` | `DropdownMessage`: `Show` · `Hide` · `Click` · `ClickOutside` · `ContextMenu` · `MouseEnter` · `MouseLeave` · `Select` · `ClearCommand` | `with_disabled(mut self, v: bool)`<br>`with_divided(mut self, v: bool)`<br>`with_item(mut self, item: DropdownItem)`<br>`with_trigger(mut self, t: DropdownTrigger)`<br>`with_hide_on_click(mut self, v: bool)` |
| empty | `Empty` | —（静态展示） | `with_description(mut self, d: impl Into<String>)`<br>`with_image(mut self, img: EmptyImage)`<br>`with_image_url(mut self, url: impl Into<String>)`<br>`with_size(mut self, s: EmptySize)`<br>`with_has_extra(mut self, v: bool)` |
| form | `FormRule` | `FormMessage`: `Validate` · `ValidateField` · `Reset` · `SetValue` | `with_min(mut self, m: usize)`<br>`with_max(mut self, m: usize)`<br>`with_message(mut self, m: impl Into<String>)`<br>`with_trigger(mut self, t: ValidateTrigger)`<br>`with_validator(mut self, v: Box<dyn Fn(&str) -> bool + Send + Sync>)`<br>`with_rule(mut self, r: FormRule)`<br>`with_inline(mut self, v: bool)`<br>`with_label_position(mut self, p: impl Into<String>)`<br>…共 10 个 |
| grid | `GridProps` | `GridMessage`: `Select` · `ClearSelection` | `with_columns(mut self, c: usize)`<br>`with_gap(mut self, g: f32)`<br>`with_card_width(mut self, w: f32)`<br>`with_items(mut self, items: Vec<T>)`<br>`with_props(mut self, props: GridProps)` |
| hang_order | `HangOrderItem` | `HangOrderMessage`: `Hang` · `Take` · `Delete` · `ClearAll` | — |
| input | `Input` | `InputMessage`: `Char` · `Backspace` · `Clear` · `Focused` · `Blurred` · `ImeEvent` | `with_mode(mut self, mode: InputMode)`<br>`with_placeholder(mut self, p: impl Into<String>)`<br>`with_clearable(mut self, v: bool)`<br>`with_maxlength(mut self, n: usize)`<br>`with_prefix(mut self, p: impl Into<String>)`<br>`with_suffix(mut self, s: impl Into<String>)` |
| input_number | `InputNumber` | `InputNumberMessage`: `Increment` · `Decrement` · `SetValue` | `with_min(mut self, v: f64)`<br>`with_max(mut self, v: f64)`<br>`with_step(mut self, v: f64)`<br>`with_precision(mut self, p: u8)`<br>`with_controls_position(mut self, p: ControlsPosition)` |
| keypad | `Keypad` | `KeypadMessage`: `Digit` · `DoubleZero` · `Dot` · `Backspace` · `Clear` · `Ok` | `with_mode(mut self, m: KeypadMode)`<br>`with_max(mut self, v: f64)`<br>`with_min(mut self, v: f64)` |
| link | `Link` | `LinkMessage`: `Click` · `ResetClick` | `with_type(mut self, t: LinkType)`<br>`with_text(mut self, t: impl Into<String>)`<br>`with_href(mut self, h: impl Into<String>)`<br>`with_underline(mut self, v: bool)`<br>`with_disabled(mut self, v: bool)`<br>`with_icon(mut self, i: impl Into<String>)` |
| loading | `Loading` | `LoadingMessage`: `Start` · `Stop` · `Toggle` · `StartWithText` | `with_text(mut self, t: impl Into<String>)`<br>`with_fullscreen(mut self, v: bool)`<br>`with_lock(mut self, v: bool)` |
| menu | `MenuItem` | `MenuMessage`: `Select` · `ToggleSubmenu` · `CloseOthers` · `Collapse` | `with_child(mut self, child: MenuItem)`<br>`with_mode(mut self, m: MenuMode)`<br>`with_collapse(mut self, v: bool)`<br>`with_unique_opened(mut self, v: bool)` |
| message | `MessageItem` | —（静态展示） | `with_duration(mut self, ms: u64)` |
| message_box | `MessageBox` | `MessageBoxMessage`: `Open` · `Close` · `Confirm` · `Cancel` · `ClickModal` · `Input` | `with_type(mut self, t: MessageBoxType)`<br>`with_show_confirm(mut self, s: bool)`<br>`with_show_cancel(mut self, s: bool)`<br>`with_show_close(mut self, s: bool)`<br>`with_center(mut self, c: bool)`<br>`with_close_on_click_modal(mut self, c: bool)`<br>`with_confirm_text(mut self, t: &str)`<br>`with_cancel_text(mut self, t: &str)` |
| notification | `Notification` | `NotificationMessage`: `Close` | `with_type(mut self, t: NotificationType)`<br>`with_duration(mut self, d: u64)`<br>`with_position(mut self, p: NotificationPosition)`<br>`with_show_close(mut self, s: bool)`<br>`with_offset(mut self, o: u32)` |
| page_header | `PageHeader` | `PageHeaderMessage`: `Back` · `ResetBack` | `with_title(mut self, t: impl Into<String>)`<br>`with_subtitle(mut self, s: impl Into<String>)`<br>`with_content(mut self, c: impl Into<String>)`<br>`with_icon(mut self, i: impl Into<String>)`<br>`with_has_extra(mut self, v: bool)` |
| pagination | `Pagination` | `PaginationMessage`: `Next` · `Prev` · `JumpTo` · `ChangePageSize` | `with_page_sizes(mut self, sizes: Vec<u32>)` |
| payment | `Payment` | `PaymentMessage`: `Received` · `SwitchMethod` · `UpdateTotal` · `Confirm` · `Cancel` | `with_method(mut self, m: PaymentMethod)` |
| popconfirm | `Popconfirm` | `PopconfirmMessage`: `Click` · `MouseEnter` · `MouseLeave` · `Focus` · `Blur` · `Show` · `Hide` · `Confirm` · `Cancel` · `ClickOutside` | `with_visible(mut self, v: bool)`<br>`with_trigger(mut self, t: PopconfirmTrigger)`<br>`with_placement(mut self, p: PopconfirmPlacement)`<br>`with_width(mut self, w: u32)`<br>`with_show_arrow(mut self, s: bool)`<br>`with_disabled(mut self, d: bool)`<br>`with_confirm_text(mut self, t: &str)`<br>`with_cancel_text(mut self, t: &str)` |
| popover | `Popover` | `PopoverMessage`: `Click` · `MouseEnter` · `MouseLeave` · `Focus` · `Blur` · `Show` · `Hide` · `ClickOutside` | `with_visible(mut self, v: bool)`<br>`with_trigger(mut self, t: PopoverTrigger)`<br>`with_placement(mut self, p: PopoverPlacement)`<br>`with_title(mut self, t: &str)`<br>`with_content(mut self, c: &str)`<br>`with_width(mut self, w: u32)`<br>`with_show_arrow(mut self, s: bool)`<br>`with_disabled(mut self, d: bool)` |
| progress | `Progress` | `ProgressMessage`: `SetPercentage` · `Increment` · `Reset` | `with_type(mut self, t: ProgressType)`<br>`with_status(mut self, s: ProgressStatus)`<br>`with_stroke_width(mut self, w: u32)`<br>`with_show_text(mut self, v: bool)`<br>`with_color(mut self, c: impl Into<String>)` |
| radio | `Radio` | `RadioMessage`: `SetChecked` · `SetValue` · `Clear` · `ToggleOption` | `with_checked(mut self, c: bool)`<br>`with_disabled(mut self, d: bool)`<br>`with_size(mut self, s: RadioSize)`<br>`with_border(mut self, b: bool)`<br>`with_value(mut self, v: &str)`<br>`with_disabled(mut self, d: bool)`<br>`with_size(mut self, s: RadioSize)` |
| rate | `Rate` | `RateMessage`: `SetValue` · `Increase` · `Decrease` · `Clear` | `with_max(mut self, m: u32)`<br>`with_disabled(mut self, v: bool)`<br>`with_allow_half(mut self, v: bool)`<br>`with_show_text(mut self, v: bool)`<br>`with_show_score(mut self, v: bool)`<br>`with_clearable(mut self, v: bool)` |
| result_page | `ResultPage` | —（静态展示） | `with_type(mut self, t: ResultType)`<br>`with_title(mut self, t: impl Into<String>)`<br>`with_sub_title(mut self, s: impl Into<String>)`<br>`with_icon_url(mut self, url: impl Into<String>)`<br>`with_has_extra(mut self, v: bool)` |
| scrollbar | `Scrollbar` | `ScrollbarMessage`: `ScrollTo` · `ScrollBy` · `Reset` | `with_height(mut self, h: u32)`<br>`with_max_height(mut self, h: u32)`<br>`with_always_visible(mut self, v: bool)`<br>`with_native(mut self, v: bool)`<br>`with_max_scroll(mut self, max_x: u32, max_y: u32)` |
| select | `SelectOption` | `SelectMessage`: `Open` · `Close` · `Choose` · `Clear` · `Query` | `with_option(mut self, opt: SelectOption)`<br>`with_multiple(mut self, v: bool)`<br>`with_filterable(mut self, v: bool)`<br>`with_clearable(mut self, v: bool)`<br>`with_disabled(mut self, v: bool)` |
| skeleton | `SkeletonItem` | —（静态展示） | `with_width(mut self, w: impl Into<String>)`<br>`with_height(mut self, h: impl Into<String>)`<br>`with_item(mut self, item: SkeletonItem)`<br>`with_template(mut self, items: Vec<SkeletonItem>)`<br>`with_animated(mut self, v: bool)`<br>`with_loading(mut self, v: bool)`<br>`with_count(mut self, c: u32)` |
| slider | `Slider` | `SliderMessage`: `SetValue` · `SetRange` · `Increase` · `Decrease` | `with_min(mut self, v: f64)`<br>`with_max(mut self, v: f64)`<br>`with_step(mut self, v: f64)`<br>`with_disabled(mut self, v: bool)`<br>`with_vertical(mut self, v: bool)`<br>`with_show_input(mut self, v: bool)`<br>`with_show_stops(mut self, v: bool)`<br>`with_show_tooltip(mut self, v: bool)`<br>…共 9 个 |
| space | `Space` | `SpaceMessage`: `AddItem` · `RemoveItem` · `Clear` | `with_direction(mut self, d: SpaceDirection)`<br>`with_size(mut self, s: u32)`<br>`with_wrap(mut self, v: bool)`<br>`with_fill(mut self, v: bool)`<br>`with_alignment(mut self, a: SpaceAlignment)` |
| statistic | `Statistic` | —（静态展示） | `with_title(mut self, t: impl Into<String>)`<br>`with_prefix(mut self, p: impl Into<String>)`<br>`with_suffix(mut self, s: impl Into<String>)`<br>`with_precision(mut self, p: u32)`<br>`with_grouping(mut self, g: bool)`<br>`with_value_color(mut self, c: Color)` |
| status_bar | `StatusItem` | `StatusBarMessage`: `UpdateLeft` · `UpdateRight` · `SetHardwareStatus` · `ClearAll` | `with_detail(mut self, d: impl Into<String>)`<br>`with_left_item(mut self, item: StatusItem)`<br>`with_right_item(mut self, item: StatusItem)` |
| steps | `Step` | `StepsMessage`: `Next` · `Prev` · `JumpTo` · `Finish` · `Reset` | `with_description(mut self, d: impl Into<String>)`<br>`with_icon(mut self, i: impl Into<String>)`<br>`with_step(mut self, s: Step)`<br>`with_direction(mut self, d: StepsDirection)`<br>`with_simple(mut self, v: bool)` |
| switch | `Switch` | `SwitchMessage`: `Toggle` · `SetValue` | `with_value(initial: bool)`<br>`with_width(mut self, w: u32)`<br>`with_disabled(mut self, d: bool)`<br>`with_loading(mut self, l: bool)`<br>`with_active_color(mut self, c: &str)`<br>`with_inactive_color(mut self, c: &str)`<br>`with_active_text(mut self, t: &str)`<br>`with_inactive_text(mut self, t: &str)`<br>…共 10 个 |
| table | `TableColumn` | `TableMessage`: `SortBy` · `RowClicked` · `ClearSelection` · `Scroll` · `SetViewportHeight` · `ScrollX` · `ResizeColumn` · `ResizeStart` · `ResizeMove` · `ResizeEnd` | `with_width(mut self, w: f32)`<br>`with_fixed(mut self, side: FixedSide)`<br>`with_resize_bounds(mut self, min: f32, max: f32)`<br>`with_stripe(mut self, v: bool)`<br>`with_border(mut self, v: bool)`<br>`with_empty_text(mut self, s: impl Into<String>)`<br>`with_columns(mut self, cols: Vec<TableColumn>)`<br>`with_rows(mut self, rows: Vec<R>)`<br>…共 12 个 |
| tabs | `TabItem` | `TabsMessage`: `Select` · `Close` · `Add` | `with_type(mut self, t: TabsType)`<br>`with_position(mut self, p: TabPosition)`<br>`with_closable(mut self, v: bool)`<br>`with_addable(mut self, v: bool)`<br>`with_lazy(mut self, v: bool)`<br>`with_item(mut self, item: TabItem)` |
| tag | `Tag` | `TagMessage`: `Close` · `UpdateText` | `with_type(mut self, t: TagType)`<br>`with_effect(mut self, e: TagEffect)`<br>`with_size(mut self, s: TagSize)`<br>`with_closable(mut self, v: bool)`<br>`with_hit(mut self, v: bool)`<br>`with_color(mut self, c: impl Into<String>)` |
| text | `Text` | —（静态展示） | `with_type(mut self, t: TextType)`<br>`with_size(mut self, s: TextSize)`<br>`with_content(mut self, c: impl Into<String>)`<br>`with_truncated(mut self, v: bool)`<br>`with_tag(mut self, t: TextTag)`<br>`with_copyable(mut self, v: bool)`<br>`with_max_lines(mut self, n: u32)` |
| time_picker | `TimeValue` | `TimePickerMessage`: `Open` · `Close` · `Select` · `SelectStart` · `SelectEnd` · `Clear` · `IncHour` · `DecHour` · `IncMinute` · `DecMinute` · `IncSecond` · `DecSecond` | `with_disabled(mut self, v: bool)`<br>`with_clearable(mut self, v: bool)`<br>`with_range(mut self, v: bool)`<br>`with_format(mut self, f: impl Into<String>)`<br>`with_placeholder(mut self, p: impl Into<String>)` |
| timeline | `TimelineItem` | `TimelineMessage`: `AddItem` · `RemoveAt` · `Clear` · `SetReverse` | `with_type(mut self, t: TimelineItemType)`<br>`with_color(mut self, c: impl Into<String>)`<br>`with_size(mut self, s: TimelineItemSize)`<br>`with_placement(mut self, p: TimelineItemPlacement)`<br>`with_hollow(mut self, v: bool)`<br>`with_reverse(mut self, v: bool)` |
| tooltip | `Tooltip` | `TooltipMessage`: `Click` · `MouseEnter` · `MouseLeave` · `Focus` · `Blur` · `Show` · `Hide` · `EnterTooltip` · `LeaveTooltip` | `with_visible(mut self, v: bool)`<br>`with_trigger(mut self, t: TooltipTrigger)`<br>`with_placement(mut self, p: TooltipPlacement)`<br>`with_effect(mut self, e: TooltipEffect)`<br>`with_show_arrow(mut self, s: bool)`<br>`with_enterable(mut self, e: bool)`<br>`with_disabled(mut self, d: bool)`<br>`with_hide_after(mut self, ms: u64)` |
| tree | `TreeNode` | `TreeMessage`: `ToggleExpand` · `ToggleCheck` · `Filter` · `LoadChildren` · `SetData` | `with_child(mut self, child: TreeNode)`<br>`with_disabled(mut self, v: bool)`<br>`with_extra(mut self, key: impl Into<String>, value: impl Into<String>)`<br>`with_data(mut self, roots: Vec<TreeNode>)`<br>`with_show_checkbox(mut self, v: bool)`<br>`with_default_expand_all(mut self, v: bool)` |
| upload | `UploadFile` | `UploadMessage`: `AddFile` · `Drop` · `Remove` · `Clear` · `StartUpload` · `Progress` · `UploadSuccess` · `UploadError` | `with_multiple(mut self, v: bool)`<br>`with_limit(mut self, l: usize)`<br>`with_accept(mut self, a: Vec<String>)` |


---

## 六、验收提示（给 AI 会话）

用本文件写示例时：只使用上表出现的类型/方法名；拿不准的 API 用 `grep -n "pub fn" crates/components/src/<组件>.rs` 原地核对。
