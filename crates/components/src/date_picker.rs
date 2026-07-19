//! DatePicker 日期选择器 — 参考 Element Plus `<el-date-picker>`。
//!
//! 支持：4 种 type（date/datetime/month/year/daterange）、placeholder、disabled、clearable、范围选择。
//!
//! ## 自绘日历面板
//! 内置月面板 6×7 网格（复用蔡勒公式），不依赖 iced 渲染层即可计算面板数据。
//! 调用方通过 `panel_grid()` 获取 42 格日期数据后自行渲染（iced 无原生日历）。

/// 选择器类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DatePickerType {
    #[default]
    Date,
    DateTime,
    Month,
    Year,
    DateRange,
}

/// 简化日期（复用 calendar 模块的同名类型）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SimpleDate {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl SimpleDate {
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }
}

/// DatePicker 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatePickerMessage {
    Open,
    Close,
    SelectDate(SimpleDate),
    SelectDateTime(SimpleDate, u32, u32, u32),
    SelectRangeStart(SimpleDate),
    SelectRangeEnd(SimpleDate),
    Clear,
    /// 面板导航：上一个月
    PrevMonth,
    /// 面板导航：下一个月
    NextMonth,
    /// 面板导航：上一年
    PrevYear,
    /// 面板导航：下一年
    NextYear,
    /// 面板导航：跳到指定年月
    JumpTo(i32, u32),
}

/// DatePicker 组件
#[derive(Debug, Clone)]
pub struct DatePicker {
    picker_type: DatePickerType,
    value: Option<SimpleDate>,
    time_part: Option<(u32, u32, u32)>,
    range_start: Option<SimpleDate>,
    range_end: Option<SimpleDate>,
    visible: bool,
    disabled: bool,
    clearable: bool,
    placeholder: String,
    /// 面板显示的年月（默认为今天的年月）
    panel_year: i32,
    panel_month: u32,
    /// 今天（外部可设置用于测试）
    today: SimpleDate,
}

impl Default for DatePicker {
    fn default() -> Self {
        Self::new()
    }
}

impl DatePicker {
    pub fn new() -> Self {
        let today = SimpleDate::new(2026, 7, 19);
        Self {
            picker_type: DatePickerType::Date,
            value: None,
            time_part: None,
            range_start: None,
            range_end: None,
            visible: false,
            disabled: false,
            clearable: false,
            placeholder: "选择日期".to_string(),
            panel_year: today.year,
            panel_month: today.month,
            today,
        }
    }

    pub fn with_type(mut self, t: DatePickerType) -> Self {
        self.picker_type = t;
        self
    }

