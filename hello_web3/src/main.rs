// 导入 Alloy 预选件：Provider 用于与节点交互，ProviderBuilder 是构建连接的工厂
use alloy::providers::{Provider, ProviderBuilder};
// 导入 eyre 的 Result 类型，用于处理可能出现的各种网络或解析错误
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // --------------------------------------------------
    // 1. 配置 RPC 节点地址
    // --------------------------------------------------
    // 我们使用你之前排坑成功的 Arbitrum Sepolia 官方节点
    // .parse() 会将字符串转换为 Alloy 内部使用的 Url 类型
    let rpc_url = "https://sepolia-rollup.arbitrum.io/rpc".parse()?;

    // --------------------------------------------------
    // 2. 构建 Provider 实例
    // --------------------------------------------------
    // Provider 是我们与区块链对话的“翻译官”
    // ProviderBuilder::new() 创建一个空白工厂
    // .on_http(rpc_url) 指定我们通过 HTTP 协议连接到上述节点
    // 相比 ethers，Alloy 的这种构建方式更容易后续添加身份验证 (Auth) 或重试 (Retry) 机制
    let provider = ProviderBuilder::new().on_http(rpc_url);

    println!("--------------------------------------------------");
    println!("📡 正在尝试使用 Alloy 框架连接 Arbitrum Sepolia...");

    // --------------------------------------------------
    // 3. 异步调用：获取最新区块号
    // --------------------------------------------------
    // .get_block_number() 发送一个 eth_blockNumber 的 JSON-RPC 请求
    // .await 表示程序会在此等待节点返回数据，而不会阻塞整个系统线程
    // 如果之前遇到的 429 限流报错，依然会在这里抛出异常并由 Result 捕获
    match provider.get_block_number().await {
        Ok(block_num) => {
            println!("✅ 连接成功！");
            println!("🚀 Hello Web3 运行顺利！");
            println!("📍 当前最新区块高度是: {}", block_num);
        }
        Err(e) => {
            // 如果连接失败，这里会打印出详细的错误原因，例如网络超时或限流
            eprintln!("❌ 获取区块高度失败: {:?}", e);
        }
    }
    println!("--------------------------------------------------");

    Ok(())
}