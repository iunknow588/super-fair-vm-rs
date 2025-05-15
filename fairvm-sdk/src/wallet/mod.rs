//! FairVM钱包实现

use crate::wallet::hardware::{HardwareAccount, HardwareWallet, HardwareWalletError};
use crate::wallet::transaction::{TransactionInfo, TransactionManager, TransactionStatus};
use ethers::types::transaction::eip712::TypedData as EthersTypedData;
use ethers::{
    core::k256::SecretKey,
    core::types::{Address, Bytes, NameOrAddress, Signature, TransactionRequest, H256, U256, U64},
    middleware::Middleware,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
    types::{Eip1559TransactionRequest, Transaction},
    utils::hash_message,
};
use generic_array::GenericArray;
use hex;
use message::{MessageSigner as MessageSignerImpl, TypedData as MessageTypedData};
use rlp;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;
use tokio::sync::RwLock;
use typenum::U32;

pub mod firmware;
pub mod hardware;
pub mod keystore;
pub mod message;
pub mod mnemonic;
pub mod transaction;

pub use ethers::types::transaction::eip712::EIP712Domain;
pub use firmware::{FirmwareError, FirmwareRequirement, LedgerFirmware, TrezorFirmware};
pub use hardware::{DeviceModel, HardwareWalletType};
pub use keystore::KeyStore;
pub use mnemonic::MnemonicManager;
pub use transaction::TransactionError;

/// 费用建议
#[derive(Debug, Clone)]
pub struct FeesSuggestion {
    /// 基础费用
    pub base_fee: U256,
    /// 建议最大费用
    pub max_fee_per_gas: U256,
    /// 建议优先费用
    pub max_priority_fee_per_gas: U256,
}

impl std::fmt::Display for FeesSuggestion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "基础费用: {} wei\n最大费用: {} wei\n优先费用: {} wei",
            self.base_fee, self.max_fee_per_gas, self.max_priority_fee_per_gas
        )
    }
}

/// 钱包错误类型
#[derive(Debug, Error)]
pub enum WalletError {
    #[error("私钥格式错误: {0}")]
    InvalidPrivateKey(String),

    #[error("签名错误: {0}")]
    SigningError(String),

    #[error("交易错误: {0}")]
    TransactionError(String),

    #[error("余额不足: 需要 {required}, 可用 {available}")]
    InsufficientFunds { required: U256, available: U256 },

    #[error("Gas 估算失败: {0}")]
    GasEstimationFailed(String),

    #[error("网络错误: {0}")]
    NetworkError(String),

    #[error("Nonce 错误: 预期 {expected}, 实际 {actual}")]
    InvalidNonce { expected: u64, actual: u64 },

    #[error("Gas 价格过低: 最低 {minimum}, 提供 {provided}")]
    GasPriceTooLow { minimum: U256, provided: U256 },

    #[error("助记词错误: {0}")]
    MnemonicError(String),

    #[error("钱包存储错误: {0}")]
    StorageError(String),

    #[error("硬件钱包错误: {0}")]
    HardwareWalletError(String),

    #[error("钱包错误: {0}")]
    WalletError(String),

    #[error("其他错误: {0}")]
    Other(String),

    #[error("验证错误: {0}")]
    VerificationError(String),

    #[error("硬件钱包错误: {0}")]
    HardwareError(String),

    #[error("消息签名错误: {0}")]
    MessageSignError(String),

    #[error("账户错误: {0}")]
    AccountError(String),
}

impl From<TransactionError> for WalletError {
    fn from(err: TransactionError) -> Self {
        WalletError::TransactionError(err.to_string())
    }
}

impl From<ethers::signers::WalletError> for WalletError {
    fn from(err: ethers::signers::WalletError) -> Self {
        WalletError::SigningError(err.to_string())
    }
}

impl From<ethers::providers::ProviderError> for WalletError {
    fn from(err: ethers::providers::ProviderError) -> Self {
        WalletError::TransactionError(err.to_string())
    }
}

impl From<HardwareWalletError> for WalletError {
    fn from(err: HardwareWalletError) -> Self {
        WalletError::HardwareWalletError(err.to_string())
    }
}

