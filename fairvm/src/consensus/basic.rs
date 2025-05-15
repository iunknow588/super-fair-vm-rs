use crate::account::Address;
use crate::state::State;
use crate::transaction::Transaction as ConsensusTransaction;
use ethers::types::H256;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 共识引擎 trait
#[async_trait::async_trait]
pub trait ConsensusEngine: Send + Sync {
    /// 初始化共识引擎
    async fn initialize(&mut self, state: Arc<RwLock<State>>) -> Result<(), ConsensusError>;

    /// 启动共识引擎
    async fn start(&mut self) -> Result<(), ConsensusError>;

    /// 停止共识引擎
    async fn stop(&mut self) -> Result<(), ConsensusError>;

    /// 提交交易
    async fn submit_transaction(&mut self, tx: ConsensusTransaction) -> Result<(), ConsensusError>;

    /// 获取共识状态
    async fn get_consensus_state(&self) -> Result<ConsensusState, ConsensusError>;
}

/// 共识错误类型
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ConsensusError {
    #[error("共识引擎未初始化")]
    NotInitialized,

    #[error("共识引擎未启动")]
    NotStarted,

    #[error("共识引擎已启动")]
    AlreadyStarted,

    #[error("共识引擎已停止")]
    AlreadyStopped,

    #[error("交易错误: {0}")]
    TransactionError(String),

    #[error("状态错误: {0}")]
    StateError(String),

    #[error("其他错误: {0}")]
    Other(String),

    #[error("共识引擎已初始化")]
    AlreadyInitialized,
}

/// 共识参数
#[derive(Debug, Clone)]
pub struct ConsensusParams {
    /// 区块时间（秒）
    pub block_time: u64,
    /// 最大区块大小
    pub max_block_size: usize,
    /// 最小区块大小
    pub min_block_size: usize,
    /// 最大交易数
    pub max_transactions: usize,
    /// 最小交易数
    pub min_transactions: usize,
}

impl Default for ConsensusParams {
    fn default() -> Self {
        Self {
            block_time: 1,
            max_block_size: 1024 * 1024, // 1MB
            min_block_size: 0,
            max_transactions: 1000,
            min_transactions: 0,
        }
    }
}

/// 共识状态
#[derive(Debug, Clone, PartialEq)]
pub struct ConsensusState {
    /// 当前高度
    pub height: u64,
    /// 验证者列表
    pub validators: Vec<Address>,
    /// 最后提交时间
    pub last_commit_time: u64,
    /// 最后提交哈希
    pub last_commit_hash: H256,
}

/// 共识交易
#[derive(Debug, Clone)]
pub struct Transaction {
    /// 发送方地址
    pub from: Address,
    /// 接收方地址
    pub to: Address,
    /// 交易金额
    pub value: u64,
    /// 交易数据
    pub data: Vec<u8>,
    /// 交易签名
    pub signature: Vec<u8>,
}

/// 基础共识引擎
#[derive(Debug)]
pub struct BasicConsensus {
    /// 共识参数
    #[allow(dead_code)]
    params: ConsensusParams,
    /// 状态
    state: Option<Arc<RwLock<State>>>,
    /// 引擎状态
    engine_state: ConsensusState,
}

impl Default for BasicConsensus {
    fn default() -> Self {
        Self {
            params: ConsensusParams::default(),
            state: None,
            engine_state: ConsensusState {
                height: 0,
                validators: Vec::new(),
                last_commit_time: 0,
                last_commit_hash: H256::zero(),
            },
        }
    }
}

impl BasicConsensus {
    /// 创建新的基本共识引擎实例
    pub fn new() -> Self {
        Self::default()
    }

    /// 使用自定义参数创建基本共识引擎实例
    pub fn with_params(params: ConsensusParams) -> Self {
        Self {
            params,
            ..Default::default()
        }
    }
}

#[async_trait::async_trait]
impl ConsensusEngine for BasicConsensus {
    async fn initialize(&mut self, state: Arc<RwLock<State>>) -> Result<(), ConsensusError> {
        if self.state.is_some() {
            return Err(ConsensusError::AlreadyStarted);
        }

        self.state = Some(state);
        Ok(())
    }

    async fn start(&mut self) -> Result<(), ConsensusError> {
        if self.state.is_none() {
            return Err(ConsensusError::NotInitialized);
        }

        Ok(())
    }

    async fn stop(&mut self) -> Result<(), ConsensusError> {
        if self.state.is_none() {
            return Err(ConsensusError::AlreadyStopped);
        }

        self.state = None;
        Ok(())
    }

