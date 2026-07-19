//! Loading 组件 — 加载指示器
//!
//! 参考 Element Plus `ElLoading`。
//! 支持：旋转动画、自定义文案、全屏加载、lock 锁定、自定义图标。

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
