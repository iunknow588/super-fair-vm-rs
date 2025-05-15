use crate::account::Address;
use ethers::types::{H256, U256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionType {
    Legacy,
    EIP2930,
    EIP1559,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub hash: H256,
    pub from: Address,
    pub to: Option<Address>,
    pub value: U256,
    pub nonce: u64,
    pub gas_limit: u64,
    pub gas_price: Option<U256>,
    pub data: Vec<u8>,
    pub signature: Vec<u8>,
    pub transaction_type: TransactionType,
    pub chain_id: u64,
    pub max_fee_per_gas: Option<U256>,
    pub max_priority_fee_per_gas: Option<U256>,
}

impl Transaction {
    /// 创建新的交易
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        hash: H256,
        from: Address,
        to: Option<Address>,
        value: U256,
        nonce: u64,
        gas_limit: u64,
        gas_price: Option<U256>,
        data: Vec<u8>,
        signature: Vec<u8>,
        transaction_type: TransactionType,
        chain_id: u64,
        max_fee_per_gas: Option<U256>,
        max_priority_fee_per_gas: Option<U256>,
    ) -> Self {
        Self {
            hash,
            from,
            to,
            value,
            nonce,
            gas_limit,
            gas_price,
            data,
            signature,
            transaction_type,
            chain_id,
            max_fee_per_gas,
            max_priority_fee_per_gas,
        }
    }

    pub fn hash(&self) -> H256 {
        self.hash
    }

    pub fn from(&self) -> &Address {
        &self.from
    }

    pub fn to(&self) -> Option<&Address> {
        self.to.as_ref()
    }

    pub fn value(&self) -> U256 {
        self.value
    }

    pub fn nonce(&self) -> u64 {
        self.nonce
    }

    pub fn gas_limit(&self) -> u64 {
        self.gas_limit
    }

    pub fn gas_price(&self) -> Option<U256> {
        self.gas_price
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn signature(&self) -> &[u8] {
        &self.signature
    }

    pub fn transaction_type(&self) -> &TransactionType {
        &self.transaction_type
    }

    pub fn chain_id(&self) -> u64 {
        self.chain_id
    }

    pub fn max_fee_per_gas(&self) -> Option<U256> {
        self.max_fee_per_gas
    }

    pub fn max_priority_fee_per_gas(&self) -> Option<U256> {
        self.max_priority_fee_per_gas
    }

    /// 验证交易签名
    pub fn verify_signature(&self) -> bool {
        // TODO: 实现实际的签名验证逻辑
        // 目前返回 true 作为占位符
        true
    }

    /// 验证交易是否满足最低 gas 价格要求
    pub fn validate(&self, base_fee: U256, min_gas_price: U256) -> bool {
        match self.transaction_type {
            TransactionType::Legacy => {
                if let Some(gas_price) = self.gas_price {
                    gas_price >= min_gas_price
                } else {
                    false
                }
            }
            TransactionType::EIP1559 => {
                if let (Some(max_fee), Some(max_priority_fee)) =
                    (self.max_fee_per_gas, self.max_priority_fee_per_gas)
                {
                    max_fee >= base_fee && max_priority_fee >= min_gas_price
                } else {
                    false
                }
            }
            TransactionType::EIP2930 => {
                if let Some(gas_price) = self.gas_price {
                    gas_price >= min_gas_price
                } else {
                    false
                }
            }
        }
    }
}
