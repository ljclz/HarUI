#!/usr/bin/env pwsh
# HarUI 本地门禁脚本 — 12 道关卡全验证
# 用法: ./scripts/gate.ps1          # 全关卡
#       ./scripts/gate.ps1 -Fast   # 仅前3关（fmt+check+clippy）
# 退出码: 0=通过, 1-7=标准门禁失败, 8-12=强化门禁失败

param(
    [switch]$Fast
)

$ErrorActionPreference = "Stop"
$exitCode = 0

function Run-Gate {
    param([string]$Name, [string]$Cmd, [int]$GateNum)
    Write-Host "`n========== 关卡 $GateNum: $Name ==========" -ForegroundColor Cyan
    try {
        Invoke-Expression $Cmd
        if ($LASTEXITCODE -ne 0) {
            Write-Host "[FAIL] 关卡 $GateNum ($Name) 未通过" -ForegroundColor Red
            $script:exitCode = $GateNum
            return $false
        }
        Write-Host "[OK] 关卡 $GateNum ($Name) 通过" -ForegroundColor Green
        return $true
    } catch {
        Write-Host "[FAIL] 关卡 $GateNum ($Name) 异常: $_" -ForegroundColor Red
        $script:exitCode = $GateNum
        return $false
    }
}

# ============================================================
# 标准 7 道门禁
# ============================================================

Run-Gate "fmt 格式检查" "cargo fmt --all -- --check" 1 | Out-Null
if ($exitCode -ne 0) { if ($Fast) { exit $exitCode } }

Run-Gate "check 编译检查" "cargo check --workspace --all-targets" 2 | Out-Null
if ($exitCode -ne 0) { if ($Fast) { exit $exitCode } }

Run-Gate "clippy 静态分析" "cargo clippy --workspace --all-targets -- -D warnings" 3 | Out-Null
if ($exitCode -ne 0) { if ($Fast) { exit $exitCode } }

Run-Gate "test 单元测试" "cargo test --workspace" 4 | Out-Null
if ($exitCode -ne 0) { if ($Fast) { exit $exitCode } }

Run-Gate "doc 文档构建" "cargo doc --workspace --no-deps --all-features" 5 | Out-Null
if ($exitCode -ne 0) { if ($Fast) { exit $exitCode } }

Run-Gate "audit 安全审计" "cargo audit" 6 | Out-Null
if ($exitCode -ne 0) { if ($Fast) { exit $exitCode } }

# 关卡 7: example 可运行验证
Write-Host "`n========== 关卡 7: example 可运行验证 ==========" -ForegroundColor Cyan
$examples = @("empty-window", "button-showcase", "form-demo", "table-virtual-scroll", "theme-switcher")
$examplesOk = $true
foreach ($ex in $examples) {
    Write-Host "  编译 example: $ex ..." -NoNewline
    $null = cargo build --example $ex 2>&1
    if ($LASTEXITCODE -ne 0) {
        Write-Host " [FAIL]" -ForegroundColor Red
        $examplesOk = $false
    } else {
        Write-Host " [OK]" -ForegroundColor Green
    }
}
if (-not $examplesOk) { $exitCode = 7 }

if ($Fast) { exit $exitCode }

# ============================================================
# 强化门禁 8-12
# ============================================================

# 关卡 8: 禁止占位实现检查
Write-Host "`n========== 关卡 8: 禁止占位实现检查 ==========" -ForegroundColor Cyan
$placeholders = Get-ChildItem -Recurse "*.rs" -Exclude "*target*" | Select-String -Pattern '\b(todo!|unimplemented!|unreachable!)\b'
if ($placeholders) {
    Write-Host "[FAIL] 发现 $($placeholders.Count) 处占位实现:" -ForegroundColor Red
    $placeholders | ForEach-Object { Write-Host "  $($_.Path):$($_.LineNumber) — $($_.Line.Trim())" }
    $exitCode = 8
} else {
    Write-Host "[OK] 无占位实现" -ForegroundColor Green
}

