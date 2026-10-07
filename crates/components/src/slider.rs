//! Slider 滑块 — 参考 Element Plus `<el-slider>`。
//!
//! 支持：min/max/step、value、disabled、vertical、show_input、show_stops、
//! range、Increase/Decrease、step 吸附、范围钳制与顺序交换。

use har_ui_core::theme::Theme;
use iced::widget::{button, container, text};
use iced::{Color, Element, Length, Padding};

/// Slider 消息
#[derive(Debug, Clone, PartialEq)]
pub enum SliderMessage {
    /// 设置单值
    SetValue(f64),
    /// 设置范围值（range 模式）
    SetRange(f64, f64),
    /// 增加 step
    Increase,
    /// 减少 step
    Decrease,
}

/// Slider 组件
#[derive(Debug, Clone)]
pub struct Slider {
    min: f64,
    max: f64,
    step: f64,
    value: f64,
    range_value: (f64, f64),
    disabled: bool,
    vertical: bool,
    show_input: bool,
    show_stops: bool,
    show_tooltip: bool,
    range: bool,
}

impl Default for Slider {
    fn default() -> Self {
        Self::new()
    }
}

impl Slider {
    pub fn new() -> Self {
        Self {
            min: 0.0,
            max: 100.0,
            step: 1.0,
            value: 0.0,
            range_value: (0.0, 0.0),
            disabled: false,
            vertical: false,
            show_input: false,
            show_stops: false,
            show_tooltip: true,
            range: false,
        }
    }

    pub fn with_min(mut self, v: f64) -> Self {
        self.min = v;
        self
    }

    pub fn with_max(mut self, v: f64) -> Self {
        self.max = v;
        self
    }

    pub fn with_step(mut self, v: f64) -> Self {
        self.step = v;
        self
    }

