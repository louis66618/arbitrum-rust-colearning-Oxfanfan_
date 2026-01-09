// ============================================================================
// Ethers Provider 工厂
// 成熟稳定的 Web3 库，向后兼容
// ============================================================================

use ethers::prelude::*;
use eyre::Result;

/// 创建 Ethers Provider 实例
pub fn create_ethers_provider(rpc_url: &str) -> Result<Provider<Http>> {
    let provider = Provider::<Http>::try_from(rpc_url)?;
    Ok(provider)
}

/// 便捷函数：创建 Arbitrum Sepolia Provider
pub fn create_arbitrum_sepolia_ethers_provider() -> Result<Provider<Http>> {
    create_ethers_provider("https://sepolia-rollup.arbitrum.io/rpc")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_ethers_provider() {
        let result = create_arbitrum_sepolia_ethers_provider();
        assert!(result.is_ok());
    }
}
