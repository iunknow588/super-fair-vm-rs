//! Block implementation for FairVM.

use avalanche_types::ids;
use ethers::types::{Transaction, H256, U256};
use serde::{Deserialize, Serialize};
use sha2::Digest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub parent_id: ids::Id,
    pub timestamp: u64,
    pub height: u64,
    pub gas_limit: u64,
    pub gas_used: u64,
    pub state_root: H256,
    pub transactions_root: H256,
    pub receipts_root: H256,
    /// EIP-1559基础费用
    pub base_fee: U256,
    /// 区块gas费用（用于Avalanche子网特殊费用计算）
    pub block_gas_cost: U256,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub hash: H256,
    pub parent_hash: H256,
    pub number: u64,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub state_root: H256,
    pub receipts_root: H256,
    pub gas_used: u64,
    pub gas_limit: u64,
    pub extra_data: Vec<u8>,
    /// EIP-1559基础费用
    pub base_fee: U256,
    /// 区块gas费用（用于Avalanche子网特殊费用计算）
    pub block_gas_cost: U256,
}

impl Block {
    pub fn new(
        parent_hash: H256,
        number: u64,
        timestamp: u64,
        transactions: Vec<Transaction>,
        state_root: H256,
        gas_limit: u64,
    ) -> Self {
        let mut block = Self {
            hash: H256::zero(),
            parent_hash,
            number,
            timestamp,
            transactions,
            state_root,
            receipts_root: H256([0; 32]),
            gas_used: 0,
            gas_limit,
            extra_data: Vec::new(),
            base_fee: U256::from(1_000_000_000), // 默认1 GWei
            block_gas_cost: U256::zero(),
        };

        block.hash = block.calculate_hash();
        block
    }

    /// 创建包含完整参数的区块
    #[allow(clippy::too_many_arguments)]
    pub fn new_with_fees(
        parent_hash: H256,
        number: u64,
        timestamp: u64,
        transactions: Vec<Transaction>,
        state_root: H256,
        gas_limit: u64,
        gas_used: u64,
        base_fee: U256,
        block_gas_cost: U256,
    ) -> Self {
        let mut block = Self {
            hash: H256::zero(),
            parent_hash,
            number,
            timestamp,
            transactions,
            state_root,
            receipts_root: H256([0; 32]),
            gas_used,
            gas_limit,
            extra_data: Vec::new(),
            base_fee,
            block_gas_cost,
        };

        block.hash = block.calculate_hash();
        block
    }

    fn calculate_hash(&self) -> H256 {
        let mut hasher = sha3::Keccak256::new();
        let timestamp_bytes = self.timestamp.to_be_bytes();
        hasher.update(timestamp_bytes);
        hasher.update(self.parent_hash.as_bytes());

        let result = hasher.finalize();
        H256::from_slice(&result)
    }

    pub fn id(&self) -> ids::Id {
        let bytes = serde_json::to_vec(self).unwrap();
        let hash = sha3::Keccak256::digest(&bytes);
        ids::Id::from_slice(&hash)
    }

    /// 计算下一个区块的基础费用
    /// 如果当前区块gas使用量超过目标值，基础费用增加
    /// 如果当前区块gas使用量低于目标值，基础费用减少
    pub fn calculate_next_base_fee(&self, gas_target: u64, denominator: u64) -> U256 {
        if self.gas_used == gas_target {
            return self.base_fee;
        }

        if self.gas_used > gas_target {
            let delta = self.gas_used - gas_target;
            let delta_fee = (self.base_fee * U256::from(delta))
                / U256::from(gas_target)
                / U256::from(denominator);
            self.base_fee + delta_fee
        } else {
            let delta = gas_target - self.gas_used;
            let delta_fee = (self.base_fee * U256::from(delta))
                / U256::from(gas_target)
                / U256::from(denominator);

            if delta_fee >= self.base_fee {
                U256::from(1) // 最小基础费用
            } else {
                self.base_fee - delta_fee
            }
        }
    }
}
