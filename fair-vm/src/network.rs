use crate::blockchain::Block;
use crate::transaction::Transaction;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// 网络配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// 节点 ID
    pub node_id: String,
    /// 监听地址
    pub listen_addr: String,
    /// 引导节点列表
    pub bootstrap_nodes: Vec<String>,
    /// 最大连接数
    pub max_connections: usize,
    /// 最小连接数
    pub min_connections: usize,
}

/// 网络消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkMessage {
    /// 新区块
    NewBlock(Block),
    /// 新交易
    NewTransaction(Transaction),
    /// 获取区块请求
    GetBlock(u64),
    /// 获取区块响应
    BlockResponse(Option<Block>),
    /// 获取交易请求
    GetTransaction(String),
    /// 获取交易响应
    TransactionResponse(Option<Transaction>),
}

/// 网络接口
#[async_trait]
pub trait NetworkExt: Send + Sync {
    /// 启动网络
    async fn start(&self) -> Result<(), String>;

    /// 停止网络
    async fn stop(&self) -> Result<(), String>;

    /// 广播消息
    async fn broadcast(&self, message: NetworkMessage) -> Result<(), String>;

    /// 发送消息到指定节点
    async fn send_to(&self, node_id: &str, message: NetworkMessage) -> Result<(), String>;
}
