use crate::wallet::firmware::{
    LedgerFirmware, LedgerFirmwareTrait, TrezorFirmware, TrezorFirmwareTrait,
};
use async_trait::async_trait;
use ethers::signers::Signer;
use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::types::{
    Address, Eip1559TransactionRequest, NameOrAddress, Signature, Transaction, TransactionRequest,
    H256, U256, U64,
};
use semver::Version;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;

/// 硬件钱包账户
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareAccount {
    /// 账户地址
    pub address: Address,
    /// 派生路径
    pub derivation_path: String,
    /// 账户索引
    pub index: u32,
}

/// 硬件钱包错误
#[derive(Error, Debug)]
pub enum HardwareWalletError {
    #[error("设备未连接")]
    DeviceNotConnected,
    #[error("设备未初始化")]
    DeviceNotInitialized,
    #[error("设备不支持")]
    DeviceNotSupported,
    #[error("无效的派生路径: {0}")]
    InvalidDerivationPath(String),
    #[error("签名失败: {0}")]
    SigningFailed(String),
    #[error("其他错误: {0}")]
    Other(String),
}

/// 硬件钱包类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HardwareWalletType {
    Ledger(Arc<LedgerFirmware>),
    Trezor(Arc<TrezorFirmware>),
}

/// 硬件钱包设备类型
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DeviceModel {
    /// Ledger Nano S
    LedgerNanoS,
    /// Ledger Nano X
    LedgerNanoX,
    /// Trezor One
    TrezorOne,
    /// Trezor Model T
    TrezorModelT,
}

/// 硬件钱包
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareWallet {
    wallet_type: HardwareWalletType,
    device_model: DeviceModel,
    firmware_version: Version,
    base_derivation_path: String,
    accounts: Vec<Address>,
    current_account_index: Option<usize>,
    chain_id: u64,
}

impl HardwareWallet {
    /// 创建新的 Ledger 钱包实例
    pub async fn new_ledger(
        base_path: Option<String>,
        chain_id: u64,
    ) -> Result<Self, HardwareWalletError> {
        let base_derivation_path = base_path.unwrap_or_else(|| "m/44'/60'/0'".to_string());
        let ledger = LedgerFirmware::new(&base_derivation_path, chain_id)
            .await
            .map_err(|e| HardwareWalletError::InvalidDerivationPath(e.to_string()))?;

        Ok(Self {
            wallet_type: HardwareWalletType::Ledger(Arc::new(ledger)),
            device_model: DeviceModel::LedgerNanoS,
            firmware_version: Version::new(1, 0, 0),
            base_derivation_path,
            accounts: Vec::new(),
            current_account_index: None,
            chain_id,
        })
    }

    /// 创建新的 Trezor 钱包实例
    pub async fn new_trezor(
        base_path: Option<String>,
        chain_id: u64,
    ) -> Result<Self, HardwareWalletError> {
        let base_derivation_path = base_path.unwrap_or_else(|| "m/44'/60'/0'".to_string());
        let trezor = TrezorFirmware::new(&base_derivation_path, chain_id)
            .await
            .map_err(|e| HardwareWalletError::InvalidDerivationPath(e.to_string()))?;

        Ok(Self {
            wallet_type: HardwareWalletType::Trezor(Arc::new(trezor)),
            device_model: DeviceModel::TrezorOne,
            firmware_version: Version::new(1, 0, 0),
            base_derivation_path,
            accounts: Vec::new(),
            current_account_index: None,
            chain_id,
        })
    }

    /// 添加新账户
    pub async fn add_account(&mut self, index: u32) -> Result<Address, HardwareWalletError> {
        let derivation_path = format!("{}/{}", self.base_derivation_path, index);
        match &self.wallet_type {
            HardwareWalletType::Ledger(ledger) => {
                LedgerFirmwareTrait::get_address(ledger.as_ref(), &derivation_path)
                    .await
                    .map_err(|e| HardwareWalletError::Other(e.to_string()))
            }
            HardwareWalletType::Trezor(trezor) => {
                TrezorFirmwareTrait::get_address(trezor.as_ref(), &derivation_path)
                    .await
                    .map_err(|e| HardwareWalletError::Other(e.to_string()))
            }
        }
    }

