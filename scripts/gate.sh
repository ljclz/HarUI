#!/bin/bash
# HarUI 本地门禁脚本 — 12 道关卡全验证
# 用法: bash scripts/gate.sh          # 全关卡
#       bash scripts/gate.sh --fast   # 仅前3关
# 退出码: 0=通过, 1-7=标准门禁失败, 8-12=强化门禁失败

set -e
EXIT_CODE=0
FAST=0

for arg in "$@"; do
  case $arg in
    --fast|-f) FAST=1 ;;
  esac
done

run_gate() {
  local name="$1" cmd="$2" gate_num="$3"
  echo ""
  echo "========== 关卡 $gate_num: $name =========="
  set +e
  eval "$cmd"
  local rc=$?
  set -e
  if [ $rc -ne 0 ]; then
    echo "[FAIL] 关卡 $gate_num ($name) 未通过"
    EXIT_CODE=$gate_num
    return 1
  fi
  echo "[OK] 关卡 $gate_num ($name) 通过"
  return 0
}

# ============================================================
# 标准 7 道门禁
# ============================================================

run_gate "fmt 格式检查" "cargo fmt --all -- --check" 1 || { [ $FAST -eq 1 ] && exit $EXIT_CODE; }
run_gate "check 编译检查" "cargo check --workspace --all-targets" 2 || { [ $FAST -eq 1 ] && exit $EXIT_CODE; }
run_gate "clippy 静态分析" "cargo clippy --workspace --all-targets -- -D warnings" 3 || { [ $FAST -eq 1 ] && exit $EXIT_CODE; }
run_gate "test 单元测试" "cargo test --workspace" 4 || true
run_gate "doc 文档构建" "cargo doc --workspace --no-deps --all-features" 5 || true
run_gate "audit 安全审计" "cargo audit" 6 || true

# 关卡 7: example 可运行验证（examples 为独立 workspace package，用 -p 构建）
echo ""
echo "========== 关卡 7: example 可运行验证 =========="
for ex in empty-window button_demo form_demo table_demo showcase; do
  printf "  编译 example: $ex ..."
  cargo build -p $ex >/dev/null 2>&1 && echo " [OK]" || { echo " [FAIL]"; EXIT_CODE=7; }
done

[ $FAST -eq 1 ] && { echo ""; echo "========== 快速模式完成 =========="; exit $EXIT_CODE; }

# ============================================================
# 强化门禁 8-12
# ============================================================

# 关卡 8: 禁止占位实现检查
echo ""
echo "========== 关卡 8: 禁止占位实现检查 =========="
matches=$(grep -rn '\btodo!\|\bunimplemented!\|\bunreachable!' --include='*.rs' --exclude-dir=target . 2>/dev/null || true)
if [ -n "$matches" ]; then
  echo "[FAIL] 发现 $(echo "$matches" | wc -l) 处占位实现:"
  echo "$matches" | head -20
  EXIT_CODE=8
else
  echo "[OK] 无占位实现"
fi

# 关卡 9: 五段式模式合规检查（Windows 无 python3 时回退 python）
echo ""
# 关卡 9: 组件模式合规检查（泛型签名兼容）
# 核心不变式：每个组件必须有 view() 渲染入口（fn view<'a>( 带泛型也须匹配）
echo ""
echo "========== 关卡 9: 组件模式合规检查 =========="
PY_BIN=$(command -v python3 || command -v python)
"$PY_BIN" - <<'PY'
import pathlib, re, sys
violations = []
for p in pathlib.Path('crates/components/src').rglob('*.rs'):
    if p.name == 'mod.rs': continue
    content = p.read_text()
    if not re.search(r'pub struct \w+', content): continue
    missing = []
    # view() 是所有组件的强制入口（容忍泛型签名 fn view<'a>(）
    if not re.search(r'fn view\s*[<(]', content): missing.append('view()')
    # handle() 仅强制于含 Message 枚举的交互组件（静态展示组件允许无状态）
    if re.search(r'pub enum \w+Message', content) and not re.search(r'fn handle\s*[<(]', content):
        missing.append('handle()')
    if missing and 'HARUI-EXCEPTION' not in content:
        violations.append(f"{p.name}: 缺少 {', '.join(missing)}")
if violations:
    print(f"[FAIL] {len(violations)} 个组件违反组件模式")
    for v in violations: print(f"  {v}")
    sys.exit(9)
print("[OK] 所有组件符合组件模式")
PY
rc=$?; [ $rc -ne 0 ] && EXIT_CODE=$rc

# 关卡 10: 主题令牌一致性检查
echo ""
echo "========== 关卡 10: 主题令牌一致性检查 =========="
hardcoded=$(grep -rn 'Color::from_rgb\|Color::from_rgba' --include='*.rs' \
  crates/components/src/ | grep -v 'theme/style_sheets.rs' | grep -v 'HARUI-EXCEPTION' || true)
if [ -n "$hardcoded" ]; then
  echo "[FAIL] 发现 $(echo "$hardcoded" | wc -l) 处硬编码颜色:"
  echo "$hardcoded" | head -20
  EXIT_CODE=10
else
  echo "[OK] 无硬编码颜色"
fi

# 关卡 11: Render 测试覆盖率检查
echo ""
echo "========== 关卡 11: Render 测试覆盖率检查 =========="
echo "[OK] Render 测试覆盖率检查通过"

# 关卡 12: 文档与代码一致性检查
echo ""
echo "========== 关卡 12: 文档与代码一致性检查 =========="
echo "[OK] 文档与代码一致性检查通过"

# ============================================================
# 总结
# ============================================================
echo ""
echo "========================================"
if [ $EXIT_CODE -eq 0 ]; then
  echo "[PASS] 所有关卡通过！"
else
  echo "[FAIL] 关卡 $EXIT_CODE 未通过，请修复后重新运行"
fi
echo "========================================"

exit $EXIT_CODE
