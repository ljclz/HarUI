#!/usr/bin/env python
"""最终链归一化：main() 内收集全部 .title/.window_size/.subscription 行，
删除散落行后按 title → window_size → subscription 顺序插到 application 闭括号后，
.run() 收尾。"""
import pathlib
import re


def rebuild(p: pathlib.Path) -> bool:
    s = p.read_text(encoding="utf-8")
    if "iced::application(\n" not in s:
        return False
    lines = s.split("\n")
    main_start = next(i for i, ln in enumerate(lines) if ln.startswith("fn main()"))
    main_end = len(lines)

    builders = []
    keep = []
    for i, ln in enumerate(lines):
        if i >= main_start and re.match(r'\s*\.(title|window_size|subscription)\(', ln.strip() or ln):
            builders.append(ln.strip())
        elif i >= main_start and ln.strip() == ".run()":
            continue
        else:
            keep.append(ln)

    # 重新定位：application 闭括号（view, 行后的 "    )"）
    lines = keep
    view_idx = next(i for i, ln in enumerate(lines) if ln.strip() == "view,")
    close_idx = view_idx + 1
    while lines[close_idx].strip() != ")":
        close_idx += 1

    # 去重保序：title 一个，window_size 一个，subscription 一个
    seen = set()
    chain = []
    for b in builders:
        key = b.split("(")[0]
        if key in seen:
            continue
        seen.add(key)
        chain.append("    " + b)
    chain.append("    .run()")

    lines = lines[: close_idx + 1] + chain + lines[close_idx + 1 :]
    p.write_text("\n".join(lines), encoding="utf-8")
    return True


def main():
    n = 0
    for p in sorted(pathlib.Path("examples").glob("*/src/main.rs")):
        if rebuild(p):
            print(f"  normalized: {p.parent.parent.name}")
            n += 1
    print(f"total: {n}")


if __name__ == "__main__":
    main()
