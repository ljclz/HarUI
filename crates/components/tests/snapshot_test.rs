//! T3 快照测试 — 状态序列化对比
//!
//! 由于 iced 视觉渲染需 GPU 后端，这里采用"状态序列化快照"方案：
//! 对每个组件的稳定 getter 字段做 format! 快照，对比基线字符串。
//! 任何状态字段变化都会触发快照差异，需人工确认后更新基线。
//!
//! 设计原则：使用稳定的 getter API（而非整个组件的 Debug 输出），
//! 这样内部字段重命名不会触发快照失效，只有公共状态变化才需更新基线。

use har_ui_components::alert::{Alert, AlertType};
use har_ui_components::avatar::Avatar;
use har_ui_components::badge::{Badge, BadgeValue};
use har_ui_components::breadcrumb::{Breadcrumb, BreadcrumbItem};
use har_ui_components::button::{Button, ButtonType};
use har_ui_components::calendar::Calendar;
use har_ui_components::carousel::Carousel;
use har_ui_components::collapse::{Collapse, CollapseItem};
use har_ui_components::divider::Divider;
use har_ui_components::empty::Empty;
use har_ui_components::link::{Link, LinkType};
use har_ui_components::progress::{Progress, ProgressStatus};
use har_ui_components::rate::Rate;
use har_ui_components::skeleton::Skeleton;
use har_ui_components::slider::Slider;
use har_ui_components::steps::{Step, Steps};
use har_ui_components::tag::{Tag, TagType};
use har_ui_components::text::Text;
use har_ui_components::timeline::{Timeline, TimelineItem};
use har_ui_components::upload::{Upload, UploadFile, UploadMessage};

#[test]
fn snap_alert_default() {
    let a = Alert::new();
    let snap = format!(
        "type={:?} title={:?} desc={:?} closable={} center={} show_icon={} visible={}",
        a.alert_type(),
        a.title(),
        a.description(),
        a.closable(),
        a.center(),
        a.show_icon(),
        a.visible(),
    );
    assert_eq!(
        snap,
        "type=Info title=\"\" desc=None closable=true center=false show_icon=true visible=true"
    );
}

#[test]
fn snap_alert_success_with_description() {
    let a = Alert::new()
        .with_type(AlertType::Success)
        .with_title("成功")
        .with_description("操作已完成");
    let snap = format!(
        "type={:?} title={:?} desc={:?} visible={}",
        a.alert_type(),
        a.title(),
        a.description(),
        a.visible(),
    );
    assert_eq!(
        snap,
        "type=Success title=\"成功\" desc=Some(\"操作已完成\") visible=true"
    );
}

#[test]
fn snap_button_primary_disabled() {
    let b = Button::new("提交")
        .with_type(ButtonType::Primary)
        .disabled(true);
    let p = b.props();
    let snap = format!(
        "text={:?} type={:?} size={:?} disabled={} loading={} plain={} round={} circle={}",
        p.text, p.button_type, p.size, p.disabled, p.loading, p.plain, p.round, p.circle,
    );
    assert_eq!(
        snap,
        "text=\"提交\" type=Primary size=Default disabled=true loading=false plain=false round=false circle=false"
    );
}

#[test]
fn snap_tag_success() {
    let t = Tag::new("完成").with_type(TagType::Success);
    let snap = format!(
        "text={:?} type={:?} closable={} hit={}",
        t.text(),
        t.tag_type(),
        t.closable(),
        t.hit(),
    );
    assert_eq!(snap, "text=\"完成\" type=Success closable=false hit=false");
}

#[test]
fn snap_divider_default() {
    let d = Divider::new();
    let snap = format!(
        "direction={:?} content_position={:?} has_text={} dashed={}",
        d.direction(),
        d.content_position(),
        d.text().is_some(),
        d.border_dashed(),
    );
    assert_eq!(
        snap,
        "direction=Horizontal content_position=Center has_text=false dashed=false"
    );
}

#[test]
fn snap_empty_default() {
    let e = Empty::new();
    let snap = format!(
        "description={:?} image={:?} has_extra={}",
        e.description(),
        e.image(),
        e.has_extra(),
    );
    assert_eq!(
        snap,
        "description=\"暂无数据\" image=Default has_extra=false"
    );
}

#[test]
fn snap_skeleton_default() {
    let s = Skeleton::new();
    let snap = format!(
        "items={} animated={} loading={} count={}",
        s.items().len(),
        s.animated(),
        s.loading(),
        s.count(),
    );
    assert_eq!(snap, "items=0 animated=false loading=false count=1");
}

#[test]
fn snap_link_primary_underline() {
    let l = Link::new()
        .with_text("返回")
        .with_type(LinkType::Primary)
        .with_underline(true);
    let snap = format!(
        "text={:?} type={:?} underline={} disabled={}",
        l.text(),
        l.link_type(),
        l.underline(),
        l.disabled(),
    );
    assert_eq!(
        snap,
        "text=\"返回\" type=Primary underline=true disabled=false"
    );
}

#[test]
fn snap_text_default() {
    let t = Text::new().with_content("内容");
    let snap = format!(
        "content={:?} type={:?} size={:?} truncated={} copyable={} max_lines={:?}",
        t.content(),
        t.text_type(),
        t.size(),
        t.truncated(),
        t.copyable(),
        t.max_lines(),
    );
    assert_eq!(
        snap,
        "content=\"内容\" type=Default size=Default truncated=false copyable=false max_lines=None"
    );
}

