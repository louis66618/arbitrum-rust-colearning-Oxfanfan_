// ============================================================================
// Alloy Provider 工厂
// 新一代 Web3 库，推荐用于新项目
// ============================================================================

use alloy::providers::ProviderBuilder;
use alloy::transports::http::Client;
use alloy::transports::http::Http;
use eyre::Result;

/// Alloy Provider 类型别名
pub type AlloyProvider = alloy::providers::RootProvider<Http<Client>>;

/// 创建 Alloy Provider 实例
pub async fn create_alloy_provider(rpc_url: &str) -> Result<AlloyProvider> {
    let rpc_url = rpc_url.parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);
    Ok(provider)
}

/// 便捷函数：创建 Arbitrum Sepolia Provider
pub async fn create_arbitrum_sepolia_provider() -> Result<AlloyProvider> {
    create_alloy_provider("https://sepolia-rollup.arbitrum.io/rpc").await
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_provider() {
        let result = create_arbitrum_sepolia_provider().await;
        assert!(result.is_ok());
    }
}
