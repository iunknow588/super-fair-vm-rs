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
pub mod blockchain;
pub mod consensus;
pub mod event;
pub mod evm;
pub mod genesis;
pub mod network;
pub mod nft;
pub mod state;
pub mod storage;
pub mod transaction;
pub mod types;
pub mod vm;

pub use account::{Account, Address};
pub use api::VmExt;
pub use block::Block;
pub use blockchain::*;
pub use consensus::basic;
pub use consensus::{ConsensusEngine, ConsensusEngineTrait, ConsensusError, ConsensusState};
pub use event::{Event, EventHandler, EventHandlerManager, EventManager, EventType};
pub use evm::*;
pub use genesis::{FeesConfig, GasLimitConfig, Genesis};
pub use network::*;
pub use nft::NFTContract;
pub use state::*;
pub use storage::*;
pub use transaction::{Transaction, TransactionType};

use async_trait::async_trait;
use chrono::Utc;
use ethers::types::{H256, U256};
use fair_vm_core::config::Config;
use fair_vm_core::types::Transaction as CoreTransaction;
use fair_vm_core::vm::{ExecutionResult, State as StateTrait, Vm};
use jsonrpc_core::Error;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;

/// FairVM 错误类型
#[derive(Debug, thiserror::Error)]
pub enum FairVMError {
    #[error("VM 错误: {0}")]
    VMError(String),

    #[error("共识错误: {0}")]
    ConsensusError(ConsensusError),

    #[error("状态错误: {0}")]
    StateError(String),

    #[error("交易错误: {0}")]
    TransactionError(String),

    #[error("NFT 错误: {0}")]
    NFTError(String),

    #[error("其他错误: {0}")]
    Other(String),
}

impl From<ConsensusError> for FairVMError {
    fn from(err: ConsensusError) -> Self {
        FairVMError::ConsensusError(err)
    }
}

impl From<basic::ConsensusError> for FairVMError {
    fn from(err: basic::ConsensusError) -> Self {
        FairVMError::ConsensusError(ConsensusError::Other(err.to_string()))
    }
}

impl From<basic::ConsensusState> for consensus::ConsensusState {
    fn from(state: basic::ConsensusState) -> Self {
        Self {
            height: state.height,
            validators: state.validators,
            latest_block_hash: state.last_commit_hash.0,
            last_commit_time: state.last_commit_time,
            last_commit_hash: state.last_commit_hash,
        }
    }
}

/// FairVM 实现
pub struct FairVM {
    /// 状态实例
    state: Arc<RwLock<State>>,
    /// 存储实例
    storage: Arc<RwLock<Box<dyn Storage + Send + Sync>>>,
    /// 共识引擎
    consensus: Option<Arc<RwLock<dyn ConsensusEngineTrait + Send + Sync>>>,
    /// 事件管理器
    event_manager: Arc<RwLock<EventManager>>,
    /// 事件处理器管理器
    #[allow(dead_code)]
    event_handler_manager: Arc<RwLock<EventHandlerManager>>,
    /// 是否正在运行
    is_running: bool,
    /// 链ID
    chain_id: u64,
}

impl FairVM {
    /// 创建新的 FairVM 实例
    pub fn new() -> Self {
        let storage = Arc::new(RwLock::new(
            Box::new(MemoryStorage::default()) as Box<dyn Storage + Send + Sync>
        ));
        let state = Arc::new(RwLock::new(State::default()));
        let event_manager = Arc::new(RwLock::new(EventManager::default()));
        let event_handler_manager = Arc::new(RwLock::new(EventHandlerManager::default()));

        Self {
            state,
            storage,
            consensus: None,
            event_manager,
            event_handler_manager,
            is_running: false,
            chain_id: 1,
        }
    }

