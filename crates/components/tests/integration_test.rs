//! T2 集成测试 — 多组件组合场景
//!
//! 场景 1：Form + Button 表单提交（按钮驱动验证）
//! 场景 2：Tabs + Form 多标签页独立表单
//! 场景 3：Dialog + Form 弹窗表单（打开→填写→关闭）
//! 场景 4：Steps + Form 分步向导（3 步推进）
//! 场景 5：Drawer + Form 抽屉表单
//! 场景 6：Upload + Progress 上传+进度联动
//! 场景 7：Cascader + Tag 级联选择+标签展示
//! 场景 8：Alert + Button 警告+按钮联动关闭

use har_ui_components::alert::{Alert, AlertMessage, AlertType};
use har_ui_components::button::{Button, ButtonMessage};
use har_ui_components::cascader::{Cascader, CascaderMessage, CascaderNode};
use har_ui_components::dialog::{Dialog, DialogMessage};
use har_ui_components::drawer::{Drawer, DrawerMessage};
use har_ui_components::form::{Form, FormItem, FormMessage, FormRule};
use har_ui_components::progress::{Progress, ProgressMessage, ProgressStatus};
use har_ui_components::steps::{Step, Steps, StepsMessage};
use har_ui_components::tabs::{TabItem, Tabs, TabsMessage};
use har_ui_components::tag::{Tag, TagType};
use har_ui_components::upload::{Upload, UploadFile, UploadMessage};

#[test]
fn scenario_1_form_button_submit() {
    // 场景：表单填写 → 按钮点击 → 验证通过 → 显示成功 Alert
    let mut rule = FormRule::new("username");
    rule.required = true;
    let mut form = Form::new().with_item(FormItem::new("username", "用户名").with_rule(rule));
    let mut submit_btn = Button::new("提交");
    let alert = Alert::new().with_type(AlertType::Success).with_title("提交成功");

    // 1. 设置字段值
    form.handle(FormMessage::SetValue("username".into(), "张三".into()));
    // 2. 按钮点击
    let emitted = submit_btn.handle(ButtonMessage::Clicked);
    assert_eq!(emitted, ButtonMessage::Clicked);
    // 3. 验证通过
    let errors = form.validate().expect("validate ok");
    assert!(errors.is_empty(), "表单验证应通过");
    // 4. Alert 显示成功
    assert!(alert.visible());
    assert_eq!(alert.alert_type(), AlertType::Success);
}

#[test]
fn scenario_2_tabs_form_independent_state() {
    // 场景：多 Tab 各自独立 Form，切换不丢失数据
    let mut tabs = Tabs::new()
        .with_item(TabItem::new("basic", "基本信息"))
        .with_item(TabItem::new("contact", "联系方式"));
    let mut form_basic = Form::new().with_item(FormItem::new("name", "姓名"));
    let mut form_contact = Form::new().with_item(FormItem::new("phone", "电话"));

    // 1. 在 basic Tab 填写姓名
    form_basic.handle(FormMessage::SetValue("name".into(), "李四".into()));

    // 2. 切换到 contact Tab
    tabs.handle(TabsMessage::Select("contact".into()));
    assert_eq!(tabs.active().unwrap(), "contact");

    // 3. 填写电话
    form_contact.handle(FormMessage::SetValue("phone".into(), "13800138000".into()));

    // 4. 切回 basic Tab，Form 状态独立保留
    tabs.handle(TabsMessage::Select("basic".into()));
    assert_eq!(tabs.active().unwrap(), "basic");
    assert!(form_basic.validate().expect("basic ok").is_empty());
    assert!(form_contact.validate().expect("contact ok").is_empty());
}

#[test]
fn scenario_3_dialog_form_popup() {
    // 场景：打开 Dialog → 填写 Form → 关闭 Dialog
    let mut dialog = Dialog::new("编辑用户", "请填写用户信息");
    let mut form = Form::new().with_item(FormItem::new("email", "邮箱"));

    // 1. 打开 Dialog（Opening → Open 需动画完成）
    dialog.handle(DialogMessage::Open);
    dialog.handle(DialogMessage::AnimationFinished);
    assert!(dialog.is_visible());

    // 2. 填写邮箱
    form.handle(FormMessage::SetValue("email".into(), "test@example.com".into()));
    assert!(form.validate().expect("form ok").is_empty());

    // 3. 关闭 Dialog（Closing → Closed 需动画完成）
    dialog.handle(DialogMessage::Close);
    dialog.handle(DialogMessage::AnimationFinished);
    assert!(!dialog.is_visible());
}

