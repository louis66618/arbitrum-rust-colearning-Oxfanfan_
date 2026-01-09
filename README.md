# arbitrum-rust-colearning-louis
HackQuest Arbitrum的共学营学习项目

---

## 🚀 任务进度 (Task Progress)

- [x] **Task-1: Hello Web3 - 环境搭建与基础链上连接**
- [x] **Task-2: 查询 Arbitrum 测试网地址余额**
- [x] **Task-3: 计算 Arbitrum 转账 Gas 费用**

---

## 📦 项目架构升级

本项目已升级为 **Cargo Workspace** 架构，支持 Alloy 和 Ethers 双库共存：

```
crates/
├── web3-utils/              # 共享库（Provider 工厂、工具函数、配置管理）
├── task1-hello-web3/        # Task-1: 基础连接（Alloy）
├── task2-balance-query/     # Task-2: 余额查询（Ethers）
└── task3-gas-estimation/    # Task-3: Gas 估算（Ethers）
```

### 快速运行

```bash
# 运行单个 Task
cargo run -p task1-hello-web3
cargo run -p task2-balance-query
cargo run -p task3-gas-estimation

# 或使用 PowerShell 脚本（Windows）
.\run_tasks.ps1 1              # Task-1
.\run_tasks.ps1 all            # 全部
```

详见 `WORKSPACE_GUIDE.md` 和 `QUICK_REFERENCE.md`。

---

## 🛠 Task-1 实践笔记

### 项目实现

本 Task 使用 **Alloy 0.1**（新一代 Web3 框架）实现，代码位置：`crates/task1-hello-web3/src/main.rs`

**核心功能**：连接 Arbitrum Sepolia 测试网，获取最新区块高度

```rust
let provider = ProviderBuilder::new().on_http(rpc_url);
let block_num = provider.get_block_number().await?;
```

### 环境搭建排坑记录