    /// 使用自定义配置创建 FairVM 实例
    pub fn with_config(_config: Config) -> Self {
        let storage = Arc::new(RwLock::new(
            Box::new(MemoryStorage::default()) as Box<dyn Storage + Send + Sync>
        ));
        let state = Arc::new(RwLock::new(State::default()));
        let event_manager = Arc::new(RwLock::new(EventManager::default()));
        let event_handler_manager = Arc::new(RwLock::new(EventHandlerManager::default()));

        Self {
            state,
            storage,
            consensus: None,
            event_manager,
            event_handler_manager,
            is_running: false,
            chain_id: 1,
        }
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
        consensus: impl ConsensusEngineTrait + 'static,
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

        if let Some(consensus) = &self.consensus {
            consensus.write().await.start().await?;

            // 发布共识事件
            if let Ok(state) = consensus.read().await.get_consensus_state().await {
                let event = Event {
                    event_type: EventType::Consensus {
                        height: state.height,
                        validators: state.validators,
                    },
                    timestamp: Utc::now(),
                    data: json!({}),
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
    pub async fn add_event_handler(&self, handler: Arc<dyn event::EventHandler>) {
        let mut event_manager = self.event_manager.write().await;
        event_manager.add_handler(handler);
    }

    /// 移除事件处理器
    pub async fn remove_event_handler(&self, index: usize) {
        let mut event_manager = self.event_manager.write().await;
        event_manager.remove_handler(index);
    }

    /// 发布事件
    pub async fn publish_event(&self, event: Event) -> Result<(), FairVMError> {
        let event_manager = self.event_manager.read().await;
        event_manager.publish(event).map_err(FairVMError::Other)
    }

    /// 启动事件处理
    pub async fn start_event_handling(&self) {
        let event_manager = self.event_manager.read().await;
        let mut subscriber = event_manager.subscribe();

        tokio::spawn(async move {
            while let Ok(event) = subscriber.recv().await {
                // 事件处理逻辑
                log::info!("收到事件: {:?}", event);
            }
        });
    }

    /// 提交交易
    pub async fn submit_transaction(&self, tx: Transaction) -> Result<(), FairVMError> {
        if !self.is_running {
            return Err(FairVMError::Other("FairVM 未运行".into()));
        }

        let tx_type = tx.transaction_type;

        if let Some(consensus) = &self.consensus {
            let consensus_tx = ConsensusTransaction {
                hash: H256(tx.hash.0),
                from: tx.from,
                to: tx.to,
                value: tx.value,
                nonce: tx.nonce,
                gas_limit: tx.gas_limit,
                gas_price: tx.gas_price,
                data: tx.data,
                signature: tx.signature,
                transaction_type: tx_type,
                chain_id: tx.chain_id,
                max_fee_per_gas: tx.max_fee_per_gas,
                max_priority_fee_per_gas: tx.max_priority_fee_per_gas,
            };
            consensus
                .write()
                .await
                .submit_transaction(consensus_tx)
                .await?;
            Ok(())
        } else {
            Err(FairVMError::Other("未设置共识引擎".into()))
        }
    }

    /// 获取账户信息
    pub async fn get_account(&self, address: &account::Address) -> Option<Account> {
        let state = self.state.read().await;
        state.get_account(address).await
    }

    /// 获取NFT合约信息
    pub async fn get_nft_contract(&self, _address: &account::Address) -> Option<NFTContract> {
        None // TODO: 实现NFT合约查询
    }

    /// 获取共识状态
    pub async fn get_consensus_state(&self) -> Result<ConsensusState, FairVMError> {
        if let Some(consensus) = &self.consensus {
            consensus
                .read()
                .await
                .get_consensus_state()
                .await
                .map(|state| state.into())
                .map_err(FairVMError::from)
        } else {
            Err(FairVMError::Other("未设置共识引擎".into()))
        }
    }

    /// 获取账户nonce
    pub async fn get_nonce(&self, address: account::Address) -> Result<u64, FairVMError> {
        let state = self.state.read().await;
        let account = state.get_account(&address).await;
        Ok(account.map_or(0, |acc| acc.nonce))
    }

    /// 创建新交易
    pub async fn create_transaction(
        &self,
        from: account::Address,
        to: Option<account::Address>,
        value: U256,
        data: Vec<u8>,
    ) -> Result<Transaction, FairVMError> {
        let nonce = self.get_nonce(from).await?;
        let gas_limit = U256::from(21000); // 基本 gas 限制
        let gas_price = U256::from(1); // 基本 gas 价格
        let chain_id = self.chain_id;

        let transaction = Transaction::new(
            H256::zero(), // 临时哈希，将在签名后更新
            from,
            to,
            value,
            nonce,
            gas_limit.as_u64(),
            Some(gas_price),
            data,
            Vec::new(), // 临时签名
            TransactionType::Legacy,
            chain_id,
            None,
            None,
        );

        Ok(transaction)
    }
}

impl Default for FairVM {
    fn default() -> Self {
        Self::new()
    }
}

pub type ConsensusTransaction = Transaction;

#[async_trait]
impl Vm for FairVM {
    async fn execute_transaction(
        &self,
        transaction: &CoreTransaction,
        _state: &dyn StateTrait,
    ) -> Result<ExecutionResult, Box<dyn std::error::Error>> {
        // 将 CoreTransaction 转换为内部 Transaction 类型
        let _tx = Transaction {
            hash: H256(transaction.hash.0.into()),
            from: Address(transaction.from.0.into()),
            to: transaction.to.map(|addr| Address(addr.0.into())),
            value: transaction.value,
            nonce: transaction.nonce,
            gas_limit: transaction.gas_limit,
            gas_price: Some(transaction.gas_price),
            data: transaction.data.clone(),
            signature: vec![], // 暂时为空
            transaction_type: transaction::TransactionType::Legacy,
            chain_id: self.chain_id,
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None,
        };

        // TODO: 实现实际的交易执行逻辑
        Ok(ExecutionResult {
            gas_used: 0,
            return_data: vec![],
            status: true,
        })
    }
}

#[async_trait]
impl VmExt for FairVM {
    async fn get_state(&self) -> Arc<RwLock<State>> {
        self.state.clone()
    }

    async fn get_storage_arc(&self) -> Arc<RwLock<Box<dyn Storage + Send + Sync>>> {
        self.storage.clone()
    }

    async fn get_storage(
        &self,
        address: &ethers::types::H160,
        key: &ethers::types::H256,
    ) -> Result<ethers::types::H256, Error> {
        let state = self.state.read().await;
        let storage = state.storage().read().await;
        let value = storage.get_storage_value(&Address(address.0), key.0).await;
        Ok(ethers::types::H256(value))
    }

    async fn get_consensus(&self) -> Option<Arc<RwLock<dyn ConsensusEngineTrait + Send + Sync>>> {
        self.consensus.clone()
    }

    async fn get_account(&self, address: &account::Address) -> Option<Account> {
        let state = self.state.read().await;
        state.get_account(address).await
    }

    async fn get_account_transactions(&self, address: &account::Address) -> Vec<Transaction> {
        let state = self.state.read().await;
        state.get_account_transactions(address).await
    }

    async fn get_transaction_receipt(
        &self,
        tx_hash: &[u8],
    ) -> Option<ethers::types::TransactionReceipt> {
        let state = self.state.read().await;
        state.get_transaction_receipt(tx_hash).await
    }

    async fn get_code(&self, address: &ethers::types::H160) -> Result<Vec<u8>, Error> {
        let state = self.state.read().await;
        let account = state.get_account(&Address(address.0)).await;
        match account {
            Some(_acc) => {
                let storage = state.storage().read().await;
                // 使用 Storage trait 的 get_code_hash 方法
                let code_hash = storage.get_code_hash(&Address(address.0)).await;
                if code_hash.is_zero() {
                    Ok(Vec::new())
                } else {
                    // 从存储中获取代码
                    let code = storage
                        .get_storage_value(&Address(address.0), code_hash.0)
                        .await;
                    Ok(code.to_vec())
                }
            }
            None => Err(Error::internal_error()),
        }
    }
}

mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[tokio::test]
    async fn test_fairvm_lifecycle() {
        let mut fairvm = FairVM::new();
        assert!(!fairvm.is_running);

        fairvm.start().await.unwrap();
        assert!(fairvm.is_running);

        fairvm.stop().await.unwrap();
        assert!(!fairvm.is_running);
    }

    #[tokio::test]
    async fn test_fairvm_transaction() {
        let mut fairvm = FairVM::new();

        // 设置共识引擎
        let consensus = basic::BasicConsensus::new();
        fairvm.set_consensus(consensus).await.unwrap();

        // 启动事件处理
        fairvm.start_event_handling().await;

        fairvm.start().await.unwrap();

        let tx = Transaction {
            from: Address([0u8; 20]),
            to: Some(Address([1u8; 20])),
            value: U256::from(100),
            data: vec![],
            nonce: 0,
            gas_price: Some(U256::from(1)),
            gas_limit: 21000,
            signature: Vec::new(),
            transaction_type: TransactionType::Legacy,
            hash: H256::zero(),
            chain_id: 1,
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None,
        };

        fairvm.submit_transaction(tx).await.unwrap();
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    struct TestEventHandler {
        event_count: AtomicUsize,
    }

    impl TestEventHandler {
        #[allow(dead_code)]
        fn new() -> Self {
            Self {
                event_count: AtomicUsize::new(0),
            }
        }

        #[allow(dead_code)]
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
        fairvm.start().await.unwrap();

        let handler = Arc::new(TestEventHandler::new());
        fairvm.add_event_handler(handler.clone()).await;

        // 启动事件处理
        fairvm.start_event_handling().await;

        let event = Event {
            event_type: EventType::Block {
                number: 1,
                hash: H256([0; 32]),
                timestamp: 0,
            },
            timestamp: Utc::now(),
            data: json!({}),
        };

        fairvm.publish_event(event).await.unwrap();
        assert_eq!(handler.count(), 1);
    }
}