fn serialize_local_wallet<S>(wallet: &LocalWallet, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let private_key = wallet.signer().to_bytes();
    hex::encode(private_key).serialize(serializer)
}

fn deserialize_local_wallet<'de, D>(deserializer: D) -> Result<LocalWallet, D::Error>
where
    D: Deserializer<'de>,
{
    let hex_str = String::deserialize(deserializer)?;
    let private_key_bytes = hex::decode(hex_str).map_err(serde::de::Error::custom)?;
    let private_key_array: GenericArray<u8, U32> =
        GenericArray::clone_from_slice(&private_key_bytes);
    let secret_key = SecretKey::from_bytes(&private_key_array).map_err(serde::de::Error::custom)?;
    Ok(LocalWallet::from(secret_key))
}

/// 钱包类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WalletType {
    /// 本地钱包
    Local(
        #[serde(
            serialize_with = "serialize_local_wallet",
            deserialize_with = "deserialize_local_wallet"
        )]
        LocalWallet,
    ),
    /// 硬件钱包
    Hardware(HardwareWallet),
}

/// 钱包接口
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FairWallet {
    inner: WalletType,
    chain_id: u64,
    mnemonic: Option<String>,
    #[serde(skip)]
    transaction_manager: Arc<RwLock<TransactionManager>>,
}

impl FairWallet {
    /// 生成新的助记词钱包
    pub fn generate_new(chain_id: u64) -> Result<Self, WalletError> {
        let mnemonic = mnemonic::MnemonicManager::generate()?;
        let inner = mnemonic.create_wallet(None, chain_id)?;
        Ok(Self {
            inner,
            chain_id,
            mnemonic: Some(mnemonic.get_phrase().to_string()),
            transaction_manager: Arc::new(RwLock::new(TransactionManager::new(100))),
        })
    }

    /// 从助记词创建钱包
    pub fn from_mnemonic(phrase: &str, chain_id: u64) -> Result<Self, WalletError> {
        let mnemonic = mnemonic::MnemonicManager::from_phrase(phrase)?;
        let inner = mnemonic.create_wallet(None, chain_id)?;
        Ok(Self {
            inner,
            chain_id,
            mnemonic: Some(phrase.to_string()),
            transaction_manager: Arc::new(RwLock::new(TransactionManager::new(100))),
        })
    }

    /// 从私钥创建钱包
    pub fn from_private_key(private_key: &str, chain_id: u64) -> Result<Self, WalletError> {
        let private_key_bytes =
            hex::decode(private_key).map_err(|e| WalletError::InvalidPrivateKey(e.to_string()))?;
        let wallet = LocalWallet::from_bytes(&private_key_bytes)
            .map_err(|e| WalletError::InvalidPrivateKey(e.to_string()))?
            .with_chain_id(chain_id);

        Ok(Self {
            inner: WalletType::Local(wallet),
            chain_id,
            mnemonic: None,
            transaction_manager: Arc::new(RwLock::new(TransactionManager::new(100))),
        })
    }

    /// 连接 Ledger 钱包
    pub async fn connect_ledger(
        derivation_path: Option<String>,
        chain_id: u64,
    ) -> Result<Self, WalletError> {
        let hw_wallet = HardwareWallet::new_ledger(derivation_path, chain_id)
            .await
            .map_err(|e| WalletError::HardwareWalletError(e.to_string()))?;

        Ok(Self {
            inner: WalletType::Hardware(hw_wallet),
            chain_id,
            mnemonic: None,
            transaction_manager: Arc::new(RwLock::new(TransactionManager::new(100))),
        })
    }

    /// 连接 Trezor 钱包
    pub async fn connect_trezor(
        derivation_path: Option<String>,
        chain_id: u64,
    ) -> Result<Self, WalletError> {
        let hw_wallet = HardwareWallet::new_trezor(derivation_path, chain_id)
            .await
            .map_err(|e| WalletError::HardwareWalletError(e.to_string()))?;

        Ok(Self {
            inner: WalletType::Hardware(hw_wallet),
            chain_id,
            mnemonic: None,
            transaction_manager: Arc::new(RwLock::new(TransactionManager::new(100))),
        })
    }

    /// 获取助记词
    pub fn get_mnemonic(&self) -> Option<&str> {
        self.mnemonic.as_deref()
    }

