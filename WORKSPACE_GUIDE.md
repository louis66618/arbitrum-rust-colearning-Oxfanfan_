# Arbitrum Rust 学习项目 - Workspace 架构指南

## 📦 项目结构

```
arbitrum-rust-colearning/
├── Cargo.toml                          # Workspace 根配置
├── .env                                # 全局环境变量
├── crates/
│   ├── web3-utils/                     # 共享库（核心）
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs                  # 库入口
│   │       ├── config.rs               # 配置管理
│   │       ├── alloy_provider.rs       # Alloy Provider 工厂
│   │       ├── ethers_provider.rs      # Ethers Provider 工厂
│   │       └── utils.rs                # 通用工具函数
│   │
│   ├── task1-hello-web3/               # Task-1: 基础连接（Alloy）
│   │   ├── Cargo.toml
│   │   └── src/main.rs
│   │
│   ├── task2-balance-query/            # Task-2: 余额查询（Ethers）
│   │   ├── Cargo.toml
│   │   └── src/main.rs
│   │
│   └── task3-gas-estimation/           # Task-3: Gas 估算（Ethers）
│       ├── Cargo.toml
│       └── src/main.rs
│
└── docs/                               # 文档和截图
```

## 🎯 核心优势

### 1. **双库共存架构**
- **Alloy**：新一代 Web3 框架，用于新项目和前沿功能
- **Ethers**：成熟稳定库，用于向后兼容和现有项目

### 2. **统一依赖管理**
- 所有依赖在 `Cargo.toml` 的 `[workspace.dependencies]` 中定义
- 版本一致，避免冲突
- 新增 task 时自动继承所有依赖

### 3. **代码复用**
- `web3-utils` 库提供：
  - Provider 工厂函数
  - 配置管理
  - 单位转换工具
  - 地址解析工具

### 4. **易于扩展**
- 新增 task 只需在 `crates/` 下创建新目录
- 自动继承 workspace 配置
- 可选择使用 Alloy 或 Ethers

## 🚀 快速开始

### 运行单个 Task

```bash
# Task-1: Hello Web3（Alloy）
cargo run -p task1-hello-web3

# Task-2: 余额查询（Ethers）
cargo run -p task2-balance-query

# Task-3: Gas 估算（Ethers）
cargo run -p task3-gas-estimation
```

### 运行所有 Task

```bash
cargo build --workspace
cargo test --workspace
```

### 添加新 Task

1. 在 `crates/` 下创建新目录：
   ```bash
   mkdir crates/task4-xxx
   ```

2. 创建 `Cargo.toml`：
   ```toml
   [package]
   name = "task4-xxx"
   version.workspace = true
   edition.workspace = true
   authors.workspace = true
   license.workspace = true

   [dependencies]
   web3-utils = { path = "../web3-utils" }
   # 选择 alloy 或 ethers
   alloy = { workspace = true }
   # 或
   ethers = { workspace = true }
   tokio = { workspace = true }
   eyre = { workspace = true }
   ```

3. 创建 `src/main.rs`，导入 `web3-utils` 中的工具

## 📝 环境变量配置

在项目根目录的 `.env` 文件中配置：

```env
# Arbitrum Sepolia RPC 端点（可选，有默认值）
ARBITRUM_SEPOLIA_RPC=https://sepolia-rollup.arbitrum.io/rpc

# 目标钱包地址（可选，有默认值）
TARGET_ADDRESS=0xa8fF167e4f362B54FF612546a782A301BD521a0B
```

## 🔧 web3-utils 库 API

### Provider 工厂

```rust
// Alloy
use web3_utils::alloy_provider::*;
let provider = create_arbitrum_sepolia_provider().await?;

// Ethers
use web3_utils::ethers_provider::*;
let provider = create_arbitrum_sepolia_ethers_provider()?;
```

### 工具函数

```rust
use web3_utils::utils::*;

// 单位转换
let eth = format_ether_ethers(wei);
let gwei = format_units_ethers(wei, "gwei")?;

// 地址解析
let addr = parse_address("0x...")?;
```

### 配置管理

```rust
use web3_utils::Config;

// 从环境变量加载
let config = Config::from_env()?;

// 或使用默认测试网配置
let config = Config::default_testnet();
```

## 📚 后续 Task 规划

- [ ] Task-4: 合约交互
- [ ] Task-5: 事件监听
- [ ] Task-6: 交易签名
- [ ] Task-7: 多链支持
- [ ] ...

每个新 task 都可以：
- 选择使用 Alloy 或 Ethers
- 复用 `web3-utils` 中的工具
- 贡献新的工具函数到 `web3-utils`

## 🎓 学习路径

1. **Task-1**：理解 Alloy 的 Provider 模式
2. **Task-2**：学习 Ethers 的链上查询
3. **Task-3**：掌握 Gas 费用计算
4. **Task-4+**：逐步深入合约交互、事件监听等高级功能

## 💡 最佳实践

1. **优先使用 web3-utils**：避免重复代码
2. **新功能先在 web3-utils 中实现**：便于其他 task 复用
3. **选择合适的库**：
   - Alloy：新项目、前沿功能
   - Ethers：稳定性优先、向后兼容
4. **保持配置集中**：所有 RPC URL、地址等放在 `.env` 中

---

**Happy Coding! 🚀**
