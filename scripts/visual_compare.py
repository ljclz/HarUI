#!/usr/bin/env python
"""EP 像素级视觉对照管线 — AE / SSIM 双指标（docs/screenshot_test.md 标准）

用法：
    python scripts/visual_compare.py <基准图> <当前图> [--threshold-ae 100] [--threshold-ssim 0.95]

指标：
- AE（Absolute Error）：逐像素 RGB 绝对差均值，< 100 通过（自建基准口径）
- SSIM（结构相似性）：灰度窗口化 SSIM 均值（8x8 高斯近似→均匀窗），> 0.95 通过

依赖：Pillow + numpy（pip install pillow numpy）
"""
import argparse
import sys

import numpy as np
from PIL import Image


def load_rgb(path: str) -> np.ndarray:
    img = Image.open(path).convert("RGB")
    return np.asarray(img, dtype=np.float64)


def resize_to_match(a: np.ndarray, b: np.ndarray) -> tuple[np.ndarray, np.ndarray]:
    """尺寸不一致时把 b 缩放到 a 的尺寸（宽容不同 DPR 的对比）。"""
    if a.shape == b.shape:
        return a, b
    h, w = a.shape[:2]
    img = Image.fromarray(b.astype(np.uint8)).resize((w, h), Image.LANCZOS)
    return a, np.asarray(img, dtype=np.float64)


def absolute_error(a: np.ndarray, b: np.ndarray) -> float:
    """逐像素 RGB 绝对差均值（0-255）"""
    return float(np.abs(a - b).mean())


def ssim(a: np.ndarray, b: np.ndarray) -> float:
    """灰度窗口化 SSIM（均匀 8x8 窗，步长 4；C1/C2 标准常数）"""
    if a.ndim == 3:
        a = a @ [0.299, 0.587, 0.114]
        b = b @ [0.299, 0.587, 0.114]
    C1, C2 = (0.01 * 255) ** 2, (0.03 * 255) ** 2
    win, step = 8, 4
    h, w = a.shape
    vals = []
    for y in range(0, h - win, step):
        for x in range(0, w - win, step):
            wa = a[y : y + win, x : x + win]
            wb = b[y : y + win, x : x + win]
            ma, mb = wa.mean(), wb.mean()
            va, vb = wa.var(), wb.var()
            cov = ((wa - ma) * (wb - mb)).mean()
            s = ((2 * ma * mb + C1) * (2 * cov + C2)) / (
                (ma * ma + mb * mb + C1) * (va + vb + C2)
            )
            vals.append(s)
    return float(np.mean(vals)) if vals else 0.0


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("baseline")
    ap.add_argument("current")
    ap.add_argument("--threshold-ae", type=float, default=100.0)
    ap.add_argument("--threshold-ssim", type=float, default=0.95)
    args = ap.parse_args()

    a = load_rgb(args.baseline)
    b = load_rgb(args.current)
    a, b = resize_to_match(a, b)

    ae = absolute_error(a, b)
    ssim_v = ssim(a, b)
    ae_ok = ae < args.threshold_ae
    ssim_ok = ssim_v > args.threshold_ssim

    print(f"AE   = {ae:.2f}  (阈值 < {args.threshold_ae})  {'PASS' if ae_ok else 'FAIL'}")
    print(f"SSIM = {ssim_v:.4f}  (阈值 > {args.threshold_ssim})  {'PASS' if ssim_ok else 'FAIL'}")
    if ae_ok and ssim_ok:
        print("结论: PASS — 视觉对齐达标")
        return 0
    print("结论: FAIL — 视觉对齐未达标（逐像素区域差异过大，建议导出差异图人工定位）")
    return 1


if __name__ == "__main__":
    sys.exit(main())