    /// 导出私钥
    pub fn export_private_key(&self) -> String {
        match &self.inner {
            WalletType::Local(wallet) => hex::encode(wallet.signer().to_bytes()),
            WalletType::Hardware(_) => "Hardware wallet does not expose private key".to_string(),
        }
    }

    /// 获取钱包地址
    pub async fn address(&self) -> Result<Address, WalletError> {
        match &self.inner {
            WalletType::Local(wallet) => Ok(wallet.address()),
            WalletType::Hardware(hw_wallet) => {
                Ok(hw_wallet.get_current_account().unwrap_or_default())
            }
        }
    }

    /// 签名消息
    pub async fn sign_message(&self, message: &[u8]) -> Result<Signature, WalletError> {
        match &self.inner {
            WalletType::Local(wallet) => wallet
                .sign_message(message)
                .await
                .map_err(|e| WalletError::SigningError(e.to_string())),
            WalletType::Hardware(hw_wallet) => hw_wallet
                .sign_message(message)
                .await
                .map_err(|e| WalletError::HardwareWalletError(e.to_string())),
        }
    }

    /// 验证签名
    pub async fn verify_signature(
        &self,
        message: &[u8],
        signature: &Signature,
    ) -> Result<bool, WalletError> {
        let address = self.address().await?;
        Ok(signature.verify(message, address).is_ok())
    }

    /// 签名交易
    pub async fn sign_transaction(
        &self,
        tx: TransactionRequest,
    ) -> Result<Transaction, WalletError> {
        let tx_for_local = tx.clone();
        let tx_for_hardware = tx.clone();
        let tx_for_build = tx;
        match &self.inner {
            WalletType::Local(local) => {
                let signature = local
                    .sign_transaction(
                        &ethers::types::transaction::eip2718::TypedTransaction::Legacy(
                            tx_for_local.clone(),
                        ),
                    )
                    .await
                    .map_err(|e| WalletError::SigningError(format!("本地钱包签名失败: {}", e)))?;
                Ok(Transaction {
                    hash: H256::zero(),
                    nonce: tx_for_local.nonce.unwrap_or_default(),
                    block_hash: None,
                    block_number: None,
                    transaction_index: None,
                    from: local.address(),
                    to: tx_for_local.to.map(|addr| match addr {
                        NameOrAddress::Address(addr) => addr,
                        NameOrAddress::Name(_) => Address::zero(),
                    }),
                    value: tx_for_local.value.unwrap_or_default(),
                    gas_price: Some(tx_for_local.gas_price.unwrap_or_default()),
                    gas: tx_for_local.gas.unwrap_or_default(),
                    input: tx_for_local.data.clone().unwrap_or_default(),
                    v: U64::from(signature.v),
                    r: signature.r,
                    s: signature.s,
                    transaction_type: None,
                    access_list: None,
                    max_fee_per_gas: None,
                    max_priority_fee_per_gas: None,
                    chain_id: Some(U256::from(self.chain_id)),
                    other: Default::default(),
                })
            }
            WalletType::Hardware(hardware) => {
                let signature = hardware
                    .sign_transaction(tx_for_hardware)
                    .await
                    .map_err(|e| WalletError::SigningError(format!("硬件钱包签名失败: {}", e)))?;
                Ok(Transaction {
                    hash: H256::zero(),
                    nonce: tx_for_build.nonce.unwrap_or_default(),
                    block_hash: None,
                    block_number: None,
                    transaction_index: None,
                    from: hardware.get_current_account().unwrap_or_default(),
                    to: tx_for_build.to.map(|addr| match addr {
                        NameOrAddress::Address(addr) => addr,
                        NameOrAddress::Name(_) => Address::zero(),
                    }),
                    value: tx_for_build.value.unwrap_or_default(),
                    gas_price: Some(tx_for_build.gas_price.unwrap_or_default()),
                    gas: tx_for_build.gas.unwrap_or_default(),
                    input: tx_for_build.data.clone().unwrap_or_default(),
                    v: U64::from(signature.v),
                    r: signature.r,
                    s: signature.s,
                    transaction_type: None,
                    access_list: None,
                    max_fee_per_gas: None,
                    max_priority_fee_per_gas: None,
                    chain_id: Some(U256::from(self.chain_id)),
                    other: Default::default(),
                })
            }
        }
    }

