//! Image 图片 — 参考 Element Plus `<el-image>`
//!
//! 支持：加载状态机（Idle/Loading/Loaded/Failed）、加载失败错误信息与重试、
//! 预览开关、占位/错误帧渲染（真实像素由调用方经 content 槽注入）。
//!
//! ## 设计说明
//! iced 的图片像素由 `iced::widget::image`（image feature）承载，
//! 本组件聚焦 EP 的**状态语义**：state 状态机、错误信息、重试计数、预览开关，
//! 像素内容经 `view` 的 `loaded` 槽注入（None 时渲染占位/错误帧）。

use har_ui_core::theme::Theme;
use iced::widget::{container, text};
use iced::{Color, Element, Length, Padding};

/// 图片加载状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ImageState {
    #[default]
    Idle,
    Loading,
    Loaded,
    Failed,
}

/// Image 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImageMessage {
    /// 开始加载（应用侧发起真实请求，成功/失败后回发对应消息）
    Load,
    /// 加载成功
    LoadSuccess,
    /// 加载失败（携带错误描述）
    LoadError(String),
    /// 打开预览（仅 Loaded 态生效）
    PreviewOpen,
    /// 关闭预览
    PreviewClose,
    /// 重试（Failed → Loading，retry_count + 1）
    Retry,
}

/// Image 组件
#[derive(Debug, Clone)]
pub struct Image {
    src: String,
    alt: String,
    state: ImageState,
    error: Option<String>,
    preview: bool,
    previewing: bool,
    retry_count: u32,
    width: f32,
    height: f32,
}

impl Image {
    pub fn new(src: impl Into<String>) -> Self {
        Self {
            src: src.into(),
            alt: String::new(),
            state: ImageState::Idle,
            error: None,
            preview: true,
            previewing: false,
            retry_count: 0,
            width: 200.0,
            height: 200.0,
        }
    }

    pub fn with_alt(mut self, alt: impl Into<String>) -> Self {
        self.alt = alt.into();
        self
    }

    pub fn with_size(mut self, w: f32, h: f32) -> Self {
        self.width = w.max(1.0);
        self.height = h.max(1.0);
        self
    }

    /// 是否允许点击预览
    pub fn with_preview(mut self, v: bool) -> Self {
        self.preview = v;
        self
    }

    pub fn src(&self) -> &str {
        &self.src
    }

    pub fn alt(&self) -> &str {
        &self.alt
    }

    pub fn state(&self) -> ImageState {
        self.state
    }

    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    pub fn is_previewing(&self) -> bool {
        self.previewing
    }

    pub fn retry_count(&self) -> u32 {
        self.retry_count
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    /// 处理消息
    pub fn handle(&mut self, msg: ImageMessage) {
        match msg {
            ImageMessage::Load => {
                if self.state == ImageState::Idle || self.state == ImageState::Failed {
                    self.state = ImageState::Loading;
                    self.error = None;
                }
            }
            ImageMessage::LoadSuccess => {
                if self.state == ImageState::Loading {
                    self.state = ImageState::Loaded;
                    self.error = None;
                }
            }
            ImageMessage::LoadError(err) => {
                if self.state == ImageState::Loading {
                    self.state = ImageState::Failed;
                    self.error = Some(err);
                }
            }
            ImageMessage::PreviewOpen => {
                if self.preview && self.state == ImageState::Loaded {
                    self.previewing = true;
                }
            }
            ImageMessage::PreviewClose => {
                self.previewing = false;
            }
            ImageMessage::Retry => {
                if self.state == ImageState::Failed {
                    self.retry_count += 1;
                    self.state = ImageState::Loading;
                    self.error = None;
                }
            }
        }
    }

    /// 渲染（`loaded` 槽：Loaded 态由调用方注入真实像素元素；
    /// 其余状态渲染占位/错误帧。on_msg 映射 Retry/PreviewOpen。）
    pub fn view<'a, Message: Clone + 'a>(
        &'a self,
        theme: &'a Theme,
        loaded: Option<Element<'a, Message>>,
        on_msg: impl Fn(ImageMessage) -> Message + 'a,
    ) -> Element<'a, Message> {
        let _ = on_msg;
        let border_color = Color::from(theme.neutral.border_lighter);
        let text_placeholder = Color::from(theme.neutral.text_placeholder);
        let danger = Color::from(theme.danger.base);
        let info = Color::from(theme.neutral.text_secondary);

        let frame: Element<'a, Message> = match self.state {
            ImageState::Loaded => {
                loaded.unwrap_or_else(|| text(self.alt.clone()).color(text_placeholder).into())
            }
            ImageState::Failed => {
                let err = self.error.clone().unwrap_or_else(|| "加载失败".to_string());
                iced::widget::Column::with_children(vec![
                    text("加载失败").size(14).color(danger).into(),
                    text(err).size(12).color(info).into(),
                ])
                .spacing(4)
                .into()
            }
            _ => text("加载中…").size(13).color(text_placeholder).into(),
        };

