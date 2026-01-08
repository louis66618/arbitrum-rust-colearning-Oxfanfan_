#[allow(unused_imports)]
use ethers::prelude::*;
use std::env;
use dotenvy::dotenv;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. 加载根目录的 .env 文件 (确保文件名是以点开头的 .env)
    dotenv().ok(); 

    // 2. 连接 Arbitrum Sepolia 官方 RPC 节点
    let rpc_url = "https://sepolia-rollup.arbitrum.io/rpc";
    let provider = Provider::<Http>::try_from(rpc_url)?;

    // 3. 从环境变量获取目标地址 (用于展示)
    let target_addr_str = env::var("TARGET_ADDRESS")
        .expect("❌ 错误：在 .env 文件中未找到 TARGET_ADDRESS");
    let target_address = target_addr_str.parse::<Address>()?;

    println!("--------------------------------------------------");
    println!("🚀 Task-3: 动态计算转账 Gas 费用");
    println!("📍 目标地址: {:?}", target_address);

    // 4. 核心逻辑：动态获取实时 Gas Price (单位: wei)
    // 严禁硬编码数字，必须调用 provider 获取真实链上数据
    let gas_price = provider.get_gas_price().await?;
    
    // 5. 设置标准转账 Gas Limit
    // 对于标准的 ETH 转账，行业通用值（Gas Limit）为 21,000
    let gas_limit = U256::from(21000);

    // 6. 计算预估费用 (公式: Gas Price * Gas Limit)
    let fee_wei = gas_price * gas_limit;

    // 7. 转换单位用于显示 (wei -> ETH)
    let fee_eth = ethers::utils::format_ether(fee_wei);
    let gas_price_gwei = ethers::utils::format_units(gas_price, "gwei")?;

    println!("✅ 数据获取与计算完成:");
    println!("📈 当前实时 Gas Price: {} Gwei ({} wei)", gas_price_gwei, gas_price);
    println!("⛽ 标准 Gas Limit: {}", gas_limit);
    println!("💰 预估总 Gas 费用: {} ETH", fee_eth);
    println!("--------------------------------------------------");

    Ok(())
}