// 导入 ethers 库的预定义模块，包含连接链、处理数据等工具
use ethers::prelude::*;
// 导入标准库中用于转换类型的工具
use std::convert::TryFrom;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // 1. 更换为更稳定的公共节点（BlockPI 提供的 Arbitrum Sepolia 节点）
    // 原来的 omniatech 节点目前正处于限流状态
    // let rpc_url = "https://arbitrum-sepolia.blockpi.network/v1/rpc/public";
    
    // 如果上面的还不行，可以备选这个官方节点：
    let rpc_url = "https://sepolia-rollup.arbitrum.io/rpc";

    let provider = Provider::<Http>::try_from(rpc_url)?;

    println!("正在尝试连接 Arbitrum Sepolia 测试网...");

    // 2. 获取当前最新区块号
    // 报错位置曾在这里，因为节点拒绝了请求
    let block_number = provider.get_block_number().await?;

    println!("--------------------------------------------------");
    println!("🚀 恭喜！Hello Web3 运行成功！");
    println!("📍 当前 Arbitrum Sepolia 的最新区块高度是: {}", block_number);
    println!("--------------------------------------------------");

    Ok(())
}