#[test]
fn scenario_4_steps_form_wizard() {
    // 场景：3 步向导，每步填一个 Form 字段
    let mut steps = Steps::new()
        .with_step(Step::new("基本信息"))
        .with_step(Step::new("联系方式"))
        .with_step(Step::new("确认提交"));
    let mut form1 = Form::new().with_item(FormItem::new("name", "姓名"));
    let mut form2 = Form::new().with_item(FormItem::new("phone", "电话"));
    let mut next_btn = Button::new("下一步");

    assert_eq!(steps.current(), 0);

    // Step 1: 填写姓名
    form1.handle(FormMessage::SetValue("name".into(), "王五".into()));
    assert!(form1.validate().expect("f1 ok").is_empty());
    let _ = next_btn.handle(ButtonMessage::Clicked);
    steps.handle(StepsMessage::Next);
    assert_eq!(steps.current(), 1);

    // Step 2: 填写电话
    form2.handle(FormMessage::SetValue("phone".into(), "13900139000".into()));
    assert!(form2.validate().expect("f2 ok").is_empty());
    steps.handle(StepsMessage::Next);
    assert_eq!(steps.current(), 2);

    // Step 3: 完成
    steps.handle(StepsMessage::Finish);
    assert!(steps.is_finished());
}

#[test]
fn scenario_5_drawer_form_slidein() {
    // 场景：抽屉打开 → 填写 Form → 关闭抽屉
    let mut drawer = Drawer::new().with_title("筛选条件");
    let mut form = Form::new().with_item(FormItem::new("keyword", "关键字"));

    drawer.handle(DrawerMessage::Open);
    drawer.handle(DrawerMessage::AnimationEnd);
    assert!(drawer.visible());

    form.handle(FormMessage::SetValue("keyword".into(), "苹果".into()));
    assert!(form.validate().expect("form ok").is_empty());

    drawer.handle(DrawerMessage::Close);
    drawer.handle(DrawerMessage::AnimationEnd);
    assert!(!drawer.visible());
}

#[test]
fn scenario_6_upload_progress_linkage() {
    // 场景：添加文件 → 上传 → 进度推进 → 完成
    let mut upload = Upload::new().with_accept(vec!["jpg".into(), "png".into()]);
    let mut progress = Progress::new();

    // 1. 添加文件
    upload.handle(UploadMessage::AddFile(UploadFile::new("photo.jpg", 1024)));
    assert_eq!(upload.file_list().len(), 1);

    // 2. 开始上传，进度推进
    progress.handle(ProgressMessage::SetPercentage(50));
    assert_eq!(progress.percentage(), 50);

    // 3. 完成上传，Progress 在 100% 时自动转为 Success
    progress.handle(ProgressMessage::SetPercentage(100));
    assert_eq!(progress.percentage(), 100);
    assert_eq!(progress.status(), ProgressStatus::Success);

    // Tag 切换为成功
    let tag = Tag::new("完成").with_type(TagType::Success);
    assert_eq!(tag.tag_type(), TagType::Success);
}

#[test]
fn scenario_7_cascader_tag_display() {
    // 场景：级联选择 → 用 Tag 展示选中路径
    let opts = vec![CascaderNode::new("zhejiang", "浙江").with_children(vec![
        CascaderNode::new("hangzhou", "杭州").with_children(vec![
            CascaderNode::new("xihu", "西湖区"),
        ]),
    ])];
    let mut cascader = Cascader::new().with_options(opts);

    // 1. 打开面板
    cascader.handle(CascaderMessage::TogglePanel);
    assert!(cascader.panel_visible());

    // 2. 选中"西湖区"，面板自动关闭
    cascader.handle(CascaderMessage::Select("xihu".into()));
    assert!(!cascader.panel_visible());
    assert_eq!(cascader.value(), Some("xihu"));
    assert_eq!(cascader.selected_path().len(), 3);

    // 3. 用多个 Tag 展示路径
    let tags: Vec<Tag> = cascader
        .selected_path()
        .iter()
        .map(|p| Tag::new(p.clone()).with_type(TagType::Info))
        .collect();
    assert_eq!(tags.len(), 3);
}

#[test]
fn scenario_8_alert_button_close() {
    // 场景：Alert 显示警告 → 按钮 Click → Alert 关闭
    let mut alert = Alert::new()
        .with_type(AlertType::Warning)
        .with_title("请确认操作")
        .with_closable(true);
    let mut btn = Button::new("知道了");

    assert!(alert.visible());
    assert_eq!(alert.alert_type(), AlertType::Warning);

    // 按钮点击后触发 Alert Close
    let _ = btn.handle(ButtonMessage::Clicked);
    alert.handle(AlertMessage::Close);
    assert!(!alert.visible());

    // 重新打开
    alert.handle(AlertMessage::Open);
    assert!(alert.visible());
}