### 1. 钱包网络配置
* **痛点**：通过 `chainlist.org` 自动添加网络时，受 VPN 影响，MetaMask 往往无法弹出确认框。
* **对策**：采用手动配置 RPC 参数。
    * **Chain ID**: `421614`
    * **RPC URL**: `https://endpoints.omniatech.io/v1/arbitrum/sepolia/public`
    * **Explorer**: [https://sepolia.arbiscan.io](https://sepolia.arbiscan.io)
> **相关截图：**
![测试网参数确认](./docs/task1/metamask_setup.png)

### 2. 测试币申领 (Faucet)
* **Alchemy 限制**：领水页面因 IP 识别问题多次失败。
* **成功路径**：使用 [Sepolia PoW Faucet](https://sepolia-faucet.pk910.de/) 通过本地算力挖矿获得 L1 Sepolia ETH，随后通过 [Arbitrum Bridge](https://bridge.arbitrum.io/) 成功跨链至 L2。
> **相关截图：**
![挖矿操作记录](./docs/task1/sepolia_faucet_mining.png)
![跨链操作记录](./docs/task1/faucet_bridge.png)

### 3. Rust 与 C++ 编译环境安装 (Windows 11)
* **环境安装**：通过 Windows 官方包管理器 `winget` 一键安装 Rust 工具链：`winget install Rustlang.Rustup`。
* **解决 C++ 依赖 (Critical)**：Rust 在 Windows 编译 `ethers` 等 Web3 库时依赖 C++ 生成工具。通过手动安装 **Visual Studio Build Tools 2022** 并勾选 **“使用 C++ 的桌面开发”** 工作负载，成功解决了 `Linker` 缺失导致的编译错误，安装好后重启电脑。
> **相关截图：**
![Gemini咨询与命令确认](./docs/task1/fast_command_install.png)
![验证环境是否安装成功](./docs/task1/rust_env_check.png)
![C++依赖缺失](./docs/task1/build_error_msvc.png)

### 4. Hello Web3 代码实现与排坑
* **RPC 限流 (429 Error)**：初次运行程序时，由于公共 RPC 节点（Omniatech）限流，导致 `Too many requests` 报错。
* **对策**：将 `rpc_url` 更换为 Arbitrum 官方 RPC 节点，程序成功通过编译并实时读取链上数据。
* **验证结果**：
  > 🚀 恭喜！Hello Web3 运行成功！  
  > 📍 当前 Arbitrum Sepolia 的最新区块高度是: 23168096
> **相关截图：**
![hello_web3运行成功](./docs/task1/hello_web3_success.png)

---

## 🛠 Task-2 实践笔记：查询 Arbitrum 测试网地址余额

### 项目实现

本 Task 使用 **Ethers 2.0**（成熟稳定库）实现，代码位置：`crates/task2-balance-query/src/main.rs`

**核心功能**：查询指定地址的 ETH 余额，并转换为可读格式

```rust
let provider = Provider::<Http>::try_from(rpc_url)?;
let balance = provider.get_balance(address, None).await?;
let eth = format_ether(balance);
```

### 功能实现
* **核心逻辑**：编写 Rust 函数，通过 `ethers-rs` 库连接 Arbitrum Sepolia 节点，查询指定地址的 ETH 余额。
* **单位转换**：利用 `ethers::utils::format_ether` 将余额从 **wei** 转换为可读的 **ETH** 格式。

### 2. 编译排坑
* **报错处理**：初次编译遇到 `use of unresolved module or unlinked crate utils` 错误。
* **解决**：确认 `utils` 是 `ethers` 的子模块，通过 `use ethers::utils;` 导入后解决。

### 3. 运行结果验证
* **查询地址**：`0xa8ff167e4f362b54ff612546a782a301bd521a0b`
* **查询余额**：约 `0.337 ETH`
* **相关截图**：
![Task-2 运行成功截图](./docs/task2/balance_result.png)

---

## 🛠 Task-3 实践笔记：计算 Arbitrum 转账 Gas 费用

### 项目实现

本 Task 使用 **Ethers 2.0** 实现，代码位置：`crates/task3-gas-estimation/src/main.rs`

**核心功能**：动态获取 Gas 价格，计算标准转账的预估费用

```rust
let gas_price = provider.get_gas_price().await?;
let gas_limit = U256::from(21000);
let fee = gas_price * gas_limit;
```

### 功能实现
* **动态获取 Gas Price**：拒绝硬编码，通过 `provider.get_gas_price()` 实时从 Arbitrum Sepolia 获取链上 Gas 价格（单位：wei），确保计算结果的实时性。
* **物理公式计算**：严格执行以下公式进行预估计算：
  $$Gas\ Fee = Gas\ Price \times Gas\ Limit$$
  针对标准 ETH 转账，设置 `Gas Limit` 为行业通用值 **21,000**。
* **环境变量管理**：引入 `dotenvy` 库，将 `TARGET_ADDRESS` 存储在根目录的 `.env` 文件中，实现配置与代码逻辑的解耦。

### 2. 运行结果验证
* **计算逻辑**：程序成功通过 RPC 节点获取实时价格，并完成从 `wei` 到 `ETH` 的单位换算。
* **相关截图**：
![Task-3 运行成功截图](./docs/task3/gas_result.png)

> **注**：由于 Arbitrum 属于 Layer 2 网络，其实际转账费用极低，预估费用通常在 $10^{-6}$ ETH 数量级。

---

## 🚀 如何运行 (How to Run)

### 前置要求
- Rust 1.75+ （支持 Edition 2024）
- Cargo

### 方式 1：使用 Cargo 命令

```bash
# 运行单个 Task
cargo run -p task1-hello-web3
cargo run -p task2-balance-query
cargo run -p task3-gas-estimation

# 检查编译
cargo check --workspace

# 构建项目
cargo build --workspace
```

### 方式 2：使用 PowerShell 脚本（Windows）

```powershell
# 运行单个 Task
.\run_tasks.ps1 1              # Task-1
.\run_tasks.ps1 2              # Task-2
.\run_tasks.ps1 3              # Task-3

# 运行所有 Tasks
.\run_tasks.ps1 all

# Release 模式
.\run_tasks.ps1 1 -Release
```

### 环境配置

克隆本项目后，请按照以下步骤操作：

1. **复制示例配置**：
   ```bash
   cp .env.example .env
   ```

2. **编辑配置文件**：在 `.env` 中修改你的钱包地址（可选，已有默认值）：
   ```text
   TARGET_ADDRESS=你的钱包地址
   ARBITRUM_SEPOLIA_RPC=https://sepolia-rollup.arbitrum.io/rpc
   ```

> **注意**：`.env` 文件包含敏感信息，已在 `.gitignore` 中配置，不会被提交到 Git。
📁 仓库说明

* **/docs**: 存放任务相关的操作截图及详细说明文档。
* **/crates**: Workspace 中的所有项目
  * **web3-utils**: 共享库（Provider 工厂、工具函数、配置管理）
  * **task1-hello-web3**: Task-1 的 Rust 工程源代码（Alloy）
  * **task2-balance-query**: Task-2 的 Rust 工程源代码（Ethers）
  * **task3-gas-estimation**: Task-3 的 Rust 工程源代码（Ethers）
* **LICENSE**: 本项目采用 MIT 开源许可证。

### 文档导航

详见 `DOCS.md` 文档导航。

| 文档 | 说明 |
|------|------|
| `README.md` | 项目说明和排坑记录（本文件） |
| `WORKSPACE_GUIDE.md` | Workspace 架构详细指南 |
| `VERIFICATION_REPORT.md` | 功能验证报告 |
| `DOCS.md` | 文档导航和使用指南 |

📜 许可证 (License)
本项目采用 MIT License。详情请参阅 LICENSE 文件。