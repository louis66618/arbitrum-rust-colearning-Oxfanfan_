# ============================================================================
# Arbitrum Rust Learning Project - Task Runner Script
# ============================================================================

param(
    [string]$Task = "all",
    [switch]$Release = $false
)

$buildFlag = if ($Release) { "--release" } else { "" }

Write-Host "Arbitrum Rust Learning Project - Task Runner" -ForegroundColor Cyan
Write-Host "===========================================" -ForegroundColor Cyan

switch ($Task) {
    "1" {
        Write-Host "`nRunning Task-1: Hello Web3 (Alloy)" -ForegroundColor Green
        cargo run -p task1-hello-web3 $buildFlag
    }
    "2" {
        Write-Host "`nRunning Task-2: Balance Query (Ethers)" -ForegroundColor Green
        cargo run -p task2-balance-query $buildFlag
    }
    "3" {
        Write-Host "`nRunning Task-3: Gas Estimation (Ethers)" -ForegroundColor Green
        cargo run -p task3-gas-estimation $buildFlag
    }
    "4" {
        Write-Host "`nRunning Task-4: Transaction Script (Ethers)" -ForegroundColor Green
        cargo run -p task4-transaction $buildFlag
    }
    "all" {
        Write-Host "`nRunning All Tasks" -ForegroundColor Green
        Write-Host "`nTask-1: Hello Web3" -ForegroundColor Yellow
        cargo run -p task1-hello-web3 $buildFlag
        
        Write-Host "`nTask-2: Balance Query" -ForegroundColor Yellow
        cargo run -p task2-balance-query $buildFlag
        
        Write-Host "`nTask-3: Gas Estimation" -ForegroundColor Yellow
        cargo run -p task3-gas-estimation $buildFlag
        
        Write-Host "`nTask-4: Transaction Script" -ForegroundColor Yellow
        cargo run -p task4-transaction $buildFlag
    }
    "check" {
        Write-Host "`nChecking All Projects" -ForegroundColor Green
        cargo check --workspace
    }
    "test" {
        Write-Host "`nRunning All Tests" -ForegroundColor Green
        cargo test --workspace
    }
    "build" {
        Write-Host "`nBuilding All Projects" -ForegroundColor Green
        cargo build --workspace $buildFlag
    }
    default {
        Write-Host "`nUnknown task: $Task" -ForegroundColor Red
        Write-Host "`nUsage:" -ForegroundColor Yellow
        Write-Host "  .\run_tasks.ps1 1              # Run Task-1"
        Write-Host "  .\run_tasks.ps1 2              # Run Task-2"
        Write-Host "  .\run_tasks.ps1 3              # Run Task-3"
        Write-Host "  .\run_tasks.ps1 4              # Run Task-4"
        Write-Host "  .\run_tasks.ps1 all            # Run All Tasks"
        Write-Host "  .\run_tasks.ps1 check          # Check Compilation"
        Write-Host "  .\run_tasks.ps1 test           # Run Tests"
        Write-Host "  .\run_tasks.ps1 build          # Build Project"
        Write-Host "  .\run_tasks.ps1 1 -Release     # Run in Release Mode"
    }
}