//! HarUI Form 示例

use har_ui_components::form::{Form, FormItem, FormMessage, FormRule, FormState};
use har_ui_core::Theme;
use iced::widget::{Row, button, column, container, text, text_input};
use iced::{Element, Length, Padding, Task};

pub struct State {
    theme: Theme,
    form: Form,
}

#[derive(Debug, Clone)]
pub enum Message {
    Submit,
    Reset,
    FormMsg(FormMessage),
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Submit => {
            state.form.handle(FormMessage::Validate);
            Task::none()
        }
        Message::Reset => {
            state.form.handle(FormMessage::Reset);
            Task::none()
        }
        Message::FormMsg(msg) => {
            state.form.handle(msg);
            Task::none()
        }
    }
}

fn view(state: &State) -> Element<'_, Message> {
    let theme = &state.theme;
    let form = &state.form;

    let form_view = form.view(theme, move |item| {
        let field = item.field();
        let val = form.value_of(field).map(|s| s.as_str()).unwrap_or("");
        let field_owned = field.to_string();
        let placeholder = match field {
            "username" => "Enter username",
            "password" => "Enter password",
            _ => "",
        };
        let secure = field == "password";
        let ti = text_input(placeholder, val)
            .secure(secure)
            .on_input(move |s| Message::FormMsg(FormMessage::SetValue(field_owned.clone(), s)));
        ti.into()
    });

    let submit_btn = button("Submit")
        .on_press(Message::Submit)
        .padding(Padding::from([8u16, 16u16]));
    let reset_btn = button("Reset")
        .on_press(Message::Reset)
        .padding(Padding::from([8u16, 16u16]));
    let actions = Row::new().push(submit_btn).push(reset_btn).spacing(8);

    let state_label = match state.form.state() {
        FormState::Idle => "Idle",
        FormState::Validating => "Validating",
        FormState::Passed => "Passed",
        FormState::Failed => "Failed",
    };

    let content = column![
        text("HarUI — Form Demo").size(24),
        form_view,
        actions,
        text(format!("Form state: {}", state_label)).size(14),
    ]
    .spacing(16)
    .padding(40);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .into()
}

fn main() -> iced::Result {
    iced::application("HarUI — Form Demo", update, view)
        .window_size(iced::Size::new(600.0, 500.0))
        .run_with(|| {
            let form = Form::new()
                .with_label_width(100)
                .with_item(
                    FormItem::new("username", "Username").with_rule(
                        FormRule::new("username")
                            .required(true)
                            .with_message("Username is required"),
                    ),
                )
                .with_item(
                    FormItem::new("password", "Password").with_rule(
                        FormRule::new("password")
                            .required(true)
                            .with_message("Password is required"),
                    ),
                );

            let state = State {
                theme: Theme::element_light(),
                form,
            };
            (state, Task::none())
        })
}
