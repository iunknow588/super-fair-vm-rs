use crate::account::{Account, Address};
use ethers::types::{H256, U256};

pub mod memory;
pub use memory::MemoryStorage;

pub trait Storage: Send + Sync + std::fmt::Debug {
    fn get_account(&self, address: &Address) -> Option<Account>;
    fn set_account(&mut self, account: &Account);
    fn get_balance(&self, address: &Address) -> U256;
    fn set_balance(&mut self, address: &Address, balance: U256);
    fn get_nonce(&self, address: &Address) -> u64;
    fn set_nonce(&mut self, address: &Address, nonce: u64);
    fn get_code_hash(&self, address: &Address) -> H256;
    fn set_code_hash(&mut self, address: &Address, code_hash: H256);
    fn get_storage_root(&self, address: &Address) -> H256;
    fn set_storage_root(&mut self, address: &Address, storage_root: H256);
}
