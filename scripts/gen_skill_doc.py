#!/usr/bin/env python
# -*- coding: utf-8 -*-
"""生成 HarUI AI 技能包的 API 速查段（路线图 W6）。

从 crates/components/src/*.rs 源码正则抽取每个组件的：
- 主结构体（首个 pub struct）
- Message 枚举变体
- with_* 构建器签名

输出 markdown 表格到 stdout，供拼接进 docs/skill/har-ui-skills.md。
再生成命令：python scripts/gen_skill_doc.py > docs/skill/_api_tables.md
（速查段头部标注"自动生成"，勿手改）
"""

import pathlib
import re
import sys

SRC = pathlib.Path(__file__).resolve().parent.parent / "crates" / "components" / "src"


def extract_enum_variants(src: str, enum_name: str) -> list[str]:
    """提取 pub enum <enum_name> 的变体名（含元组字段只取名）。"""
    m = re.search(r"pub enum %s\b\s*\{(.*?)\n\}" % re.escape(enum_name), src, re.S)
    if not m:
        return []
    variants = []
    for line in m.group(1).splitlines():
        line = line.strip()
        vm = re.match(r"^([A-Z]\w*)\b", line)
        if vm and not line.startswith("//"):
            variants.append(vm.group(1))
    return variants


def extract_builders(src: str) -> list[str]:
    """提取 pub fn with_* 构建器签名（跨行括号容忍）。"""
    sigs = []
    for m in re.finditer(r"pub fn (with_\w+)\s*\(([^{]*)\)", src):
        name = m.group(1)
        args = re.sub(r"\s+", " ", m.group(2)).strip()
        args = re.sub(r":\s*", ": ", args)
        args = re.sub(r",\s*", ", ", args)
        args = re.sub(r"\s*\.\.\.", "...", args)
        sigs.append(f"{name}({args})")
    return sigs


def main() -> None:
    rows = []
    files = sorted(p for p in SRC.glob("*.rs") if p.name != "lib.rs")
    for path in files:
        src = path.read_text(encoding="utf-8")
        struct_m = re.search(r"pub struct (\w+)(?:<[^>]*>)?", src)
        struct_name = struct_m.group(1) if struct_m else path.stem
        msg_m = re.search(r"pub enum (\w*Message)\b", src)
        msg_name = msg_m.group(1) if msg_m else None
        variants = extract_enum_variants(src, msg_name) if msg_name else []
        builders = extract_builders(src)

        msg_col = (
            f"`{msg_name}`: " + " · ".join(f"`{v}`" for v in variants)
            if msg_name
            else "—（静态展示）"
        )
        builder_col = "<br>".join(f"`{b}`" for b in builders[:8]) if builders else "—"
        if len(builders) > 8:
            builder_col += f"<br>…共 {len(builders)} 个"
        rows.append((path.stem, struct_name, msg_col, builder_col))

    print("| 模块 | 主结构体 | Message 变体 | 常用构建器（with_*） |")
    print("|------|---------|--------------|---------------------|")
    for stem, struct_name, msg_col, builder_col in rows:
        print(f"| {stem} | `{struct_name}` | {msg_col} | {builder_col} |")
    print()
    print(f"> 共 {len(rows)} 个组件模块，自动生成于源码扫描，勿手工编辑本段。", file=sys.stderr)


if __name__ == "__main__":
    main()