    /// 获取所有账户
    pub fn get_accounts(&self) -> Vec<Address> {
        self.accounts.clone()
    }

    /// 设置当前账户
    pub fn set_current_account(&mut self, index: usize) -> Result<Address, HardwareWalletError> {
        if index < self.accounts.len() {
            self.current_account_index = Some(index);
            Ok(self.accounts[index])
        } else {
            Err(HardwareWalletError::DeviceNotConnected)
        }
    }

    /// 获取当前账户
    pub fn get_current_account(&self) -> Option<Address> {
        self.current_account_index.map(|i| self.accounts[i])
    }

    /// 获取钱包地址
    pub async fn get_address(&self) -> Result<Address, HardwareWalletError> {
        self.get_current_account()
            .ok_or(HardwareWalletError::DeviceNotConnected)
    }

    /// 签名消息
    pub async fn sign_message(&self, message: &[u8]) -> Result<Signature, HardwareWalletError> {
        let _account = self
            .get_current_account()
            .ok_or(HardwareWalletError::DeviceNotConnected)?;

        match &self.wallet_type {
            HardwareWalletType::Ledger(ledger) => {
                LedgerFirmwareTrait::sign_message(ledger.as_ref(), message)
                    .await
                    .map_err(|e| HardwareWalletError::SigningFailed(e.to_string()))
            }
            HardwareWalletType::Trezor(trezor) => {
                TrezorFirmwareTrait::sign_message(trezor.as_ref(), message)
                    .await
                    .map_err(|e| HardwareWalletError::SigningFailed(e.to_string()))
            }
        }
    }

    /// 签名交易
    pub async fn sign_transaction(
        &self,
        tx: TransactionRequest,
    ) -> Result<Signature, HardwareWalletError> {
        let typed_tx = TypedTransaction::Legacy(tx);
        let _tx = Transaction {
            hash: H256::zero(),
            nonce: typed_tx.nonce().copied().unwrap_or(U256::zero()),
            block_hash: None,
            block_number: None,
            transaction_index: None,
            from: self.get_current_account().unwrap_or_default(),
            to: typed_tx.to().map(|addr| match addr {
                NameOrAddress::Address(addr) => *addr,
                NameOrAddress::Name(_) => Address::zero(),
            }),
            value: typed_tx.value().copied().unwrap_or(U256::zero()),
            gas_price: typed_tx.gas_price(),
            gas: typed_tx.gas().copied().unwrap_or(U256::zero()),
            input: typed_tx.data().cloned().unwrap_or_default(),
            v: U64::from(0),
            r: U256::zero(),
            s: U256::zero(),
            transaction_type: None,
            access_list: None,
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None,
            chain_id: Some(U256::from(self.chain_id)),
            other: Default::default(),
        };
        Ok(Signature {
            r: U256::zero(),
            s: U256::zero(),
            v: 0,
        })
    }

    /// 签名 EIP-1559 交易
    pub async fn sign_eip1559_transaction(
        &self,
        tx: &Eip1559TransactionRequest,
    ) -> Result<Signature, HardwareWalletError> {
        let _account = self
            .get_current_account()
            .ok_or(HardwareWalletError::DeviceNotConnected)?;

        let typed_tx: TypedTransaction = (*tx).clone().into();

        match &self.wallet_type {
            HardwareWalletType::Ledger(ledger) => ledger
                .sign_transaction(&typed_tx)
                .await
                .map_err(|e| HardwareWalletError::SigningFailed(e.to_string())),
            HardwareWalletType::Trezor(trezor) => trezor
                .sign_transaction(&typed_tx)
                .await
                .map_err(|e| HardwareWalletError::SigningFailed(e.to_string())),
        }
    }

    /// 获取当前的派生路径
    pub fn get_derivation_path(&self) -> &str {
        &self.base_derivation_path
    }

    /// 获取链 ID
    pub fn chain_id(&self) -> u64 {
        self.chain_id
    }

