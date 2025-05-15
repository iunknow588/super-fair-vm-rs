use ethers::types::{Address, Bytes, Signature, H256, U256};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// 交易状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionStatus {
    /// 待处理
    Pending,
    /// 已签名
    Signed,
    /// 已发送
    Sent,
    /// 已确认
    Confirmed,
    /// 失败
    Failed,
}

/// 交易错误
#[derive(Debug, Error)]
pub enum TransactionError {
    #[error("交易已存在")]
    TransactionExists,
    #[error("交易不存在")]
    TransactionNotFound,
    #[error("交易状态错误")]
    InvalidTransactionStatus,
    #[error("交易验证失败")]
    TransactionVerificationFailed,
    #[error("交易执行失败")]
    TransactionExecutionFailed,
    #[error("Gas 不足")]
    InsufficientGas,
    #[error("余额不足")]
    InsufficientBalance,
    #[error("网络错误")]
    NetworkError,
    #[error("其他错误: {0}")]
    Other(String),
}

/// 交易信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionInfo {
    /// 交易哈希
    pub tx_hash: H256,
    /// 发送方地址
    pub from: Address,
    /// 接收方地址
    pub to: Option<Address>,
    /// 交易金额
    pub value: U256,
    /// 交易数据
    pub data: Bytes,
    /// 交易序号
    pub nonce: u64,
    /// Gas 价格
    pub gas_price: U256,
    /// Gas 限制
    pub gas_limit: U256,
    /// 交易状态
    pub status: TransactionStatus,
    /// 交易签名
    pub signature: Option<Signature>,
    /// 交易时间戳
    pub timestamp: u64,
    /// 区块号
    pub block_number: Option<u64>,
    /// 区块哈希
    pub block_hash: Option<H256>,
}

/// 交易管理器
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionManager {
    /// 交易列表
    transactions: HashMap<H256, TransactionInfo>,
    /// 最大待处理交易数
    max_size: usize,
}

impl TransactionManager {
    /// 创建新的交易管理器
    pub fn new(max_size: usize) -> Self {
        Self {
            transactions: HashMap::new(),
            max_size,
        }
    }

    /// 添加交易
    pub fn add_transaction(&mut self, tx: TransactionInfo) {
        if self.transactions.len() >= self.max_size {
            // 移除最旧的交易
            let oldest = self
                .transactions
                .iter()
                .min_by_key(|(_, tx)| tx.timestamp)
                .map(|(hash, _)| *hash);
            if let Some(hash) = oldest {
                self.transactions.remove(&hash);
            }
        }
        self.transactions.insert(tx.tx_hash, tx);
    }

    /// 更新交易状态
    pub fn update_transaction_status(&mut self, tx_hash: H256, status: TransactionStatus) {
        if let Some(tx) = self.transactions.get_mut(&tx_hash) {
            tx.status = status;
        }
    }

    /// 获取交易信息
    pub fn get_transaction(&self, tx_hash: H256) -> Option<&TransactionInfo> {
        self.transactions.get(&tx_hash)
    }

    /// 获取所有交易
    pub fn get_all_transactions(&self) -> Vec<&TransactionInfo> {
        self.transactions.values().collect()
    }

    /// 获取待处理交易
    pub fn get_pending_transactions(&self) -> Vec<&TransactionInfo> {
        self.transactions
            .values()
            .filter(|tx| matches!(tx.status, TransactionStatus::Pending))
            .collect()
    }

    /// 清理已确认的交易
    pub fn cleanup_confirmed_transactions(&mut self) {
        self.transactions
            .retain(|_, tx| !matches!(tx.status, TransactionStatus::Confirmed));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::types::Address;

    #[test]
    fn test_transaction_manager() {
        let mut manager = TransactionManager::new(0);
        let addr = Address::random();
        let tx = TransactionInfo {
            tx_hash: H256::zero(),
            from: addr,
            to: None,
            value: U256::zero(),
            data: Bytes::new(),
            nonce: 0,
            gas_price: U256::zero(),
            gas_limit: U256::zero(),
            status: TransactionStatus::Pending,
            signature: None,
            timestamp: 0,
            block_number: None,
            block_hash: None,
        };

        manager.add_transaction(tx.clone());
        assert_eq!(manager.get_all_transactions().len(), 1);

        manager.cleanup_confirmed_transactions();
        assert_eq!(manager.get_all_transactions().len(), 0);
    }
}
