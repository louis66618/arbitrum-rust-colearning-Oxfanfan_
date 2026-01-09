// crates/task5-contract-call/src/main.rs
// ============================================================================
// Task-5: 通用合约元数据读取 (支持 ERC20 / ERC721)
// ============================================================================

use web3_utils::{
    ethers_provider::create_arbitrum_sepolia_ethers_provider,
    utils::parse_address,
    Config,
};
use ethers::prelude::*;
use eyre::Result;
use std::sync::Arc;
use tracing::{info, warn};

// 1. 定义多套 ABI
// 使用 abigen! 一次性生成多个模块，分别对应不同的合约标准
abigen!(
    // 定义 ERC20 接口 (标准代币)
    ERC20,
    r#"[
        function name() external view returns (string)
        function symbol() external view returns (string)
        function decimals() external view returns (uint8)
        function totalSupply() external view returns (uint256)
    ]"#;

    // 定义 ERC721 接口 (NFT)
    // 注意：标准 ERC721 只有 name/symbol，没有 decimals
    ERC721,
    r#"[
        function name() external view returns (string)
        function symbol() external view returns (string)
    ]"#
);

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    // 加载配置
    let config = Config::from_env()?;
    
    // 初始化 Provider
    let provider = create_arbitrum_sepolia_ethers_provider()?;
    let client = Arc::new(provider);

    // 解析目标地址
    let contract_addr = parse_address(&config.contract_address)?;

    info!("🎯 目标合约类型: {}", config.contract_type);
    info!("📍 合约地址: {:?}", contract_addr);

    // 2. 根据配置决定调用哪套逻辑
    match config.contract_type.as_str() {
        "ERC20" => {
            fetch_erc20(contract_addr, client).await?;
        }
        "ERC721" => {
            fetch_erc721(contract_addr, client).await?;
        }
        _ => {
            // 默认回退方案
            warn!("⚠️ 未知的合约类型: {}, 将尝试按 ERC20 读取...", config.contract_type);
            fetch_erc20(contract_addr, client).await?;
        }
    }

    Ok(())
}

// --- 独立的 ERC20 处理逻辑 ---
async fn fetch_erc20(addr: Address, client: Arc<Provider<Http>>) -> Result<()> {
    // 实例化 ERC20 合约 (类型安全)
    let contract = ERC20::new(addr, client);
    
    info!("🚀 读取 ERC20 元数据...");
    
    // 异步调用合约方法
    // 使用 unwrap_or_else 增加容错性，防止某些非标准合约调用失败导致程序崩溃
    let name = contract.name().call().await.unwrap_or_else(|_| "Unknown Name".to_string());
    let symbol = contract.symbol().call().await.unwrap_or_else(|_| "Unknown Symbol".to_string());
    let decimals = contract.decimals().call().await.unwrap_or(18); // 默认精度 18
    let total_supply = contract.total_supply().call().await.unwrap_or(U256::zero());
    
    // 格式化输出
    let total_fmt = ethers::utils::format_units(total_supply, decimals as u32)?;

    info!("----------------------------------------");
    info!("📛 Name:      {}", name);
    info!("💲 Symbol:    {}", symbol);
    info!("🔢 Decimals:  {}", decimals);
    info!("💰 Total:     {} {}", total_fmt, symbol);
    info!("----------------------------------------");
    Ok(())
}

// --- 独立的 ERC721 处理逻辑 ---
async fn fetch_erc721(addr: Address, client: Arc<Provider<Http>>) -> Result<()> {
    // 实例化 ERC721 合约
    let contract = ERC721::new(addr, client);
    
    info!("🚀 读取 ERC721 (NFT) 元数据...");
    
    let name = contract.name().call().await
        .unwrap_or_else(|_| "Unknown Collection".to_string());
    let symbol = contract.symbol().call().await
        .unwrap_or_else(|_| "Unknown Symbol".to_string());
    
    info!("----------------------------------------");
    info!("📛 Collection: {}", name);
    info!("💲 Symbol:     {}", symbol);
    info!("🖼️ Type:       Non-Fungible Token (ERC721)");
    info!("----------------------------------------");
    Ok(())
}