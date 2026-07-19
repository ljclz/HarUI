//! HarUI 图标系统 — 内嵌 SVG
//!
//! 设计原则：
//! - 不依赖外部图标文件，减少依赖和体积
//! - SVG data 在编译期内嵌
//! - 中后台 + POS 共 50 个常用图标
//!
//! 图标来源：参考 @element-plus/icons-vue 的 SVG path 数据
//! 为降低实现复杂度，本模块手绘了简化版本的 SVG path。

/// 图标名称枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IconName {
    // 数值操作
    Plus,
    Minus,
    Delete,
    Edit,
    // 功能操作
    Search,
    Refresh,
    Print,
    Setting,
    // 用户/会员
    User,
    // 支付方式
    WeChat,
    Alipay,
    UnionPay,
    Cash,
    BankCard,
    Coupon,
    // 订单
    HangOrder,
    TakeOrder,
    Clear,
    // 导航
    Back,
    Close,
    Expand,
    Fold,
    // 状态
    Success,
    Warning,
    Error,
    Info,
    Loading,
    More,
    Upload,
    // 媒体
    Image,
    File,
    // 数据
    Data,
    Statistic,
    // 时间
    Calendar,
    Clock,
    Filter,
    // 方向
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowDown,
}

impl IconName {
    /// 返回所有图标名称（用于测试和遍历）
    pub fn all() -> Vec<IconName> {
        vec![
            IconName::Plus, IconName::Minus, IconName::Delete, IconName::Edit,
            IconName::Search, IconName::Refresh, IconName::Print, IconName::Setting,
            IconName::User,
            IconName::WeChat, IconName::Alipay, IconName::UnionPay,
            IconName::Cash, IconName::BankCard, IconName::Coupon,
            IconName::HangOrder, IconName::TakeOrder, IconName::Clear,
            IconName::Back, IconName::Close, IconName::Expand, IconName::Fold,
            IconName::Success, IconName::Warning, IconName::Error, IconName::Info,
            IconName::Loading, IconName::More, IconName::Upload,
            IconName::Image, IconName::File,
            IconName::Data, IconName::Statistic,
            IconName::Calendar, IconName::Clock, IconName::Filter,
            IconName::ArrowLeft, IconName::ArrowRight, IconName::ArrowUp, IconName::ArrowDown,
        ]
    }
}

/// 图标组件（Builder 模式）
#[derive(Debug, Clone)]
pub struct Icon {
    pub name: IconName,
    pub size: f32,
    pub loading: bool,
}

impl Icon {
    pub fn new(name: IconName) -> Self {
        Self {
            name,
            size: 16.0,
            loading: false,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }
}

/// 图标库 — 提供 IconName → SVG data 的映射
pub struct IconLibrary {
    svgs: std::collections::HashMap<IconName, &'static str>,
}

impl Default for IconLibrary {
    fn default() -> Self {
        let mut svgs: std::collections::HashMap<IconName, &'static str> = std::collections::HashMap::new();

        // 数值操作
        svgs.insert(IconName::Plus, SVG_PLUS);
        svgs.insert(IconName::Minus, SVG_MINUS);
        svgs.insert(IconName::Delete, SVG_DELETE);
        svgs.insert(IconName::Edit, SVG_EDIT);
        // 功能操作
        svgs.insert(IconName::Search, SVG_SEARCH);
        svgs.insert(IconName::Refresh, SVG_REFRESH);
        svgs.insert(IconName::Print, SVG_PRINT);
        svgs.insert(IconName::Setting, SVG_SETTING);
        // 用户
        svgs.insert(IconName::User, SVG_USER);
        // 支付方式
        svgs.insert(IconName::WeChat, SVG_WECHAT);
        svgs.insert(IconName::Alipay, SVG_ALIPAY);
        svgs.insert(IconName::UnionPay, SVG_UNIONPAY);
        svgs.insert(IconName::Cash, SVG_CASH);
        svgs.insert(IconName::BankCard, SVG_BANKCARD);
        svgs.insert(IconName::Coupon, SVG_COUPON);
        // 订单
        svgs.insert(IconName::HangOrder, SVG_HANG_ORDER);
        svgs.insert(IconName::TakeOrder, SVG_TAKE_ORDER);
        svgs.insert(IconName::Clear, SVG_CLEAR);
        // 导航
        svgs.insert(IconName::Back, SVG_BACK);
        svgs.insert(IconName::Close, SVG_CLOSE);
        svgs.insert(IconName::Expand, SVG_EXPAND);
        svgs.insert(IconName::Fold, SVG_FOLD);
        // 状态
        svgs.insert(IconName::Success, SVG_SUCCESS);
        svgs.insert(IconName::Warning, SVG_WARNING);
        svgs.insert(IconName::Error, SVG_ERROR);
        svgs.insert(IconName::Info, SVG_INFO);
        svgs.insert(IconName::Loading, SVG_LOADING);
        svgs.insert(IconName::More, SVG_MORE);
        svgs.insert(IconName::Upload, SVG_UPLOAD);
        // 媒体
        svgs.insert(IconName::Image, SVG_IMAGE);
        svgs.insert(IconName::File, SVG_FILE);
        // 数据
        svgs.insert(IconName::Data, SVG_DATA);
        svgs.insert(IconName::Statistic, SVG_STATISTIC);
        // 时间
        svgs.insert(IconName::Calendar, SVG_CALENDAR);
        svgs.insert(IconName::Clock, SVG_CLOCK);
        svgs.insert(IconName::Filter, SVG_FILTER);
        // 方向
        svgs.insert(IconName::ArrowLeft, SVG_ARROW_LEFT);
        svgs.insert(IconName::ArrowRight, SVG_ARROW_RIGHT);
        svgs.insert(IconName::ArrowUp, SVG_ARROW_UP);
        svgs.insert(IconName::ArrowDown, SVG_ARROW_DOWN);

        Self { svgs }
    }
}

impl IconLibrary {
    /// 获取指定图标的 SVG 字符串
    pub fn get(&self, name: IconName) -> &'static str {
        self.svgs.get(&name).copied().unwrap_or("")
    }