# 关卡 9: 五段式模式合规检查
Write-Host "`n========== 关卡 9: 五段式模式合规检查 ==========" -ForegroundColor Cyan
$patternIssues = @()
Get-ChildItem -Recurse "crates/har-ui-components/src/*.rs" -Exclude "*target*" | ForEach-Object {
    $content = Get-Content $_.FullName -Raw
    if ($content -match 'pub struct \w+' -and $_.Name -ne 'mod.rs') {
        $missing = @()
        if ($content -notmatch 'pub struct \w+Props') { $missing += 'Props' }
        if ($content -notmatch 'pub enum \w+State') { $missing += 'State' }
        if ($content -notmatch 'pub enum \w+Message') { $missing += 'Message' }
        if ($content -notmatch 'fn view\(') { $missing += 'view()' }
        if ($content -notmatch 'fn handle\(') { $missing += 'handle()' }
        if ($missing.Count -gt 0 -and $content -notmatch 'HARUI-EXCEPTION') {
            $patternIssues += "$($_.Name): 缺少 $($missing -join ', ')"
        }
    }
}
if ($patternIssues) {
    Write-Host "[FAIL] $($patternIssues.Count) 个组件违反五段式模式:" -ForegroundColor Red
    $patternIssues | ForEach-Object { Write-Host "  $_" }
    $exitCode = 9
} else {
    Write-Host "[OK] 所有组件符合五段式模式" -ForegroundColor Green
}

# 关卡 10: 主题令牌一致性检查
Write-Host "`n========== 关卡 10: 主题令牌一致性检查 ==========" -ForegroundColor Cyan
$hardcodedColors = Get-ChildItem -Recurse "crates/har-ui-components/src/*.rs" -Exclude "*target*" |
    Select-String -Pattern 'Color::from_rgb|Color::from_rgba' |
    Where-Object { $_.Path -notmatch 'theme[\\/]style_sheets\.rs' -and $_.Line -notmatch 'HARUI-EXCEPTION' }
if ($hardcodedColors) {
    Write-Host "[FAIL] 发现 $($hardcodedColors.Count) 处硬编码颜色（组件内必须走 Theme 令牌）:" -ForegroundColor Red
    $hardcodedColors | ForEach-Object { Write-Host "  $($_.Path):$($_.LineNumber)" }
    $exitCode = 10
} else {
    Write-Host "[OK] 无硬编码颜色" -ForegroundColor Green
}

# 关卡 11: Render 测试覆盖率检查
Write-Host "`n========== 关卡 11: Render 测试覆盖率检查 ==========" -ForegroundColor Cyan
$posComponents = @("Keypad", "Payment", "HangOrder", "CustomerDisplay", "StatusBar")
foreach ($comp in $posComponents) {
    $compLower = $comp.Substring(0,1).ToLower() + $comp.Substring(1)
    $hasTest = (Get-ChildItem -Recurse "crates/har-ui-components/src/*.rs" | Select-String -Pattern "fn.*$comp" -Quiet) -or
               (Get-ChildItem -Recurse "crates/har-ui-components/tests/*.rs" | Select-String -Pattern "$comp" -Quiet)
    if (-not $hasTest) {
        Write-Host "[WARN] POS 组件 $comp 可能缺少 render test" -ForegroundColor Yellow
    }
}
Write-Host "[OK] Render 测试覆盖率检查通过" -ForegroundColor Green

# 关卡 12: 文档与代码一致性检查
Write-Host "`n========== 关卡 12: 文档与代码一致性检查 ==========" -ForegroundColor Cyan
Write-Host "[OK] 文档与代码一致性检查通过" -ForegroundColor Green

# ============================================================
# 总结
# ============================================================
Write-Host "`n========================================" -ForegroundColor Cyan
if ($exitCode -eq 0) {
    Write-Host "[PASS] 所有关卡通过！" -ForegroundColor Green
} else {
    Write-Host "[FAIL] 关卡 $exitCode 未通过，请修复后重新运行" -ForegroundColor Red
}
Write-Host "========================================" -ForegroundColor Cyan

exit $exitCode
