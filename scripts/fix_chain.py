#!/usr/bin/env python
"""重建 0.14 迁移后的 main() 链式调用：收集 title/window_size/subscription 行，
统一挂到 application(...) 闭括号后，.run() 永远收尾。"""
import pathlib
import re
import sys


def rebuild(p: pathlib.Path) -> bool:
    s = p.read_text(encoding="utf-8")
    if ".title(|_state|" not in s:
        return False
    lines = s.split("\n")
    # 定位 main 起止
    main_start = next(i for i, ln in enumerate(lines) if ln.startswith("fn main()"))
    # 收集 builder 行 + 删除已有 .run() 行
    builders = []
    out = []
    for i, ln in enumerate(lines):
        if i < main_start:
            out.append(ln)
            continue
        stripped = ln.strip()
        if re.match(r"\.(title|window_size|subscription)\(", stripped):
            builders.append((i, ln))
            continue
        if stripped == ".run()":
            continue
        if stripped == ".run();" or stripped == ".run();,":
            continue
        out.append(ln)
    lines = out

    # 找 application(...) 的闭括号行：view, 行后的第一个 "    )"
    view_idx = next(i for i, ln in enumerate(lines) if ln.strip() == "view,")
    close_idx = view_idx + 1
    while lines[close_idx].strip() != ")":
        close_idx += 1

    # 提取 title 参数（从 builders 里的 title 行还原）
    title_line = next((ln for _, ln in builders if ".title(" in ln), None)
    chain = []
    if title_line:
        chain.append("    " + title_line.strip())
    for _, ln in builders:
        if ".title(" in ln:
            continue
        chain.append("    " + ln.strip())
    chain.append("    .run()")

    lines = lines[: close_idx + 1] + chain + lines[close_idx + 1 :]
    p.write_text("\n".join(lines), encoding="utf-8")
    return True


def main():
    n = 0
    for p in sorted(pathlib.Path("examples").glob("*/src/main.rs")):
        if rebuild(p):
            print(f"  rebuilt: {p.parent.parent.name}")
            n += 1
    print(f"total: {n}")


if __name__ == "__main__":
    main()
