// ============================================================================
// Task-3: 计算 Arbitrum 转账 Gas 费用
// 使用 Ethers 库 + web3-utils 共享库
// ============================================================================

use web3_utils::{
    ethers_provider::create_arbitrum_sepolia_ethers_provider,
    utils::{format_ether_ethers, format_units_ethers, parse_address},
    Config,
};
use ethers::prelude::*;
use ethers::providers::Middleware;
use eyre::Result;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 1. 加载配置
    let config = Config::from_env()?;
    info!("🚀 Task-3: 动态计算转账 Gas 费用");
    info!("📍 目标地址: {}", config.target_address);

    // 2. 创建 Provider
    let provider = create_arbitrum_sepolia_ethers_provider()?;

    // 3. 解析地址
    let _target_address = parse_address(&config.target_address)?;

    // 4. 获取实时 Gas Price
    let gas_price = provider.get_gas_price().await?;

    // 5. 设置标准转账 Gas Limit
    let gas_limit = U256::from(21000);

    // 6. 计算预估费用
    let fee_wei = gas_price * gas_limit;

    // 7. 转换单位
    let fee_eth = format_ether_ethers(fee_wei);
    let gas_price_gwei = format_units_ethers(gas_price, "gwei")?;

    info!("✅ 数据获取与计算完成:");
    info!("📈 当前实时 Gas Price: {} Gwei ({} wei)", gas_price_gwei, gas_price);
    info!("⛽ 标准 Gas Limit: {}", gas_limit);
    info!("💰 预估总 Gas 费用: {} ETH", fee_eth);

    Ok(())
}