    /// 图标库大小
    pub fn len(&self) -> usize {
        self.svgs.len()
    }

    /// 是否为空
    pub fn is_empty(&self) -> bool {
        self.svgs.is_empty()
    }
}

// ============ 内嵌 SVG 数据 ============
//
// 所有 SVG 使用 1024x1024 viewBox，与 @element-plus/icons-vue 一致。
// 简化实现：仅保留核心 path，足够识别即可。

const SVG_WRAP_START: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg">"#;
const SVG_WRAP_END: &str = "</svg>";

// 通用包装宏
const fn _wrap(_inner: &str) -> String { String::new() }

// 数值操作
const SVG_PLUS: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><path d="M477.866667 213.333333h68.266666v597.333334h-68.266666z"/><path d="M213.333333 477.866667h597.333334v68.266666H213.333333z"/></svg>"#;
const SVG_MINUS: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><path d="M213.333333 477.866667h597.333334v68.266666H213.333333z"/></svg>"#;
const SVG_DELETE: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><path d="M213.333333 256h597.333334v68.266667H213.333333z M281.6 256h469.333333v597.333333H281.6z M384 384h68.266667v341.333333H384z M554.666667 384h68.266666v341.333333h-68.266666z"/></svg>"#;
const SVG_EDIT: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><path d="M810.666667 725.333333h68.266666v136.533334H145.066667v-68.266667h665.6z M213.333333 597.333333l170.666667-170.666666 341.333333 341.333333-170.666666 170.666667z M426.666667 384l170.666666-170.666667 136.533334 136.533334-170.666667 170.666666z"/></svg>"#;

// 功能操作
const SVG_SEARCH: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><circle cx="426.666667" cy="426.666667" r="256" fill="none" stroke="currentColor" stroke-width="68"/><path d="M631.466667 631.466667l170.666666 170.666666h-68.266666l-170.666667-170.666666z"/></svg>"#;
const SVG_REFRESH: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><path d="M512 213.333333a298.666667 298.666667 0 0 1 298.666667 298.666667h68.266666a366.933333 366.933333 0 0 0-366.933333-366.933333z M512 810.666667a298.666667 298.666667 0 0 1-298.666667-298.666667H145.066667a366.933333 366.933333 0 0 0 366.933333 366.933333z"/></svg>"#;
const SVG_PRINT: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><path d="M213.333333 384h597.333334v256H213.333333z M256 213.333333h512v170.666667H256z M298.666667 640h426.666666v170.666667H298.666667z"/></svg>"#;
const SVG_SETTING: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><circle cx="512" cy="512" r="128" fill="none" stroke="currentColor" stroke-width="68"/><path d="M512 85.333333l42.666667 128h-85.333334z M512 938.666667l-42.666667-128h85.333334z M85.333333 512l128 42.666667v-85.333334z M938.666667 512l-128-42.666667v85.333334z"/></svg>"#;