    async fn submit_transaction(
        &mut self,
        _tx: ConsensusTransaction,
    ) -> Result<(), ConsensusError> {
        // TODO: 实现交易提交逻辑
        Ok(())
    }

    async fn get_consensus_state(&self) -> Result<ConsensusState, ConsensusError> {
        if self.state.is_none() {
            return Err(ConsensusError::NotInitialized);
        }

        Ok(self.engine_state.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::MemoryStorage;
    use crate::storage::Storage;

    #[tokio::test]
    async fn test_basic_consensus_default() {
        let consensus = BasicConsensus::default();
        assert!(consensus.state.is_none());
        assert_eq!(consensus.engine_state.height, 0);
        assert!(consensus.engine_state.validators.is_empty());
        assert_eq!(consensus.engine_state.last_commit_time, 0);
        assert_eq!(consensus.engine_state.last_commit_hash, H256::zero());
    }

    #[tokio::test]
    async fn test_basic_consensus_new() {
        let consensus = BasicConsensus::new();
        assert!(consensus.state.is_none());
    }

    #[tokio::test]
    async fn test_basic_consensus_with_params() {
        let params = ConsensusParams {
            block_time: 2,
            max_block_size: 2048,
            min_block_size: 1024,
            max_transactions: 500,
            min_transactions: 10,
        };
        let consensus = BasicConsensus::with_params(params);
        assert!(consensus.state.is_none());
    }

    #[tokio::test]
    async fn test_basic_consensus_lifecycle() {
        let mut consensus = BasicConsensus::new();
        let storage = Arc::new(RwLock::new(
            Box::new(MemoryStorage::default()) as Box<dyn Storage + Send + Sync>
        ));
        let state = Arc::new(RwLock::new(State::new(Some(storage))));

        // 测试初始化
        assert!(consensus.initialize(state.clone()).await.is_ok());
        assert!(consensus.state.is_some());

        // 测试启动
        assert!(consensus.start().await.is_ok());
        assert!(consensus.state.is_some());

        // 测试停止
        assert!(consensus.stop().await.is_ok());
        assert!(consensus.state.is_none());
    }

    #[tokio::test]
    async fn test_basic_consensus_errors() {
        let mut consensus = BasicConsensus::new();

        // 测试未初始化就启动
        assert_eq!(consensus.start().await, Err(ConsensusError::NotInitialized));

        // 测试未初始化就停止
        assert_eq!(consensus.stop().await, Err(ConsensusError::AlreadyStopped));

        let storage = Arc::new(RwLock::new(
            Box::new(MemoryStorage::default()) as Box<dyn Storage + Send + Sync>
        ));
        let state = Arc::new(RwLock::new(State::new(Some(storage))));

        // 测试初始化
        assert!(consensus.initialize(state.clone()).await.is_ok());

        // 测试重复初始化
        assert_eq!(
            consensus.initialize(state).await,
            Err(ConsensusError::AlreadyStarted)
        );

        // 测试启动
        assert!(consensus.start().await.is_ok());

        // 测试重复启动
        assert_eq!(consensus.start().await, Err(ConsensusError::AlreadyStarted));
    }

    #[tokio::test]
    async fn test_basic_consensus_state() {
        let mut consensus = BasicConsensus::new();
        let storage = Arc::new(RwLock::new(
            Box::new(MemoryStorage::default()) as Box<dyn Storage + Send + Sync>
        ));
        let state = Arc::new(RwLock::new(State::new(Some(storage))));

        // 测试未初始化就获取状态
        assert!(matches!(
            consensus.get_consensus_state().await,
            Err(ConsensusError::NotInitialized)
        ));

        // 初始化并获取状态
        assert!(consensus.initialize(state).await.is_ok());
        let state = consensus.get_consensus_state().await.unwrap();
        assert_eq!(state.height, 0);
        assert!(state.validators.is_empty());
        assert_eq!(state.last_commit_time, 0);
        assert_eq!(state.last_commit_hash, H256::zero());
    }

    #[tokio::test]
    async fn test_basic_consensus_already_initialized() {
        let mut consensus = BasicConsensus::new();
        let storage = Arc::new(RwLock::new(
            Box::new(MemoryStorage::default()) as Box<dyn Storage + Send + Sync>
        ));
        let state = Arc::new(RwLock::new(State::new(Some(storage))));

        // 第一次初始化
        assert!(consensus.initialize(state.clone()).await.is_ok());

        // 第二次初始化应该失败
        assert_eq!(
            consensus.initialize(state).await,
            Err(ConsensusError::AlreadyStarted)
        );
    }
}
