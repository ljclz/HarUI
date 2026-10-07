#!/usr/bin/env python
"""iced 0.13 → 0.14 应用构造迁移（括号匹配版）：
iced::application("T", update, view)...run_with(|| {INIT})
→ iced::application(|| {INIT}, update, view).title("T")....run()

关键：用括号匹配定位 run_with 闭包体（非贪婪正则会截断嵌套闭包）。
"""
import pathlib
import re


def find_matching(src: str, open_idx: int) -> int:
    """从 open_idx 处的 '{' 找配对 '}'（含字符串字面量状态机），返回其下标。"""
    depth = 0
    in_str = False
    i = open_idx
    while i < len(src):
        ch = src[i]
        if in_str:
            if ch == "\\":
                i += 2
                continue
            if ch == '"':
                in_str = False
        else:
            if ch == '"':
                in_str = True
            elif ch == "{":
                depth += 1
            elif ch == "}":
                depth -= 1
                if depth == 0:
                    return i
        i += 1
    raise ValueError("unbalanced braces")


def migrate(src: str) -> str | None:
    m = re.search(r'iced::application\(\s*"((?:[^"\\]|\\.)*)"\s*,\s*update\s*,\s*view\s*\)', src)
    if not m:
        return None
    title = m.group(1)

    rw = re.search(r"\.run_with\(\|\| \{", src)
    if not rw:
        return None
    open_brace = src.index("{", rw.end() - 1)
    close_brace = find_matching(src, open_brace)
    boot_body = src[open_brace + 1 : close_brace]
    # run_with 调用整体：从 .run_with( 到闭括号后的 ')'
    run_end = src.find(")", close_brace + 1)
    run_with_full = src[rw.start() : run_end + 1]

    new_head = (
        "iced::application(\n"
        "        || {\n"
        f"{boot_body}\n"
        "        },\n"
        "        update,\n"
        "        view,\n"
        "    )\n"
        f'    .title(|_state: &State| String::from("{title}"))'
    )
    src = src[: m.start()] + new_head + src[m.end() :]
    src = src.replace(run_with_full, "", 1)
    # 链尾补 .run()：在 title 行后插入
    src = src.replace(
        f'.title(|_state: &State| String::from("{title}"))',
        f'.title(|_state: &State| String::from("{title}"))\n    .run()',
        1,
    )
    return src


def main():
    n = 0
    for p in sorted(pathlib.Path("examples").glob("*/src/main.rs")):
        s = p.read_text(encoding="utf-8")
        if "iced::application(" not in s:
            continue
        s2 = migrate(s)
        if s2:
            p.write_text(s2, encoding="utf-8")
            print(f"  migrated: {p.parent.parent.name}")
            n += 1
    print(f"total: {n}")


if __name__ == "__main__":
    main()
