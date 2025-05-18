use crate::types::{Address, Hash};
use crate::vm::State as StateTrait;
use async_trait::async_trait;
use primitive_types::U256;
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, RwLock};

/// 账户状态
#[derive(Debug, Clone)]
pub struct Account {
    /// 余额
    pub balance: U256,
    /// nonce
    pub nonce: u64,
    /// 代码
    pub code: Vec<u8>,
    /// 存储
    pub storage: HashMap<Hash, Hash>,
}

impl Account {
    /// 创建新账户
    pub fn new() -> Self {
        Self {
            balance: U256::zero(),
            nonce: 0,
            code: vec![],
            storage: HashMap::new(),
        }
    }
}

impl Default for Account {
    fn default() -> Self {
        Self::new()
    }
}

/// 状态实现
#[derive(Clone)]
pub struct State {
    /// 账户状态
    accounts: Arc<RwLock<HashMap<Address, Account>>>,
}

impl State {
    /// 创建新状态
    pub fn new() -> Self {
        Self {
            accounts: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 获取账户
    fn get_account(&self, address: &Address) -> Account {
        self.accounts
            .read()
            .unwrap()
            .get(address)
            .cloned()
            .unwrap_or_default()
    }

    /// 更新账户
    fn update_account(&self, address: &Address, f: impl FnOnce(&mut Account)) {
        let mut accounts = self.accounts.write().unwrap();
        let account = accounts.entry(*address).or_default();
        f(account);
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl StateTrait for State {
    async fn get_balance(&self, address: &Address) -> Result<U256, Box<dyn Error>> {
        Ok(self.get_account(address).balance)
    }

    async fn get_nonce(&self, address: &Address) -> Result<u64, Box<dyn Error>> {
        Ok(self.get_account(address).nonce)
    }

    async fn get_code(&self, address: &Address) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(self.get_account(address).code)
    }

    async fn get_storage(&self, address: &Address, key: &Hash) -> Result<Hash, Box<dyn Error>> {
        Ok(self
            .get_account(address)
            .storage
            .get(key)
            .copied()
            .unwrap_or(Hash::from_bytes([0u8; 32])))
    }

    async fn set_storage(
        &self,
        address: &Address,
        key: &Hash,
        value: &Hash,
    ) -> Result<(), Box<dyn Error>> {
        self.update_account(address, |account| {
            account.storage.insert(*key, *value);
        });
        Ok(())
    }

    async fn add_balance(&self, address: &Address, amount: U256) -> Result<(), Box<dyn Error>> {
        self.update_account(address, |account| {
            account.balance += amount;
        });
        Ok(())
    }

    async fn sub_balance(&self, address: &Address, amount: U256) -> Result<(), Box<dyn Error>> {
        self.update_account(address, |account| {
            account.balance -= amount;
        });
        Ok(())
    }

    async fn increment_nonce(&self, address: &Address) -> Result<(), Box<dyn Error>> {
        self.update_account(address, |account| {
            account.nonce += 1;
        });
        Ok(())
    }

    async fn set_code(&self, address: &Address, code: Vec<u8>) -> Result<(), Box<dyn Error>> {
        self.update_account(address, |account| {
            account.code = code;
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_state() {
        let state = State::new();
        let address = Address::random();

        // 测试余额
        assert_eq!(state.get_balance(&address).await.unwrap(), U256::zero());
        state.add_balance(&address, U256::from(100)).await.unwrap();
        assert_eq!(state.get_balance(&address).await.unwrap(), U256::from(100));
        state.sub_balance(&address, U256::from(50)).await.unwrap();
        assert_eq!(state.get_balance(&address).await.unwrap(), U256::from(50));

        // 测试nonce
        assert_eq!(state.get_nonce(&address).await.unwrap(), 0);
        state.increment_nonce(&address).await.unwrap();
        assert_eq!(state.get_nonce(&address).await.unwrap(), 1);

        // 测试代码
        let code = vec![1, 2, 3];
        state.set_code(&address, code.clone()).await.unwrap();
        assert_eq!(state.get_code(&address).await.unwrap(), code);

        // 测试存储
        let key = Hash::random();
        let value = Hash::random();
        state.set_storage(&address, &key, &value).await.unwrap();
        assert_eq!(state.get_storage(&address, &key).await.unwrap(), value);
    }
}
