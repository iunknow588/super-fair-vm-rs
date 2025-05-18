use crate::account::{Account, Address};
use crate::storage::Storage;
use async_trait::async_trait;
use ethers::types::{H256, U256};
use std::collections::HashMap;

/// 内存存储实现
#[derive(Debug, Default)]
pub struct MemoryStorage {
    /// 账户存储
    accounts: HashMap<Address, Account>,
    /// 存储映射
    storage: HashMap<Address, HashMap<[u8; 32], [u8; 32]>>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl Storage for MemoryStorage {
    async fn get_account(&self, address: &Address) -> Option<Account> {
        self.accounts.get(address).cloned()
    }

    async fn set_account(&mut self, account: &Account) {
        self.accounts.insert(account.address, account.clone());
    }

    async fn get_balance(&self, address: &Address) -> U256 {
        self.accounts
            .get(address)
            .map(|account| account.balance)
            .unwrap_or_else(U256::zero)
    }

    async fn set_balance(&mut self, address: &Address, balance: U256) {
        if let Some(account) = self.accounts.get_mut(address) {
            account.balance = balance;
        }
    }

    async fn get_nonce(&self, address: &Address) -> u64 {
        self.accounts
            .get(address)
            .map(|account| account.nonce)
            .unwrap_or(0)
    }

    async fn set_nonce(&mut self, address: &Address, nonce: u64) {
        if let Some(account) = self.accounts.get_mut(address) {
            account.nonce = nonce;
        }
    }

    async fn get_code_hash(&self, address: &Address) -> H256 {
        self.accounts
            .get(address)
            .map(|account| account.code_hash)
            .unwrap_or_else(H256::zero)
    }

    async fn set_code_hash(&mut self, address: &Address, code_hash: H256) {
        if let Some(account) = self.accounts.get_mut(address) {
            account.code_hash = code_hash;
        }
    }

    async fn get_storage_root(&self, address: &Address) -> H256 {
        self.accounts
            .get(address)
            .map(|account| account.storage_root)
            .unwrap_or_else(H256::zero)
    }

    async fn set_storage_root(&mut self, address: &Address, storage_root: H256) {
        if let Some(account) = self.accounts.get_mut(address) {
            account.storage_root = storage_root;
        }
    }

    async fn get_storage_value(&self, address: &Address, key: [u8; 32]) -> [u8; 32] {
        println!(
            "MemoryStorage::get_storage_value: address={:?}, key={:?}",
            address, key
        );
        if let Some(account_storage) = self.storage.get(address) {
            if let Some(value) = account_storage.get(&key) {
                println!("MemoryStorage::get_storage_value: found value={:?}", value);
                return *value;
            }
        }
        println!("MemoryStorage::get_storage_value: not found, returning [0; 32]");
        [0u8; 32]
    }

    async fn set_storage_value(&mut self, address: &Address, key: [u8; 32], value: [u8; 32]) {
        let account_storage = self.storage.entry(*address).or_default();
        println!(
            "MemoryStorage::set_storage_value: address={:?}, key={:?}, value={:?}",
            address, key, value
        );
        account_storage.insert(key, value);
    }
}

// 手动实现 Send 和 Sync
unsafe impl Send for MemoryStorage {}
unsafe impl Sync for MemoryStorage {}
