use ethers::prelude::*;
use std::convert::TryFrom;
use eyre::Result;
use ethers::utils; 
#[allow(unused_imports)]
use ethers::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. 设置 RPC URL
    // 使用你之前验证成功的 Arbitrum Sepolia 官方节点
    let rpc_url = "https://sepolia-rollup.arbitrum.io/rpc";
    
    // 2. 实例化 Provider
    let provider = Provider::<Http>::try_from(rpc_url)?;

    // 3. 设置要查询的地址
    // 建议替换为你自己的钱包地址，以便截图证明
    let target_address = "0xa8fF167e4f362B54FF612546a782A301BD521a0B".parse::<Address>()?;

    println!("正在查询 Arbitrum Sepolia 地址余额...");
    println!("📍 地址: {:?}", target_address);

    // 4. 调用 get_balance 查询余额 (返回单位是 U256 类型的 wei)
    let balance_wei: U256 = provider.get_balance(target_address, None).await?;

    // 5. 将 wei 转换为 ETH (可读格式)
    // ethers 提供 utils::format_ether 工具，自动处理 10^18 的换算
    let balance_eth = utils::format_ether(balance_wei);

    println!("--------------------------------------------------");
    println!("✅ 查询成功！");
    println!("💰 余额 (wei): {}", balance_wei);
    println!("💎 余额 (ETH): {}", balance_eth);
    println!("--------------------------------------------------");

    Ok(())
}