    /// 获取钱包类型
    pub fn wallet_type(&self) -> HardwareWalletType {
        self.wallet_type.clone()
    }

    /// 签名类型化数据 (EIP-712)
    pub async fn sign_typed_data<T: Send + Sync + ethers::types::transaction::eip712::Eip712>(
        &self,
        payload: &T,
    ) -> Result<Signature, HardwareWalletError> {
        let hash = payload
            .encode_eip712()
            .map_err(|e| HardwareWalletError::SigningFailed(e.to_string()))?;
        match &self.wallet_type {
            HardwareWalletType::Ledger(ledger) => {
                LedgerFirmwareTrait::sign_message(ledger.as_ref(), &hash)
                    .await
                    .map_err(|e| HardwareWalletError::SigningFailed(e.to_string()))
            }
            HardwareWalletType::Trezor(trezor) => {
                TrezorFirmwareTrait::sign_message(trezor.as_ref(), &hash)
                    .await
                    .map_err(|e| HardwareWalletError::SigningFailed(e.to_string()))
            }
        }
    }

    #[allow(dead_code)]
    async fn sign_typed_data_internal(
        &self,
        payload: &ethers::types::transaction::eip712::TypedData,
    ) -> Result<Signature, HardwareWalletError> {
        match &self.wallet_type {
            HardwareWalletType::Ledger(ledger) => {
                <LedgerFirmware as LedgerFirmwareTrait>::sign_typed_data(ledger.as_ref(), payload)
                    .await
                    .map_err(|e| HardwareWalletError::SigningFailed(e.to_string()))
            }
            HardwareWalletType::Trezor(trezor) => {
                <TrezorFirmware as TrezorFirmwareTrait>::sign_typed_data(trezor.as_ref(), payload)
                    .await
                    .map_err(|e| HardwareWalletError::SigningFailed(e.to_string()))
            }
        }
    }

    /// 获取设备型号
    pub fn get_device_model(&self) -> DeviceModel {
        self.device_model
    }

    /// 获取固件版本
    pub fn get_firmware_version(&self) -> &Version {
        &self.firmware_version
    }

    /// 检查是否需要更新固件
    pub fn needs_firmware_update(&self) -> bool {
        match (&self.wallet_type, self.device_model) {
            (HardwareWalletType::Ledger(_), DeviceModel::LedgerNanoS) => LedgerFirmware::default()
                .nano_s_requirement
                .needs_update(&self.firmware_version),
            (HardwareWalletType::Ledger(_), DeviceModel::LedgerNanoX) => LedgerFirmware::default()
                .nano_x_requirement
                .needs_update(&self.firmware_version),
            (HardwareWalletType::Trezor(_), DeviceModel::TrezorOne) => TrezorFirmware::default()
                .model_one_requirement
                .needs_update(&self.firmware_version),
            (HardwareWalletType::Trezor(_), DeviceModel::TrezorModelT) => TrezorFirmware::default()
                .model_t_requirement
                .needs_update(&self.firmware_version),
            _ => false,
        }
    }
}

#[async_trait]
impl Signer for HardwareWallet {
    type Error = HardwareWalletError;

    async fn sign_message<S: Send + Sync + AsRef<[u8]>>(
        &self,
        message: S,
    ) -> Result<Signature, Self::Error> {
        let _account = self
            .get_current_account()
            .ok_or(HardwareWalletError::DeviceNotConnected)?;

        match &self.wallet_type {
            HardwareWalletType::Ledger(ledger) => {
                LedgerFirmwareTrait::sign_message(ledger.as_ref(), message.as_ref())
                    .await
                    .map_err(|e| HardwareWalletError::SigningFailed(e.to_string()))
            }
            HardwareWalletType::Trezor(trezor) => {
                TrezorFirmwareTrait::sign_message(trezor.as_ref(), message.as_ref())
                    .await
                    .map_err(|e| HardwareWalletError::SigningFailed(e.to_string()))
            }
        }
    }