    /// 发送交易
    pub async fn send_transaction(
        &self,
        client: &Provider<Http>,
        tx: TransactionRequest,
    ) -> Result<H256, WalletError> {
        let signed_tx = self.sign_transaction(tx.clone()).await?;
        let rlp = rlp::encode(&signed_tx.rlp().to_vec());
        let pending_tx = client
            .send_raw_transaction(Bytes::from(rlp.to_vec()))
            .await
            .map_err(|e| WalletError::TransactionError(e.to_string()))?;
        Ok(pending_tx.tx_hash())
    }

    /// 发送 EIP-1559 交易
    pub async fn send_eip1559_transaction(
        &self,
        client: &Provider<Http>,
        tx: Eip1559TransactionRequest,
    ) -> Result<H256, WalletError> {
        let signed_tx = self.sign_transaction(tx.into()).await?;
        let rlp = rlp::encode(&signed_tx.rlp().to_vec());
        let pending_tx = client
            .send_raw_transaction(Bytes::from(rlp.to_vec()))
            .await
            .map_err(|e| WalletError::TransactionError(e.to_string()))?;
        Ok(pending_tx.tx_hash())
    }

    /// 估算交易所需的 gas
    pub async fn estimate_gas(
        &self,
        provider: &Provider<Http>,
        to: Option<Address>,
        value: U256,
        data: Vec<u8>,
    ) -> Result<u64, WalletError> {
        let mut tx = TransactionRequest::new()
            .from(self.address().await?)
            .value(value)
            .data(data);

        if let Some(addr) = to {
            tx = tx.to(NameOrAddress::Address(addr));
        }

        let typed_tx = ethers::types::transaction::eip2718::TypedTransaction::Legacy(tx);
        provider
            .estimate_gas(&typed_tx, None)
            .await
            .map(|gas| gas.as_u64())
            .map_err(|e| WalletError::GasEstimationFailed(e.to_string()))
    }

    /// 获取当前网络的费用建议
    pub async fn get_fees(&self, provider: &Provider<Http>) -> Result<FeesSuggestion, WalletError> {
        let fee_history = provider
            .fee_history(1, ethers::types::BlockNumber::Latest, &[0.0])
            .await
            .map_err(|e| WalletError::NetworkError(e.to_string()))?;

        let base_fee = fee_history
            .base_fee_per_gas
            .first()
            .copied()
            .unwrap_or_default();

        // 建议的最大费用为基础费用的2倍
        let max_fee_per_gas = base_fee * 2;

        // 建议的优先费用为基础费用的10%
        let max_priority_fee_per_gas = base_fee / 10;

        Ok(FeesSuggestion {
            base_fee,
            max_fee_per_gas,
            max_priority_fee_per_gas,
        })
    }

    /// 获取账户 nonce
    pub async fn get_nonce(
        &self,
        provider: &Provider<Http>,
        address: Address,
    ) -> Result<u64, WalletError> {
        provider
            .get_transaction_count(address, None)
            .await
            .map(|nonce| nonce.as_u64())
            .map_err(|e| WalletError::NetworkError(e.to_string()))
    }

    /// 保存钱包到加密的密钥库文件
    pub fn save_to_keystore(
        &self,
        path: impl AsRef<Path>,
        password: &str,
    ) -> Result<(), WalletError> {
        match &self.inner {
            WalletType::Local(wallet) => {
                let private_key = wallet.signer().to_bytes();
                let keystore = keystore::KeyStore::new(&private_key, password)?;
                keystore.save_to_file(path)
            }
            _ => Err(WalletError::WalletError(
                "只有本地钱包支持导出密钥库".to_string(),
            )),
        }
    }

    /// 从加密的密钥库文件加载钱包
    pub fn load_from_keystore(
        path: impl AsRef<Path>,
        password: &str,
        chain_id: u64,
    ) -> Result<Self, WalletError> {
        let keystore = keystore::KeyStore::load_from_file(path)?;
        let private_key = keystore.decrypt(password)?;
        Self::from_private_key(&hex::encode(private_key), chain_id)
    }

