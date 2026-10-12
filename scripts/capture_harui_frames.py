#!/usr/bin/env python
"""用 headless Chrome 批量捕获 ep-parity 71 组件帧（Arimo 字体基线）。

用法：
    python scripts/capture_harui_frames.py [--out docs/visual-baseline/harui] [--par 6]
"""
import argparse
import os
import re
import shutil
import subprocess
import sys
import tempfile
import time
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

CHROME = r"C:\Program Files\Google\Chrome\Application\chrome.exe"
BASE_URL = "http://127.0.0.1:8140/index.html"


def capture(component: str, out_dir: Path, budget: int = 45000) -> tuple[str, bool]:
    out_png = out_dir / f"{component}.png"
    tmp_user = tempfile.mkdtemp(prefix="chrome-harui-")
    cmd = [
        CHROME,
        "--headless=new",
        "--disable-gpu",
        "--no-sandbox",
        "--hide-scrollbars",
        "--window-size=1264,569",
        f"--user-data-dir={tmp_user}",
        f"--virtual-time-budget={budget}",
        f"--screenshot={out_png}",
        f"{BASE_URL}#{component}",
    ]
    try:
        proc = subprocess.run(
            cmd,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
            timeout=90,
        )
        ok = proc.returncode == 0 and out_png.exists() and out_png.stat().st_size > 2000
        return component, ok
    except Exception:
        return component, False
    finally:
        shutil.rmtree(tmp_user, ignore_errors=True)


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", default="docs/visual-baseline/harui")
    ap.add_argument("--par", type=int, default=6)
    ap.add_argument("--budget", type=int, default=45000)
    args = ap.parse_args()

    root = Path(__file__).resolve().parent.parent
    out_dir = Path(args.out)
    if not out_dir.is_absolute():
        out_dir = root / out_dir
    out_dir.mkdir(parents=True, exist_ok=True)

    list_file = root / "scripts/_component_list.txt"
    if not list_file.exists():
        # 清单缺失时自动从 ep-parity main.rs 的 ALL 常量重新生成
        src = (root / "examples/ep-parity/src/main.rs").read_text(encoding="utf-8")
        block = src.split("const ALL", 1)[1].split("];", 1)[0]
        names = re.findall(r'"([a-z_]+)"', block)
        list_file.write_text("\n".join(names), encoding="utf-8")
        print(f"regenerated {list_file} ({len(names)} components)")
    components = list_file.read_text(encoding="utf-8").split()
    print(f"capturing {len(components)} components -> {out_dir} (par={args.par})")

    failed = []
    start = time.time()
    with ThreadPoolExecutor(max_workers=args.par) as ex:
        futs = {
            ex.submit(capture, c, out_dir, args.budget): c for c in components
        }
        from concurrent.futures import as_completed

        done = 0
        for fut in as_completed(futs):
            c, ok = fut.result()
            done += 1
            status = "OK " if ok else "FAIL"
            print(f"[{done}/{len(components)}] {status} {c}")
            if not ok:
                failed.append(c)

    elapsed = time.time() - start
    print(f"\ndone in {elapsed:.0f}s. failed={len(failed)} {failed}")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())