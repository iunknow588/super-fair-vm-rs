//! FairVM客户端实现

use crate::wallet::FairWallet as Wallet;
use crate::SdkConfig;
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::types::H256;
use ethers::types::{
    Address, BlockId, BlockNumber, Transaction, TransactionReceipt, TransactionRequest, TxHash,
    U256,
};
use std::sync::Arc;
use thiserror::Error;
use url::Url;

/// 客户端错误类型
#[derive(Debug, Error)]
pub enum ClientError {
    #[error("网络错误: {0}")]
    NetworkError(String),

    #[error("交易错误: {0}")]
    TransactionError(String),

    #[error("Gas 估算失败: {0}")]
    GasEstimationFailed(String),

    #[error("余额不足: 需要 {required}, 可用 {available}")]
    InsufficientFunds { required: U256, available: U256 },

    #[error("Nonce 错误: 预期 {expected}, 实际 {actual}")]
    InvalidNonce { expected: u64, actual: u64 },

    #[error("Gas 价格过低: 最低 {minimum}, 提供 {provided}")]
    GasPriceTooLow { minimum: U256, provided: U256 },

    #[error("其他错误: {0}")]
    Other(String),
}

/// FairVM客户端
pub struct Client {
    /// SDK配置
    config: SdkConfig,
    /// HTTP客户端
    #[allow(dead_code)]
    http_client: reqwest::Client,
    provider: Arc<Provider<Http>>,
    #[allow(dead_code)]
    wallet: Option<Wallet>,
}

impl Client {
    /// 创建新的客户端实例
    pub fn new(rpc_url: &str) -> Result<Self, String> {
        let url = Url::parse(rpc_url).map_err(|e| e.to_string())?;
        let http_client = Http::new(url);
        let provider = Provider::new(http_client);

        Ok(Self {
            http_client: reqwest::Client::new(),
            config: SdkConfig::default(),
            provider: Arc::new(provider),
            wallet: None,
        })
    }

    /// 使用钱包创建新的客户端实例
    pub fn with_wallet(provider: Provider<Http>, wallet: Wallet) -> Self {
        Self {
            provider: Arc::new(provider),
            config: SdkConfig::default(),
            http_client: reqwest::Client::new(),
            wallet: Some(wallet),
        }
    }
    /// 获取链信息
    pub async fn get_chain_info(&self) -> Result<serde_json::Value, reqwest::Error> {
        // TODO: 实现真实的API调用
        Ok(serde_json::json!({
            "chainId": self.config.chain_id,
            "networkId": self.config.network_id,
            "blockHeight": 0,
        }))
    }

    /// 获取账户交易数量
    pub async fn get_transaction_count(
        &self,
        address: Address,
        block: Option<BlockId>,
    ) -> Result<u64, String> {
        self.provider
            .get_transaction_count(address, block)
            .await
            .map(|n| n.as_u64())
            .map_err(|e| e.to_string())
    }

    /// 发送原始交易
    pub async fn send_raw_transaction(&self, tx: Vec<u8>) -> Result<H256, String> {
        let pending = self
            .provider
            .send_raw_transaction(tx.into())
            .await
            .map_err(|e| e.to_string())?;
        let tx_hash = pending.tx_hash();
        Ok(tx_hash)
    }

    /// 获取交易收据
    pub async fn get_transaction_receipt(
        &self,
        tx_hash: TxHash,
    ) -> Result<Option<TransactionReceipt>, String> {
        self.provider
            .get_transaction_receipt(tx_hash)
            .await
            .map_err(|e| e.to_string())
    }

    /// 获取交易详情
    pub async fn get_transaction(&self, tx_hash: TxHash) -> Result<Option<Transaction>, String> {
        self.provider
            .get_transaction(tx_hash)
            .await
            .map_err(|e| e.to_string())
    }

    /// 获取账户余额
    pub async fn get_balance(
        &self,
        address: Address,
        block: Option<BlockId>,
    ) -> Result<U256, String> {
        self.provider
            .get_balance(address, block)
            .await
            .map_err(|e| e.to_string())
    }

    /// 估算交易所需的 gas
    pub async fn estimate_gas(
        &self,
        tx: &TransactionRequest,
        block: Option<BlockId>,
    ) -> Result<u64, String> {
        let typed_tx: TypedTransaction = tx.clone().into();
        self.provider
            .estimate_gas(&typed_tx, block)
            .await
            .map(|n| n.as_u64())
            .map_err(|e| e.to_string())
    }

    /// 获取当前区块的基础费用
    pub async fn get_base_fee(&self) -> Result<U256, String> {
        let block = self
            .provider
            .get_block(BlockId::Number(BlockNumber::Latest))
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "无法获取最新区块".to_string())?;

        block
            .base_fee_per_gas
            .ok_or_else(|| "区块中没有基础费用信息".to_string())
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new("http://localhost:8545").expect("Failed to create default client")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() {
        let client = Client::new("http://localhost:8545").unwrap();
        assert!(client.get_balance(Address::zero(), None).await.is_ok());
    }
}
