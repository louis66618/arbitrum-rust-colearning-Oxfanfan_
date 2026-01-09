// ============================================================================
// 通用工具函数
// ============================================================================

use ethers::prelude::*;
use eyre::Result;

/// 将 Wei 转换为 ETH（Ethers 库）
pub fn format_ether_ethers(wei: U256) -> String {
    ethers::utils::format_ether(wei)
}

/// 将 Wei 转换为指定单位（Ethers 库）
pub fn format_units_ethers(wei: U256, units: &str) -> Result<String> {
    ethers::utils::format_units(wei, units).map_err(|e| eyre::eyre!("{}", e))
}

/// 解析地址字符串为 Address（Ethers 库）
pub fn parse_address(addr_str: &str) -> Result<Address> {
    addr_str.parse::<Address>().map_err(|e| eyre::eyre!("{}", e))
}

/// 格式化地址为可读形式
pub fn format_address(addr: Address) -> String {
    format!("{:?}", addr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_ether() {
        let wei = U256::from(1_000_000_000_000_000_000u64); // 1 ETH
        let eth = format_ether_ethers(wei);
        assert_eq!(eth, "1.0");
    }

    #[test]
    fn test_parse_address() {
        let addr_str = "0xa8fF167e4f362B54FF612546a782A301BD521a0B";
        let result = parse_address(addr_str);
        assert!(result.is_ok());
    }
}