    pub fn with_disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }

    pub fn with_vertical(mut self, v: bool) -> Self {
        self.vertical = v;
        self
    }

    pub fn with_show_input(mut self, v: bool) -> Self {
        self.show_input = v;
        self
    }

    pub fn with_show_stops(mut self, v: bool) -> Self {
        self.show_stops = v;
        self
    }

    pub fn with_show_tooltip(mut self, v: bool) -> Self {
        self.show_tooltip = v;
        self
    }

    pub fn with_range(mut self, v: bool) -> Self {
        self.range = v;
        self
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn max(&self) -> f64 {
        self.max
    }

    pub fn step(&self) -> f64 {
        self.step
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn range_value(&self) -> (f64, f64) {
        self.range_value
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn vertical(&self) -> bool {
        self.vertical
    }

    pub fn show_input(&self) -> bool {
        self.show_input
    }

    pub fn show_stops(&self) -> bool {
        self.show_stops
    }

    pub fn show_tooltip(&self) -> bool {
        self.show_tooltip
    }

    pub fn range(&self) -> bool {
        self.range
    }

    /// 将原始值钳制到 [min, max] 并吸附到 step 网格
    fn snap(&self, raw: f64) -> f64 {
        let clamped = raw.clamp(self.min, self.max);
        if self.step <= 0.0 {
            return clamped;
        }
        let n = ((clamped - self.min) / self.step).round();
        let snapped = self.min + n * self.step;
        snapped.clamp(self.min, self.max)
    }

    pub fn handle(&mut self, msg: SliderMessage) {
        if self.disabled {
            return;
        }
        match msg {
            SliderMessage::SetValue(v) => {
                self.value = self.snap(v);
            }
            SliderMessage::SetRange(a, b) => {
                let lo = self.snap(a.min(b));
                let hi = self.snap(a.max(b));
                self.range_value = (lo, hi);
            }
            SliderMessage::Increase => {
                self.value = self.snap(self.value + self.step);
            }
            SliderMessage::Decrease => {
                self.value = self.snap(self.value - self.step);
            }
        }
    }

    /// 渲染 Slider 为 iced::Element
    ///
    /// # 参数
    /// - `theme`: HarUI 主题引用
    /// - `on_change`: 值变化回调；参数为新值（f64）
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        on_change: impl Fn(f64) -> Message + 'a,
    ) -> Element<'a, Message> {
        let text_primary = Color::from(theme.neutral.text_primary);
        let text_regular = Color::from(theme.neutral.text_regular);
        let text_disabled = Color::from(theme.neutral.text_disabled);
        let border_lighter = Color::from(theme.neutral.border_lighter);
        let border_base = Color::from(theme.neutral.border_base);
        let primary = Color::from(theme.primary.base);
        let bg_overlay = Color::from(theme.neutral.bg_overlay);

        let span = self.max - self.min;
        let ratio = if span.abs() < f64::EPSILON {
            0.0
        } else {
            (self.value - self.min) / span
        };
        let ratio_clamped = ratio.clamp(0.0, 1.0);
        let portion_left = (ratio_clamped * 100.0).round() as u16;
        let portion_right = 100u16.saturating_sub(portion_left);

        let left_track = container(text(""))
            .width(Length::FillPortion(portion_left))
            .height(Length::Fixed(4.0))
            .style(move |_t| iced::widget::container::Style {
                text_color: None,
                background: Some(iced::Background::Color(primary)),
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
                snap: false,
            });

        let mut handle_btn = button(text(""))
            .width(Length::Fixed(16.0))
            .height(Length::Fixed(16.0))
            .style(move |_t, _status| iced::widget::button::Style {
                background: Some(iced::Background::Color(primary)),
                text_color: primary,
                border: iced::Border {
                    color: border_base,
                    width: 2.0,
                    radius: iced::border::radius(8.0),
                },
                shadow: iced::Shadow::default(),
                snap: false,
            });
        if !self.disabled {
            handle_btn = handle_btn.on_press(on_change(self.value));
        }

        let right_track = container(text(""))
            .width(Length::FillPortion(portion_right))
            .height(Length::Fixed(4.0))
            .style(move |_t| iced::widget::container::Style {
                text_color: None,
                background: Some(iced::Background::Color(border_lighter)),
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
                snap: false,
            });

        let track_row = iced::widget::Row::new()
            .push(left_track)
            .push(handle_btn)
            .push(right_track)
            .align_y(iced::Alignment::Center)
            .spacing(0);

        let track_elem: Element<'a, Message> = if self.show_input {
            let value_text = format!("{}", self.value);
            let value_label = text(value_text).color(text_regular).size(14.0);
            iced::widget::Row::new()
                .push(track_row)
                .push(iced::widget::Space::new().width(Length::Fixed(12.0)))
                .push(value_label)
                .align_y(iced::Alignment::Center)
                .into()
        } else {
            track_row.into()
        };

        let is_disabled = self.disabled;
        container(track_elem)
            .width(Length::Fill)
            .padding(Padding::from([8u16, 12u16]))
            .style(move |_t| iced::widget::container::Style {
                text_color: Some(if is_disabled {
                    text_disabled
                } else {
                    text_primary
                }),
                background: Some(iced::Background::Color(bg_overlay)),
                border: iced::Border {
                    color: if is_disabled {
                        border_base
                    } else {
                        border_lighter
                    },
                    width: 1.0,
                    radius: iced::border::radius(4.0),
                },
                shadow: iced::Shadow::default(),
                snap: false,
            })
            .into()
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_snap_basic() {
        let s = Slider::new().with_min(0.0).with_max(100.0).with_step(10.0);
        assert_eq!(s.snap(23.0), 20.0);
        assert_eq!(s.snap(27.0), 30.0);
    }

    #[test]
    fn test_snap_clamp() {
        let s = Slider::new().with_min(10.0).with_max(50.0).with_step(5.0);
        assert_eq!(s.snap(0.0), 10.0);
        assert_eq!(s.snap(100.0), 50.0);
    }
}
