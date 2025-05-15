//! FairVM implementation for Avalanche subnet.
//!
//! FairVM 是一个基于 EVM 的虚拟机实现，支持 NFT 和自定义共识。
//! 主要功能包括：
//! - EVM 兼容的智能合约执行
//! - NFT 合约支持 (ERC721/ERC1155)
//! - 可插拔的共识引擎
//! - 状态管理和存储
//! - JSON-RPC API 支持
//! - 事件通知系统

pub mod account;
pub mod api;
pub mod block;
pub mod config;
pub mod consensus;
pub mod event;
pub mod evm;
pub mod genesis;
pub mod nft;
pub mod state;
pub mod storage;
pub mod transaction;
pub mod vm;

use consensus::basic::{ConsensusEngine, ConsensusError, ConsensusState};
use event::{Event, EventHandler, EventHandlerManager, EventManager, EventType};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Re-export common types
pub use account::{Account, Address};
pub use block::Block;
pub use config::Config;
pub use genesis::{FeesConfig, GasLimitConfig, Genesis};
pub use nft::{Attribute, NFTContract, NFTMetadata, NFTStandard, NFTToken};
pub use state::State;
pub use storage::{MemoryStorage, Storage};
pub use transaction::{Transaction, TransactionType};
pub use vm::VM;

/// FairVM 错误类型
#[derive(Debug, thiserror::Error)]
pub enum FairVMError {
    #[error("VM 错误: {0}")]
    VMError(String),

    #[error("共识错误: {0}")]
    ConsensusError(#[from] ConsensusError),

    #[error("状态错误: {0}")]
    StateError(String),

    #[error("交易错误: {0}")]
    TransactionError(String),

    #[error("NFT 错误: {0}")]
    NFTError(String),

    #[error("其他错误: {0}")]
    Other(String),
}

/// FairVM 实现
pub struct FairVM {
    /// VM 实例
    vm: Arc<RwLock<VM>>,
    /// 状态实例
    state: Arc<RwLock<State>>,
    /// 存储实例
    storage: Arc<RwLock<Box<dyn Storage + Send + Sync>>>,
    /// 共识引擎
    consensus: Option<Arc<RwLock<dyn ConsensusEngine>>>,
    /// 事件管理器
    event_manager: Arc<RwLock<EventManager>>,
    /// 事件处理器管理器
    event_handler_manager: Arc<RwLock<EventHandlerManager>>,
    /// 是否正在运行
    is_running: bool,
}

impl FairVM {
    /// 创建新的 FairVM 实例
    pub fn new() -> Self {
        let storage = Arc::new(RwLock::new(
            Box::new(MemoryStorage::default()) as Box<dyn Storage + Send + Sync>
        ));
        let state = Arc::new(RwLock::new(State::new(Some(storage.clone()))));
        let event_manager = Arc::new(RwLock::new(EventManager::default()));
        let event_handler_manager =
            Arc::new(RwLock::new(EventHandlerManager::new(event_manager.clone())));

        Self {
            vm: Arc::new(RwLock::new(VM::new(Some(storage.clone())))),
            state,
            storage,
            consensus: None,
            event_manager,
            event_handler_manager,
            is_running: false,
        }
    }

    /// 使用自定义配置创建 FairVM 实例
    pub fn with_config(config: Config) -> Self {
        let storage = Arc::new(RwLock::new(
            Box::new(MemoryStorage::default()) as Box<dyn Storage + Send + Sync>
        ));
        let state = Arc::new(RwLock::new(State::new(Some(storage.clone()))));
        let event_manager = Arc::new(RwLock::new(EventManager::default()));
        let event_handler_manager =
            Arc::new(RwLock::new(EventHandlerManager::new(event_manager.clone())));

        Self {
            vm: Arc::new(RwLock::new(VM::with_config(config, Some(storage.clone())))),
            state,
            storage,
            consensus: None,
            event_manager,
            event_handler_manager,
            is_running: false,
        }
    }

    /// 获取 VM 实例
    pub fn vm(&self) -> Arc<RwLock<VM>> {
        self.vm.clone()
    }

    /// 获取状态实例
    pub fn state(&self) -> Arc<RwLock<State>> {
        self.state.clone()
    }

    /// 获取存储实例
    pub fn storage(&self) -> Arc<RwLock<Box<dyn Storage + Send + Sync>>> {
        self.storage.clone()
    }

    /// 设置共识引擎
    pub async fn set_consensus(
        &mut self,
        consensus: impl ConsensusEngine + 'static,
    ) -> Result<(), FairVMError> {
        if self.is_running {
            return Err(FairVMError::Other(
                "FairVM 正在运行，无法更改共识引擎".into(),
            ));
        }
        let consensus = Arc::new(RwLock::new(consensus));
        consensus.write().await.initialize(self.state()).await?;
        self.consensus = Some(consensus);
        Ok(())
    }

    /// 启动 FairVM
    pub async fn start(&mut self) -> Result<(), FairVMError> {
        if self.is_running {
            return Err(FairVMError::Other("FairVM 已经在运行".into()));
        }

        // 启动事件处理
        self.start_event_handling().await;

        if let Some(consensus) = &self.consensus {
            consensus.write().await.start().await?;

            // 发布共识事件
            if let Ok(state) = consensus.read().await.get_consensus_state().await {
                let event = Event {
                    event_type: EventType::Consensus {
                        height: state.height,
                        validators: state.validators,
                    },
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                };
                self.publish_event(event).await?;
            }
        }

        self.is_running = true;
        Ok(())
    }

