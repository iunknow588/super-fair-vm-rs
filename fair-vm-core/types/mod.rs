use ethers::types::{H160, H256, U256};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt;

/// 账户地址类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Address(pub [u8; 20]);

impl Address {
    /// 创建新的地址
    pub fn new(bytes: [u8; 20]) -> Self {
        Self(bytes)
    }

    /// 获取零地址
    pub fn zero() -> Self {
        Self([0; 20])
    }

    /// 获取地址字节
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// 生成随机地址
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let mut bytes = [0u8; 20];
        rng.fill(&mut bytes);
        Self(bytes)
    }
}

impl From<H160> for Address {
    fn from(h160: H160) -> Self {
        Self(h160.0)
    }
}

impl From<Address> for H160 {
    fn from(addr: Address) -> Self {
        H160(addr.0)
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{}", hex::encode(self.0))
    }
}

/// 哈希类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Hash(pub [u8; 32]);

impl Hash {
    /// 创建新的哈希
    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// 获取零哈希
    pub fn zero() -> Self {
        Self([0; 32])
    }

    /// 获取哈希字节
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// 生成随机哈希
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let mut bytes = [0u8; 32];
        rng.fill(&mut bytes);
        Self(bytes)
    }
}

impl From<H256> for Hash {
    fn from(h256: H256) -> Self {
        Self(h256.0)
    }
}

impl From<Hash> for H256 {
    fn from(hash: Hash) -> Self {
        H256(hash.0)
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{}", hex::encode(self.0))
    }
}

/// 区块头类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    /// 父区块哈希
    pub parent_hash: Hash,
    /// 区块创建者地址
    pub coinbase: Address,
    /// 状态根
    pub state_root: Hash,
    /// 交易根
    pub transactions_root: Hash,
    /// 收据根
    pub receipts_root: Hash,
    /// 区块日志
    pub logs_bloom: Vec<u8>,
    /// 区块难度
    pub difficulty: U256,
    /// 区块高度
    pub number: u64,
    /// 区块 gas 限制
    pub gas_limit: u64,
    /// 区块已使用 gas
    pub gas_used: u64,
    /// 区块时间戳
    pub timestamp: u64,
    /// 区块额外数据
    pub extra_data: Vec<u8>,
    /// 区块混合哈希
    pub mix_hash: Hash,
    /// 区块 nonce
    pub nonce: u64,
}

impl Header {
    /// 创建新的区块头
    pub fn new(
        parent_hash: Hash,
        coinbase: Address,
        state_root: Hash,
        transactions_root: Hash,
        receipts_root: Hash,
        logs_bloom: Vec<u8>,
        difficulty: U256,
        number: u64,
        gas_limit: u64,
        gas_used: u64,
        timestamp: u64,
        extra_data: Vec<u8>,
        mix_hash: Hash,
        nonce: u64,
    ) -> Self {
        Self {
            parent_hash,
            coinbase,
            state_root,
            transactions_root,
            receipts_root,
            logs_bloom,
            difficulty,
            number,
            gas_limit,
            gas_used,
            timestamp,
            extra_data,
            mix_hash,
            nonce,
        }
    }
}

/// 交易类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// 交易 nonce
    pub nonce: u64,
    /// 交易 gas 价格
    pub gas_price: U256,
    /// 交易 gas 限制
    pub gas_limit: u64,
    /// 交易接收者地址
    pub to: Option<Address>,
    /// 交易金额
    pub value: U256,
    /// 交易数据
    pub data: Vec<u8>,
    /// 交易签名
    pub signature: Option<[u8; 65]>,
}

impl Transaction {
    /// 创建新的交易
    pub fn new(
        nonce: u64,
        gas_price: U256,
        gas_limit: u64,
        to: Option<Address>,
        value: U256,
        data: Vec<u8>,
        signature: Option<[u8; 65]>,
    ) -> Self {
        Self {
            nonce,
            gas_price,
            gas_limit,
            to,
            value,
            data,
            signature,
        }
    }

    /// 检查交易是否为合约创建
    pub fn is_contract_creation(&self) -> bool {
        self.to.is_none()
    }
}