    /// 获取硬件钱包类型
    pub async fn get_hardware_wallet_type(&self) -> Option<HardwareWalletType> {
        match &self.inner {
            WalletType::Hardware(hw) => Some(hw.wallet_type()),
            _ => None,
        }
    }

    /// 获取硬件钱包派生路径
    pub async fn get_hardware_derivation_path(&self) -> Option<String> {
        match &self.inner {
            WalletType::Hardware(hw) => Some(hw.get_derivation_path().to_string()),
            _ => None,
        }
    }

    /// 获取硬件钱包账户列表
    pub async fn get_hardware_accounts(&self) -> Result<Vec<HardwareAccount>, WalletError> {
        match &self.inner {
            WalletType::Hardware(hw) => {
                let addresses = hw.get_accounts();
                Ok(addresses
                    .into_iter()
                    .map(|addr| HardwareAccount {
                        address: addr,
                        derivation_path: hw.get_derivation_path().to_string(),
                        index: 0,
                    })
                    .collect())
            }
            _ => Err(WalletError::HardwareWalletError("不是硬件钱包".to_string())),
        }
    }

    /// 设置当前硬件钱包账户
    pub async fn set_hardware_current_account(
        &mut self,
        address: Address,
    ) -> Result<Address, WalletError> {
        match &mut self.inner {
            WalletType::Hardware(hardware) => {
                let accounts = hardware.get_accounts();
                if let Some(index) = accounts.iter().position(|&addr| addr == address) {
                    hardware
                        .set_current_account(index)
                        .map_err(|e| WalletError::HardwareWalletError(e.to_string()))
                } else {
                    Err(WalletError::HardwareWalletError("账户不存在".to_string()))
                }
            }
            _ => Err(WalletError::HardwareWalletError("不是硬件钱包".to_string())),
        }
    }

    /// 获取当前硬件钱包账户
    pub async fn get_current_hardware_account(
        &self,
    ) -> Result<Option<HardwareAccount>, WalletError> {
        match &self.inner {
            WalletType::Hardware(hw) => Ok(hw.get_current_account().map(|addr| HardwareAccount {
                address: addr,
                derivation_path: hw.get_derivation_path().to_string(),
                index: 0,
            })),
            _ => Err(WalletError::HardwareWalletError("不是硬件钱包".to_string())),
        }
    }

    /// 创建新的钱包实例
    pub async fn new(wallet_type: WalletType, chain_id: u64, nonce: U256) -> Self {
        Self {
            inner: wallet_type,
            chain_id,
            mnemonic: None,
            transaction_manager: Arc::new(RwLock::new(TransactionManager::new(
                nonce.as_u64() as usize
            ))),
        }
    }

    /// 获取交易
    pub async fn get_transactions(&self) -> Vec<TransactionInfo> {
        let manager = self.transaction_manager.read().await;
        manager
            .get_all_transactions()
            .into_iter()
            .cloned()
            .collect()
    }

    /// 获取所有待处理的交易
    pub async fn get_pending_transactions(&self) -> Vec<TransactionInfo> {
        let manager = self.transaction_manager.read().await;
        manager
            .get_pending_transactions()
            .into_iter()
            .cloned()
            .collect()
    }

    /// 清理已完成的交易
    pub async fn cleanup_completed_transactions(&self) {
        let mut manager = self.transaction_manager.write().await;
        manager.cleanup_confirmed_transactions();
    }

    /// 重置失败的交易
    pub async fn reset_failed_transactions(&self) {
        let mut manager = self.transaction_manager.write().await;
        let failed_txs: Vec<H256> = manager
            .get_all_transactions()
            .iter()
            .filter(|tx| matches!(tx.status, TransactionStatus::Failed))
            .map(|tx| tx.tx_hash)
            .collect();

        for tx_hash in failed_txs {
            manager.update_transaction_status(tx_hash, TransactionStatus::Pending);
        }
    }

    /// 获取当前 nonce
    pub async fn get_current_nonce(&self) -> U256 {
        let manager = self.transaction_manager.read().await;
        U256::from(manager.get_all_transactions().len() as u64)
    }