// 用户
const SVG_USER: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><circle cx="512" cy="341.333333" r="170.666667" fill="none" stroke="currentColor" stroke-width="68"/><path d="M213.333333 896a298.666667 298.666667 0 0 1 597.333334 0z"/></svg>"#;

// 支付方式
const SVG_WECHAT: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><circle cx="384" cy="384" r="85.333333"/><circle cx="640" cy="384" r="85.333333"/><path d="M512 213.333333C298.666667 213.333333 128 341.333333 128 512s170.666667 298.666667 384 298.666667a499.2 499.2 0 0 0 128-17.066667L810.666667 832l-42.666667-128C874.666667 640 896 576 896 512 896 341.333333 725.333333 213.333333 512 213.333333z"/></svg>"#;
const SVG_ALIPAY: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><rect x="128" y="213.333333" width="768" height="597.333334" rx="85.333333" fill="none" stroke="currentColor" stroke-width="68"/><path d="M341.333333 512h341.333334"/></svg>"#;
const SVG_UNIONPAY: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><rect x="128" y="213.333333" width="768" height="597.333334" rx="85.333333" fill="none" stroke="currentColor" stroke-width="68"/><path d="M341.333333 384l170.666667 256h-85.333333L256 384z M597.333333 384l170.666667 256h-85.333333L512 384z"/></svg>"#;
const SVG_CASH: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><rect x="128" y="298.666667" width="768" height="426.666666" rx="42.666667" fill="none" stroke="currentColor" stroke-width="68"/><circle cx="512" cy="512" r="85.333333"/></svg>"#;
const SVG_BANKCARD: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><rect x="128" y="256" width="768" height="512" rx="42.666667" fill="none" stroke="currentColor" stroke-width="68"/><path d="M128 384h768"/></svg>"#;
const SVG_COUPON: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><path d="M128 341.333333a85.333333 85.333333 0 0 1 85.333333-85.333333h597.333334a85.333333 85.333333 0 0 1 85.333333 85.333333v128a85.333333 85.333333 0 0 0 0 213.333334v128a85.333333 85.333333 0 0 1-85.333333 85.333333H213.333333a85.333333 85.333333 0 0 1-85.333333-85.333333v-128a85.333333 85.333333 0 0 0 0-213.333334z M512 341.333333v341.333334"/></svg>"#;

// 订单
const SVG_HANG_ORDER: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><path d="M213.333333 213.333333h597.333334v597.333334H213.333333z M298.666667 384h426.666666 M298.666667 554.666667h426.666666 M298.666667 725.333333h213.333333"/></svg>"#;
const SVG_TAKE_ORDER: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><path d="M213.333333 213.333333h597.333334v597.333334H213.333333z M384 512l85.333333 85.333333 170.666667-170.666666"/></svg>"#;
const SVG_CLEAR: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><path d="M341.333333 213.333333h341.333334v68.266667H341.333333z M256 281.6h512v68.266667H256z M298.666667 349.866667h426.666666v469.333333H298.666667z"/></svg>"#;

// 导航
const SVG_BACK: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><path d="M640 213.333333L341.333333 512 640 810.666667v-128L469.333333 512 640 341.333333z"/></svg>"#;
const SVG_CLOSE: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><path d="M256 256l512 512 M768 256L256 768"/></svg>"#;
const SVG_EXPAND: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><path d="M213.333333 213.333333h256v68.266667H281.6v187.733333H213.333333z M810.666667 213.333333h-256v68.266667h187.733333v187.733333h68.266667z M213.333333 810.666667h256v-68.266667H281.6v-187.733333H213.333333z M810.666667 810.666667h-256v-68.266667h187.733333v-187.733333h68.266667z"/></svg>"#;
const SVG_FOLD: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><path d="M384 384L213.333333 213.333333M384 384h-170.666667M384 384v-170.666667 M640 384l170.666667-170.666667M640 384h170.666667M640 384v-170.666667 M384 640l-170.666667 170.666667M384 640h-170.666667M384 640v170.666667 M640 640l170.666667 170.666667M640 640h170.666667M640 640v170.666667"/></svg>"#;

