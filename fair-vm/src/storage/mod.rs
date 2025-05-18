use crate::account::{Account, Address};
use async_trait::async_trait;
use ethers::types::{H256, U256};
use std::option::Option;

pub mod memory;
pub use memory::MemoryStorage;

#[async_trait]
pub trait Storage: Send + Sync + std::fmt::Debug {
    async fn get_account(&self, address: &Address) -> Option<Account>;
    async fn set_account(&mut self, account: &Account);
    async fn get_balance(&self, address: &Address) -> U256;
    async fn set_balance(&mut self, address: &Address, balance: U256);
    async fn get_nonce(&self, address: &Address) -> u64;
    async fn set_nonce(&mut self, address: &Address, nonce: u64);
    async fn get_code_hash(&self, address: &Address) -> H256;
    async fn set_code_hash(&mut self, address: &Address, code_hash: H256);
    async fn get_storage_root(&self, address: &Address) -> H256;
    async fn set_storage_root(&mut self, address: &Address, storage_root: H256);
    async fn get_storage_value(&self, address: &Address, key: [u8; 32]) -> [u8; 32];
    async fn set_storage_value(&mut self, address: &Address, key: [u8; 32], value: [u8; 32]);
}
