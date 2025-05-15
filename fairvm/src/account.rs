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
        let mut rng = rand::rng();
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

/// 账户类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Account {
    /// 账户地址
    pub address: Address,
    /// 账户余额
    pub balance: U256,
    /// 账户 nonce
    pub nonce: u64,
    /// 账户代码哈希
    pub code_hash: H256,
    /// 账户存储根
    pub storage_root: H256,
}

impl Account {
    /// 创建新账户
    pub fn new(address: Address) -> Self {
        Self {
            address,
            balance: U256::zero(),
            nonce: 0,
            code_hash: H256::zero(),
            storage_root: H256::zero(),
        }
    }

    /// 检查账户是否为空
    pub fn is_empty(&self) -> bool {
        self.balance.is_zero() && self.nonce == 0 && self.code_hash.is_zero()
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
    fn test_account_new() {
        let addr = Address::random();
        let account = Account::new(addr);
        assert_eq!(account.address, addr);
        assert!(account.balance.is_zero());
        assert_eq!(account.nonce, 0);
        assert!(account.code_hash.is_zero());
        assert!(account.storage_root.is_zero());
    }

    #[test]
    fn test_account_is_empty() {
        let addr = Address::random();
        let mut account = Account::new(addr);
        assert!(account.is_empty());

        account.balance = U256::from(100);
        assert!(!account.is_empty());

        account.balance = U256::zero();
        account.nonce = 1;
        assert!(!account.is_empty());

        account.nonce = 0;
        account.code_hash = H256::random();
        assert!(!account.is_empty());
    }
}