// 状态
const SVG_SUCCESS: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><circle cx="512" cy="512" r="426.666667" fill="none" stroke="currentColor" stroke-width="68"/><path d="M341.333333 512l128 128 213.333334-256"/></svg>"#;
const SVG_WARNING: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><path d="M512 85.333333L938.666667 896H85.333333z M512 426.666667v170.666666 M512 725.333333v42.666667"/></svg>"#;
const SVG_ERROR: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><circle cx="512" cy="512" r="426.666667" fill="none" stroke="currentColor" stroke-width="68"/><path d="M384 384l256 256 M640 384l-256 256"/></svg>"#;
const SVG_INFO: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><circle cx="512" cy="512" r="426.666667" fill="none" stroke="currentColor" stroke-width="68"/><path d="M512 426.666667v256 M512 298.666667v42.666666"/></svg>"#;
const SVG_LOADING: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><path d="M512 128a384 384 0 1 0 384 384" fill="none" stroke="currentColor" stroke-width="68"/></svg>"#;
const SVG_MORE: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><circle cx="256" cy="512" r="85.333333"/><circle cx="512" cy="512" r="85.333333"/><circle cx="768" cy="512" r="85.333333"/></svg>"#;
const SVG_UPLOAD: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><path d="M512 256l-213.333333 213.333333h128v256h170.666666v-256h128z M213.333333 810.666667h597.333334v68.266666H213.333333z"/></svg>"#;

// 媒体
const SVG_IMAGE: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><rect x="128" y="213.333333" width="768" height="597.333334" rx="42.666667" fill="none" stroke="currentColor" stroke-width="68"/><circle cx="341.333333" cy="384" r="42.666667"/><path d="M213.333333 725.333333l213.333334-213.333333 170.666666 170.666667 128-128 85.333334 85.333333"/></svg>"#;
const SVG_FILE: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><path d="M213.333333 128h426.666667l170.666667 170.666667v597.333333H213.333333z M640 128v170.666667h170.666667"/></svg>"#;

// 数据
const SVG_DATA: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><ellipse cx="512" cy="213.333333" rx="341.333333" ry="85.333333" fill="none" stroke="currentColor" stroke-width="68"/><path d="M170.666667 213.333333v256c0 47.061333 152.746667 85.333333 341.333333 85.333334s341.333333-38.272 341.333333-85.333334v-256 M170.666667 469.333333v256c0 47.061333 152.746667 85.333333 341.333333 85.333334s341.333333-38.272 341.333333-85.333334v-256"/></svg>"#;
const SVG_STATISTIC: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><path d="M213.333333 725.333333l213.333334-213.333333 128 128 256-256 M213.333333 810.666667h597.333334"/></svg>"#;

// 时间
const SVG_CALENDAR: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><rect x="128" y="213.333333" width="768" height="597.333334" rx="42.666667" fill="none" stroke="currentColor" stroke-width="68"/><path d="M128 384h768 M341.333333 128v170.666667 M682.666667 128v170.666667"/></svg>"#;
const SVG_CLOCK: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><circle cx="512" cy="512" r="426.666667" fill="none" stroke="currentColor" stroke-width="68"/><path d="M512 256v256l170.666667 85.333333"/></svg>"#;
const SVG_FILTER: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><path d="M128 213.333333h768l-298.666667 341.333334v256l-170.666666-85.333334v-170.666666z"/></svg>"#;

// 方向
const SVG_ARROW_LEFT: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><path d="M597.333333 256L341.333333 512l256 256v-170.666667h256v-170.666666h-256z"/></svg>"#;
const SVG_ARROW_RIGHT: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><path d="M426.666667 256L682.666667 512l-256 256v-170.666667H128v-170.666666h298.666667z"/></svg>"#;
const SVG_ARROW_UP: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><path d="M256 597.333333L512 341.333333l256 256h-170.666667v256h-170.666666v-256z"/></svg>"#;
const SVG_ARROW_DOWN: &str = r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><path d="M256 426.666667L512 682.666667l256-256h-170.666667V128h-170.666666v298.666667z"/></svg>"#;

// 防止未使用警告
#[allow(dead_code)]
const _ENSURE_USE: (&str, &str) = (SVG_WRAP_START, SVG_WRAP_END);
