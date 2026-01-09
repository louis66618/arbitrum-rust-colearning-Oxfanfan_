// ============================================================================
// 配置管理模块
// ============================================================================

use eyre::Result;
use std::env;

/// 全局配置结构体
#[derive(Debug, Clone)]
pub struct Config {
    /// Arbitrum Sepolia RPC 端点
    pub arbitrum_sepolia_rpc: String,
    /// 目标钱包地址
    pub target_address: String,
    //  私钥字段 (Optional)
    pub private_key: Option<String>,
    //  指定合约类型
    pub contract_type: String,
    //  合约地址
    pub contract_address: String,
}

impl Config {
    /// 从环境变量加载配置
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        let arbitrum_sepolia_rpc = env::var("ARBITRUM_SEPOLIA_RPC")
            .unwrap_or_else(|_| "https://sepolia-rollup.arbitrum.io/rpc".to_string());

        let target_address = env::var("TARGET_ADDRESS")
            .unwrap_or_else(|_| "0xa8fF167e4f362B54FF612546a782A301BD521a0B".to_string());

        //  读取私钥
        let private_key = env::var("PRIVATE_KEY").ok();

        // 读取合约类型，默认为 ERC20
        let contract_type = env::var("CONTRACT_TYPE")
            .unwrap_or_else(|_| "ERC20".to_string())
            .to_uppercase(); // 统一转大写处理
        
        // 读取 CONTRACT_ADDRESS，如果没有配置，默认用 WETH
        let contract_address = env::var("CONTRACT_ADDRESS")
            .unwrap_or_else(|_| "0x980B62Da83eFf3D4576C647993b0c1D7faf17c73".to_string());

        Ok(Config {
            arbitrum_sepolia_rpc,
            target_address,
            private_key,
            contract_type,
            contract_address,
        })
    }

    /// 创建默认配置（用于测试）
    pub fn default_testnet() -> Self {
        Config {
            arbitrum_sepolia_rpc: "https://sepolia-rollup.arbitrum.io/rpc".to_string(),
            target_address: "0xa8fF167e4f362B54FF612546a782A301BD521a0B".to_string(),
            private_key: None,
            // 添加 .to_string()，把 &str 转为 String
            contract_type: "ERC20".to_string(),
            contract_address: "0x980B62Da83eFf3D4576C647993b0c1D7faf17c73".to_string(),
        }
    }
}
