use crate::types::{Address, Hash};
use async_trait::async_trait;
use primitive_types::U256;
use std::error::Error;

/// 执行上下文
pub struct ExecutionContext {
    /// 区块号
    pub block_number: u64,
    /// 区块时间戳
    pub block_timestamp: u64,
    /// 区块难度
    pub block_difficulty: u64,
    /// 区块 gas 限制
    pub block_gas_limit: u64,
    /// 区块奖励
    pub block_reward: u64,
}

/// 执行结果
pub struct ExecutionResult {
    /// 使用的 gas
    pub gas_used: u64,
    /// 返回数据
    pub return_data: Vec<u8>,
    /// 状态
    pub status: bool,
}

/// 状态接口
#[async_trait]
pub trait State: Send + Sync {
    /// 获取账户余额
    async fn get_balance(&self, address: &Address) -> Result<U256, Box<dyn Error>>;

    /// 获取账户 nonce
    async fn get_nonce(&self, address: &Address) -> Result<u64, Box<dyn Error>>;

    /// 获取账户代码
    async fn get_code(&self, address: &Address) -> Result<Vec<u8>, Box<dyn Error>>;

    /// 获取存储值
    async fn get_storage(&self, address: &Address, key: &Hash) -> Result<Hash, Box<dyn Error>>;

    /// 设置存储值
    async fn set_storage(
        &self,
        address: &Address,
        key: &Hash,
        value: &Hash,
    ) -> Result<(), Box<dyn Error>>;

    /// 增加账户余额
    async fn add_balance(&self, address: &Address, amount: U256) -> Result<(), Box<dyn Error>>;

    /// 减少账户余额
    async fn sub_balance(&self, address: &Address, amount: U256) -> Result<(), Box<dyn Error>>;

    /// 增加账户 nonce
    async fn increment_nonce(&self, address: &Address) -> Result<(), Box<dyn Error>>;

    /// 设置账户代码
    async fn set_code(&self, address: &Address, code: Vec<u8>) -> Result<(), Box<dyn Error>>;
}

/// 虚拟机接口
#[async_trait]
pub trait Vm: Send + Sync {
    /// 执行交易
    async fn execute_transaction(
        &self,
        transaction: &crate::types::Transaction,
        state: &dyn State,
    ) -> Result<ExecutionResult, Box<dyn Error>>;
}

/// 基本虚拟机实现
pub struct BasicVm;

#[async_trait]
impl Vm for BasicVm {
    async fn execute_transaction(
        &self,
        _transaction: &crate::types::Transaction,
        _state: &dyn State,
    ) -> Result<ExecutionResult, Box<dyn Error>> {
        // 实现执行交易的逻辑
        Ok(ExecutionResult {
            gas_used: 0,
            return_data: vec![],
            status: true,
        })
    }
}
