# ============================================================================
# Arbitrum Rust 学习项目 - 快速运行脚本
# ============================================================================

param(
    [string]$Task = "all",
    [switch]$Release = $false
)

$buildFlag = if ($Release) { "--release" } else { "" }

Write-Host "🚀 Arbitrum Rust 学习项目 - 任务运行器" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

switch ($Task) {
    "1" {
        Write-Host "`n▶️  运行 Task-1: Hello Web3 (Alloy)" -ForegroundColor Green
        cargo run -p task1-hello-web3 $buildFlag
    }
    "2" {
        Write-Host "`n▶️  运行 Task-2: 余额查询 (Ethers)" -ForegroundColor Green
        cargo run -p task2-balance-query $buildFlag
    }
    "3" {
        Write-Host "`n▶️  运行 Task-3: Gas 估算 (Ethers)" -ForegroundColor Green
        cargo run -p task3-gas-estimation $buildFlag
    }
    "all" {
        Write-Host "`n▶️  运行所有 Tasks" -ForegroundColor Green
        Write-Host "`n📍 Task-1: Hello Web3" -ForegroundColor Yellow
        cargo run -p task1-hello-web3 $buildFlag
        
        Write-Host "`n📍 Task-2: 余额查询" -ForegroundColor Yellow
        cargo run -p task2-balance-query $buildFlag
        
        Write-Host "`n📍 Task-3: Gas 估算" -ForegroundColor Yellow
        cargo run -p task3-gas-estimation $buildFlag
    }
    "check" {
        Write-Host "`n▶️  检查所有项目编译" -ForegroundColor Green
        cargo check --workspace
    }
    "test" {
        Write-Host "`n▶️  运行所有测试" -ForegroundColor Green
        cargo test --workspace
    }
    "build" {
        Write-Host "`n▶️  构建所有项目" -ForegroundColor Green
        cargo build --workspace $buildFlag
    }
    default {
        Write-Host "`n❌ 未知的任务: $Task" -ForegroundColor Red
        Write-Host "`n用法:" -ForegroundColor Yellow
        Write-Host "  .\run_tasks.ps1 1              # 运行 Task-1"
        Write-Host "  .\run_tasks.ps1 2              # 运行 Task-2"
        Write-Host "  .\run_tasks.ps1 3              # 运行 Task-3"
        Write-Host "  .\run_tasks.ps1 all            # 运行所有 Tasks"
        Write-Host "  .\run_tasks.ps1 check          # 检查编译"
        Write-Host "  .\run_tasks.ps1 test           # 运行测试"
        Write-Host "  .\run_tasks.ps1 build          # 构建项目"
        Write-Host "  .\run_tasks.ps1 1 -Release     # 以 Release 模式运行"
    }
}