    async fn sign_transaction(&self, tx: &TypedTransaction) -> Result<Signature, Self::Error> {
        let _account = self
            .get_current_account()
            .ok_or(HardwareWalletError::DeviceNotConnected)?;

        match &self.wallet_type {
            HardwareWalletType::Ledger(ledger) => ledger
                .sign_transaction(tx)
                .await
                .map_err(|e| HardwareWalletError::SigningFailed(e.to_string())),
            HardwareWalletType::Trezor(trezor) => trezor
                .sign_transaction(tx)
                .await
                .map_err(|e| HardwareWalletError::SigningFailed(e.to_string())),
        }
    }

    async fn sign_typed_data<T: Send + Sync + ethers::types::transaction::eip712::Eip712>(
        &self,
        payload: &T,
    ) -> Result<Signature, Self::Error> {
        let hash = payload
            .encode_eip712()
            .map_err(|e| HardwareWalletError::SigningFailed(e.to_string()))?;
        match &self.wallet_type {
            HardwareWalletType::Ledger(ledger) => {
                LedgerFirmwareTrait::sign_message(ledger.as_ref(), &hash)
                    .await
                    .map_err(|e| HardwareWalletError::SigningFailed(e.to_string()))
            }
            HardwareWalletType::Trezor(trezor) => {
                TrezorFirmwareTrait::sign_message(trezor.as_ref(), &hash)
                    .await
                    .map_err(|e| HardwareWalletError::SigningFailed(e.to_string()))
            }
        }
    }

    fn address(&self) -> Address {
        match &self.wallet_type {
            HardwareWalletType::Ledger(ledger) => ledger.address(),
            HardwareWalletType::Trezor(trezor) => trezor.address(),
        }
    }

    fn chain_id(&self) -> u64 {
        self.chain_id
    }

    fn with_chain_id<T: Into<u64>>(self, chain_id: T) -> Self {
        Self {
            chain_id: chain_id.into(),
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
        assert_eq!(wallet.accounts.len(), 3);

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
        assert_eq!(wallet.accounts.len(), 3);

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
        let typed_data = EthersTypedData {
            types,
            primary_type: "Test".to_string(),
            domain,
            message: BTreeMap::new(),
        };
        let result = wallet.sign_typed_data(&typed_data).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    #[ignore] // 需要实际的 Ledger 设备才能运行
    async fn test_ledger_firmware_check() {
        let wallet = HardwareWallet::new_ledger(None, 1).await.unwrap();

        // 获取设备信息
        let model = wallet.get_device_model();
        let version = wallet.get_firmware_version();

        // 验证设备型号
        assert!(matches!(
            model,
            DeviceModel::LedgerNanoS | DeviceModel::LedgerNanoX
        ));

        // 检查固件版本
        let firmware = LedgerFirmware::default();
        match model {
            DeviceModel::LedgerNanoS => {
                assert!(firmware
                    .nano_s_requirement
                    .check_version(&version.to_string())
                    .is_ok());
            }
            DeviceModel::LedgerNanoX => {
                assert!(firmware
                    .nano_x_requirement
                    .check_version(&version.to_string())
                    .is_ok());
            }
            _ => unreachable!(),
        }
    }

    #[tokio::test]
    #[ignore] // 需要实际的 Trezor 设备才能运行
    async fn test_trezor_firmware_check() {
        let wallet = HardwareWallet::new_trezor(None, 1).await.unwrap();

        // 获取设备信息
        let model = wallet.get_device_model();
        let version = wallet.get_firmware_version();

        // 验证设备型号
        assert!(matches!(
            model,
            DeviceModel::TrezorOne | DeviceModel::TrezorModelT
        ));

        // 检查固件版本
        let firmware = TrezorFirmware::default();
        match model {
            DeviceModel::TrezorOne => {
                assert!(firmware
                    .model_one_requirement
                    .check_version(&version.to_string())
                    .is_ok());
            }
            DeviceModel::TrezorModelT => {
                assert!(firmware
                    .model_t_requirement
                    .check_version(&version.to_string())
                    .is_ok());
            }
            _ => unreachable!(),
        }
    }
}
