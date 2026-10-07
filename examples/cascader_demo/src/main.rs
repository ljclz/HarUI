//! HarUI Cascader 示例

use har_ui_components::cascader::{Cascader, CascaderMessage, CascaderNode};
use har_ui_core::Theme;
use iced::widget::{column, container, text};
use iced::{Element, Length, Task};

pub struct State {
    theme: Theme,
    cascader: Cascader,
}

#[derive(Debug, Clone)]
pub enum Message {
    PathSelected(Vec<String>),
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::PathSelected(path) => {
            if path.as_slice() == ["__toggle__"] {
                state.cascader.handle(CascaderMessage::TogglePanel);
            } else if let Some(last) = path.last() {
                state.cascader.handle(CascaderMessage::Select(last.clone()));
            }
            Task::none()
        }
    }
}

fn view(state: &State) -> Element<'_, Message> {
    let path = state.cascader.selected_path();
    let path_display = if path.is_empty() {
        "（未选择）".to_string()
    } else {
        let mut labels: Vec<String> = Vec::new();
        let mut current: &[CascaderNode] = state.cascader.options();
        for value in path {
            if let Some(node) = current.iter().find(|n| n.value() == value.as_str()) {
                labels.push(node.label().to_string());
                current = node.children();
            } else {
                break;
            }
        }
        labels.join(" / ")
    };
    let status = text(format!("已选路径: {}", path_display)).size(14);

    let cascader_view = state.cascader.view(&state.theme, Message::PathSelected);

    let content = column![
        text("HarUI — Cascader Demo").size(24),
        status,
        cascader_view,
    ]
    .spacing(16)
    .padding(40);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .into()
}

fn build_options() -> Vec<CascaderNode> {
    vec![
        CascaderNode::new("guangdong", "广东").with_children(vec![
            CascaderNode::new("guangzhou", "广州").with_children(vec![
                CascaderNode::new("tianhe", "天河区"),
                CascaderNode::new("yuexiu", "越秀区"),
                CascaderNode::new("haizhu", "海珠区"),
            ]),
            CascaderNode::new("shenzhen", "深圳").with_children(vec![
                CascaderNode::new("nanshan", "南山区"),
                CascaderNode::new("futian", "福田区"),
                CascaderNode::new("luohu", "罗湖区"),
            ]),
        ]),
        CascaderNode::new("zhejiang", "浙江").with_children(vec![
            CascaderNode::new("hangzhou", "杭州").with_children(vec![
                CascaderNode::new("xihu", "西湖区"),
                CascaderNode::new("binjiang", "滨江区"),
            ]),
            CascaderNode::new("ningbo", "宁波").with_children(vec![
                CascaderNode::new("haishu", "海曙区"),
                CascaderNode::new("jiangbei", "江北区"),
            ]),
        ]),
        CascaderNode::new("jiangsu", "江苏").with_children(vec![
            CascaderNode::new("nanjing", "南京").with_children(vec![
                CascaderNode::new("xuanwu", "玄武区"),
                CascaderNode::new("gulou", "鼓楼区"),
            ]),
            CascaderNode::new("suzhou", "苏州").with_children(vec![
                CascaderNode::new("gusu", "姑苏区"),
                CascaderNode::new("wuzhong", "吴中区"),
            ]),
        ]),
    ]
}

fn main() -> iced::Result {
    iced::application(
        || {
            let cascader = Cascader::new().with_options(build_options());
            let state = State {
                theme: Theme::element_light(),
                cascader,
            };
            (state, Task::none())
        },
        update,
        view,
    )
    .title(|_state: &State| String::from("HarUI — Cascader Demo"))
    .window_size(iced::Size::new(900.0, 600.0))
    .run()
}
