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
}

impl Config {
    /// 从环境变量加载配置
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        let arbitrum_sepolia_rpc = env::var("ARBITRUM_SEPOLIA_RPC")
            .unwrap_or_else(|_| "https://sepolia-rollup.arbitrum.io/rpc".to_string());

        let target_address = env::var("TARGET_ADDRESS")
            .unwrap_or_else(|_| "0xa8fF167e4f362B54FF612546a782A301BD521a0B".to_string());

        //  新增：读取私钥
        let private_key = env::var("PRIVATE_KEY").ok();

        Ok(Config {
            arbitrum_sepolia_rpc,
            target_address,
            private_key,
        })
    }

    /// 创建默认配置（用于测试）
    pub fn default_testnet() -> Self {
        Config {
            arbitrum_sepolia_rpc: "https://sepolia-rollup.arbitrum.io/rpc".to_string(),
            target_address: "0xa8fF167e4f362B54FF612546a782A301BD521a0B".to_string(),
            private_key: None,
        }
    }
}
