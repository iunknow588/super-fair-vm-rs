use crate::account::{Account, Address};
use crate::storage::Storage;
use ethers::types::{H256, U256};
use std::collections::HashMap;

/// 内存存储实现
#[derive(Debug, Default)]
pub struct MemoryStorage {
    /// 账户存储
    accounts: HashMap<Address, Account>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Storage for MemoryStorage {
    fn get_account(&self, address: &Address) -> Option<Account> {
        self.accounts.get(address).cloned()
    }

    fn set_account(&mut self, account: &Account) {
        self.accounts.insert(account.address, account.clone());
    }

    fn get_balance(&self, address: &Address) -> U256 {
        self.accounts
            .get(address)
            .map(|account| account.balance)
            .unwrap_or_else(U256::zero)
    }

    fn set_balance(&mut self, address: &Address, balance: U256) {
        if let Some(account) = self.accounts.get_mut(address) {
            account.balance = balance;
        }
    }

    fn get_nonce(&self, address: &Address) -> u64 {
        self.accounts
            .get(address)
            .map(|account| account.nonce)
            .unwrap_or(0)
    }

    fn set_nonce(&mut self, address: &Address, nonce: u64) {
        if let Some(account) = self.accounts.get_mut(address) {
            account.nonce = nonce;
        }
    }

    fn get_code_hash(&self, address: &Address) -> H256 {
        self.accounts
            .get(address)
            .map(|account| account.code_hash)
            .unwrap_or_else(H256::zero)
    }

    fn set_code_hash(&mut self, address: &Address, code_hash: H256) {
        if let Some(account) = self.accounts.get_mut(address) {
            account.code_hash = code_hash;
        }
    }

    fn get_storage_root(&self, address: &Address) -> H256 {
        self.accounts
            .get(address)
            .map(|account| account.storage_root)
            .unwrap_or_else(H256::zero)
    }

    fn set_storage_root(&mut self, address: &Address, storage_root: H256) {
        if let Some(account) = self.accounts.get_mut(address) {
            account.storage_root = storage_root;
        }
    }
}

// 手动实现 Send 和 Sync
unsafe impl Send for MemoryStorage {}
unsafe impl Sync for MemoryStorage {}