    /// 停止 FairVM
    pub async fn stop(&mut self) -> Result<(), FairVMError> {
        if !self.is_running {
            return Err(FairVMError::Other("FairVM 未运行".into()));
        }

        if let Some(consensus) = &self.consensus {
            consensus.write().await.stop().await?;
        }

        self.is_running = false;
        Ok(())
    }

    /// 添加事件处理器
    pub async fn add_event_handler(&self, handler: Arc<dyn EventHandler>) {
        self.event_handler_manager
            .write()
            .await
            .add_handler(handler);
    }

    /// 移除事件处理器
    pub async fn remove_event_handler(&self, index: usize) {
        self.event_handler_manager
            .write()
            .await
            .remove_handler(index);
    }

    /// 发布事件
    pub async fn publish_event(&self, event: Event) -> Result<(), FairVMError> {
        self.event_manager
            .write()
            .await
            .publish(event)
            .map_err(|e| FairVMError::Other(format!("发布事件失败: {}", e)))
    }

    /// 启动事件处理
    pub async fn start_event_handling(&self) {
        self.event_handler_manager.read().await.start().await;
    }

    /// 提交交易
    pub async fn submit_transaction(&self, tx: Transaction) -> Result<(), FairVMError> {
        if let Some(consensus) = &self.consensus {
            consensus
                .write()
                .await
                .submit_transaction(tx)
                .await
                .map_err(FairVMError::ConsensusError)?;
        }
        Ok(())
    }

    /// 获取账户信息
    pub async fn get_account(&self, address: &Address) -> Option<Account> {
        let state = self.state.read().await;
        state.get_account(address)
    }

    /// 获取 NFT 合约
    pub async fn get_nft_contract(&self, _address: &Address) -> Option<NFTContract> {
        let _state = self.state.read().await;
        // TODO: 实现 NFT 合约获取逻辑
        None
    }

    /// 获取当前共识状态
    pub async fn get_consensus_state(&self) -> Result<ConsensusState, FairVMError> {
        if let Some(consensus) = &self.consensus {
            consensus
                .read()
                .await
                .get_consensus_state()
                .await
                .map_err(FairVMError::from)
        } else {
            Err(FairVMError::Other("未设置共识引擎".into()))
        }
    }
}

impl Default for FairVM {
    fn default() -> Self {
        Self::new()
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consensus::basic::BasicConsensus;
    use crate::event::{Event, EventHandler, EventType};
    use crate::transaction::TransactionType;
    use ethers::types::{H256, U256};
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[tokio::test]
    async fn test_fairvm_lifecycle() {
        let mut fairvm = FairVM::new();

        // 测试设置共识引擎
        let consensus = BasicConsensus::new();
        assert!(fairvm.set_consensus(consensus).await.is_ok());

        // 测试启动
        assert!(fairvm.start().await.is_ok());
        assert!(fairvm.is_running);

        // 测试停止
        assert!(fairvm.stop().await.is_ok());
        assert!(!fairvm.is_running);
    }

    #[tokio::test]
    async fn test_fairvm_transaction() {
        let mut fairvm = FairVM::new();
        let consensus = BasicConsensus::new();
        fairvm.set_consensus(consensus).await.unwrap();
        fairvm.start().await.unwrap();

        let from = Address([1u8; 20]);
        let to = Some(Address([2u8; 20]));
        let tx = Transaction {
            hash: H256([0; 32]),
            from,
            to,
            value: U256::from(100),
            nonce: 0,
            gas_limit: 21000,
            gas_price: Some(U256::from(1)),
            data: vec![],
            signature: vec![],
            transaction_type: TransactionType::Legacy,
            chain_id: 1,
            max_fee_per_gas: Some(U256::from(2)),
            max_priority_fee_per_gas: Some(U256::from(1)),
        };

        // 测试提交交易
        let result = fairvm.submit_transaction(tx).await;
        assert!(result.is_ok());
    }

    struct TestEventHandler {
        event_count: AtomicUsize,
    }

    impl TestEventHandler {
        fn new() -> Self {
            Self {
                event_count: AtomicUsize::new(0),
            }
        }

        fn count(&self) -> usize {
            self.event_count.load(Ordering::SeqCst)
        }
    }

    impl EventHandler for TestEventHandler {
        fn handle_event(&self, _event: &Event) {
            self.event_count.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[tokio::test]
    async fn test_fairvm_events() {
        let mut fairvm = FairVM::new();

        // 添加测试事件处理器
        let handler = Arc::new(TestEventHandler::new());
        fairvm.add_event_handler(handler.clone()).await;

        // 启动 FairVM
        fairvm.start().await.unwrap();

        // 发布测试事件
        let event = Event {
            event_type: EventType::Block {
                number: 1,
                hash: H256([0; 32]),
                timestamp: 0,
            },
            timestamp: 0,
        };
        fairvm.publish_event(event).await.unwrap();

        // 等待事件处理
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        assert_eq!(handler.count(), 1);
    }
}
