use crate::account::{Account, Address};
use crate::evm::EvmContext;
use crate::storage::Storage;
use ethers::types::{H256, U256};
use std::fmt;
use std::sync::Arc;
use tokio::sync::RwLock;

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

impl Storage for State {
    fn get_account(&self, address: &Address) -> Option<Account> {
        if let Some(storage) = &self.storage {
            let storage = storage.blocking_read();
            storage.get_account(address)
        } else {
            None
        }
    }

    fn set_account(&mut self, account: &Account) {
        if let Some(storage) = &self.storage {
            let mut storage = storage.blocking_write();
            storage.set_account(account);
        }
    }

    fn get_balance(&self, address: &Address) -> U256 {
        if let Some(storage) = &self.storage {
            let storage = storage.blocking_read();
            storage.get_balance(address)
        } else {
            U256::zero()
        }
    }

    fn set_balance(&mut self, address: &Address, balance: U256) {
        if let Some(storage) = &self.storage {
            let mut storage = storage.blocking_write();
            storage.set_balance(address, balance);
        }
    }

    fn get_nonce(&self, address: &Address) -> u64 {
        if let Some(storage) = &self.storage {
            let storage = storage.blocking_read();
            storage.get_nonce(address)
        } else {
            0
        }
    }

    fn set_nonce(&mut self, address: &Address, nonce: u64) {
        if let Some(storage) = &self.storage {
            let mut storage = storage.blocking_write();
            storage.set_nonce(address, nonce);
        }
    }

    fn get_code_hash(&self, address: &Address) -> H256 {
        if let Some(storage) = &self.storage {
            let storage = storage.blocking_read();
            storage.get_code_hash(address)
        } else {
            H256::zero()
        }
    }

    fn set_code_hash(&mut self, address: &Address, code_hash: H256) {
        if let Some(storage) = &self.storage {
            let mut storage = storage.blocking_write();
            storage.set_code_hash(address, code_hash);
        }
    }

    fn get_storage_root(&self, address: &Address) -> H256 {
        if let Some(storage) = &self.storage {
            let storage = storage.blocking_read();
            storage.get_storage_root(address)
        } else {
            H256::zero()
        }
    }

    fn set_storage_root(&mut self, address: &Address, storage_root: H256) {
        if let Some(storage) = &self.storage {
            let mut storage = storage.blocking_write();
            storage.set_storage_root(address, storage_root);
        }
    }
}

impl State {
    /// 创建新状态实例
    pub fn new(storage: Option<Arc<RwLock<Box<dyn Storage + Send + Sync>>>>) -> Self {
        Self { storage }
    }

    /// 获取账户信息
    pub fn get_account(&self, address: &Address) -> Option<Account> {
        if let Some(storage) = &self.storage {
            let storage = storage.blocking_read();
            storage.get_account(address)
        } else {
            None
        }
    }

    /// 设置账户信息
    pub fn set_account(&mut self, account: Account) -> Result<(), String> {
        if let Some(storage) = &self.storage {
            let mut storage = storage.blocking_write();
            storage.set_account(&account);
            Ok(())
        } else {
            Err("未设置存储实例".into())
        }
    }

    /// 获取账户余额
    pub fn get_balance(&self, address: &Address) -> U256 {
        if let Some(storage) = &self.storage {
            let storage = storage.blocking_read();
            storage.get_balance(address)
        } else {
            U256::zero()
        }
    }

    /// 设置账户余额
    pub fn set_balance(&mut self, address: &Address, balance: U256) -> Result<(), String> {
        if let Some(storage) = &self.storage {
            let mut storage = storage.blocking_write();
            storage.set_balance(address, balance);
            Ok(())
        } else {
            Err("未设置存储实例".into())
        }
    }

    /// 获取账户 nonce
    pub fn get_nonce(&self, address: &Address) -> u64 {
        if let Some(storage) = &self.storage {
            let storage = storage.blocking_read();
            storage.get_nonce(address)
        } else {
            0
        }
    }

    /// 设置账户 nonce
    pub fn set_nonce(&mut self, address: &Address, nonce: u64) -> Result<(), String> {
        if let Some(storage) = &self.storage {
            let mut storage = storage.blocking_write();
            storage.set_nonce(address, nonce);
            Ok(())
        } else {
            Err("未设置存储实例".into())
        }
    }

