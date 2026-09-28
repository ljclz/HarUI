//! Progress 进度条 — 参考 Element Plus `<el-progress>`。
//!
//! 支持：百分比钳制、3 种 type（line/circle/dashboard）、status（default/success/exception）、stroke_width、color、show_text。

use har_ui_core::theme::Theme;
use iced::widget::{container, text};
use iced::{Color, Element, Length};

/// 类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProgressType {
    #[default]
    Line,
    Circle,
    Dashboard,
}

/// 状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProgressStatus {
    #[default]
    Default,
    Success,
    Exception,
    Warning,
}

/// Progress 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgressMessage {
    SetPercentage(i32),
    Increment(i32),
    Reset,
}

/// Progress 组件
#[derive(Debug, Clone)]
pub struct Progress {
    picker_type: ProgressType,
    percentage: i32,
    status: ProgressStatus,
    stroke_width: u32,
    show_text: bool,
    color: Option<String>,
    /// 用户显式设置的状态（不为 None 时覆盖自动推导）
    manual_status: Option<ProgressStatus>,
}

impl Default for Progress {
    fn default() -> Self {
        Self::new()
    }
}

impl Progress {
    pub fn new() -> Self {
        Self {
            picker_type: ProgressType::Line,
            percentage: 0,
            status: ProgressStatus::Default,
            stroke_width: 6,
            show_text: true,
            color: None,
            manual_status: None,
        }
    }

    pub fn with_type(mut self, t: ProgressType) -> Self {
        self.picker_type = t;
        self
    }

    pub fn with_status(mut self, s: ProgressStatus) -> Self {
        self.manual_status = Some(s);
        self.status = s;
        self
    }

    pub fn with_stroke_width(mut self, w: u32) -> Self {
        self.stroke_width = w;
        self
    }

    pub fn with_show_text(mut self, v: bool) -> Self {
        self.show_text = v;
        self
    }

    pub fn with_color(mut self, c: impl Into<String>) -> Self {
        self.color = Some(c.into());
        self
    }

    pub fn picker_type(&self) -> ProgressType {
        self.picker_type
    }

    pub fn percentage(&self) -> i32 {
        self.percentage
    }

    pub fn status(&self) -> ProgressStatus {
        self.status
    }

    pub fn stroke_width(&self) -> u32 {
        self.stroke_width
    }

    pub fn show_text(&self) -> bool {
        self.show_text
    }

    pub fn color(&self) -> Option<&str> {
        self.color.as_deref()
    }

    /// 格式化文本（line 类型）
    pub fn formatted_text(&self) -> String {
        format!("{}%", self.percentage)
    }

    fn recompute_status(&mut self) {
        if self.manual_status.is_some() {
            return;
        }
        self.status = if self.percentage >= 100 {
            ProgressStatus::Success
        } else {
            ProgressStatus::Default
        };
    }

    pub fn handle(&mut self, msg: ProgressMessage) {
        match msg {
            ProgressMessage::SetPercentage(pct) => {
                self.percentage = pct.clamp(0, 100);
                self.recompute_status();
            }
            ProgressMessage::Increment(delta) => {
                self.percentage = (self.percentage + delta).clamp(0, 100);
                self.recompute_status();
            }
            ProgressMessage::Reset => {
                self.percentage = 0;
                self.manual_status = None;
                self.status = ProgressStatus::Default;
            }
        }
    }

    /// 按 status 推导主色
    fn status_color(&self, theme: &Theme) -> Color {
        if let Some(c) = self.color.as_deref().and_then(parse_hex) {
            return c;
        }
        match self.status {
            ProgressStatus::Success => Color::from(theme.success.base),
            ProgressStatus::Exception => Color::from(theme.danger.base),
            ProgressStatus::Warning => Color::from(theme.warning.base),
            ProgressStatus::Default => Color::from(theme.primary.base),
        }
    }

    /// 渲染 Progress 为 iced::Element
    pub fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, ()> {
        let main_color = self.status_color(theme);
        let track_color = Color::from(theme.neutral.border_light);
        let text_color = Color::from(theme.neutral.text_regular);
        let pct = self.percentage.clamp(0, 100) as f32 / 100.0;

        match self.picker_type {
            ProgressType::Line => {
                // 横条进度条：外层 track + 内层 fill + 可选文本
                let track_height = self.stroke_width.max(2) as f32;
                let fill_width = Length::FillPortion((pct * 100.0) as u16);

                let fill = container(text(""))
                    .width(fill_width)
                    .height(Length::Fixed(track_height))
                    .style(move |_t| iced::widget::container::Style {
                        text_color: None,
                        background: Some(iced::Background::Color(main_color)),
                        border: iced::Border::default(),
                        shadow: iced::Shadow::default(),
                    });

                let track = container(fill)
                    .width(Length::Fill)
                    .height(Length::Fixed(track_height))
                    .style(move |_t| iced::widget::container::Style {
                        text_color: None,
                        background: Some(iced::Background::Color(track_color)),
                        border: iced::Border {
                            color: Color::TRANSPARENT,
                            width: 0.0,
                            radius: iced::border::radius(track_height / 2.0),
                        },
                        shadow: iced::Shadow::default(),
                    });

                if self.show_text {
                    let label = text(self.formatted_text()).color(text_color).size(14);
                    iced::widget::Row::new()
                        .push(track)
                        .push(iced::widget::Space::with_width(Length::Fixed(8.0)))
                        .push(label)
                        .align_y(iced::Alignment::Center)
                        .into()
                } else {
                    track.into()
                }
            }
            ProgressType::Circle | ProgressType::Dashboard => {
                // 圆形进度：用 Unicode 字符 + 百分比文本占位（iced 0.13 无 canvas 简易）
                // 实际圆形需 canvas widget，这里用文本表示百分比
                let circle_label = if self.show_text {
                    text(self.formatted_text()).color(main_color).size(20)
                } else {
                    text("").size(20)
                };
                container(circle_label)
                    .width(Length::Fixed(80.0))
                    .height(Length::Fixed(80.0))
                    .align_x(iced::alignment::Horizontal::Center)
                    .align_y(iced::alignment::Vertical::Center)
                    .style(move |_t| iced::widget::container::Style {
                        text_color: Some(main_color),
                        background: Some(iced::Background::Color(Color::TRANSPARENT)),
                        border: iced::Border {
                            color: main_color,
                            width: self.stroke_width as f32,
                            radius: iced::border::radius(40.0),
                        },
                        shadow: iced::Shadow::default(),
                    })
                    .into()
            }
        }
    }
}

/// 解析 hex 颜色
fn parse_hex(hex: &str) -> Option<Color> {
    let hex = hex.trim_start_matches('#');
    if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        return Some(Color::from_rgb8(r, g, b)); // HARUI-EXCEPTION: 用户自定义进度色动态转换
    }
    None
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_progress_default_stroke_6() {
        let p = Progress::new();
        assert_eq!(p.stroke_width(), 6);
        assert!(p.show_text());
        assert_eq!(p.picker_type(), ProgressType::Line);
    }
}
