# HarUI 项目集成层强制门禁规范

> 适用范围：所有基于 iced 0.13 + HarUI 的 Rust GUI 项目
> 版本：v1.0  日期：2026-07-20
> 来源：菜场收银台 v0.1.0 工程实践提炼（1873 测试通过 / 0 panic）

---

## 一、目标

在 `cargo test` 之外建立"集成层强制门禁"，使每次提交/合并都必须通过 7 道关卡才能进入主分支。门禁不是装饰，是**抵御回归的最后一道防线**。

GUI 项目的回归风险尤其高：
- iced 0.13 API 闭坑（Padding 不支持 4 元素数组、scrollable 无 padding 方法等）
- 状态转移破坏（Message 处理顺序变更导致 UI 不更新）
- 像素级布局破坏（1366×768 下滚动条溢出、文本截断等）

---

## 二、七道门禁（gate.ps1）

| 顺序 | 关卡 | 命令 | 阻断条件 | 容许时长 |
|------|------|------|---------|---------|
| 1 | `fmt` | `cargo fmt --all -- --check` | 任何文件未格式化 | <10s |
| 2 | `check` | `cargo check --all-features --all-targets` | 编译错误 | <120s |
| 3 | `clippy` | `cargo clippy --all-features --all-targets -- -D warnings` | 任何 warning | <180s |
| 4 | `test` | `cargo test --all-features` | 任何测试失败 | <300s |
| 5 | `doc` | `cargo doc --all-features --no-deps` | 文档构建失败 / rustdoc 警告 | <120s |
| 6 | API 变更扫描 | `audit-api-changes.ps1` | 未在 `api-contracts.md` 备案的破坏性变更 | <5s |
| 7 | 契约测试 | `cargo test --test contract` | 违反 State/Message 行为契约 | <60s |

**全部通过**才允许合入主分支；任何一道失败即整体失败，禁止"跳过"。

> **注意**：GUI 项目必须用 `--all-features`，因为 `api/hardware/comm` feature 影响代码路径。
> 单 binary crate 不能用 `--workspace`，必须直接 `cargo test --all-features`。

---

## 三、门禁脚本骨架（PowerShell）

`scripts/gate.ps1`：

```powershell
[CmdletBinding()]
param(
    [switch]$SkipApiAudit  # 仅本地调试使用，CI 永远不传
)

$ErrorActionPreference = 'Stop'
$root = Split-Path $PSScriptRoot -Parent
Set-Location $root

function Invoke-Gate([string]$name, [scriptblock]$action) {
    Write-Host "==> [$name] start..." -ForegroundColor Cyan
    $sw = [System.Diagnostics.Stopwatch]::StartNew()
    & $action
    if ($LASTEXITCODE -ne 0) {
        Write-Host "==> [$name] FAILED (exit $LASTEXITCODE)" -ForegroundColor Red
        exit 1
    }
    $sw.Stop()
    Write-Host "==> [$name] OK ($($sw.Elapsed.TotalSeconds)s)" -ForegroundColor Green
}

Invoke-Gate 'fmt'    { cargo fmt --all -- --check }
Invoke-Gate 'check'  { cargo check --all-features --all-targets }
Invoke-Gate 'clippy' { cargo clippy --all-features --all-targets -- -D warnings }
Invoke-Gate 'test'   { cargo test --all-features }
Invoke-Gate 'doc'    { cargo doc --all-features --no-deps }
if (-not $SkipApiAudit) {
    Invoke-Gate 'api-audit' { & "$PSScriptRoot/audit-api-changes.ps1" }
}
Invoke-Gate 'contract' { cargo test --test contract }

Write-Host "All 7 gates passed." -ForegroundColor Green
```

---

## 四、API 变更扫描脚本骨架

`scripts/audit-api-changes.ps1`：扫描 `pub fn/struct/enum` 与 `pub enum AppMessage` 变体变化。