    pub fn with_disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }

    pub fn with_clearable(mut self, v: bool) -> Self {
        self.clearable = v;
        self
    }

    pub fn with_placeholder(mut self, p: impl Into<String>) -> Self {
        self.placeholder = p.into();
        self
    }

    /// 设置今天日期（用于测试）
    pub fn with_today(mut self, d: SimpleDate) -> Self {
        self.panel_year = d.year;
        self.panel_month = d.month;
        self.today = d;
        self
    }

    pub fn picker_type(&self) -> DatePickerType {
        self.picker_type
    }

    pub fn value(&self) -> Option<&SimpleDate> {
        self.value.as_ref()
    }

    pub fn range_start(&self) -> Option<&SimpleDate> {
        self.range_start.as_ref()
    }

    pub fn range_end(&self) -> Option<&SimpleDate> {
        self.range_end.as_ref()
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn clearable(&self) -> bool {
        self.clearable
    }

    pub fn placeholder(&self) -> &str {
        &self.placeholder
    }

    pub fn panel_year(&self) -> i32 {
        self.panel_year
    }

    pub fn panel_month(&self) -> u32 {
        self.panel_month
    }

    pub fn today(&self) -> SimpleDate {
        self.today
    }

    /// 计算某年某月的天数
    fn days_in_month(year: i32, month: u32) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if Self::is_leap_year(year) { 29 } else { 28 }
            }
            _ => 30,
        }
    }

    fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    /// 蔡勒公式：返回 0=周一, 1=周二, ..., 6=周日
    fn day_of_week(year: i32, month: u32, day: u32) -> u32 {
        let (y, m) = if month < 3 { (year - 1, month + 12) } else { (year, month) };
        let k = y % 100;
        let j = y / 100;
        let h = (day as i32 + (13 * (m as i32 + 1)) / 5 + k + k / 4 + j / 4 + 5 * j) % 7;
        ((h + 5) % 7) as u32
    }

    /// 月面板 6×7 = 42 格网格（含上下月填充）
    ///
    /// 调用方根据每格的 `is_current_month` 字段决定是否灰显。
    pub fn panel_grid(&self) -> Vec<PanelCell> {
        let first_dow = Self::day_of_week(self.panel_year, self.panel_month, 1);
        let mut grid = Vec::with_capacity(42);
        let (mut y, mut m, mut d) = (self.panel_year, self.panel_month, 1u32);
        // 回退到面板起始日
        for _ in 0..first_dow {
            if d == 1 {
                if m == 1 {
                    m = 12;
                    y -= 1;
                } else {
                    m -= 1;
                }
                d = Self::days_in_month(y, m);
            } else {
                d -= 1;
            }
        }
        // 顺序填充 42 天
        for _ in 0..42 {
            let date = SimpleDate::new(y, m, d);
            let is_current_month = (y == self.panel_year) && (m == self.panel_month);
            let is_today = (y == self.today.year) && (m == self.today.month) && (d == self.today.day);
            let is_selected = self.value.map_or(false, |v| {
                v.year == y && v.month == m && v.day == d
            });
            let in_range = match (self.range_start, self.range_end) {
                (Some(s), Some(e)) => {
                    let val = date_to_ord(&date);
                    let sv = date_to_ord(&s);
                    let ev = date_to_ord(&e);
                    let (lo, hi) = if sv <= ev { (sv, ev) } else { (ev, sv) };
                    val >= lo && val <= hi
                }
                _ => false,
            };
            grid.push(PanelCell {
                date,
                is_current_month,
                is_today,
                is_selected,
                in_range,
            });
            d += 1;
            let dim = Self::days_in_month(y, m);
            if d > dim {
                d = 1;
                if m == 12 {
                    m = 1;
                    y += 1;
                } else {
                    m += 1;
                }
            }
        }
        grid
    }

    /// 格式化值字符串
    pub fn formatted_value(&self) -> String {
        match self.picker_type {
            DatePickerType::Date | DatePickerType::Month | DatePickerType::Year => {
                match self.value {
                    Some(d) => format!("{:04}-{:02}-{:02}", d.year, d.month, d.day),
                    None => String::new(),
                }
            }
            DatePickerType::DateTime => {
                match (self.value, self.time_part) {
                    (Some(d), Some((h, m, s))) => {
                        format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}", d.year, d.month, d.day, h, m, s)
                    }
                    _ => String::new(),
                }
            }
            DatePickerType::DateRange => {
                match (self.range_start, self.range_end) {
                    (Some(s), Some(e)) => {
                        format!("{:04}-{:02}-{:02} ~ {:04}-{:02}-{:02}",
                            s.year, s.month, s.day, e.year, e.month, e.day)
                    }
                    _ => String::new(),
                }
            }
        }
    }

    pub fn handle(&mut self, msg: DatePickerMessage) {
        if self.disabled {
            if matches!(msg, DatePickerMessage::Close) {
                self.visible = false;
            }
            return;
        }
        match msg {
            DatePickerMessage::Open => self.visible = true,
            DatePickerMessage::Close => self.visible = false,
            DatePickerMessage::SelectDate(d) => {
                self.value = Some(d);
                self.visible = false;
            }
            DatePickerMessage::SelectDateTime(d, h, m, s) => {
                self.value = Some(d);
                self.time_part = Some((h, m, s));
                self.visible = false;
            }
            DatePickerMessage::SelectRangeStart(d) => {
                self.range_start = Some(d);
            }
            DatePickerMessage::SelectRangeEnd(d) => {
                self.range_end = Some(d);
                self.visible = false;
            }
            DatePickerMessage::Clear => {
                if self.clearable {
                    self.value = None;
                    self.time_part = None;
                    self.range_start = None;
                    self.range_end = None;
                }
            }
            DatePickerMessage::PrevMonth => {
                if self.panel_month == 1 {
                    self.panel_month = 12;
                    self.panel_year -= 1;
                } else {
                    self.panel_month -= 1;
                }
            }
            DatePickerMessage::NextMonth => {
                if self.panel_month == 12 {
                    self.panel_month = 1;
                    self.panel_year += 1;
                } else {
                    self.panel_month += 1;
                }
            }
            DatePickerMessage::PrevYear => {
                self.panel_year -= 1;
            }
            DatePickerMessage::NextYear => {
                self.panel_year += 1;
            }
            DatePickerMessage::JumpTo(y, m) => {
                self.panel_year = y;
                self.panel_month = m.clamp(1, 12);
            }
        }
    }
}

