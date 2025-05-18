use crate::account::Address;
use crate::transaction::Transaction;
use async_trait::async_trait;
use fair_vm_core::vm::{ExecutionResult as CoreExecutionResult, State as StateTrait};
use std::sync::Arc;
use tokio::sync::RwLock;

/// 虚拟机执行结果
#[derive(Debug)]
pub struct VmExecutionResult {
    /// 返回数据
    pub return_data: Vec<u8>,
    /// 使用的 gas
    pub gas_used: u64,
    /// 是否成功
    pub success: bool,
}

/// 虚拟机接口
#[async_trait]
pub trait VmExt: Send + Sync {
    /// 获取状态
    async fn get_state(&self) -> Arc<RwLock<Box<dyn StateTrait>>>;

    /// 获取存储
    async fn get_storage_arc(&self) -> Arc<RwLock<Box<dyn crate::storage::Storage + Send + Sync>>>;

    /// 执行交易
    async fn execute_transaction(
        &self,
        transaction: &Transaction,
        state: &dyn StateTrait,
    ) -> Result<VmExecutionResult, String>;

    /// 获取代码
    async fn get_code(&self, address: &Address) -> Result<Vec<u8>, String>;
}

impl From<VmExecutionResult> for CoreExecutionResult {
    fn from(result: VmExecutionResult) -> Self {
        Self {
            gas_used: result.gas_used,
            return_data: result.return_data,
            status: result.success,
        }
    }
}

impl From<CoreExecutionResult> for VmExecutionResult {
    fn from(result: CoreExecutionResult) -> Self {
        Self {
            gas_used: result.gas_used,
            return_data: result.return_data,
            success: result.status,
        }
    }
}
