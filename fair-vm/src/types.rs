pub use ethers::types::{H160, H256, U256};

/// 地址类型
pub type Address = H160;

/// 哈希类型
pub type Hash = H256;

/// 余额类型
pub type Balance = U256;

/// 交易类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionType {
    /// 传统交易
    Legacy,
    /// EIP-2930 交易
    EIP2930,
    /// EIP-1559 交易
    EIP1559,
}
