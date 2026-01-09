// ============================================================================
// Task-4: Arbitrum 测试网转账脚本
// ============================================================================

use web3_utils::{
    ethers_provider::create_arbitrum_sepolia_ethers_provider,
    utils::parse_address,
    Config,
};
use ethers::prelude::*;
// 1. 显式引入 parse_ether 解决第一个报错
use ethers::utils::parse_ether;
// 2. 引入具体类型以辅助编译器推断
use ethers::providers::{Http, PendingTransaction}; 
use eyre::{eyre, Result};
use tracing::info;

// Arbitrum Sepolia Chain ID
const CHAIN_ID: u64 = 421614;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    // 1. 加载配置
    let config = Config::from_env()?;
    
    // 获取私钥
    let private_key = config.private_key
        .ok_or_else(|| eyre!("❌ 未在 .env 中找到 PRIVATE_KEY，无法执行签名交易"))?;

    // 2. 初始化钱包
    let wallet = private_key
        .parse::<LocalWallet>()?
        .with_chain_id(CHAIN_ID);

    info!("🔑 钱包加载成功，地址: {:?}", wallet.address());

    // 3. 连接 Provider 并构建客户端
    // create_arbitrum_sepolia_ethers_provider 返回的是 Provider<Http>
    let provider = create_arbitrum_sepolia_ethers_provider()?;
    let client = SignerMiddleware::new(provider, wallet);

    // 4. 准备交易参数
    let to_address = parse_address(&config.target_address)?;
    let transfer_amount = parse_ether("0.0001")?; // ✅ parse_ether 现在可用了

    info!("📍 目标地址: {:?}", to_address);
    info!("💸 转账金额: 0.0001 ETH");

    // 5. 估算 Gas
    let gas_price = client.get_gas_price().await?;
    let gas_limit = 21000; // 标准转账 Gas Limit
    
    info!("⛽ 当前 Gas Price: {} wei", gas_price);
    
    // 检查余额
    let balance = client.get_balance(client.address(), None).await?;
    let cost = gas_price * gas_limit + transfer_amount;
    if balance < cost {
        return Err(eyre!("❌ 余额不足！当前余额: {}, 需要: {}", balance, cost));
    }

    // 6. 构造交易请求
    let tx = TransactionRequest::new()
        .to(to_address)
        .value(transfer_amount)
        .gas(gas_limit)
        .gas_price(gas_price);

    info!("🚀 正在发送交易，请稍候...");

    // 7. 发送交易并等待确认
    // 🛠️ 关键修复：显式标注类型 PendingTransaction<'_, Http>
    // 编译器无法自动推断 SignerMiddleware 内部的 Provider 类型，这里手动指定为 Http
    let pending_tx: PendingTransaction<'_, Http> = client.send_transaction(tx, None).await?;
    
    info!("⏳ 交易已广播，Hash: {:?}", pending_tx.tx_hash());
    info!("⏳ 等待区块确认...");

    let receipt = pending_tx.await?
        .ok_or_else(|| eyre!("❌ 交易未被打包 (Dropped)"))?;

    // 8. 输出结果
    info!("✅ 交易成功！");
    info!("🔗 交易哈希: {:?}", receipt.transaction_hash);
    info!("📦 区块高度: {:?}", receipt.block_number);
    info!("🌍 浏览器查看: https://sepolia.arbiscan.io/tx/{:?}", receipt.transaction_hash);

    Ok(())
}