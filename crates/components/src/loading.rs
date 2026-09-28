//! Loading 组件 — 加载指示器
//!
//! 参考 Element Plus `El Loading`。
//! 支持：旋转动画、自定义文案、全屏加载、lock 锁定、自定义图标。

use har_ui_core::theme::Theme;
use iced::widget::{container, text};
use iced::{Color, Element, Length};

/// 默认旋转速度（度/毫秒），约 2.25s 一圈
const DEFAULT_ROTATION_SPEED: f32 = 360.0 / 2250.0;

/// Loading 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadingMessage {
    Start,
    Stop,
    Toggle,
    StartWithText(String),
}

/// Loading 组件
#[derive(Debug, Clone)]
pub struct Loading {
    active: bool,
    fullscreen: bool,
    lock: bool,
    text: Option<String>,
    rotation: f32,
}

impl Default for Loading {
    fn default() -> Self {
        Self::new()
    }
}

impl Loading {
    pub fn new() -> Self {
        Self {
            active: false,
            fullscreen: false,
            lock: false,
            text: None,
            rotation: 0.0,
        }
    }

    pub fn with_text(mut self, t: impl Into<String>) -> Self {
        self.text = Some(t.into());
        self
    }

    pub fn with_fullscreen(mut self, v: bool) -> Self {
        self.fullscreen = v;
        if v {
            self.lock = true; // fullscreen 时自动 lock
        }
        self
    }

    pub fn with_lock(mut self, v: bool) -> Self {
        self.lock = v;
        self
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn fullscreen(&self) -> bool {
        self.fullscreen
    }

    pub fn lock(&self) -> bool {
        self.lock
    }

    pub fn text(&self) -> Option<&String> {
        self.text.as_ref()
    }

    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    /// 推进时间（毫秒），更新旋转角度
    pub fn tick(&mut self, ms: u32) {
        if !self.active {
            return;
        }
        self.rotation += DEFAULT_ROTATION_SPEED * (ms as f32);
        // 循环到 [0, 360)
        if self.rotation >= 360.0 {
            self.rotation %= 360.0;
        }
    }

    /// 处理消息
    pub fn handle(&mut self, msg: LoadingMessage) {
        match msg {
            LoadingMessage::Start => {
                self.active = true;
            }
            LoadingMessage::Stop => {
                self.active = false;
                self.rotation = 0.0;
            }
            LoadingMessage::Toggle => {
                if self.active {
                    self.handle(LoadingMessage::Stop);
                } else {
                    self.handle(LoadingMessage::Start);
                }
            }
            LoadingMessage::StartWithText(t) => {
                self.text = Some(t);
                self.active = true;
            }
        }
    }

    /// 渲染 Loading 为 iced::Element
    ///
    /// 当 `active()` 为 true 时，包裹的内容上方叠加遮罩 + 旋转指示器 + 可选加载文字。
    /// 否则直接返回 content。
    ///
    /// # 参数
    /// - `theme`: HarUI 主题引用
    /// - `content`: 被包裹的内容
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        content: Element<'a, Message>,
    ) -> Element<'a, Message> {
        if !self.active {
            return content;
        }

        let primary = Color::from(theme.primary.base);
        let text_color = Color::from(theme.neutral.text_primary);
        // 遮罩取主题 bg_overlay（light #FFFFFF / dark #141414）加透明度，随主题自动切换
        let mask_alpha = if self.fullscreen { 0.9 } else { 0.7 };
        let mask = Color::from(har_ui_core::utils::color_utils::with_alpha(
            theme.neutral.bg_overlay,
            mask_alpha,
        ));

        // 旋转字符（按当前 rotation 取近似符号）
        let spinner = match (self.rotation / 90.0) as u32 % 4 {
            0 => "◜",
            1 => "◝",
            2 => "◞",
            _ => "◟",
        };

        let mut col_children: Vec<Element<'a, Message>> = Vec::new();
        col_children.push(text(spinner.to_string()).color(primary).size(28.0).into());
        if let Some(t) = &self.text {
            col_children.push(iced::widget::Space::with_height(Length::Fixed(8.0)).into());
            col_children.push(text(t.clone()).color(text_color).size(14.0).into());
        }

        let indicator = iced::widget::Column::with_children(col_children)
            .spacing(0)
            .align_x(iced::Alignment::Center);

        let mask_layer = container(indicator)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_t| iced::widget::container::Style {
                text_color: None,
                background: Some(iced::Background::Color(mask)),
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
            });

        // content + 遮罩叠加：使用 Column 让两者堆叠
        iced::widget::Column::new()
            .push(content)
            .push(mask_layer)
            .into()
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_loading_default_inactive() {
        let l = Loading::new();
        assert!(!l.active());
        assert_eq!(l.rotation(), 0.0);
    }

    #[test]
    fn test_loading_fullscreen_auto_lock() {
        let l = Loading::new().with_fullscreen(true);
        assert!(l.fullscreen());
        assert!(l.lock());
    }

    #[test]
    fn test_loading_tick_only_when_active() {
        let mut l = Loading::new();
        l.tick(100);
        assert_eq!(l.rotation(), 0.0);
        l.handle(LoadingMessage::Start);
        l.tick(100);
        assert!(l.rotation() > 0.0);
    }
}
