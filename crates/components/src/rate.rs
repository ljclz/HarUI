//! Rate 评分 — 参考 Element Plus `<el-rate>`。
//!
//! 支持：max、value、disabled、allow_half、increase/decrease、clear、show_text。

use har_ui_core::theme::Theme;
use iced::widget::{button, container, text};
use iced::{Color, Element, Length, Padding};

/// Rate 消息
#[derive(Debug, Clone, PartialEq)]
pub enum RateMessage {
    /// 设置分值（自动按 allow_half 规范化）
    SetValue(f64),
    /// 增加 1 星
    Increase,
    /// 减少 1 星
    Decrease,
    /// 清零
    Clear,
}

/// Rate 组件
#[derive(Debug, Clone)]
pub struct Rate {
    max: u32,
    value: f64,
    disabled: bool,
    allow_half: bool,
    show_text: bool,
    show_score: bool,
    clearable: bool,
}

impl Default for Rate {
    fn default() -> Self {
        Self::new()
    }
}

impl Rate {
    pub fn new() -> Self {
        Self {
            max: 5,
            value: 0.0,
            disabled: false,
            allow_half: false,
            show_text: false,
            show_score: false,
            clearable: false,
        }
    }

    pub fn with_max(mut self, m: u32) -> Self {
        self.max = m;
        self
    }

    pub fn with_disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }

    pub fn with_allow_half(mut self, v: bool) -> Self {
        self.allow_half = v;
        self
    }

    pub fn with_show_text(mut self, v: bool) -> Self {
        self.show_text = v;
        self
    }

    pub fn with_show_score(mut self, v: bool) -> Self {
        self.show_score = v;
        self
    }

    pub fn with_clearable(mut self, v: bool) -> Self {
        self.clearable = v;
        self
    }

    pub fn max(&self) -> u32 {
        self.max
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn allow_half(&self) -> bool {
        self.allow_half
    }

    pub fn show_text(&self) -> bool {
        self.show_text
    }

    pub fn show_score(&self) -> bool {
        self.show_score
    }

    pub fn clearable(&self) -> bool {
        self.clearable
    }

    /// 规范化分值：钳制到 [0, max]，按 allow_half 规则吸附到 0.5 网格
    fn normalize(&self, raw: f64) -> f64 {
        let clamped = raw.clamp(0.0, self.max as f64);
        if self.allow_half {
            // 吸附到最近的 0.5
            (clamped * 2.0).round() / 2.0
        } else {
            // 不允许半星：截断到整数（floor）
            clamped.floor()
        }
    }

    pub fn handle(&mut self, msg: RateMessage) {
        if self.disabled {
            return;
        }
        match msg {
            RateMessage::SetValue(v) => {
                self.value = self.normalize(v);
            }
            RateMessage::Increase => {
                self.value = self.normalize(self.value + 1.0);
            }
            RateMessage::Decrease => {
                self.value = self.normalize(self.value - 1.0);
            }
            RateMessage::Clear => {
                self.value = 0.0;
            }
        }
    }

    /// 渲染 Rate 为 iced::Element
    ///
    /// # 参数
    /// - `theme`: HarUI 主题引用
    /// - `on_change`: 评分变化回调；参数为新分值（u32，已按整数星计算）
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        on_change: impl Fn(u32) -> Message + 'a,
    ) -> Element<'a, Message> {
        let text_primary = Color::from(theme.neutral.text_primary);
        let text_regular = Color::from(theme.neutral.text_regular);
        let text_placeholder = Color::from(theme.neutral.text_placeholder);
        let text_disabled = Color::from(theme.neutral.text_disabled);
        let primary = Color::from(theme.primary.base);
        let bg_overlay = Color::from(theme.neutral.bg_overlay);

        let mut stars: Vec<Element<'a, Message>> = Vec::with_capacity(self.max as usize);
        for i in 1..=self.max {
            let star_char = if self.value >= i as f64 {
                "★"
            } else if self.allow_half && self.value >= (i as f64) - 0.5 {
                "⯨"
            } else {
                "☆"
            };
            let star_color = if self.disabled {
                text_disabled
            } else {
                primary
            };
            let value_to_set = i;

            let mut star_btn = button(text(star_char).color(star_color).size(20.0))
                .padding(Padding::from([2u16, 2u16]))
                .style(move |_t, _status| iced::widget::button::Style {
                    background: None,
                    text_color: star_color,
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                });
            if !self.disabled {
                star_btn = star_btn.on_press(on_change(value_to_set));
            }
            stars.push(star_btn.into());
        }

        let stars_row = iced::widget::Row::with_children(stars)
            .spacing(2)
            .align_y(iced::Alignment::Center);

        // show_text / show_score：右侧显示文本
        let row_elem: Element<'a, Message> = if self.show_text || self.show_score {
            let label = if self.show_score {
                format!("{:.1}", self.value)
            } else {
                // show_text：按 value 映射文本
                match self.value as i32 {
                    0 => "未评分".to_string(),
                    1 => "极差".to_string(),
                    2 => "失望".to_string(),
                    3 => "一般".to_string(),
                    4 => "满意".to_string(),
                    _ => "惊喜".to_string(),
                }
            };
            let label_color = if self.value == 0.0 {
                text_placeholder
            } else {
                text_regular
            };
            iced::widget::Row::new()
                .push(stars_row)
                .push(iced::widget::Space::with_width(Length::Fixed(8.0)))
                .push(text(label).color(label_color).size(14.0))
                .align_y(iced::Alignment::Center)
                .into()
        } else {
            stars_row.into()
        };

        container(row_elem)
            .width(Length::Fill)
            .padding(Padding::from([4u16, 8u16]))
            .style(move |_t| iced::widget::container::Style {
                text_color: Some(text_primary),
                background: Some(iced::Background::Color(bg_overlay)),
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
            })
            .into()
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_normalize_no_half() {
        let r = Rate::new();
        assert_eq!(r.normalize(3.4), 3.0);
        assert_eq!(r.normalize(3.6), 3.0);
    }

    #[test]
    fn test_normalize_with_half() {
        let r = Rate::new().with_allow_half(true);
        assert_eq!(r.normalize(3.2), 3.0);
        assert_eq!(r.normalize(3.3), 3.5);
        assert_eq!(r.normalize(3.7), 3.5);
        assert_eq!(r.normalize(3.8), 4.0);
    }

    #[test]
    fn test_normalize_clamp() {
        let r = Rate::new().with_max(5);
        assert_eq!(r.normalize(10.0), 5.0);
        assert_eq!(r.normalize(-3.0), 0.0);
    }
}
