#!/usr/bin/env python
"""EP ↔ HarUI 全量视觉对照批跑 — 输出 markdown 汇总表

用法：
    python scripts/batch_visual_compare.py [--ep-dir ...] [--harui-dir ...] [--out ...]

对 docs/visual-baseline/ep 与 docs/visual-baseline/harui 中同名组件帧逐对计算
AE/SSIM（复用 visual_compare.py 的指标逻辑），输出：
  - 终端汇总（通过/失败计数）
  - markdown 表（docs/parity-batch-result.md）
"""
import argparse
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import visual_compare as vc  # noqa: E402


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--ep-dir", default=None)
    ap.add_argument("--harui-dir", default=None)
    ap.add_argument("--out", default=None)
    args = ap.parse_args()

    root = Path(__file__).resolve().parent.parent
    ep_dir = Path(args.ep_dir) if args.ep_dir else root / "docs/visual-baseline/ep"
    hu_dir = Path(args.harui_dir) if args.harui_dir else root / "docs/visual-baseline/harui"
    out = Path(args.out) if args.out else root / "docs/parity-batch-result.md"

    ep_files = {p.stem: p for p in ep_dir.glob("*.png") if p.stem not in ("test",)}
    hu_files = {p.stem: p for p in hu_dir.glob("*.png")}
    common = sorted(set(ep_files) & set(hu_files))

    rows = []
    for name in common:
        a = vc.load_rgb(str(ep_files[name]))
        b = vc.load_rgb(str(hu_files[name]))
        a, b = vc.resize_to_match(a, b)
        ae = vc.absolute_error(a, b)
        ss = vc.ssim(a, b)
        rows.append((name, ae, ss))

    if not rows:
        print("no common components")
        return 1

    rows.sort(key=lambda r: r[0])
    ae_pass = sum(1 for _, ae, _ in rows if ae < 100)
    ss_pass = sum(1 for _, _, ss in rows if ss > 0.95)
    n = len(rows)

    lines = [
        "# EP ↔ HarUI 全量像素对照（批跑）",
        "",
        f"> 生成日期：2026-10-11 ｜ 对比组件数：{n} ｜ AE<100 PASS：{ae_pass}/{n} ｜ SSIM>0.95 PASS：{ss_pass}/{n}",
        f"> 说明：EP 帧为官网完整文档页，HarUI 帧为白底独立组件页——AE/SSIM 反映**整页**差异，",
        f"> 非组件区域裁剪对照；组件级 API/令牌/wasm 对齐见 parity-status-report.md。",
        "",
        "| 组件 | AE | SSIM | AE PASS | SSIM PASS |",
        "|------|----|------|---------|-----------|",
    ]
    for name, ae, ss in rows:
        lines.append(
            f"| {name} | {ae:.2f} | {ss:.4f} | {'✅' if ae < 100 else '❌'} | {'✅' if ss > 0.95 else '❌'} |"
        )
    lines.append("")
    out.write_text("\n".join(lines), encoding="utf-8")
    print(f"compared {n}, AE pass {ae_pass}, SSIM pass {ss_pass}")
    print(f"table -> {out}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
