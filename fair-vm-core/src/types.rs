use primitive_types::{H160, H256, U256};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt;

/// 地址类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Address(pub H160);

impl Address {
    /// 创建随机地址
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let mut bytes = [0u8; 20];
        rng.fill(&mut bytes);
        Self(H160(bytes))
    }

    /// 从字节数组创建地址
    pub fn from_bytes(bytes: [u8; 20]) -> Self {
        Self(H160(bytes))
    }

    /// 获取地址字节
    pub fn as_bytes(&self) -> &[u8; 20] {
        &self.0 .0
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{}", hex::encode(self.0 .0))
    }
}

/// 哈希类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hash(pub H256);

impl Hash {
    /// 创建随机哈希
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let mut bytes = [0u8; 32];
        rng.fill(&mut bytes);
        Self(H256(bytes))
    }

    /// 从字节数组创建哈希
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(H256(bytes))
    }

    /// 获取哈希字节
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0 .0
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{}", hex::encode(self.0 .0))
    }
}

/// 区块头
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    /// 父区块哈希
    pub parent_hash: Hash,
    /// 区块号
    pub number: u64,
    /// 时间戳
    pub timestamp: u64,
    /// 交易根
    pub transactions_root: Hash,
    /// 状态根
    pub state_root: Hash,
    /// 收据根
    pub receipts_root: Hash,
    /// 区块哈希
    pub hash: Hash,
}

impl Header {
    /// 创建新区块头
    pub fn new(
        parent_hash: Hash,
        number: u64,
        timestamp: u64,
        transactions_root: Hash,
        state_root: Hash,
        receipts_root: Hash,
    ) -> Self {
        let mut header = Self {
            parent_hash,
            number,
            timestamp,
            transactions_root,
            state_root,
            receipts_root,
            hash: Hash::random(), // 临时哈希，实际应该计算
        };
        header.hash = header.calculate_hash();
        header
    }

    /// 计算区块哈希
    pub fn calculate_hash(&self) -> Hash {
        // 这里应该实现实际的哈希计算
        // 暂时返回随机哈希
        Hash::random()
    }
}

/// 交易
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// 发送者地址
    pub from: Address,
    /// 接收者地址
    pub to: Option<Address>,
    /// 交易值
    pub value: U256,
    /// 交易数据
    pub data: Vec<u8>,
    /// nonce
    pub nonce: u64,
    /// gas价格
    pub gas_price: U256,
    /// gas限制
    pub gas_limit: u64,
    /// 交易哈希
    pub hash: Hash,
}

impl Transaction {
    /// 创建新交易
    pub fn new(
        from: Address,
        to: Option<Address>,
        value: U256,
        data: Vec<u8>,
        nonce: u64,
        gas_price: U256,
        gas_limit: u64,
    ) -> Self {
        let mut tx = Self {
            from,
            to,
            value,
            data,
            nonce,
            gas_price,
            gas_limit,
            hash: Hash::random(), // 临时哈希，实际应该计算
        };
        tx.hash = tx.calculate_hash();
        tx
    }

    /// 计算交易哈希
    pub fn calculate_hash(&self) -> Hash {
        // 这里应该实现实际的哈希计算
        // 暂时返回随机哈希
        Hash::random()
    }
}

/// 交易收据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receipt {
    /// 交易哈希
    pub transaction_hash: Hash,
    /// 区块哈希
    pub block_hash: Hash,
    /// 区块号
    pub block_number: u64,
    /// 发送者地址
    pub from: Address,
    /// 接收者地址
    pub to: Option<Address>,
    /// 合约地址（如果是合约创建）
    pub contract_address: Option<Address>,
    /// gas使用量
    pub gas_used: u64,
    /// 状态（true表示成功）
    pub status: bool,
    /// 日志
    pub logs: Vec<Log>,
}

/// 日志
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log {
    /// 合约地址
    pub address: Address,
    /// 主题
    pub topics: Vec<Hash>,
    /// 数据
    pub data: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address() {
        let address = Address::random();
        assert_eq!(address.as_bytes().len(), 20);
    }

    #[test]
    fn test_hash() {
        let hash = Hash::random();
        assert_eq!(hash.as_bytes().len(), 32);
    }

    #[test]
    fn test_header() {
        let parent_hash = Hash::random();
        let transactions_root = Hash::random();
        let state_root = Hash::random();
        let receipts_root = Hash::random();
        let header = Header::new(
            parent_hash,
            1,
            1234567890,
            transactions_root,
            state_root,
            receipts_root,
        );
        assert_eq!(header.parent_hash, parent_hash);
        assert_eq!(header.number, 1);
        assert_eq!(header.timestamp, 1234567890);
        assert_eq!(header.transactions_root, transactions_root);
        assert_eq!(header.state_root, state_root);
        assert_eq!(header.receipts_root, receipts_root);
    }

    #[test]
    fn test_transaction() {
        let from = Address::random();
        let to = Some(Address::random());
        let value = U256::from(100);
        let data = vec![1, 2, 3];
        let nonce = 1;
        let gas_price = U256::from(1000);
        let gas_limit = 21000;
        let tx = Transaction::new(from, to, value, data.clone(), nonce, gas_price, gas_limit);
        assert_eq!(tx.from, from);
        assert_eq!(tx.to, to);
        assert_eq!(tx.value, value);
        assert_eq!(tx.data, data);
        assert_eq!(tx.nonce, nonce);
        assert_eq!(tx.gas_price, gas_price);
        assert_eq!(tx.gas_limit, gas_limit);
    }
}
