pub mod statedb;

use crate::core::types::{Address, U256};
use crate::core::vm::EvmContext;
use async_trait::async_trait;
use std::fmt;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 存储接口
#[async_trait]
pub trait Storage {
    /// 获取账户
    async fn get_account(&self, address: &Address) -> Option<Account>;

    /// 设置账户
    async fn set_account(&mut self, account: &Account);

    /// 获取余额
    async fn get_balance(&self, address: &Address) -> U256;

    /// 设置余额
    async fn set_balance(&mut self, address: &Address, balance: U256);

    /// 获取nonce
    async fn get_nonce(&self, address: &Address) -> u64;

    /// 设置nonce
    async fn set_nonce(&mut self, address: &Address, nonce: u64);

    /// 获取代码哈希
    async fn get_code_hash(&self, address: &Address) -> U256;

    /// 设置代码哈希
    async fn set_code_hash(&mut self, address: &Address, code_hash: U256);

    /// 获取存储根
    async fn get_storage_root(&self, address: &Address) -> U256;

    /// 设置存储根
    async fn set_storage_root(&mut self, address: &Address, storage_root: U256);

    /// 设置存储值
    async fn set_storage_value(&mut self, address: &Address, key: [u8; 32], value: [u8; 32]);

    /// 获取存储值
    async fn get_storage_value(&self, address: &Address, key: [u8; 32]) -> [u8; 32];
}

/// 账户信息
#[derive(Debug, Clone)]
pub struct Account {
    /// 账户地址
    pub address: Address,
    /// 账户余额
    pub balance: U256,
    /// 账户nonce
    pub nonce: u64,
    /// 代码哈希
    pub code_hash: U256,
    /// 存储根
    pub storage_root: U256,
}

impl Account {
    /// 创建新账户
    pub fn new(address: Address) -> Self {
        Self {
            address,
            balance: U256::zero(),
            nonce: 0,
            code_hash: U256::zero(),
            storage_root: U256::zero(),
        }
    }
}

/// 状态类型
#[derive(Default)]
pub struct State {
    /// 存储
    storage: Option<Arc<RwLock<Box<dyn Storage + Send + Sync>>>>,
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("State")
            .field("storage", &"<dyn Storage>")
            .finish()
    }
}

#[async_trait]
impl Storage for State {
    async fn get_account(&self, address: &Address) -> Option<Account> {
        if let Some(storage) = &self.storage {
            let storage = storage.read().await;
            storage.get_account(address).await
        } else {
            None
        }
    }

    async fn set_account(&mut self, account: &Account) {
        if let Some(storage) = &self.storage {
            let mut storage = storage.write().await;
            storage.set_account(account).await;
        }
    }

    async fn get_balance(&self, address: &Address) -> U256 {
        if let Some(storage) = &self.storage {
            let storage = storage.read().await;
            storage.get_balance(address).await
        } else {
            U256::zero()
        }
    }

    async fn set_balance(&mut self, address: &Address, balance: U256) {
        if let Some(storage) = &self.storage {
            let mut storage = storage.write().await;
            storage.set_balance(address, balance).await;
        }
    }

    async fn get_nonce(&self, address: &Address) -> u64 {
        if let Some(storage) = &self.storage {
            let storage = storage.read().await;
            storage.get_nonce(address).await
        } else {
            0
        }
    }

    async fn set_nonce(&mut self, address: &Address, nonce: u64) {
        if let Some(storage) = &self.storage {
            let mut storage = storage.write().await;
            storage.set_nonce(address, nonce).await;
        }
    }

    async fn get_code_hash(&self, address: &Address) -> U256 {
        if let Some(storage) = &self.storage {
            let storage = storage.read().await;
            storage.get_code_hash(address).await
        } else {
            U256::zero()
        }
    }

    async fn set_code_hash(&mut self, address: &Address, code_hash: U256) {
        if let Some(storage) = &self.storage {
            let mut storage = storage.write().await;
            storage.set_code_hash(address, code_hash).await;
        }
    }

    async fn get_storage_root(&self, address: &Address) -> U256 {
        if let Some(storage) = &self.storage {
            let storage = storage.read().await;
            storage.get_storage_root(address).await
        } else {
            U256::zero()
        }
    }

    async fn set_storage_root(&mut self, address: &Address, storage_root: U256) {
        if let Some(storage) = &self.storage {
            let mut storage = storage.write().await;
            storage.set_storage_root(address, storage_root).await;
        }
    }

    async fn set_storage_value(&mut self, address: &Address, key: [u8; 32], value: [u8; 32]) {
        if let Some(storage) = &self.storage {
            let mut storage = storage.write().await;
            storage.set_storage_value(address, key, value).await;
        }
    }

    async fn get_storage_value(&self, address: &Address, key: [u8; 32]) -> [u8; 32] {
        if let Some(storage) = &self.storage {
            let storage = storage.read().await;
            storage.get_storage_value(address, key).await
        } else {
            [0u8; 32]
        }
    }
}

impl State {
    /// 创建新状态实例
    pub fn new() -> Self {
        Self { storage: None }
    }

    /// 获取账户信息
    pub async fn get_account(&self, address: &Address) -> Option<Account> {
        if let Some(storage) = &self.storage {
            let storage = storage.read().await;
            storage.get_account(address).await
        } else {
            None
        }
    }

    /// 设置账户信息
    pub async fn set_account(&mut self, account: Account) -> Result<(), String> {
        if let Some(storage) = &self.storage {
            let mut storage = storage.write().await;
            storage.set_account(&account).await;
            Ok(())
        } else {
            Err("未设置存储实例".into())
        }
    }

