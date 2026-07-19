//! Calendar 日历 — 参考 Element Plus `<el-calendar>`。
//!
//! 支持：月面板 6×7 网格、上下月切换、今天、选定日期、范围检查。
//! 不依赖 chrono，使用蔡勒公式计算星期。

/// 简化日期（不依赖 chrono）
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

/// Calendar 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalendarMessage {
    NextMonth,
    PrevMonth,
    Today,
    SelectDate(SimpleDate),
}

/// Calendar 组件
#[derive(Debug, Clone)]
pub struct Calendar {
    year: i32,
    month: u32,
    selected: Option<SimpleDate>,
    range_start: Option<SimpleDate>,
    range_end: Option<SimpleDate>,
    /// 今天的日期（外部可设置用于测试）
    today: SimpleDate,
}

impl Default for Calendar {
    fn default() -> Self {
        Self::new(2026, 7)
    }
}

impl Calendar {
    pub fn new(year: i32, month: u32) -> Self {
        Self {
            year,
            month: month.clamp(1, 12),
            selected: None,
            range_start: None,
            range_end: None,
            today: SimpleDate::new(2026, 7, 19),
        }
    }

    pub fn with_today(mut self, d: SimpleDate) -> Self {
        self.today = d;
        self
    }

    pub fn with_range(mut self, start: SimpleDate, end: SimpleDate) -> Self {
        self.range_start = Some(start);
        self.range_end = Some(end);
        self
    }

    pub fn year(&self) -> i32 {
        self.year
    }

    pub fn month(&self) -> u32 {
        self.month
    }

    pub fn selected(&self) -> Option<&SimpleDate> {
        self.selected.as_ref()
    }

    /// 计算某年某月的天数
    fn days_in_month(year: i32, month: u32) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if Self::is_leap_year(year) {
                    29
                } else {
                    28
                }
            }
            _ => 30,
        }
    }

    fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    /// 蔡勒公式：返回 0=周一, 1=周二, ..., 6=周日
    fn day_of_week(year: i32, month: u32, day: u32) -> u32 {
        // 把 1、2 月视为上一年的 13、14 月
        let (y, m) = if month < 3 { (year - 1, month + 12) } else { (year, month) };
        let k = y % 100;
        let j = y / 100;
        let h = (day as i32 + (13 * (m as i32 + 1)) / 5 + k + k / 4 + j / 4 + 5 * j) % 7;
        // h: 0=周六, 1=周日, 2=周一, ..., 6=周五
        // 转为 0=周一, ..., 6=周日
        let normalized = (h + 5) % 7;
        normalized as u32
    }

    pub fn handle(&mut self, msg: CalendarMessage) {
        match msg {
            CalendarMessage::NextMonth => {
                if self.month == 12 {
                    self.month = 1;
                    self.year += 1;
                } else {
                    self.month += 1;
                }
            }
            CalendarMessage::PrevMonth => {
                if self.month == 1 {
                    self.month = 12;
                    self.year -= 1;
                } else {
                    self.month -= 1;
                }
            }
            CalendarMessage::Today => {
                self.year = self.today.year;
                self.month = self.today.month;
            }
            CalendarMessage::SelectDate(d) => {
                self.selected = Some(d);
            }
        }
    }

    /// 当前是否今天
    pub fn is_today(&self, d: &SimpleDate) -> bool {
        d.year == self.today.year && d.month == self.today.month && d.day == self.today.day
    }

    /// 是否当前月（与显示的 year/month 比较）
    pub fn is_current_month(&self, d: &SimpleDate) -> bool {
        d.year == self.year && d.month == self.month
    }

    /// 是否在范围内
    pub fn in_range(&self, d: &SimpleDate) -> bool {
        match (self.range_start, self.range_end) {
            (Some(s), Some(e)) => {
                let val = date_to_ord(d);
                let sv = date_to_ord(&s);
                let ev = date_to_ord(&e);
                let (lo, hi) = if sv <= ev { (sv, ev) } else { (ev, sv) };
                val >= lo && val <= hi
            }
            _ => false,
        }
    }

    /// 月面板 6×7 = 42 格网格（含上下月填充）
    pub fn month_grid(&self) -> Vec<SimpleDate> {
        let first_dow = Self::day_of_week(self.year, self.month, 1);
        // 找到面板第一天：本月 1 号往前 first_dow 天
        let mut grid = Vec::with_capacity(42);
        let (mut y, mut m, mut d) = (self.year, self.month, 1u32);
        // 回退到面板起始日
        for _ in 0..first_dow {
            // 前一天
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
            grid.push(SimpleDate::new(y, m, d));
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
}

/// 把日期转成可比较的序数（用于范围比较）
fn date_to_ord(d: &SimpleDate) -> i64 {
    (d.year as i64) * 10000 + (d.month as i64) * 100 + d.day as i64
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_calendar_day_of_week_2026_07_01() {
        // 2026-07-01 应为周三（0=周一，所以是 2）
        assert_eq!(Calendar::day_of_week(2026, 7, 1), 2);
    }

    #[test]
    fn test_calendar_leap_year() {
        assert!(Calendar::is_leap_year(2024));
        assert!(!Calendar::is_leap_year(2026));
        assert!(Calendar::is_leap_year(2000));
        assert!(!Calendar::is_leap_year(1900));
    }

    #[test]
    fn test_calendar_days_in_month_feb_leap() {
        assert_eq!(Calendar::days_in_month(2024, 2), 29);
        assert_eq!(Calendar::days_in_month(2026, 2), 28);
    }
}
