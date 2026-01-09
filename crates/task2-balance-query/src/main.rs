// ============================================================================
// Task-2: 查询 Arbitrum 测试网地址余额
// 使用 Ethers 库 + web3-utils 共享库
// ============================================================================

use web3_utils::{
    ethers_provider::create_arbitrum_sepolia_ethers_provider,
    utils::{format_ether_ethers, parse_address},
    Config,
};
use ethers::providers::Middleware;
use eyre::Result;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 1. 加载配置
    let config = Config::from_env()?;
    info!("📍 目标地址: {}", config.target_address);

    // 2. 创建 Provider
    let provider = create_arbitrum_sepolia_ethers_provider()?;

    // 3. 解析地址
    let target_address = parse_address(&config.target_address)?;

    info!("正在查询 Arbitrum Sepolia 地址余额...");

    // 4. 查询余额
    let balance_wei = provider.get_balance(target_address, None).await?;

    // 5. 转换单位
    let balance_eth = format_ether_ethers(balance_wei);

    info!("✅ 查询成功！");
    info!("💰 余额 (wei): {}", balance_wei);
    info!("💎 余额 (ETH): {}", balance_eth);

    Ok(())
}
