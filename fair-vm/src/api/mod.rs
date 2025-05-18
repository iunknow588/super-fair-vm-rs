pub mod chain_handlers;
pub mod static_handlers;
pub mod wallet_handlers;

use crate::account::Address as AccountAddress;
use crate::consensus::ConsensusEngineTrait;
use crate::state::State;
use crate::storage::Storage;
use crate::transaction::{Transaction as LocalTransaction, TransactionType};
use async_trait::async_trait;
use ethers::types::{H160, H256, U256};
use fair_vm_core::types::{
    Address as CoreAddress, Hash as CoreHash, Transaction as CoreTransaction,
};
use fair_vm_core::vm::Vm;
use jsonrpc_core::Error;
use serde_json;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 将核心交易转换为本地交易
pub fn convert_transaction(tx: &CoreTransaction) -> LocalTransaction {
    let hash_bytes = tx.hash.as_bytes();
    let from_bytes = tx.from.as_bytes();
    LocalTransaction {
        hash: H256::from_slice(hash_bytes),
        from: AccountAddress::from(H160::from_slice(from_bytes)),
        to: tx
            .to
            .as_ref()
            .map(|addr| AccountAddress::from(H160::from_slice(addr.as_bytes()))),
        value: tx.value,
        nonce: tx.nonce,
        gas_limit: tx.gas_limit,
        gas_price: Some(tx.gas_price),
        data: tx.data.clone(),
        signature: Vec::new(),
        transaction_type: TransactionType::Legacy,
        chain_id: 1,
        max_fee_per_gas: Some(tx.gas_price * U256::from(2)),
        max_priority_fee_per_gas: Some(tx.gas_price),
    }
}

/// 将本地交易转换为核心交易
pub fn convert_to_core_transaction(tx: &LocalTransaction) -> CoreTransaction {
    CoreTransaction {
        from: CoreAddress::from_bytes(tx.from.0),
        to: tx.to.as_ref().map(|addr| CoreAddress::from_bytes(addr.0)),
        value: tx.value,
        data: tx.data.clone(),
        nonce: tx.nonce,
        gas_price: tx.gas_price.unwrap_or_default(),
        gas_limit: tx.gas_limit,
        hash: CoreHash::from_bytes(tx.hash.0),
    }
}

/// VM trait 扩展
#[async_trait]
pub trait VmExt: Vm + Send + Sync {
    /// 获取状态
    async fn get_state(&self) -> Arc<RwLock<State>>;
    /// 获取存储 (返回 Arc<RwLock<Box<dyn Storage + Send + Sync>>>)
    async fn get_storage_arc(&self) -> Arc<RwLock<Box<dyn Storage + Send + Sync>>>;
    /// 获取共识引擎
    async fn get_consensus(&self) -> Option<Arc<RwLock<dyn ConsensusEngineTrait + Send + Sync>>>;
    /// 获取账户信息
    async fn get_account(
        &self,
        address: &crate::account::Address,
    ) -> Option<crate::account::Account>;
    /// 获取账户交易列表
    async fn get_account_transactions(
        &self,
        address: &crate::account::Address,
    ) -> Vec<crate::transaction::Transaction>;
    /// 获取交易收据
    async fn get_transaction_receipt(
        &self,
        tx_hash: &[u8],
    ) -> Option<ethers::types::TransactionReceipt>;
    /// 获取存储值 (根据 address 和 key 返回 H256)
    async fn get_storage(
        &self,
        address: &ethers::types::H160,
        key: &ethers::types::H256,
    ) -> Result<ethers::types::H256, Error>;
    /// 获取合约代码
    async fn get_code(&self, address: &ethers::types::H160) -> Result<Vec<u8>, Error>;
}

/// API 处理器 trait
#[async_trait]
pub trait ApiHandler: Send + Sync {
    /// 获取状态
    async fn get_state(&self) -> Arc<RwLock<State>>;
    /// 获取存储
    async fn get_storage(&self) -> Arc<RwLock<Box<dyn Storage + Send + Sync>>>;
    /// 获取共识引擎
    async fn get_consensus(&self) -> Option<Arc<RwLock<dyn ConsensusEngineTrait + Send + Sync>>>;
    /// 获取虚拟机
    async fn get_vm(&self) -> Arc<RwLock<dyn Vm + Send + Sync>>;
}

pub struct ApiServer {
    vm: Arc<RwLock<dyn VmExt>>,
}

impl ApiServer {
    pub fn new(vm: Arc<RwLock<dyn VmExt>>) -> Self {
        Self { vm }
    }

    pub fn chain_handlers(&self) -> chain_handlers::ChainHandlers {
        chain_handlers::ChainHandlers::new(self.vm.clone())
    }

    pub fn static_handlers(&self) -> static_handlers::StaticHandlers {
        static_handlers::StaticHandlers::new(self.vm.clone())
    }

    pub fn wallet_handlers(&self) -> wallet_handlers::WalletHandlers {
        wallet_handlers::WalletHandlers::new(self.vm.clone())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("VM错误: {0}")]
    VmError(String),
    #[error("无效参数: {0}")]
    InvalidParams(String),
    #[error("内部错误: {0}")]
    Internal(String),
}

impl From<ApiError> for Error {
    fn from(e: ApiError) -> Self {
        match e {
            ApiError::VmError(msg) => {
                let mut err = Error::internal_error();
                err.data = Some(serde_json::Value::String(msg));
                err
            }
            ApiError::InvalidParams(msg) => Error::invalid_params(msg),
            ApiError::Internal(msg) => {
                let mut err = Error::internal_error();
                err.data = Some(serde_json::Value::String(msg));
                err
            }
        }
    }
}
