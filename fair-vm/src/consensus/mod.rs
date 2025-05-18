use crate::account::Address;
use crate::state::State;
use crate::transaction::Transaction;
use async_trait::async_trait;
use ethers::types::H256;
use jsonrpc_core::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// 共识参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusParams {
    /// 最小验证者数量
    pub min_validators: u32,
    /// 投票阈值
    pub vote_threshold: u32,
    /// 投票超时时间（毫秒）
    pub vote_timeout: u64,
    /// 区块确认时间（毫秒）
    pub block_confirm_time: u64,
}

impl Default for ConsensusParams {
    fn default() -> Self {
        Self {
            min_validators: 4,
            vote_threshold: 3,
            vote_timeout: 5000,
            block_confirm_time: 10000,
        }
    }
}

/// 区块头信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    /// 父区块哈希
    pub parent_hash: [u8; 32],
    /// 区块高度
    pub height: u64,
    /// 时间戳
    pub timestamp: u64,
    /// 交易根
    pub transactions_root: [u8; 32],
    /// 状态根
    pub state_root: [u8; 32],
    /// 区块难度
    pub difficulty: u64,
    /// 区块奖励
    pub block_reward: u64,
}

/// 区块结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusBlock {
    /// 区块头
    pub header: BlockHeader,
    /// 交易列表
    pub transactions: Vec<Transaction>,
}

/// 共识状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusState {
    /// 当前区块高度
    pub height: u64,
    /// 最新区块哈希
    pub latest_block_hash: [u8; 32],
    /// 验证者集合
    pub validators: Vec<Address>,
    /// 上次提交时间
    pub last_commit_time: u64,
    /// 上次提交区块哈希
    pub last_commit_hash: H256,
}

/// 共识引擎接口
#[async_trait]
pub trait ConsensusEngine: Send + Sync {
    /// 初始化共识引擎
    async fn initialize(&mut self, state: Arc<RwLock<State>>) -> Result<()>;

    /// 启动共识引擎
    async fn start(&mut self) -> Result<()>;

    /// 停止共识引擎
    async fn stop(&mut self) -> Result<()>;

    /// 提交交易到共识层
    async fn submit_transaction(&mut self, tx: Transaction) -> Result<()>;

    /// 获取当前共识状态
    async fn get_consensus_state(&self) -> Result<ConsensusState>;

    /// 获取指定高度的区块
    async fn get_block(&self, height: u64) -> Result<ConsensusBlock>;
}

/// 共识错误类型
#[derive(Debug, thiserror::Error)]
pub enum ConsensusError {
    #[error("共识引擎未初始化")]
    NotInitialized,

    #[error("共识引擎已停止")]
    Stopped,

    #[error("无效的交易: {0}")]
    InvalidTransaction(String),

    #[error("区块验证失败: {0}")]
    BlockValidationFailed(String),

    #[error("网络错误: {0}")]
    NetworkError(String),

    #[error("状态同步错误: {0}")]
    StateSyncError(String),

    #[error("其他错误: {0}")]
    Other(String),

    #[error("区块未找到")]
    BlockNotFound,
}

/// 基本共识引擎实现
pub struct BasicConsensus {
    /// 共识状态
    state: Option<Arc<RwLock<State>>>,
    /// 共识引擎状态
    engine_state: ConsensusState,
    /// 是否正在运行
    is_running: bool,
}

impl BasicConsensus {
    /// 创建新的基本共识引擎实例
    pub fn new() -> Self {
        Self {
            state: None,
            engine_state: ConsensusState {
                height: 0,
                validators: Vec::new(),
                last_commit_time: 0,
                last_commit_hash: H256::zero(),
                latest_block_hash: [0; 32],
            },
            is_running: false,
        }
    }

    /// 创建新的基本共识引擎实例，使用自定义参数
    pub fn with_params(_params: ConsensusParams) -> Self {
        Self::new()
    }
}

impl Default for BasicConsensus {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ConsensusEngine for BasicConsensus {
    async fn initialize(&mut self, state: Arc<RwLock<State>>) -> Result<()> {
        if self.state.is_some() {
            return Err(jsonrpc_core::Error::invalid_params("共识引擎已经初始化"));
        }
        self.state = Some(state);
        Ok(())
    }

    async fn start(&mut self) -> Result<()> {
        if self.state.is_none() {
            return Err(jsonrpc_core::Error::invalid_params("共识引擎未初始化"));
        }
        if self.is_running {
            return Err(jsonrpc_core::Error::invalid_params("共识引擎已经在运行"));
        }
        self.is_running = true;
        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        if !self.is_running {
            return Err(jsonrpc_core::Error::invalid_params("共识引擎已停止"));
        }
        self.is_running = false;
        Ok(())
    }

    async fn submit_transaction(&mut self, _tx: Transaction) -> Result<()> {
        if !self.is_running {
            return Err(jsonrpc_core::Error::invalid_params("共识引擎已停止"));
        }
        Ok(())
    }

    async fn get_consensus_state(&self) -> Result<ConsensusState> {
        if !self.is_running {
            return Err(jsonrpc_core::Error::invalid_params("共识引擎已停止"));
        }
        Ok(self.engine_state.clone())
    }

    async fn get_block(&self, _height: u64) -> Result<ConsensusBlock> {
        if !self.is_running {
            return Err(jsonrpc_core::Error::invalid_params("共识引擎已停止"));
        }
        Err(jsonrpc_core::Error::invalid_params("区块未找到"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evm::EvmContext;
    use crate::storage::MemoryStorage;

    #[tokio::test]
    async fn test_consensus_engine_lifecycle() {
        let mut engine = BasicConsensus::new();
        let storage =
            Arc::new(RwLock::new(Box::new(MemoryStorage::default())
                as Box<dyn crate::storage::Storage + Send + Sync>));
        let state = Arc::new(RwLock::new(State::new(storage, EvmContext::default())));

        // 测试初始化
        assert!(engine.initialize(state.clone()).await.is_ok());

        // 测试启动
        assert!(engine.start().await.is_ok());

        // 测试获取状态
        let state = engine.get_consensus_state().await.unwrap();
        assert_eq!(state.height, 0);

        // 测试停止
        assert!(engine.stop().await.is_ok());
    }
}

pub mod basic;

pub use basic::{
    BasicConsensus as ConsensusBasic, ConsensusEngine as ConsensusEngineTrait,
    ConsensusError as ConsensusErrorType, ConsensusParams as ConsensusParamsType,
    ConsensusState as ConsensusStateType, Transaction as ConsensusTransaction,
};
