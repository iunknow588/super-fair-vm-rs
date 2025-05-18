use crate::types::{Address, Hash};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub parent_hash: Hash,
    pub number: u64,
    pub timestamp: u64,
    pub transactions_root: Hash,
    pub state_root: Hash,
    pub difficulty: u64,
    pub block_reward: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Hash>,
    pub receipts: HashMap<Hash, TransactionReceipt>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionReceipt {
    pub transaction_hash: Hash,
    pub block_number: u64,
    pub block_hash: Hash,
    pub transaction_index: u64,
    pub from: Address,
    pub to: Option<Address>,
    pub contract_address: Option<Address>,
    pub gas_used: u64,
    pub status: bool,
    pub logs: Vec<Log>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log {
    pub address: Address,
    pub topics: Vec<Hash>,
    pub data: Vec<u8>,
    pub block_number: u64,
    pub block_hash: Hash,
    pub transaction_hash: Hash,
    pub transaction_index: u64,
    pub log_index: u64,
}

impl Block {
    pub fn new(
        parent_hash: Hash,
        number: u64,
        timestamp: u64,
        transactions_root: Hash,
        state_root: Hash,
        difficulty: u64,
        block_reward: u64,
    ) -> Self {
        Self {
            header: BlockHeader {
                parent_hash,
                number,
                timestamp,
                transactions_root,
                state_root,
                difficulty,
                block_reward,
            },
            transactions: Vec::new(),
            receipts: HashMap::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction_hash: Hash, receipt: TransactionReceipt) {
        self.transactions.push(transaction_hash);
        self.receipts.insert(transaction_hash, receipt);
    }
}
