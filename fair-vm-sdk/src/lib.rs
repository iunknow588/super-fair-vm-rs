//! FairVM SDK for interacting with FairVM blockchain.

pub mod client;
pub mod wallet;

/// 版本号
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// SDK配置
#[derive(Debug, Clone)]
pub struct SdkConfig {
    /// 节点URL
    pub node_url: String,
    /// 链ID
    pub chain_id: u64,
    /// 网络ID
    pub network_id: u64,
}

impl Default for SdkConfig {
    fn default() -> Self {
        Self {
            node_url: "http://localhost:9650".to_string(),
            chain_id: 2023,
            network_id: 1337,
        }
    }
}