/// 面板单元格
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PanelCell {
    pub date: SimpleDate,
    /// 是否面板当前月（非上下月填充）
    pub is_current_month: bool,
    /// 是否今天
    pub is_today: bool,
    /// 是否选中
    pub is_selected: bool,
    /// 是否在范围内（range 模式）
    pub in_range: bool,
}

/// 日期转可比较序数
fn date_to_ord(d: &SimpleDate) -> i64 {
    (d.year as i64) * 10000 + (d.month as i64) * 100 + d.day as i64
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_date_picker_default_placeholder() {
        let p = DatePicker::new();
        assert_eq!(p.placeholder(), "选择日期");
        assert_eq!(p.picker_type(), DatePickerType::Date);
    }

    // ============ 自绘日历面板测试 ============

    #[test]
    fn test_panel_grid_returns_42_cells() {
        let dp = DatePicker::new().with_today(SimpleDate::new(2026, 7, 19));
        let grid = dp.panel_grid();
        assert_eq!(grid.len(), 42, "面板应为 6x7=42 格");
    }

    #[test]
    fn test_panel_grid_july_2026_first_day_is_wednesday() {
        // 2026-07-01 是周三（蔡勒公式 0=Mon, 1=Tue, 2=Wed）
        let dp = DatePicker::new().with_today(SimpleDate::new(2026, 7, 19));
        // 面板第一格应为 6 月 29 日（周一）
        let grid = dp.panel_grid();
        assert_eq!(grid[0].date.day, 29, "first cell day");
        assert_eq!(grid[0].date.month, 6, "first cell month");
        assert!(!grid[0].is_current_month);
        // 索引 2（周三）应为 7 月 1 日
        assert_eq!(grid[2].date.day, 1);
        assert_eq!(grid[2].date.month, 7);
        assert!(grid[2].is_current_month);
    }

    #[test]
    fn test_panel_grid_today_marked() {
        let dp = DatePicker::new().with_today(SimpleDate::new(2026, 7, 19));
        let grid = dp.panel_grid();
        let today_cell = grid.iter().find(|c| c.is_today);
        assert!(today_cell.is_some(), "应标记今天");
        assert_eq!(today_cell.unwrap().date.day, 19);
    }

    #[test]
    fn test_panel_grid_selected_marked() {
        let mut dp = DatePicker::new().with_today(SimpleDate::new(2026, 7, 19));
        dp.handle(DatePickerMessage::SelectDate(SimpleDate::new(2026, 7, 15)));
        let grid = dp.panel_grid();
        let selected = grid.iter().find(|c| c.is_selected);
        assert!(selected.is_some());
        assert_eq!(selected.unwrap().date.day, 15);
    }

    #[test]
    fn test_panel_grid_in_range_marked() {
        let mut dp = DatePicker::new()
            .with_type(DatePickerType::DateRange)
            .with_today(SimpleDate::new(2026, 7, 19));
        dp.handle(DatePickerMessage::SelectRangeStart(SimpleDate::new(2026, 7, 10)));
        dp.handle(DatePickerMessage::SelectRangeEnd(SimpleDate::new(2026, 7, 20)));
        let grid = dp.panel_grid();
        let in_range_count = grid.iter().filter(|c| c.in_range).count();
        // 7月10日到7月20日共 11 天
        assert_eq!(in_range_count, 11, "range 内应有 11 天");
    }

    #[test]
    fn test_panel_navigation_prev_next_month() {
        let mut dp = DatePicker::new().with_today(SimpleDate::new(2026, 7, 19));
        assert_eq!(dp.panel_year(), 2026);
        assert_eq!(dp.panel_month(), 7);

        dp.handle(DatePickerMessage::PrevMonth);
        assert_eq!(dp.panel_month(), 6);
        assert_eq!(dp.panel_year(), 2026);

        // 1 月 → 上一月 → 12 月前一年
        dp.handle(DatePickerMessage::JumpTo(2026, 1));
        dp.handle(DatePickerMessage::PrevMonth);
        assert_eq!(dp.panel_month(), 12);
        assert_eq!(dp.panel_year(), 2025);

        // 12 月 → 下一月 → 1 月下一年
        dp.handle(DatePickerMessage::JumpTo(2026, 12));
        dp.handle(DatePickerMessage::NextMonth);
        assert_eq!(dp.panel_month(), 1);
        assert_eq!(dp.panel_year(), 2027);
    }

    #[test]
    fn test_panel_navigation_prev_next_year() {
        let mut dp = DatePicker::new().with_today(SimpleDate::new(2026, 7, 19));
        dp.handle(DatePickerMessage::PrevYear);
        assert_eq!(dp.panel_year(), 2025);
        assert_eq!(dp.panel_month(), 7);
        dp.handle(DatePickerMessage::NextYear);
        assert_eq!(dp.panel_year(), 2026);
    }

    #[test]
    fn test_panel_navigation_jump_to() {
        let mut dp = DatePicker::new().with_today(SimpleDate::new(2026, 7, 19));
        dp.handle(DatePickerMessage::JumpTo(2030, 3));
        assert_eq!(dp.panel_year(), 2030);
        assert_eq!(dp.panel_month(), 3);
        // month 越界钳制
        dp.handle(DatePickerMessage::JumpTo(2030, 13));
        assert_eq!(dp.panel_month(), 12);
        dp.handle(DatePickerMessage::JumpTo(2030, 0));
        assert_eq!(dp.panel_month(), 1);
    }

    #[test]
    fn test_panel_grid_leap_year_february() {
        // 2024 是闰年，2 月 29 天
        let dp = DatePicker::new().with_today(SimpleDate::new(2024, 2, 15));
        let grid = dp.panel_grid();
        // 应包含 2 月 29 日
        let feb_29 = grid.iter().find(|c| c.date.month == 2 && c.date.day == 29);
        assert!(feb_29.is_some(), "闰年 2 月应有 29 日");
        // 2026 不是闰年
        let dp2 = DatePicker::new().with_today(SimpleDate::new(2026, 2, 15));
        let grid2 = dp2.panel_grid();
        let feb_29_v2 = grid2.iter().find(|c| c.date.month == 2 && c.date.day == 29);
        assert!(feb_29_v2.is_none(), "非闰年 2 月不应有 29 日");
    }

    #[test]
    fn test_panel_grid_cross_year_boundary() {
        // 2026-01 面板：上月填充应跨到 2025-12
        let dp = DatePicker::new().with_today(SimpleDate::new(2026, 1, 15));
        let grid = dp.panel_grid();
        // 第一格应为 2025-12-29（2026-01-01 是周四，回退 3 天到周一）
        assert_eq!(grid[0].date.year, 2025);
        assert_eq!(grid[0].date.month, 12);
    }
}
