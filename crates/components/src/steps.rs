//! Steps 步骤条 — 参考 Element Plus `<el-steps>`。
//!
//! 支持：方向（horizontal/vertical）、simple 模式、当前步骤切换、finish/reset、状态计算。

use har_ui_core::theme::Theme;
use iced::widget::{container, text};
use iced::{Color, Element, Length, Padding};

/// 方向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StepsDirection {
    #[default]
    Horizontal,
    Vertical,
}

/// 步骤状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StepStatus {
    #[default]
    Wait,
    Process,
    Finish,
    Error,
    Success,
}

/// 单个步骤
#[derive(Debug, Clone)]
pub struct Step {
    title: String,
    description: Option<String>,
    icon: Option<String>,
}

impl Step {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            icon: None,
        }
    }

    pub fn with_description(mut self, d: impl Into<String>) -> Self {
        self.description = Some(d.into());
        self
    }

    pub fn with_icon(mut self, i: impl Into<String>) -> Self {
        self.icon = Some(i.into());
        self
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn icon(&self) -> Option<&str> {
        self.icon.as_deref()
    }
}

/// Steps 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepsMessage {
    Next,
    Prev,
    JumpTo(usize),
    Finish,
    Reset,
}

/// Steps 组件
#[derive(Debug, Clone)]
pub struct Steps {
    steps: Vec<Step>,
    current: usize,
    direction: StepsDirection,
    simple: bool,
    finished: bool,
}

impl Default for Steps {
    fn default() -> Self {
        Self::new()
    }
}

impl Steps {
    pub fn new() -> Self {
        Self {
            steps: Vec::new(),
            current: 0,
            direction: StepsDirection::Horizontal,
            simple: false,
            finished: false,
        }
    }

    pub fn with_step(mut self, s: Step) -> Self {
        self.steps.push(s);
        self
    }

    pub fn with_direction(mut self, d: StepsDirection) -> Self {
        self.direction = d;
        self
    }

    pub fn with_simple(mut self, v: bool) -> Self {
        self.simple = v;
        self
    }

    pub fn steps(&self) -> &[Step] {
        &self.steps
    }

    pub fn current(&self) -> usize {
        self.current
    }

    pub fn direction(&self) -> StepsDirection {
        self.direction
    }

    pub fn simple(&self) -> bool {
        self.simple
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }

    /// 计算指定索引步骤的状态
    pub fn step_status(&self, idx: usize) -> StepStatus {
        if self.finished {
            return StepStatus::Success;
        }
        if idx < self.current {
            StepStatus::Finish
        } else if idx == self.current {
            StepStatus::Process
        } else {
            StepStatus::Wait
        }
    }

    pub fn handle(&mut self, msg: StepsMessage) {
        let max = if self.steps.is_empty() { 0 } else { self.steps.len() - 1 };
        match msg {
            StepsMessage::Next => {
                if self.current < max {
                    self.current += 1;
                }
            }
            StepsMessage::Prev => {
                if self.current > 0 {
                    self.current -= 1;
                }
            }
            StepsMessage::JumpTo(idx) => {
                self.current = idx.min(max);
            }
            StepsMessage::Finish => {
                self.finished = true;
            }
            StepsMessage::Reset => {
                self.current = 0;
                self.finished = false;
            }
        }
    }

    /// 渲染 Steps 为 iced::Element
    ///
    /// # 参数
    /// - `theme`: HarUI 主题引用
    pub fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, ()> {
        if self.steps.is_empty() {
            return container(text("")).width(Length::Fill).into();
        }

        let primary = Color::from(theme.primary.base);
        let success = Color::from(theme.success.base);
        let danger = Color::from(theme.danger.base);
        let text_primary = Color::from(theme.neutral.text_primary);
        let text_regular = Color::from(theme.neutral.text_regular);
        let text_secondary = Color::from(theme.neutral.text_secondary);
        let text_disabled = Color::from(theme.neutral.text_disabled);
        let border_light = Color::from(theme.neutral.border_light);

        let mut children: Vec<Element<'a, ()>> = Vec::new();

        for (idx, step) in self.steps.iter().enumerate() {
            let status = self.step_status(idx);

            // 节点颜色
            let node_color = match status {
                StepStatus::Wait => text_disabled,
                StepStatus::Process => primary,
                StepStatus::Finish => success,
                StepStatus::Error => danger,
                StepStatus::Success => success,
            };

            // 节点内容：finish/success 显示 √，error 显示 ×，其他显示数字
            let node_text = match status {
                StepStatus::Finish | StepStatus::Success => "√".to_string(),
                StepStatus::Error => "×".to_string(),
                _ => (idx + 1).to_string(),
            };
            let node_text_color = match status {
                StepStatus::Wait => text_disabled,
                _ => Color::WHITE,
            };

            let node = container(text(node_text).color(node_text_color).size(14.0))
                .width(Length::Fixed(28.0))
                .height(Length::Fixed(28.0))
                .align_x(iced::alignment::Horizontal::Center)
                .align_y(iced::alignment::Vertical::Center)
                .style(move |_t| iced::widget::container::Style {
                    text_color: None,
                    background: Some(iced::Background::Color(node_color)),
                    border: iced::Border {
                        color: node_color,
                        width: 2.0,
                        radius: iced::border::radius(14.0),
                    },
                    shadow: iced::Shadow::default(),
                });

            let title_color = match status {
                StepStatus::Wait => text_regular,
                StepStatus::Process => text_primary,
                _ => text_primary,
            };
            let title_text = text(step.title().to_string()).color(title_color).size(14.0);

            let mut step_col_children: Vec<Element<'a, ()>> = Vec::new();
            step_col_children.push(node.into());
            step_col_children.push(
                iced::widget::Space::with_height(Length::Fixed(4.0)).into(),
            );
            step_col_children.push(title_text.into());
            if let Some(desc) = step.description() {
                step_col_children.push(
                    iced::widget::Space::with_height(Length::Fixed(2.0)).into(),
                );
                step_col_children.push(
                    text(desc.to_string())
                        .color(text_secondary)
                        .size(12.0)
                        .into(),
                );
            }

            let step_col = iced::widget::Column::with_children(step_col_children)
                .align_x(iced::alignment::Horizontal::Center)
                .spacing(0);

            let step_wrap = container(step_col)
                .width(Length::Fill)
                .padding(Padding::from([8u16, 4u16]))
                .style(move |_t| iced::widget::container::Style {
                    text_color: None,
                    background: None,
                    border: iced::Border {
                        color: border_light,
                        width: 0.0,
                        radius: iced::border::radius(0.0),
                    },
                    shadow: iced::Shadow::default(),
                });
            children.push(step_wrap.into());

            // 中间连接线（非最后一个）
            if idx + 1 < self.steps.len() {
                let line_color = if matches!(status, StepStatus::Finish | StepStatus::Success) {
                    success
                } else {
                    border_light
                };
                let line = container(text(""))
                    .width(Length::Fill)
                    .height(Length::Fixed(2.0))
                    .style(move |_t| iced::widget::container::Style {
                        text_color: None,
                        background: Some(iced::Background::Color(line_color)),
                        border: iced::Border::default(),
                        shadow: iced::Shadow::default(),
                    });
                children.push(line.into());
            }
        }

        iced::widget::Row::with_children(children)
            .spacing(0)
            .align_y(iced::Alignment::Center)
            .into()
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_steps_default_no_steps_no_current() {
        let s = Steps::new();
        assert_eq!(s.current(), 0);
        assert_eq!(s.direction(), StepsDirection::Horizontal);
        assert!(!s.simple());
        assert!(!s.is_finished());
    }
}
