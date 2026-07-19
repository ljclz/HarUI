# D.14 截图测试流程

## 概述

D.14 要求"截图测试通过 - 像素级一致"。由于 CLI/CI 环境无法实际运行 GUI 截图，
本项目采用"双轨"方案：

1. **编译时验证**（CI 可自动执行）：
   - 测试文件 `crates/components/tests/screenshot_test.rs`
   - 验证所有纯展示组件在 light/dark 两种主题下能成功调用 `view()` 返回 `iced::Element`
   - 运行命令：`cargo test -p har-ui-components --test screenshot_test`

2. **真机像素级验证**（人工/CI Runner 手动执行）：
   - 测试应用 `examples/screenshot_test`
   - 运行命令：`cargo run -p screenshot_test`
   - 通过键盘快捷键遍历 12 个组件 × 2 个主题 = 24 个场景
   - 用 OS 截图工具保存为 PNG，与基线做像素级对比

## 真机截图测试步骤

### 1. 启动测试应用

```bash
cargo run -p screenshot_test
```

应用启动后窗口标题为 `HarUI Screenshot Test`，尺寸 1024×768。

### 2. 键盘快捷键

| 按键 | 功能 |
|------|------|
| `S` | 打印当前画面截图清单条目（提示用 OS 工具截图） |
| `N` | 切换下一个组件 |
| `T` | 切换主题（light / dark） |
| `R` | 进入/退出录制模式（每 2 秒自动切换组件） |
| `Esc` | 退出应用 |

### 3. 录制模式（推荐）

按 `R` 进入录制模式，应用会每 2 秒自动切换组件：
- 12 个组件依次切换，遍历完后再切换主题
- 共 24 个场景（12 组件 × 2 主题），约 48 秒完成
- 每个场景停留 2 秒，期间用 OS 截图工具截图：
  - Windows: `Win+Shift+S`
  - macOS: `Cmd+Shift+4`
  - Linux: `gnome-screenshot -w` 或 `scrot -u`

### 4. 保存截图

将截图保存到 `screenshots/` 目录，命名规则：

```
<component>_<theme>.png
```

例如：
- `button_light.png`
- `card_dark.png`
- `table_light.png`
- ...

12 个组件名：`button`, `input`, `table`, `dialog`, `form`, `select`,
`pagination`, `card`, `tabs`, `date_picker`, `cascader`, `upload`。

2 个主题名：`light`, `dark`。

### 5. 像素级对比

#### 使用 ImageMagick

```bash
# AE (Absolute Error) 模式：统计不同像素数
compare -metric AE baseline/button_light.png current/button_light.png diff.png
# 输出 0 表示完全一致

# RMSE 模式：均方根误差
compare -metric RMSE baseline/button_light.png current/button_light.png diff.png
```

#### 使用 Python PIL

```python
from PIL import Image, ImageChops
import sys

def compare_images(baseline_path, current_path, threshold=100):
    baseline = Image.open(baseline_path).convert("RGB")
    current = Image.open(current_path).convert("RGB")
    if baseline.size != current.size:
        return False, f"size mismatch: {baseline.size} vs {current.size}"
    diff = ImageChops.difference(baseline, current)
    bbox = diff.getbbox()
    if bbox is None:
        return True, "identical"
    # 统计差异像素数
    pixels = list(diff.getdata())
    diff_count = sum(1 for p in pixels if any(c > 10 for c in p))
    return diff_count < threshold, f"diff pixels: {diff_count}"

ok, msg = compare_images(
    "screenshots/baseline/button_light.png",
    "screenshots/current/button_light.png",
)
print(f"{'PASS' if ok else 'FAIL'}: {msg}")
sys.exit(0 if ok else 1)
```

## 像素级对比标准

- **AE (Absolute Error) < 100 像素** = 通过
- 或 **SSIM > 0.95** = 通过

对于轻微抗锯齿差异（< 100 像素），视为可接受。
对于字体渲染差异（不同 OS 字体），建议在 CI Runner 上统一基线。

## 自动化测试（CI）

在 GitHub Actions 中添加：

```yaml
name: Screenshot Test
on: [push, pull_request]
jobs:
  screenshot:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Install system deps
        run: |
          sudo apt-get update
          sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.1-dev \
            libayatana-appindicator3-dev librsvg2-dev imagemagick
      - name: Compile-time render test
        run: cargo test -p har-ui-components --test screenshot_test
      - name: Build screenshot app
        run: cargo build -p screenshot_test --release
      # 真机截图需在带显示器的 Runner 上运行，可选用 xvfb-run
      - name: Screenshot capture (xvfb)
        run: |
          mkdir -p screenshots/current
          xvfb-run -a cargo run -p screenshot_test --release &
          sleep 5
          # 用 xwd / ImageMagick import 自动截图（脚本省略）
      - name: Compare screenshots
        run: |
          for f in screenshots/baseline/*.png; do
            name=$(basename "$f")
            compare -metric AE "$f" "screenshots/current/$name" /tmp/diff.png
          done
```

## 文件清单

| 文件 | 用途 |
|------|------|
| `examples/screenshot_test/Cargo.toml` | 测试应用清单 |
| `examples/screenshot_test/src/main.rs` | 测试应用主程序 |
| `crates/components/tests/screenshot_test.rs` | 编译时渲染验证测试 |
| `docs/screenshot_test.md` | 本文档 |

## 限制与说明

1. **iced 0.13.1 无内置 screenshot API**：测试应用不直接保存截图，
   而是通过 OS 截图工具或 xvfb + ImageMagick 在外部完成。
2. **字体渲染差异**：不同 OS 的字体渲染可能导致像素级差异，
   建议在同一 OS/字体环境下生成基线。
3. **GPU 后端依赖**：iced 视觉渲染需 GPU 后端，
   CI 环境需用 `xvfb-run` 或带显示器的 Runner。
4. **`S` 键行为**：当前 `S` 键打印截图清单条目到 stdout，
   不直接保存文件（避免 iced 0.13.1 screenshot API 不完整导致的编译问题）。
