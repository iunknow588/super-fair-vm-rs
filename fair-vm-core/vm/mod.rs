use crate::core::types::{Address, U256};
use crate::core::state::State;
use async_trait::async_trait;
use sha3::Digest;
use std::sync::Arc;
use tokio::sync::RwLock;

pub mod errors;
pub mod executor;
pub mod memory;
pub mod opcodes;
pub mod precompiled;
pub mod stack;

/// 重导出错误类型便于使用
pub use errors::{EvmError, StateError, TransactionError};

/// EVM执行上下文
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    /// 调用者地址
    pub caller: Address,
    /// 目标合约地址
    pub address: Address,
    /// 调用附带的以太币值
    pub value: U256,
    /// 调用数据
    pub data: Vec<u8>,
    /// Gas限制
    pub gas_limit: u64,
    /// Gas价格
    pub gas_price: u64,
    /// 公平性得分
    pub fairness_score: u64,
}

/// EVM执行结果
#[derive(Debug)]
pub struct ExecutionResult {
    /// 执行是否成功
    pub success: bool,
    /// 使用的gas数量
    pub gas_used: u64,
    /// 返回数据
    pub return_data: Vec<u8>,
    /// 错误信息（如果有）
    pub error: Option<String>,
    /// 公平性得分
    pub fairness_score: u64,
}

/// EVM上下文接口
#[async_trait]
pub trait EvmContext {
    /// 获取账户
    async fn get_account(&self, address: &Address) -> Option<Account>;

    /// 设置账户
    async fn set_account(&mut self, account: Account);

    /// 删除账户
    async fn remove_account(&mut self, address: &Address);

    /// 获取存储值
    async fn get_storage(&self, address: &Address, key: &U256) -> U256;

    /// 设置存储值
    async fn set_storage(&mut self, address: &Address, key: U256, value: U256);

    /// 获取合约代码
    async fn get_code(&self, address: &Address) -> Option<Vec<u8>>;

    /// 获取账户余额
    async fn get_balance(&self, address: &Address) -> U256;

    /// 转账
    async fn transfer(&mut self, from: &Address, to: &Address, value: U256) -> bool;
}

/// EVM实例
pub struct Evm {
    /// 状态
    state: Arc<RwLock<State>>,
}

impl Evm {
    /// 创建新的EVM实例
    pub fn new(state: Arc<RwLock<State>>) -> Self {
        Self { state }
    }

    /// 执行代码
    pub async fn execute(&mut self, context: ExecutionContext, code: Vec<u8>) -> ExecutionResult {
        let mut executor = executor::Executor::new(self.state.clone(), context);
        executor.execute(code).await
    }

    /// 部署合约
    pub async fn deploy_contract(
        &mut self,
        caller: Address,
        code: Vec<u8>,
        value: U256,
        gas_limit: u64,
        gas_price: u64,
    ) -> Result<(Address, ExecutionResult), String> {
        // 生成新的合约地址
        let nonce = self.get_nonce(&caller).await;
        let contract_address = self.generate_contract_address(&caller, nonce);

        // 创建执行上下文
        let context = ExecutionContext {
            caller,
            address: contract_address,
            value,
            data: code.clone(),
            gas_limit,
            gas_price,
            fairness_score: 0,
        };

        // 执行合约创建代码
        let result = self.execute(context, code.clone()).await;
        if result.success {
            // 存储合约代码
            let mut state_guard = self.state.write().await;
            let state = &mut *state_guard;

            // 创建新账户
            let mut account = Account::new(contract_address);
            account.balance = value;

            // 计算代码哈希
            let code_hash = if !result.return_data.is_empty() {
                let mut hasher = sha3::Keccak256::new();
                hasher.update(&result.return_data);
                U256::from_big_endian(&hasher.finalize())
            } else {
                U256::zero()
            };
            account.code_hash = code_hash;

            // 更新账户
            let _ = state.set_account(account).await;

            Ok((contract_address, result))
        } else {
            Err(result
                .error
                .unwrap_or_else(|| "Contract creation failed".to_string()))
        }
    }

    /// 获取账户nonce
    async fn get_nonce(&self, address: &Address) -> u64 {
        let account = {
            let state_guard = self.state.read().await;
            let state = &*state_guard;
            state.get_account(address).await
        };
        account.map(|account| account.nonce).unwrap_or(0)
    }

    /// 生成合约地址
    fn generate_contract_address(&self, creator: &Address, nonce: u64) -> Address {
        use sha3::{Digest, Keccak256};
        let mut hasher = Keccak256::new();
        hasher.update(creator.0);
        hasher.update(nonce.to_be_bytes());
        let result = hasher.finalize();
        let mut address = [0u8; 20];
        address.copy_from_slice(&result[12..32]);
        Address(address)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_contract_deployment() {
        let state = Arc::new(RwLock::new(State::new()));
        let mut evm = Evm::new(state);

        let caller = Address([1u8; 20]);
        let code = vec![
            0x60, 0x00, // PUSH1 0
            0x60, 0x00, // PUSH1 0
            0x52, // MSTORE
            0x60, 0x20, // PUSH1 32
            0x60, 0x00, // PUSH1 0
            0xf3, // RETURN
        ];

        let result = evm
            .deploy_contract(caller, code, U256::zero(), 100000, 1)
            .await;

        assert!(result.is_ok());
        let (_address, exec_result) = result.unwrap();
        assert!(exec_result.success);
        assert!(exec_result.gas_used > 0);
        assert_eq!(exec_result.fairness_score, 0);
    }
}