    /// 签名普通消息
    pub async fn sign_personal_message(&self, message: &[u8]) -> Result<Signature, WalletError> {
        match &self.inner {
            WalletType::Local(wallet) => {
                let signer = MessageSignerImpl::new(wallet.address());
                signer
                    .sign_message(message)
                    .map_err(|e| WalletError::SigningError(e.to_string()))
            }
            WalletType::Hardware(hw_wallet) => hw_wallet
                .sign_message(message)
                .await
                .map_err(|e| WalletError::HardwareWalletError(e.to_string())),
        }
    }

    /// 验证普通消息签名
    pub async fn verify_personal_message(
        &self,
        message: &[u8],
        signature: &Signature,
    ) -> Result<bool, WalletError> {
        let address = self.address().await?;
        let signer = MessageSignerImpl::new(address);
        signer
            .verify_message(message, signature, address)
            .map_err(|e| WalletError::VerificationError(e.to_string()))
    }

    /// 签名类型化数据
    pub async fn sign_typed_data(
        &self,
        typed_data: &MessageTypedData,
    ) -> Result<Signature, WalletError> {
        match &self.inner {
            WalletType::Local(local) => {
                let ethers_typed_data = EthersTypedData {
                    types: typed_data.types.clone().into_iter().collect(),
                    primary_type: typed_data.primary_type.clone(),
                    domain: typed_data.domain.clone(),
                    message: typed_data.message.clone(),
                };
                local
                    .sign_typed_data(&ethers_typed_data)
                    .await
                    .map_err(|e| WalletError::MessageSignError(e.to_string()))
            }
            WalletType::Hardware(hardware) => {
                let address = hardware
                    .get_current_account()
                    .ok_or_else(|| WalletError::HardwareWalletError("未选择账户".to_string()))?;
                let ethers_typed_data = EthersTypedData {
                    types: typed_data.types.clone().into_iter().collect(),
                    primary_type: typed_data.primary_type.clone(),
                    domain: typed_data.domain.clone(),
                    message: typed_data.message.clone(),
                };
                let signature = hardware.sign_typed_data(&ethers_typed_data).await?;
                let signer = MessageSignerImpl::new(address);
                signer
                    .verify_typed_data_signature(typed_data, &signature, address)
                    .map_err(|e| WalletError::MessageSignError(e.to_string()))?;
                Ok(signature)
            }
        }
    }

    /// 验证类型化数据签名
    pub async fn verify_typed_data_signature(
        &self,
        typed_data: &MessageTypedData,
        signature: &Signature,
    ) -> Result<bool, WalletError> {
        match &self.inner {
            WalletType::Local(local) => {
                let address = local.address();
                let signer = MessageSignerImpl::new(address);
                Ok(signer
                    .verify_typed_data_signature(typed_data, signature, address)
                    .map_err(|e| WalletError::VerificationError(e.to_string()))?)
            }
            WalletType::Hardware(hardware) => {
                let address = hardware
                    .get_current_account()
                    .ok_or_else(|| WalletError::HardwareWalletError("未选择账户".to_string()))?;
                let signer = MessageSignerImpl::new(address);
                Ok(signer
                    .verify_typed_data_signature(typed_data, signature, address)
                    .map_err(|e| WalletError::VerificationError(e.to_string()))?)
            }
        }
    }

    pub async fn verify_message(
        &self,
        message: &[u8],
        signature: &Signature,
    ) -> Result<bool, WalletError> {
        let address = self.address().await?;
        let message_hash = hash_message(message);
        Ok(signature.verify(message_hash, address).is_ok())
    }

    /// 创建新的本地钱包实例
    pub fn new_local(private_key: &str, chain_id: u64) -> Result<Self, WalletError> {
        let wallet = LocalWallet::from_str(private_key)
            .map_err(|e| WalletError::AccountError(format!("无效的私钥: {}", e)))?;

        Ok(Self {
            inner: WalletType::Local(wallet),
            chain_id,
            mnemonic: None,
            transaction_manager: Arc::new(RwLock::new(TransactionManager::new(100))),
        })
    }