/// 收据类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receipt {
    /// 交易状态
    pub status: bool,
    /// 交易已使用 gas
    pub gas_used: u64,
    /// 交易日志
    pub logs: Vec<Log>,
    /// 交易哈希
    pub transaction_hash: Hash,
    /// 交易区块哈希
    pub block_hash: Hash,
    /// 交易区块高度
    pub block_number: u64,
}

impl Receipt {
    /// 创建新的收据
    pub fn new(
        status: bool,
        gas_used: u64,
        logs: Vec<Log>,
        transaction_hash: Hash,
        block_hash: Hash,
        block_number: u64,
    ) -> Self {
        Self {
            status,
            gas_used,
            logs,
            transaction_hash,
            block_hash,
            block_number,
        }
    }
}

/// 日志类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log {
    /// 日志地址
    pub address: Address,
    /// 日志主题
    pub topics: Vec<Hash>,
    /// 日志数据
    pub data: Vec<u8>,
}

impl Log {
    /// 创建新的日志
    pub fn new(address: Address, topics: Vec<Hash>, data: Vec<u8>) -> Self {
        Self {
            address,
            topics,
            data,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_default() {
        let addr = Address::default();
        assert_eq!(addr.0, [0; 20]);
    }

    #[test]
    fn test_address_from_h160() {
        let h160 = H160::random();
        let addr = Address::from(h160);
        assert_eq!(addr.0, h160.0);
    }

    #[test]
    fn test_address_to_h160() {
        let addr = Address::random();
        let h160 = H160::from(addr);
        assert_eq!(h160.0, addr.0);
    }

    #[test]
    fn test_address_display() {
        let addr = Address([1; 20]);
        let s = format!("{}", addr);
        assert!(s.starts_with("0x"));
        assert_eq!(s.len(), 42);
    }

    #[test]
    fn test_hash_default() {
        let hash = Hash::default();
        assert_eq!(hash.0, [0; 32]);
    }

    #[test]
    fn test_hash_from_h256() {
        let h256 = H256::random();
        let hash = Hash::from(h256);
        assert_eq!(hash.0, h256.0);
    }

    #[test]
    fn test_hash_to_h256() {
        let hash = Hash::random();
        let h256 = H256::from(hash);
        assert_eq!(h256.0, hash.0);
    }

    #[test]
    fn test_hash_display() {
        let hash = Hash([1; 32]);
        let s = format!("{}", hash);
        assert!(s.starts_with("0x"));
        assert_eq!(s.len(), 66);
    }

    #[test]
    fn test_header_new() {
        let header = Header::new(
            Hash::random(),
            Address::random(),
            Hash::random(),
            Hash::random(),
            Hash::random(),
            vec![0; 256],
            U256::from(100),
            1,
            1000000,
            500000,
            1234567890,
            vec![1, 2, 3],
            Hash::random(),
            12345,
        );

        assert_eq!(header.number, 1);
        assert_eq!(header.gas_limit, 1000000);
        assert_eq!(header.gas_used, 500000);
        assert_eq!(header.timestamp, 1234567890);
        assert_eq!(header.nonce, 12345);
    }

    #[test]
    fn test_transaction_new() {
        let transaction = Transaction::new(
            1,
            U256::from(100),
            1000000,
            Some(Address::random()),
            U256::from(1000),
            vec![1, 2, 3],
            Some([0; 65]),
        );

        assert_eq!(transaction.nonce, 1);
        assert_eq!(transaction.gas_limit, 1000000);
        assert!(transaction.to.is_some());
        assert!(!transaction.is_contract_creation());
    }

    #[test]
    fn test_transaction_contract_creation() {
        let transaction = Transaction::new(
            1,
            U256::from(100),
            1000000,
            None,
            U256::from(1000),
            vec![1, 2, 3],
            Some([0; 65]),
        );

        assert!(transaction.is_contract_creation());
    }

    #[test]
    fn test_receipt_new() {
        let receipt = Receipt::new(
            true,
            500000,
            vec![],
            Hash::random(),
            Hash::random(),
            1,
        );

        assert!(receipt.status);
        assert_eq!(receipt.gas_used, 500000);
        assert_eq!(receipt.block_number, 1);
    }

    #[test]
    fn test_log_new() {
        let log = Log::new(
            Address::random(),
            vec![Hash::random()],
            vec![1, 2, 3],
        );

        assert_eq!(log.topics.len(), 1);
        assert_eq!(log.data, vec![1, 2, 3]);
    }
}
