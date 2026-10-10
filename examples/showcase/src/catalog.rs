//! 组件目录索引 — 全部 61 个组件的分组清单与演示状态（路线图 W5）
//!
//! 与 README 组件分类表对应（数据展示类含 README 未列出的 Tree，共 61）。
//! `Interactive` 项可点击跳转到对应交互演示页，其余为已实现静态列出。

use crate::{Category, Message};
use har_ui_core::Theme;
use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Color, Element, Length, Padding};

/// 演示状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DemoLevel {
    /// 本 showcase 有可交互演示页
    Interactive,
    /// 组件已实现，见设计规范与对应 demo
    Listed,
}

/// 目录条目
#[derive(Debug, Clone, Copy)]
pub struct CatalogEntry {
    pub name: &'static str,
    pub group: &'static str,
    pub level: DemoLevel,
}

/// 分组（README 分类表顺序）
pub const GROUPS: [&str; 7] = [
    "基础",
    "表单",
    "数据展示",
    "导航",
    "反馈",
    "其他",
    "POS 专用",
];

/// 全部 61 个组件
pub const CATALOG: [CatalogEntry; 64] = [
    // 基础 10
    CatalogEntry {
        name: "Button",
        group: "基础",
        level: DemoLevel::Interactive,
    },
    CatalogEntry {
        name: "Input",
        group: "基础",
        level: DemoLevel::Interactive,
    },
    CatalogEntry {
        name: "InputNumber",
        group: "基础",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "Select",
        group: "基础",
        level: DemoLevel::Interactive,
    },
    CatalogEntry {
        name: "Radio",
        group: "基础",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "Checkbox",
        group: "基础",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "Switch",
        group: "基础",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "Tag",
        group: "基础",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "Badge",
        group: "基础",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "Avatar",
        group: "基础",
        level: DemoLevel::Listed,
    },
    // 表单 8
    CatalogEntry {
        name: "Form",
        group: "表单",
        level: DemoLevel::Interactive,
    },
    CatalogEntry {
        name: "DatePicker",
        group: "表单",
        level: DemoLevel::Interactive,
    },
    CatalogEntry {
        name: "TimePicker",
        group: "表单",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "Cascader",
        group: "表单",
        level: DemoLevel::Interactive,
    },
    CatalogEntry {
        name: "Slider",
        group: "表单",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "Rate",
        group: "表单",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "ColorPicker",
        group: "表单",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "Upload",
        group: "表单",
        level: DemoLevel::Interactive,
    },
    CatalogEntry {
        name: "Autocomplete",
        group: "表单",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "Image",
        group: "表单",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "TreeSelect",
        group: "表单",
        level: DemoLevel::Listed,
    },
    // 数据展示 12（含 README 未列的 Tree）
    CatalogEntry {
        name: "Table",
        group: "数据展示",
        level: DemoLevel::Interactive,
    },
    CatalogEntry {
        name: "Card",
        group: "数据展示",
        level: DemoLevel::Interactive,
    },
    CatalogEntry {
        name: "Pagination",
        group: "数据展示",
        level: DemoLevel::Interactive,
    },
    CatalogEntry {
        name: "Descriptions",
        group: "数据展示",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "Timeline",
        group: "数据展示",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "Collapse",
        group: "数据展示",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "Statistic",
        group: "数据展示",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "Empty",
        group: "数据展示",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "Result",
        group: "数据展示",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "Skeleton",
        group: "数据展示",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "Progress",
        group: "数据展示",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "Tree",
        group: "数据展示",
        level: DemoLevel::Listed,
    },
    // 导航 8
    CatalogEntry {
        name: "Tabs",
        group: "导航",
        level: DemoLevel::Interactive,
    },
    CatalogEntry {
        name: "Steps",
        group: "导航",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "Breadcrumb",
        group: "导航",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "Dropdown",
        group: "导航",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "Backtop",
        group: "导航",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "Affix",
        group: "导航",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "Menu",
        group: "导航",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "PageHeader",
        group: "导航",
        level: DemoLevel::Listed,
    },
    // 反馈 10
    CatalogEntry {
        name: "Dialog",
        group: "反馈",
        level: DemoLevel::Interactive,
    },
    CatalogEntry {
        name: "Drawer",
        group: "反馈",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "MessageBox",
        group: "反馈",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "Notification",
        group: "反馈",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "Message",
        group: "反馈",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "Loading",
        group: "反馈",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "Tooltip",
        group: "反馈",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "Popover",
        group: "反馈",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "Popconfirm",
        group: "反馈",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "Alert",
        group: "反馈",
        level: DemoLevel::Listed,
    },
    // 其他 8
    CatalogEntry {
        name: "Text",
        group: "其他",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "Link",
        group: "其他",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "Divider",
        group: "其他",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "Space",
        group: "其他",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "Scrollbar",
        group: "其他",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "Calendar",
        group: "其他",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "Carousel",
        group: "其他",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "Grid",
        group: "其他",
        level: DemoLevel::Listed,
    },
    // POS 专用 5
    CatalogEntry {
        name: "Keypad",
        group: "POS 专用",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "Payment",
        group: "POS 专用",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "HangOrder",
        group: "POS 专用",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "CustomerDisplay",
        group: "POS 专用",
        level: DemoLevel::Listed,
    },
    CatalogEntry {
        name: "StatusBar",
        group: "POS 专用",
        level: DemoLevel::Listed,
    },
];