```powershell
[CmdletBinding()]
param()
$ErrorActionPreference = 'Stop'

# 抓取所有 pub 项签名 + AppMessage 变体
$pattern = '^\s*pub\s+(async\s+)?(fn|struct|enum|trait|type|const|static)\s+\w+'
$files = Get-ChildItem -Recurse -Include '*.rs' -Path 'src' |
    Where-Object { $_.FullName -notmatch '\\target\\' }

$api = foreach ($f in $files) {
    Select-String -Path $f.FullName -Pattern $pattern |
        ForEach-Object { "$($_.LineNumber):$($_.Line.Trim())" }
}
$current = ($api | Sort-Object) -join "`n"

# 与 baseline 对比
$baselineFile = 'scripts/pub-api-baseline.txt'
if (-not (Test-Path $baselineFile)) {
    $current | Out-File $baselineFile -Encoding utf8
    Write-Host "Baseline created." -ForegroundColor Yellow
    return
}

$baseline = Get-Content $baselineFile -Raw
$diff = Compare-Object $baseline.Split("`n") $current.Split("`n")

if ($diff) {
    $removed = $diff | Where-Object SideIndicator -eq '<='
    $added   = $diff | Where-Object SideIndicator -eq '=>'

    if ($removed) {
        Write-Host "[BREAKING] Removed/changed pub API:" -ForegroundColor Red
        $removed | ForEach-Object { Write-Host "  - $($_.InputObject)" }
        Write-Host "Action: update docs/api-contracts.md + bump MAJOR version" -ForegroundColor Red
        exit 2
    }
    if ($added) {
        Write-Host "[INFO] New pub API added:" -ForegroundColor Green
        $added | ForEach-Object { Write-Host "  + $($_.InputObject)" }
        Write-Host "Action: consider adding contract chapter; bump MINOR" -ForegroundColor Yellow
    }
    $current | Out-File $baselineFile -Encoding utf8
} else {
    Write-Host "No pub API changes." -ForegroundColor Green
}
```

---

## 五、契约测试入口骨架

`tests/contract.rs`：

```rust
//! 公共 API 行为契约测试
//! 每一条契约对应 docs/api-contracts.md 的一个章节

mod contract_state_transfer;   // 第 1 章：AppState 转移必须确定性
mod contract_message_handle;   // 第 2 章：AppMessage 处理必须幂等
mod component_login_view;      // 第 3 章：LoginView::view 必须无 panic
mod component_cart_store;      // 第 4 章：CartStore::push_product 必须更新 total_fee
mod api_user_login;            // 第 5 章：UserApi::login 必须返回 ApiResponse
// ... 其余章节
```

---

## 六、本地与 CI 接入

### 6.1 本地

```powershell
# 完整 7 关
.\scripts\gate.ps1

# 跳过 API 审计（仅本地调试）
.\scripts\gate.ps1 -SkipApiAudit
```

### 6.2 CI（GitHub Actions）

```yaml
- name: Run 7 Gates
  run: pwsh ./scripts/gate.ps1
  working-directory: 菜场收银台
```

---

## 七、门禁失败的处理规则

1. **禁止 `--force` 绕过**：除非用户显式同意，否则不允许 `git push --force` 推送未过门禁的提交。
2. **禁止"先合并后修"**：未过门禁的 PR 不得合并到主分支。
3. **失败时优先修复**：clippy warning 必须修复，禁止 `#[allow(...)]` 抑制（除非有 ADR 记录）。
4. **门禁耗时监控**：单关卡超时应触发告警，整套门禁应在 15 分钟内完成。
5. **GUI 特有规则**：
   - iced 0.13 API 闭坑清单（见 [03-concurrency-safety.md](03-concurrency-safety.md) 第七章）必须纳入审查
   - 像素验证测试失败必须人工复查截图，禁止直接标记 `#[ignore]`
   - State 转移测试失败必须排查是否引入了非确定性副作用

---

## 八、与其他规范的关系

- 本规范定义**如何强制执行**；
- [02-contract-audit.md](02-contract-audit.md) 定义**State/Message 契约如何编写**；
- [03-concurrency-safety.md](03-concurrency-safety.md) 定义**Task::perform 背后的并发约束**；
- [04-test-pyramid.md](04-test-pyramid.md) 定义**GUI 测试如何分层**。
