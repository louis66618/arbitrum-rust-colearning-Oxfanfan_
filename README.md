# arbitrum-rust-colearning-louis
HighQuest Arbitrum的共学营学习项目

## 🚀 任务进度 (Task Progress)

- [x] **Task-1: Hello Web3 - 环境搭建与基础链上连接**
- [ ] Task-2: Rust 基础与 Arbitrum 合约交互 (Pending)
- [ ] Task-3: Stylus 合约开发实战 (Pending)

---

## 🛠 Task-1 实践记录：环境搭建

在 Task-1 的过程中，由于网络环境（VPN）等因素遇到了一些连接挑战，以下是解决方案：

### 1. 网络配置 (Arbitrum Sepolia Testnet)

**问题描述：** 在使用 [Chainlist](https://chainlist.org/) 自动添加 Arbitrum Sepolia 网络时，由于 VPN 代理或网络波动，MetaMask 往往无法正常响应弹出请求。

**解决方案：** 通过 MetaMask 设置手动添加自定义 RPC。以下是经过验证的配置参数：

| 参数名 | 取值 |
| :--- | :--- |
| **Network Name** | Arbitrum Sepolia |
| **New RPC URL** | `https://endpoints.omniatech.io/v1/arbitrum/sepolia/public` |
| **Chain ID** | `421614` |
| **Currency Symbol** | ETH |
| **Block Explorer** | [https://sepolia.arbiscan.io](https://sepolia.arbiscan.io) |

> **💡 笔记：** 手动输入参数不仅能避开 Chainlist 的前端响应问题，还能更直观地理解 L2 网络的链参数。

---

### 2. 测试币申领 (Faucet) 与跨链

**踩坑记录：** * **Alchemy Faucet:** 即使使用了 VPN，Alchemy 领水页面仍可能因为 IP 或账户限制导致领水失败。
* **PoW 方案 (成功案例):** 最终采用了 [Sepolia PoW Faucet](https://sepolia-faucet.pk910.de/)。通过本地算力挖矿获取 Sepolia L1 ETH。

**跨链操作：** 获取 L1 测试币后，通过官方 [Arbitrum Bridge](https://bridge.arbitrum.io/) 将资金从 Sepolia 跨链至 Arbitrum Sepolia。

> **✅ 状态确认：** 已成功获取测试币，钱包余额已更新，环境准备就绪。

---

## 📁 项目结构说明
* `/docs`: 学习笔记与截图
* `/src`: 任务相关的 Rust 代码或脚本

---

## 📜 许可证
[MIT](LICENSE)




