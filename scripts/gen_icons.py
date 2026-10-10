#!/usr/bin/env python
"""生成 EP 全集图标模块（icon/ep.rs）— 数据源 iconify ep.json。

用法：python scripts/gen_icons.py <ep.json 路径>
输出：crates/core/src/icon/ep.rs（勿手改，重跑覆盖）

- EpIcon 枚举：EP 官方 PascalCase 名（~293 变体）
- svg()：EpIcon → 完整 SVG 字符串（1024 viewBox，官方 path 数据）
- from_name()：PascalCase 名 → EpIcon（业务侧字符串取用）
- ALL_NAMES：全部名称常量表
"""
import json
import sys
import pathlib


def kebab_to_pascal(name: str) -> str:
    return "".join(w.capitalize() for w in name.split("-"))


def main() -> None:
    data = json.load(open(sys.argv[1], encoding="utf-8"))
    icons = data["icons"]
    default_w = data.get("width", 1024)
    default_h = data.get("height", 1024)

    entries = []  # (pascal, svg)
    for name, info in sorted(icons.items()):
        pascal = kebab_to_pascal(name)
        w = info.get("width", default_w)
        h = info.get("height", default_h)
        body = info["body"]
        svg = (
            f'<svg viewBox="0 0 {w} {h}" xmlns="http://www.w3.org/2000/svg">'
            f"{body}</svg>"
        )
        entries.append((pascal, svg))

    pascal_names = [p for p, _ in entries]

    out = []
    out.append("//! EP 全集图标 — 由 scripts/gen_icons.py 从 iconify ep.json 生成（勿手改）")
    out.append("//!")
    out.append(f"//! 共 {len(entries)} 个图标，官方 path 数据，viewBox 0 0 1024 1024。")
    out.append("//! 数据源：@element-plus/icons-vue（iconify 集合 ep）。")
    out.append("")
    out.append("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]")
    out.append("pub enum EpIcon {")
    for p in pascal_names:
        out.append(f"    {p},")
    out.append("}")
    out.append("")
    out.append("impl EpIcon {")
    out.append("    /// PascalCase 名称")
    out.append("    pub fn name(self) -> &'static str {")
    out.append("        match self {")
    for p in pascal_names:
        out.append(f'            Self::{p} => "{p}",')
    out.append("        }")
    out.append("    }")
    out.append("")
    out.append("    /// 完整 SVG 字符串")
    out.append("    pub fn svg(self) -> &'static str {")
    out.append("        match self {")
    for p, svg in entries:
        out.append(f'            Self::{p} => EP_{p.upper()},')
    out.append("        }")
    out.append("    }")
    out.append("}")
    out.append("")
    out.append("/// PascalCase 名称 → EpIcon")
    out.append("pub fn from_name(name: &str) -> Option<EpIcon> {")
    out.append("    match name {")
    for p in pascal_names:
        out.append(f'            "{p}" => Some(EpIcon::{p}),')
    out.append("        _ => None,")
    out.append("    }")
    out.append("}")
    out.append("")
    out.append(f"/// 全部 EP 图标名（{len(entries)} 个）")
    out.append("pub const ALL_NAMES: &[&str] = &[")
    for p in pascal_names:
        out.append(f'    "{p}",')
    out.append("];")
    out.append("")
    # SVG 常量
    for p, svg in entries:
        out.append(f"const EP_{p.upper()}: &str = r#\"{svg}\"#;")
    out.append("")

    dest = pathlib.Path(__file__).resolve().parent.parent / "crates" / "core" / "src" / "icon" / "ep.rs"
    dest.write_text("\n".join(out), encoding="utf-8", newline="")
    print(f"generated {dest.name}: {len(entries)} icons, {dest.stat().st_size // 1024} KB")


if __name__ == "__main__":
    main()