/// 组件名 → 交互演示页映射（仅 Interactive 项）
pub fn category_of(name: &str) -> Option<Category> {
    Some(match name {
        "Button" => Category::Button,
        "Input" => Category::Input,
        "Table" => Category::Table,
        "Dialog" => Category::Dialog,
        "Form" => Category::Form,
        "Select" => Category::Select,
        "Pagination" => Category::Pagination,
        "Card" => Category::Card,
        "Tabs" => Category::Tabs,
        "DatePicker" => Category::DatePicker,
        "Cascader" => Category::Cascader,
        "Upload" => Category::Upload,
        _ => return None,
    })
}

/// 渲染目录索引页
pub fn index_view(theme: &Theme) -> Element<'static, Message> {
    let text_primary = Color::from(theme.neutral.text_primary);
    let text_secondary = Color::from(theme.neutral.text_secondary);
    let accent = Color::from(theme.primary.base);
    let border = Color::from(theme.neutral.border_lighter);

    let mut col = column![
        text("组件目录 — 64 components")
            .size(20)
            .color(text_primary)
    ]
    .spacing(12)
    .padding(Padding::from([16u16, 8u16]));

    let interactive_count = CATALOG
        .iter()
        .filter(|e| e.level == DemoLevel::Interactive)
        .count();
    col = col.push(
        text(format!(
            "共 {} 个组件，其中 {} 个有可交互演示（点击名称进入），其余已实现、见设计规范与对应 demo",
            CATALOG.len(),
            interactive_count
        ))
        .size(13)
        .color(text_secondary),
    );

    for group in GROUPS {
        col = col.push(
            container(text(format!("— {}", group)).size(15).color(accent))
                .padding(Padding::new(0.0).top(8.0)),
        );
        // 每组一行行排布（每行 4 个，避免溢出）
        let entries: Vec<&CatalogEntry> = CATALOG.iter().filter(|e| e.group == group).collect();
        for chunk in entries.chunks(4) {
            let mut r = row![].spacing(8);
            for entry in chunk {
                match entry.level {
                    DemoLevel::Interactive => {
                        let cat = category_of(entry.name);
                        let btn = button(text(format!("{} ▸", entry.name)).size(13).color(accent))
                            .padding(Padding::from([4u16, 10u16]))
                            .style(move |_t, _s| button::Style {
                                background: None,
                                border: iced::Border {
                                    color: border,
                                    width: 1.0,
                                    radius: iced::border::radius(4.0),
                                },
                                ..button::Style::default()
                            });
                        r = r.push(match cat {
                            Some(c) => btn.on_press(Message::SelectCategory(c)),
                            None => btn,
                        });
                    }
                    DemoLevel::Listed => {
                        r = r.push(
                            container(text(entry.name).size(13).color(text_secondary))
                                .padding(Padding::from([4u16, 10u16]))
                                .style(move |_t| container::Style {
                                    background: None,
                                    border: iced::Border {
                                        color: border,
                                        width: 1.0,
                                        radius: iced::border::radius(4.0),
                                    },
                                    ..container::Style::default()
                                }),
                        );
                    }
                }
            }
            col = col.push(r);
        }
    }

    scrollable(
        container(col)
            .width(Length::Fill)
            .align_x(iced::alignment::Horizontal::Left),
    )
    .into()
}

#[cfg(test)]
mod catalog_tests {
    use super::*;

    #[test]
    fn test_catalog_has_61_unique_entries() {
        assert_eq!(CATALOG.len(), 64);
        let names: std::collections::BTreeSet<&str> = CATALOG.iter().map(|e| e.name).collect();
        assert_eq!(names.len(), 64, "组件名不得重复");
    }

    #[test]
    fn test_all_interactive_entries_map_to_category() {
        for e in CATALOG.iter().filter(|e| e.level == DemoLevel::Interactive) {
            assert!(
                category_of(e.name).is_some(),
                "Interactive 组件 {} 缺少页面映射",
                e.name
            );
        }
    }

    #[test]
    fn test_groups_covered() {
        for g in GROUPS {
            assert!(CATALOG.iter().any(|e| e.group == g), "分组 {} 无条目", g);
        }
    }
}