    /// 从助记词创建新的本地钱包实例
    pub fn new_from_mnemonic(mnemonic: &str, chain_id: u64) -> Result<Self, WalletError> {
        let wallet = LocalWallet::from_str(mnemonic)
            .map_err(|e| WalletError::AccountError(format!("无效的助记词: {}", e)))?;

        Ok(Self {
            inner: WalletType::Local(wallet),
            chain_id,
            mnemonic: Some(mnemonic.to_string()),
            transaction_manager: Arc::new(RwLock::new(TransactionManager::new(100))),
        })
    }

    /// 创建新的硬件钱包实例
    pub async fn new_hardware(
        wallet_type: HardwareWalletType,
        base_path: &str,
        chain_id: u64,
    ) -> Result<Self, WalletError> {
        let hw_wallet = match wallet_type {
            HardwareWalletType::Ledger(_) => {
                HardwareWallet::new_ledger(Some(base_path.to_string()), chain_id).await?
            }
            HardwareWalletType::Trezor(_) => {
                HardwareWallet::new_trezor(Some(base_path.to_string()), chain_id).await?
            }
        };

        Ok(Self {
            inner: WalletType::Hardware(hw_wallet),
            chain_id,
            mnemonic: None,
            transaction_manager: Arc::new(RwLock::new(TransactionManager::new(100))),
        })
    }

    pub async fn export_keystore(&self, path: &str, password: &str) -> Result<(), WalletError> {
        match &self.inner {
            WalletType::Local(wallet) => {
                let private_key = wallet.signer().to_bytes();
                let keystore = keystore::KeyStore::new(&private_key, password)?;
                keystore.save_to_file(path)
            }
            _ => Err(WalletError::WalletError(
                "只有本地钱包支持导出密钥库".to_string(),
            )),
        }
    }

    /// 添加硬件钱包账户
    pub async fn add_hardware_account(&mut self, index: u32) -> Result<Address, WalletError> {
        match &mut self.inner {
            WalletType::Hardware(hardware) => hardware
                .add_account(index)
                .await
                .map_err(|e| WalletError::HardwareWalletError(e.to_string())),
            _ => Err(WalletError::HardwareWalletError("不是硬件钱包".to_string())),
        }
    }

    /// 添加交易
    pub async fn add_transactions(
        &self,
        transactions: Vec<TransactionRequest>,
    ) -> Result<Vec<H256>, WalletError> {
        let mut manager = self.transaction_manager.write().await;
        let mut tx_hashes = Vec::new();

        for tx in transactions {
            let tx_hash = ethers::utils::keccak256(rlp::encode(&tx.rlp().to_vec()));
            let tx_info = TransactionInfo {
                tx_hash: H256::from_slice(&tx_hash),
                from: self.address().await?,
                to: tx.to.map(|addr| match addr {
                    NameOrAddress::Address(addr) => addr,
                    NameOrAddress::Name(_) => Address::zero(),
                }),
                value: tx.value.unwrap_or_default(),
                data: tx.data.unwrap_or_default(),
                nonce: tx.nonce.unwrap_or_default().as_u64(),
                gas_price: tx.gas_price.unwrap_or_default(),
                gas_limit: tx.gas.unwrap_or_default(),
                status: TransactionStatus::Pending,
                signature: None,
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                block_number: None,
                block_hash: None,
            };
            manager.add_transaction(tx_info);
            tx_hashes.push(H256::from_slice(&tx_hash));
        }

        Ok(tx_hashes)
    }

    /// 签名所有待处理交易
    pub async fn sign_pending_transactions(&self) -> Result<Vec<TransactionRequest>, WalletError> {
        let manager = self.transaction_manager.write().await;
        let mut signed_txs = Vec::new();
        let pending = manager.get_pending_transactions();

        for tx_info in pending {
            let mut tx = TransactionRequest::new()
                .value(tx_info.value)
                .data(tx_info.data.clone())
                .nonce(tx_info.nonce)
                .gas_price(tx_info.gas_price)
                .gas(tx_info.gas_limit);

            if let Some(to) = tx_info.to {
                tx = tx.to(NameOrAddress::Address(to));
            }

            let _signed_tx = self.sign_transaction(tx.clone()).await?;
            signed_txs.push(tx);
        }
        Ok(signed_txs)
    }