#[test]
fn snap_progress_default() {
    let p = Progress::new();
    let snap = format!(
        "type={:?} percentage={} status={:?} stroke_width={} show_text={}",
        p.picker_type(),
        p.percentage(),
        p.status(),
        p.stroke_width(),
        p.show_text(),
    );
    assert_eq!(
        snap,
        "type=Line percentage=0 status=Default stroke_width=6 show_text=true"
    );
    // 验证 ProgressStatus::Success 序列化
    let s = format!("{:?}", ProgressStatus::Success);
    assert_eq!(s, "Success");
}

#[test]
fn snap_rate_default() {
    let r = Rate::new();
    let snap = format!(
        "max={} value={} disabled={} allow_half={} show_text={} show_score={} clearable={}",
        r.max(),
        r.value(),
        r.disabled(),
        r.allow_half(),
        r.show_text(),
        r.show_score(),
        r.clearable(),
    );
    assert_eq!(
        snap,
        "max=5 value=0 disabled=false allow_half=false show_text=false show_score=false clearable=false"
    );
}

#[test]
fn snap_slider_default() {
    let s = Slider::new();
    let snap = format!(
        "min={} max={} step={} value={} disabled={} range={}",
        s.min(),
        s.max(),
        s.step(),
        s.value(),
        s.disabled(),
        s.range(),
    );
    assert_eq!(
        snap,
        "min=0 max=100 step=1 value=0 disabled=false range=false"
    );
}

#[test]
fn snap_steps_3_steps() {
    let s = Steps::new()
        .with_step(Step::new("基本信息"))
        .with_step(Step::new("联系方式"))
        .with_step(Step::new("确认"));
    let snap = format!(
        "current={} finished={} steps={}",
        s.current(),
        s.is_finished(),
        s.steps().len(),
    );
    assert_eq!(snap, "current=0 finished=false steps=3");
}

#[test]
fn snap_collapse_2_items() {
    let mut c = Collapse::new().with_accordion(true);
    c.add_item(CollapseItem::new("i1", "标题1"));
    c.add_item(CollapseItem::new("i2", "标题2"));
    let snap = format!(
        "accordion={} items={} active={}",
        c.accordion(),
        c.items().len(),
        c.active_keys().len(),
    );
    assert_eq!(snap, "accordion=true items=2 active=0");
}

#[test]
fn snap_timeline_3_items() {
    let mut t = Timeline::new();
    t.handle(har_ui_components::timeline::TimelineMessage::AddItem(
        TimelineItem::new("t1", "A"),
    ));
    t.handle(har_ui_components::timeline::TimelineMessage::AddItem(
        TimelineItem::new("t2", "B"),
    ));
    t.handle(har_ui_components::timeline::TimelineMessage::AddItem(
        TimelineItem::new("t3", "C"),
    ));
    let snap = format!("items={} reverse={}", t.items().len(), t.reverse());
    assert_eq!(snap, "items=3 reverse=false");
}

#[test]
fn snap_breadcrumb_3_items() {
    let b = Breadcrumb::new()
        .with_item(BreadcrumbItem::new("首页"))
        .with_item(BreadcrumbItem::new("列表"))
        .with_item(BreadcrumbItem::new("详情"));
    let snap = format!("items={}", b.items().len());
    assert_eq!(snap, "items=3");
}

#[test]
fn snap_calendar_2026_07() {
    let c = Calendar::new(2026, 7);
    let snap = format!("year={} month={}", c.year(), c.month());
    assert_eq!(snap, "year=2026 month=7");
}

#[test]
fn snap_carousel_3_slides() {
    let c = Carousel::new()
        .with_autoplay(true)
        .with_interval(3000)
        .with_slide("A")
        .with_slide("B")
        .with_slide("C");
    let snap = format!(
        "slides={} autoplay={} interval={}",
        c.slides().len(),
        c.autoplay(),
        c.interval(),
    );
    assert_eq!(snap, "slides=3 autoplay=true interval=3000");
}

#[test]
fn snap_avatar_default() {
    let a = Avatar::new();
    // Avatar 的 getter 较少，使用 Debug 输出但只断言包含类型名
    let snap = format!("{:?}", a);
    assert!(snap.starts_with("Avatar {"), "snap = {}", snap);
}

#[test]
fn snap_badge_value_5() {
    let b = Badge::new(BadgeValue::Number(5));
    let snap = format!("value={:?}", b.value());
    assert_eq!(snap, "value=Number(5)");
}

#[test]
fn snap_upload_3_files() {
    let mut u = Upload::new().with_multiple(true);
    u.handle(UploadMessage::AddFile(UploadFile::new("a.jpg", 100)));
    u.handle(UploadMessage::AddFile(UploadFile::new("b.jpg", 200)));
    u.handle(UploadMessage::AddFile(UploadFile::new("c.jpg", 300)));
    let snap = format!("files={}", u.file_list().len());
    assert_eq!(snap, "files=3");
}

#[test]
fn snap_collapse_item_disabled() {
    let i = CollapseItem::new("i1", "标题").with_disabled(true);
    let snap = format!(
        "name={:?} title={:?} disabled={} default_active={}",
        i.name(),
        i.title(),
        i.disabled(),
        i.default_active(),
    );
    assert_eq!(
        snap,
        "name=\"i1\" title=\"标题\" disabled=true default_active=false"
    );
}
