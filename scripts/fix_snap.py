#!/usr/bin/env python
"""E0063 snap 字段自动补齐：括号匹配定位结构体字面量闭括号，闭括号前插入 snap: false。

用法：python scripts/fix_snap.py <check.json>
"""
import json
import sys
import pathlib


def find_close(lines, ln0, col0):
    """从 (ln0,col0)（0-based）找第一个 {，括号匹配到配对 }，返回 (行idx, 列idx)。"""
    depth = 0
    started = False
    in_str = False
    for li in range(ln0, len(lines)):
        line = lines[li]
        ci = col0 if li == ln0 else 0
        while ci < len(line):
            ch = line[ci]
            if in_str:
                if ch == "\\":
                    ci += 2
                    continue
                if ch == '"':
                    in_str = False
            else:
                if ch == '"':
                    in_str = True
                elif ch == "{":
                    depth += 1
                    started = True
                elif ch == "}":
                    if started and depth == 1:
                        return (li, ci)
                    depth -= 1
            ci += 1
    return None


def main():
    msgs = []
    for line in open(sys.argv[1], encoding="utf-8"):
        try:
            msgs.append(json.loads(line))
        except Exception:
            pass

    sites = []
    for m in msgs:
        if m.get("reason") != "compiler-message":
            continue
        msg = m["message"]
        code = (msg.get("code") or {}).get("code")
        if code != "E0063" or "snap" not in msg.get("message", ""):
            continue
        span = next((s for s in msg.get("spans", []) if s.get("is_primary")), None)
        if not span:
            continue
        sites.append(
            {"file": span["file_name"], "line": span["line_start"], "col": span["column_start"]}
        )
    print(f"literal starts: {len(sites)}")

    by_file = {}
    for s in sites:
        by_file.setdefault(s["file"], []).append(s)

    total = 0
    for fname, ss in by_file.items():
        p = pathlib.Path(fname)
        lines = p.read_text(encoding="utf-8").split("\n")
        closes = []
        for site in ss:
            r = find_close(lines, site["line"] - 1, site["col"] - 1)
            if r:
                closes.append(r)
        for li, _ci in sorted(set(closes), key=lambda x: -x[0]):
            prev = lines[li - 1] if li > 0 else ""
            prev_indent = prev[: len(prev) - len(prev.lstrip())]
            lines.insert(li, prev_indent + "    snap: false,")
            total += 1
        p.write_text("\n".join(lines), encoding="utf-8")
    print(f"total inserted: {total}")


if __name__ == "__main__":
    main()