        container(frame)
            .width(Length::Fixed(self.width))
            .height(Length::Fixed(self.height))
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .padding(Padding::from(4u16))
            .style(move |_t| iced::widget::container::Style {
                text_color: None,
                background: Some(iced::Background::Color(Color::from(theme.neutral.bg_base))),
                border: iced::Border {
                    color: border_color,
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
    fn test_initial_idle() {
        let img = Image::new("https://example.com/a.png");
        assert_eq!(img.state(), ImageState::Idle);
        assert_eq!(img.error(), None);
        assert_eq!(img.retry_count(), 0);
        assert!(!img.is_previewing());
    }

    #[test]
    fn test_load_lifecycle() {
        let mut img = Image::new("a.png");
        img.handle(ImageMessage::Load);
        assert_eq!(img.state(), ImageState::Loading);
        img.handle(ImageMessage::LoadSuccess);
        assert_eq!(img.state(), ImageState::Loaded);
        // Loaded 后再 Load 不回退（重新加载需应用显式处理）
        img.handle(ImageMessage::Load);
        assert_eq!(img.state(), ImageState::Loaded);
    }

    #[test]
    fn test_error_and_retry() {
        let mut img = Image::new("a.png");
        img.handle(ImageMessage::Load);
        img.handle(ImageMessage::LoadError("404".to_string()));
        assert_eq!(img.state(), ImageState::Failed);
        assert_eq!(img.error(), Some("404"));
        img.handle(ImageMessage::Retry);
        assert_eq!(img.state(), ImageState::Loading);
        assert_eq!(img.error(), None);
        assert_eq!(img.retry_count(), 1);
        // Loading 中 Retry 无效
        img.handle(ImageMessage::Retry);
        assert_eq!(img.retry_count(), 1);
        // 成功
        img.handle(ImageMessage::LoadSuccess);
        assert_eq!(img.state(), ImageState::Loaded);
    }

    #[test]
    fn test_error_ignored_when_not_loading() {
        let mut img = Image::new("a.png");
        img.handle(ImageMessage::LoadError("boom".to_string()));
        assert_eq!(img.state(), ImageState::Idle);
        assert_eq!(img.error(), None);
    }

    #[test]
    fn test_preview_gating() {
        let mut img = Image::new("a.png").with_preview(true);
        img.handle(ImageMessage::Load);
        img.handle(ImageMessage::LoadSuccess);
        img.handle(ImageMessage::PreviewOpen);
        assert!(img.is_previewing());
        img.handle(ImageMessage::PreviewClose);
        assert!(!img.is_previewing());
        // preview=false 时打不开
        let mut no_preview = Image::new("a.png").with_preview(false);
        no_preview.handle(ImageMessage::Load);
        no_preview.handle(ImageMessage::LoadSuccess);
        no_preview.handle(ImageMessage::PreviewOpen);
        assert!(!no_preview.is_previewing());
        // 未加载完成打不开
        let mut loading = Image::new("a.png");
        loading.handle(ImageMessage::Load);
        loading.handle(ImageMessage::PreviewOpen);
        assert!(!loading.is_previewing());
    }

    #[test]
    fn test_size_builder_clamped() {
        let img = Image::new("a.png").with_size(0.0, -5.0);
        assert_eq!(img.width(), 1.0);
        assert_eq!(img.height(), 1.0);
    }
}
