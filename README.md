# arbitrum-rust-colearning-louis
HighQuest Arbitrum的共学营学习项目

# arbitrum-rust-colearning-louis
> HighQuest Arbitrum 共学营学习项目实践记录

---

## 🚀 任务进度 (Task Progress)

- [x] **Task-1: Hello Web3 - 环境搭建与基础链上连接**
- [ ] Task-2: Rust 基础与 Arbitrum 合约交互
- [ ] Task-3: Stylus 合约开发实战

---

## 🛠 Task-1 实践笔记

在搭建 Arbitrum Sepolia 测试网环境时，由于网络环境（VPN）限制，遇到并解决了以下问题：

### 1. 钱包网络配置
* **痛点**：通过 `chainlist.org` 自动添加网络时，受 VPN 影响，MetaMask 往往无法弹出确认框。
* **对策**：采用手动配置 RPC 参数。
    * **Chain ID**: `421614`
    * **RPC URL**: `https://endpoints.omniatech.io/v1/arbitrum/sepolia/public`
    * **Explorer**: [https://sepolia.arbiscan.io](https://sepolia.arbiscan.io)
> **相关截图：**
![Gemini咨询与参数确认](./docs/metamask_setup.png)

### 2. 测试币申领 (Faucet)
* **Alchemy 限制**：领水页面因 IP 识别问题多次失败。
* **成功路径**：使用 [Sepolia PoW Faucet](https://sepolia-faucet.pk910.de/) 通过本地算力挖矿获得 L1 Sepolia ETH，随后通过 [Arbitrum Bridge](https://bridge.arbitrum.io/) 成功跨链至 L2。
> **相关截图：**
![挖矿操作记录](./docs/sepolia_faucet_mining.png)
![跨链操作记录](./docs/faucet_bridge.png)

### 3. Rust 与 C++ 编译环境安装 (Windows 11)
* **环境安装**：通过 Windows 官方包管理器 `winget` 一键安装 Rust 工具链：`winget install Rustlang.Rustup`。
* **解决 C++ 依赖 (Critical)**：Rust 在 Windows 编译 `ethers` 等 Web3 库时依赖 C++ 生成工具。通过手动安装 **Visual Studio Build Tools 2022** 并勾选 **“使用 C++ 的桌面开发”** 工作负载，成功解决了 `Linker` 缺失导致的编译错误，安装好后重启电脑。
> **相关截图：**
![Gemini咨询与命令确认](./docs/fast_command_install.png)
![验证环境是否安装成功](./docs/rust_env_check.png)
![C++依赖缺失](./docs/build_error_msvc.png)

### 4. Hello Web3 代码实现与排坑
* **RPC 限流 (429 Error)**：初次运行程序时，由于公共 RPC 节点（Omniatech）限流，导致 `Too many requests` 报错。
* **对策**：将 `rpc_url` 更换为 Arbitrum 官方 RPC 节点，程序成功通过编译并实时读取链上数据。
* **验证结果**：
  > 🚀 恭喜！Hello Web3 运行成功！  
  > 📍 当前 Arbitrum Sepolia 的最新区块高度是: 23168096
> **相关截图：**
![hello_web3运行成功](./docs/hello_web3_success.png)

---

📁 仓库说明
* **/docs**: 存放任务相关的操作截图及详细说明文档。
* **/hello_web3**: Task-1 的 Rust 工程源代码。
* **LICENSE**: 本项目采用 MIT 开源许可证。

📜 许可证 (License)
本项目采用 MIT License。详情请参阅 LICENSE 文件。