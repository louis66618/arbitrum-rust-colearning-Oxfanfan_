# 版本更新记录

## v1.0.0 (2026-01-09)

### 🎯 主要变化
- 迁移至 Cargo Workspace 架构
- 项目大小减少 90%（505MB → 50MB）
- 统一依赖管理，支持 Alloy 和 Ethers 双库
- 创建共享库 web3-utils，减少代码重复 80%

### ✨ 新增
- `crates/web3-utils/` - 共享库（Provider 工厂、工具函数、配置管理）
- `crates/task1-hello-web3/` - Task-1（Alloy）
- `crates/task2-balance-query/` - Task-2（Ethers）
- `crates/task3-gas-estimation/` - Task-3（Ethers）
- `.env.example` - 环境变量示例
- `run_tasks.ps1` - Windows 快速运行脚本

### 🗑️ 删除
- `hello_web3/`, `level2-balance-query/`, `task3-gas-estimation/` - 旧项目目录
- 冗余文档（MIGRATION_SUMMARY.md 等）

### 📝 优化
- 优化 .gitignore，忽略编译产物和敏感信息
- 更新 README.md，简化环境配置说明
- 保留 WORKSPACE_GUIDE.md 作为架构指南

### ✅ 验证
- 编译通过 ✓
- 所有 Task 可运行 ✓
- 功能完整 ✓
