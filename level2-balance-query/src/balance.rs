use ethers::prelude::*;
use eyre::Result;
use ethers::utils; 
use std::env;          // 🚀 必须添加这一行，否则无法使用 env::var
use dotenvy::dotenv;   // 引入 dotenv 加载工具    

#[tokio::main]
async fn main() -> Result<()> {
    // 1. 设置 RPC URL
    let rpc_url = "https://sepolia-rollup.arbitrum.io/rpc";
    
    // 2. 实例化 Provider
    let provider = Provider::<Http>::try_from(rpc_url)?;

    // 3. 加载配置并解析地址
    // 加载根目录或上级目录的 .env 文件
    dotenv().ok(); 
    
    // 从环境变量读取地址字符串
    let address_str = env::var("TARGET_ADDRESS")
        .expect("在 .env 文件中未找到 TARGET_ADDRESS，请检查根目录是否有该文件");
    
    // 解析地址字符串为 Address 类型
    let target_address = address_str.parse::<Address>()?;

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