    /// 获取交易信息
    pub async fn get_transaction(&self, tx_id: H256) -> Result<TransactionInfo, WalletError> {
        let manager = self.transaction_manager.read().await;
        manager
            .get_transaction(tx_id)
            .cloned()
            .ok_or_else(|| WalletError::TransactionError("交易不存在".to_string()))
    }

    pub async fn get_accounts(&self) -> Vec<Address> {
        match &self.inner {
            WalletType::Local(local) => vec![local.address()],
            WalletType::Hardware(hardware) => hardware.get_accounts(),
        }
    }

    pub async fn get_current_account(&self) -> Option<Address> {
        match &self.inner {
            WalletType::Local(local) => Some(local.address()),
            WalletType::Hardware(hardware) => hardware.get_current_account(),
        }
    }

    pub async fn set_current_account(&mut self, address: Address) -> Result<Address, WalletError> {
        match &mut self.inner {
            WalletType::Local(local) => {
                if local.address() == address {
                    Ok(address)
                } else {
                    Err(WalletError::WalletError(
                        "本地钱包不支持切换账户".to_string(),
                    ))
                }
            }
            WalletType::Hardware(hardware) => {
                let accounts = hardware.get_accounts();
                if let Some(index) = accounts.iter().position(|&addr| addr == address) {
                    hardware
                        .set_current_account(index)
                        .map_err(|e| WalletError::HardwareWalletError(e.to_string()))
                } else {
                    Err(WalletError::HardwareWalletError("账户不存在".to_string()))
                }
            }
        }
    }
}

pub enum TransactionType {
    Legacy,
    EIP2930,
    EIP1559,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wallet::hardware::HardwareWallet;
    use ethers::types::transaction::eip712::{EIP712Domain, TypedData as EthersTypedData};
    use ethers::utils::hash_message;
    use std::collections::BTreeMap;

    #[tokio::test]
    async fn test_ledger_wallet() {
        let mut wallet = HardwareWallet::new_ledger(None, 1).await.unwrap();

        // 添加多个账户
        let _addr1 = wallet.add_account(0).await.unwrap();
        let addr2 = wallet.add_account(1).await.unwrap();
        let _addr3 = wallet.add_account(2).await.unwrap();

        // 验证账户数量
        assert_eq!(wallet.get_accounts().len(), 3);

        // 测试账户切换
        wallet.set_current_account(1).unwrap();
        assert_eq!(wallet.get_current_account().unwrap(), addr2);

        // 测试消息签名
        let message = b"Hello, FairVM!".to_vec();
        let signature = wallet.sign_message(&message).await.unwrap();
        let message_hash = hash_message(&message);
        assert!(signature.verify(message_hash, addr2).is_ok());
    }

    #[tokio::test]
    async fn test_trezor_wallet() {
        let mut wallet = HardwareWallet::new_trezor(None, 1).await.unwrap();

        // 添加多个账户
        let _addr1 = wallet.add_account(0).await.unwrap();
        let addr2 = wallet.add_account(1).await.unwrap();
        let _addr3 = wallet.add_account(2).await.unwrap();

        // 验证账户数量
        assert_eq!(wallet.get_accounts().len(), 3);

        // 测试账户切换
        wallet.set_current_account(1).unwrap();
        assert_eq!(wallet.get_current_account().unwrap(), addr2);

        // 测试消息签名
        let message = b"Hello, FairVM!".to_vec();
        let signature = wallet.sign_message(&message).await.unwrap();
        let message_hash = hash_message(&message);
        assert!(signature.verify(message_hash, addr2).is_ok());
    }

    #[tokio::test]
    async fn test_sign_typed_data() {
        let wallet = HardwareWallet::new_ledger(None, 1).await.unwrap();
        let types: BTreeMap<String, Vec<ethers::types::transaction::eip712::Eip712DomainType>> =
            BTreeMap::new();
        let domain = EIP712Domain {
            name: Some("Test".to_string()),
            version: Some("1".to_string()),
            chain_id: Some(U256::from(1)),
            verifying_contract: Some(Address::zero()),
            salt: None,
        };
        let _typed_data = EthersTypedData {
            types,
            primary_type: "Test".to_string(),
            domain,
            message: BTreeMap::new(),
        };
        let result = wallet.sign_typed_data(&_typed_data).await;
        assert!(result.is_err());
    }
}