    /// 获取账户余额
    pub async fn get_balance(&self, address: &Address) -> U256 {
        if let Some(storage) = &self.storage {
            let storage = storage.read().await;
            storage.get_balance(address).await
        } else {
            U256::zero()
        }
    }

    /// 设置账户余额
    pub async fn set_balance(&mut self, address: &Address, balance: U256) -> Result<(), String> {
        if let Some(storage) = &self.storage {
            let mut storage = storage.write().await;
            storage.set_balance(address, balance).await;
            Ok(())
        } else {
            Err("未设置存储实例".into())
        }
    }

    /// 获取账户 nonce
    pub async fn get_nonce(&self, address: &Address) -> u64 {
        if let Some(storage) = &self.storage {
            let storage = storage.read().await;
            storage.get_nonce(address).await
        } else {
            0
        }
    }

    /// 设置账户 nonce
    pub async fn set_nonce(&mut self, address: &Address, nonce: u64) -> Result<(), String> {
        if let Some(storage) = &self.storage {
            let mut storage = storage.write().await;
            storage.set_nonce(address, nonce).await;
            Ok(())
        } else {
            Err("未设置存储实例".into())
        }
    }

    /// 获取账户代码哈希
    pub async fn get_code_hash(&self, address: &Address) -> U256 {
        if let Some(storage) = &self.storage {
            let storage = storage.read().await;
            storage.get_code_hash(address).await
        } else {
            U256::zero()
        }
    }

    /// 设置账户代码哈希
    pub async fn set_code_hash(
        &mut self,
        address: &Address,
        code_hash: U256,
    ) -> Result<(), String> {
        if let Some(storage) = &self.storage {
            let mut storage = storage.write().await;
            storage.set_code_hash(address, code_hash).await;
            Ok(())
        } else {
            Err("未设置存储实例".into())
        }
    }

    /// 获取账户存储根
    pub async fn get_storage_root(&self, address: &Address) -> U256 {
        if let Some(storage) = &self.storage {
            let storage = storage.read().await;
            storage.get_storage_root(address).await
        } else {
            U256::zero()
        }
    }

    /// 设置账户存储根
    pub async fn set_storage_root(
        &mut self,
        address: &Address,
        storage_root: U256,
    ) -> Result<(), String> {
        if let Some(storage) = &self.storage {
            let mut storage = storage.write().await;
            storage.set_storage_root(address, storage_root).await;
            Ok(())
        } else {
            Err("未设置存储实例".into())
        }
    }

    /// 获取存储实例
    pub fn storage(&self) -> Option<&Arc<RwLock<Box<dyn Storage + Send + Sync>>>> {
        self.storage.as_ref()
    }

    /// 设置存储实例
    pub fn set_storage(&mut self, storage: Arc<RwLock<Box<dyn Storage + Send + Sync>>>) {
        self.storage = Some(storage);
    }
}

#[async_trait]
impl EvmContext for State {
    async fn get_account(&self, address: &Address) -> Option<Account> {
        self.get_account(address).await
    }

    async fn set_account(&mut self, account: Account) {
        let _ = self.set_account(account).await;
    }

    async fn remove_account(&mut self, _address: &Address) {
        // 暂不实现
    }

    async fn get_storage(&self, address: &Address, key: &U256) -> U256 {
        let key_bytes = key.to_be_bytes();
        let value_bytes = self.get_storage_value(address, key_bytes).await;
        U256::from_big_endian(&value_bytes)
    }

    async fn set_storage(&mut self, address: &Address, key: U256, value: U256) {
        let key_bytes = key.to_be_bytes();
        let value_bytes = value.to_be_bytes();
        self.set_storage_value(address, key_bytes, value_bytes).await;
    }

    async fn get_code(&self, _address: &Address) -> Option<Vec<u8>> {
        // 暂不实现
        None
    }

    async fn get_balance(&self, address: &Address) -> U256 {
        self.get_balance(address).await
    }

    async fn transfer(&mut self, from: &Address, to: &Address, value: U256) -> bool {
        let from_balance = self.get_balance(from).await;
        if from_balance < value {
            return false;
        }

        let to_balance = self.get_balance(to).await;
        let _ = self.set_balance(from, from_balance - value).await;
        let _ = self.set_balance(to, to_balance + value).await;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_default() {
        let state = State::new();
        assert!(state.storage.is_none());
    }

    #[test]
    fn test_state_new() {
        let state = State::new();
        assert!(state.storage.is_none());
    }

    #[tokio::test]
    async fn test_state_account_operations() {
        let state = State::new();
        let address = Address([1u8; 20]);
        let account = Account::new(address);

        // 测试获取不存在的账户
        assert!(state.get_account(&address).await.is_none());

        // 测试设置账户
        let _ = state.set_account(account.clone()).await;

        // 测试获取已存在的账户
        let retrieved_account = state.get_account(&address).await;
        assert!(retrieved_account.is_some());
        assert_eq!(retrieved_account.unwrap().address, account.address);
    }

    #[tokio::test]
    async fn test_state_without_storage() {
        let state = State::new();
        let address = Address([1u8; 20]);

        // 测试在没有存储实例时的操作
        assert_eq!(state.get_balance(&address).await, U256::zero());
        assert_eq!(state.get_nonce(&address).await, 0);
        assert_eq!(state.get_code_hash(&address).await, U256::zero());
        assert_eq!(state.get_storage_root(&address).await, U256::zero());
    }
}