    /// 获取账户代码哈希
    pub fn get_code_hash(&self, address: &Address) -> H256 {
        if let Some(storage) = &self.storage {
            let storage = storage.blocking_read();
            storage.get_code_hash(address)
        } else {
            H256::zero()
        }
    }

    /// 设置账户代码哈希
    pub fn set_code_hash(&mut self, address: &Address, code_hash: H256) -> Result<(), String> {
        if let Some(storage) = &self.storage {
            let mut storage = storage.blocking_write();
            storage.set_code_hash(address, code_hash);
            Ok(())
        } else {
            Err("未设置存储实例".into())
        }
    }

    /// 获取账户存储根
    pub fn get_storage_root(&self, address: &Address) -> H256 {
        if let Some(storage) = &self.storage {
            let storage = storage.blocking_read();
            storage.get_storage_root(address)
        } else {
            H256::zero()
        }
    }

    /// 设置账户存储根
    pub fn set_storage_root(
        &mut self,
        address: &Address,
        storage_root: H256,
    ) -> Result<(), String> {
        if let Some(storage) = &self.storage {
            let mut storage = storage.blocking_write();
            storage.set_storage_root(address, storage_root);
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

    /// 获取状态根
    pub fn get_state_root(&self) -> H256 {
        H256::zero()
    }

    /// 设置存储值
    pub fn set_storage_value(&mut self, _address: &Address, _key: [u8; 32], _value: [u8; 32]) {
        // 实现存储值设置逻辑
    }
}

impl EvmContext for State {
    fn get_account(&self, address: &Address) -> Option<Account> {
        self.get_account(address)
    }

    fn set_account(&mut self, account: Account) {
        let _ = self.set_account(account);
    }

    fn remove_account(&mut self, _address: &Address) {
        // 实现账户删除逻辑
    }

    fn get_storage(&self, _address: &Address, _key: &H256) -> H256 {
        H256::zero()
    }

    fn set_storage(&mut self, address: &Address, key: H256, value: H256) {
        self.set_storage_value(address, key.0, value.0);
    }

    fn get_code(&self, _address: &Address) -> Option<Vec<u8>> {
        None
    }

    fn get_balance(&self, address: &Address) -> U256 {
        self.get_balance(address)
    }

    fn transfer(&mut self, from: &Address, to: &Address, value: U256) -> bool {
        let from_balance = self.get_balance(from);
        if from_balance < value {
            return false;
        }

        let to_balance = self.get_balance(to);
        let _ = self.set_balance(from, from_balance - value);
        let _ = self.set_balance(to, to_balance + value);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::MemoryStorage;

    #[test]
    fn test_state_default() {
        let state = State::default();
        assert!(state.storage.is_none());
    }

    #[test]
    fn test_state_new() {
        let storage = Arc::new(RwLock::new(
            Box::new(MemoryStorage::default()) as Box<dyn Storage + Send + Sync>
        ));
        let state = State::new(Some(storage.clone()));
        assert!(state.storage.is_some());
    }

    #[test]
    fn test_state_account_operations() {
        let storage = Arc::new(RwLock::new(
            Box::new(MemoryStorage::default()) as Box<dyn Storage + Send + Sync>
        ));
        let mut state = State::new(Some(storage.clone()));
        let address = Address([0; 20]);
        let account = Account::new(address);

        // 测试设置账户
        assert!(state.set_account(account.clone()).is_ok());
        assert_eq!(state.get_account(&address), Some(account));

        // 测试设置余额
        let balance = U256::from(100);
        assert!(state.set_balance(&address, balance).is_ok());
        assert_eq!(state.get_balance(&address), balance);

        // 测试设置 nonce
        let nonce = 42;
        assert!(state.set_nonce(&address, nonce).is_ok());
        assert_eq!(state.get_nonce(&address), nonce);

        // 测试设置代码哈希
        let code_hash = H256::random();
        assert!(state.set_code_hash(&address, code_hash).is_ok());
        assert_eq!(state.get_code_hash(&address), code_hash);
    }

    #[test]
    fn test_state_without_storage() {
        let mut state = State::default();
        let address = Address([0; 20]);
        let account = Account::new(address);

        // 测试无存储时的操作
        assert!(state.set_account(account).is_err());
        assert!(state.set_balance(&address, U256::from(100)).is_err());
        assert!(state.set_nonce(&address, 42).is_err());
        assert!(state.set_code_hash(&address, H256::random()).is_err());
    }
}
