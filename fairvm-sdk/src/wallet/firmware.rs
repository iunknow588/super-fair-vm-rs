use async_trait::async_trait;
use ethers::signers::Signer;
use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::types::{Address, Signature};
use semver::Version;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// 固件版本错误
#[derive(Debug, Error)]
pub enum FirmwareError {
    #[error("版本解析错误: {0}")]
    ParseError(#[from] semver::Error),

    #[error("版本过低: 当前 {current}, 需要 {required}")]
    VersionTooLow { current: Version, required: Version },

    #[error("版本不兼容: {0}")]
    IncompatibleVersion(String),

    #[error("固件检查失败: {0}")]
    CheckFailed(String),

    #[error("设备未连接")]
    DeviceNotConnected,

    #[error("设备错误: {0}")]
    DeviceError(String),

    #[error("签名错误: {0}")]
    SigningError(String),

    #[error("派生路径错误: {0}")]
    DerivationPathError(String),

    #[error("设备型号不匹配: {0}")]
    DeviceModelMismatch(String),

    #[error("签名失败: {0}")]
    SigningFailed(String),

    #[error("不支持的固件版本: {0}")]
    UnsupportedVersion(Version),
}

/// 固件版本要求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirmwareRequirement {
    /// 最低版本要求
    pub min_version: Version,
    /// 推荐版本
    pub recommended_version: Version,
    /// 不兼容版本列表
    pub incompatible_versions: Vec<Version>,
}

impl FirmwareRequirement {
    /// 创建新的固件版本要求
    pub fn new(
        min_version: &str,
        recommended_version: &str,
        incompatible_versions: &[&str],
    ) -> Result<Self, FirmwareError> {
        Ok(Self {
            min_version: Version::parse(min_version)?,
            recommended_version: Version::parse(recommended_version)?,
            incompatible_versions: incompatible_versions
                .iter()
                .map(|v| Version::parse(v))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }

    /// 检查版本是否满足要求
    pub fn check_version(&self, version: &str) -> Result<bool, FirmwareError> {
        let version = Version::parse(version)?;

        if self.incompatible_versions.contains(&version) {
            return Err(FirmwareError::IncompatibleVersion(format!(
                "固件版本 {} 不兼容",
                version
            )));
        }

        if version < self.min_version {
            return Err(FirmwareError::IncompatibleVersion(format!(
                "固件版本 {} 低于最低要求版本 {}",
                version, self.min_version
            )));
        }

        Ok(version >= self.recommended_version)
    }

    /// 检查是否需要更新固件
    pub fn needs_update(&self, version: &Version) -> bool {
        version < &self.recommended_version
    }
}

/// Ledger固件版本管理器
#[derive(Debug, Serialize, Deserialize)]
pub struct LedgerFirmware {
    /// Nano S 的版本要求
    pub nano_s_requirement: FirmwareRequirement,
    /// Nano X 的版本要求
    pub nano_x_requirement: FirmwareRequirement,
    /// 当前设备型号
    device_model: Option<DeviceModel>,
    /// 当前固件版本
    firmware_version: Option<Version>,
    /// 派生路径
    derivation_path: String,
    /// 链 ID
    chain_id: u64,
    /// 连接状态
    pub connected: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DeviceModel {
    LedgerNanoS,
    LedgerNanoX,
}

impl LedgerFirmware {
    /// 创建新的 Ledger 固件实例
    pub async fn new(base_path: &str, chain_id: u64) -> Result<Self, FirmwareError> {
        // 验证派生路径格式
        if !base_path.starts_with("m/44'/60'") {
            return Err(FirmwareError::DerivationPathError(
                "无效的派生路径格式".to_string(),
            ));
        }

        let mut firmware = Self {
            nano_s_requirement: FirmwareRequirement::new("2.1.0", "2.2.0", &["2.0.0"])?,
            nano_x_requirement: FirmwareRequirement::new("2.0.0", "2.1.0", &[])?,
            device_model: None,
            firmware_version: None,
            derivation_path: base_path.to_string(),
            chain_id,
            connected: true,
        };

        // 尝试连接设备并获取信息
        firmware.connect_device().await?;

        Ok(firmware)
    }

    /// 连接设备并获取信息
    async fn connect_device(&mut self) -> Result<(), FirmwareError> {
        // TODO: 实现实际的设备连接逻辑
        // 这里应该使用 ledger-rs 或其他库来连接设备
        // 目前使用模拟数据
        self.device_model = Some(DeviceModel::LedgerNanoS);
        self.firmware_version = Some(Version::new(2, 1, 0));
        Ok(())
    }

    /// 检查固件版本
    pub fn check_firmware(&self) -> Result<bool, FirmwareError> {
        let version = self
            .firmware_version
            .as_ref()
            .ok_or(FirmwareError::DeviceNotConnected)?;

        let model = self.device_model.ok_or(FirmwareError::DeviceNotConnected)?;

        match model {
            DeviceModel::LedgerNanoS => self.nano_s_requirement.check_version(&version.to_string()),
            DeviceModel::LedgerNanoX => self.nano_x_requirement.check_version(&version.to_string()),
        }
    }

    /// 获取设备型号
    pub fn get_device_model(&self) -> Option<DeviceModel> {
        self.device_model
    }

    /// 获取固件版本
    pub fn get_firmware_version(&self) -> Option<&Version> {
        self.firmware_version.as_ref()
    }
}

/// Trezor固件版本管理器
#[derive(Debug, Serialize, Deserialize)]
pub struct TrezorFirmware {
    /// Model One 的版本要求
    pub model_one_requirement: FirmwareRequirement,
    /// Model T 的版本要求
    pub model_t_requirement: FirmwareRequirement,
    /// 当前设备型号
    device_model: Option<TrezorModel>,
    /// 当前固件版本
    firmware_version: Option<Version>,
    /// 派生路径
    derivation_path: String,
    /// 链 ID
    chain_id: u64,
    /// 连接状态
    pub connected: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TrezorModel {
    ModelOne,
    ModelT,
}

impl TrezorFirmware {
    /// 创建新的 Trezor 固件实例
    pub async fn new(base_path: &str, chain_id: u64) -> Result<Self, FirmwareError> {
        // 验证派生路径格式
        if !base_path.starts_with("m/44'/60'") {
            return Err(FirmwareError::DerivationPathError(
                "无效的派生路径格式".to_string(),
            ));
        }

        let mut firmware = Self {
            model_one_requirement: FirmwareRequirement::new("1.10.0", "1.11.0", &["1.9.0"])?,
            model_t_requirement: FirmwareRequirement::new("2.4.0", "2.5.0", &[])?,
            device_model: None,
            firmware_version: None,
            derivation_path: base_path.to_string(),
            chain_id,
            connected: true,
        };

        // 尝试连接设备并获取信息
        firmware.connect_device().await?;

        Ok(firmware)
    }

    /// 连接设备并获取信息
    async fn connect_device(&mut self) -> Result<(), FirmwareError> {
        // TODO: 实现实际的设备连接逻辑
        // 这里应该使用 trezor-rs 或其他库来连接设备
        // 目前使用模拟数据
        self.device_model = Some(TrezorModel::ModelOne);
        self.firmware_version = Some(Version::new(1, 10, 0));
        Ok(())
    }

    /// 检查固件版本
    pub fn check_firmware(&self) -> Result<bool, FirmwareError> {
        let version = self
            .firmware_version
            .as_ref()
            .ok_or(FirmwareError::DeviceNotConnected)?;

        let model = self.device_model.ok_or(FirmwareError::DeviceNotConnected)?;

        match model {
            TrezorModel::ModelOne => self
                .model_one_requirement
                .check_version(&version.to_string()),
            TrezorModel::ModelT => self.model_t_requirement.check_version(&version.to_string()),
        }
    }

    /// 获取设备型号
    pub fn get_device_model(&self) -> Option<TrezorModel> {
        self.device_model
    }

    /// 获取固件版本
    pub fn get_firmware_version(&self) -> Option<&Version> {
        self.firmware_version.as_ref()
    }
}

#[async_trait]
pub trait LedgerFirmwareTrait {
    async fn get_address(&self, derivation_path: &str) -> Result<Address, FirmwareError>;
    async fn sign_message(&self, message: &[u8]) -> Result<Signature, FirmwareError>;
    async fn sign_typed_data(
        &self,
        typed_data: &ethers::types::transaction::eip712::TypedData,
    ) -> Result<Signature, FirmwareError>;
}

#[async_trait]
pub trait TrezorFirmwareTrait {
    async fn get_address(&self, derivation_path: &str) -> Result<Address, FirmwareError>;
    async fn sign_message(&self, message: &[u8]) -> Result<Signature, FirmwareError>;
    async fn sign_typed_data(
        &self,
        typed_data: &ethers::types::transaction::eip712::TypedData,
    ) -> Result<Signature, FirmwareError>;
}

#[async_trait]
impl LedgerFirmwareTrait for LedgerFirmware {
    async fn get_address(&self, _derivation_path: &str) -> Result<Address, FirmwareError> {
        // TODO: 实现实际的地址获取逻辑，使用 derivation_path
        Ok(Address::zero())
    }

    async fn sign_message(&self, _message: &[u8]) -> Result<Signature, FirmwareError> {
        // TODO: 实现实际的消息签名逻辑
        Err(FirmwareError::SigningError("未实现".to_string()))
    }

    async fn sign_typed_data(
        &self,
        _typed_data: &ethers::types::transaction::eip712::TypedData,
    ) -> Result<Signature, FirmwareError> {
        // TODO: 实现实际的类型化数据签名逻辑
        Err(FirmwareError::SigningError("未实现".to_string()))
    }
}

#[async_trait]
impl TrezorFirmwareTrait for TrezorFirmware {
    async fn get_address(&self, _derivation_path: &str) -> Result<Address, FirmwareError> {
        // TODO: 实现实际的地址获取逻辑，使用 derivation_path
        Ok(Address::zero())
    }

    async fn sign_message(&self, _message: &[u8]) -> Result<Signature, FirmwareError> {
        // TODO: 实现实际的消息签名逻辑
        Err(FirmwareError::SigningError("未实现".to_string()))
    }

    async fn sign_typed_data(
        &self,
        _typed_data: &ethers::types::transaction::eip712::TypedData,
    ) -> Result<Signature, FirmwareError> {
        // TODO: 实现实际的类型化数据签名逻辑
        Err(FirmwareError::SigningError("未实现".to_string()))
    }
}

#[async_trait]
impl Signer for LedgerFirmware {
    type Error = FirmwareError;

    async fn sign_message<S: Send + Sync + AsRef<[u8]>>(
        &self,
        message: S,
    ) -> Result<Signature, Self::Error> {
        LedgerFirmwareTrait::sign_message(self, message.as_ref()).await
    }

    async fn sign_transaction(&self, _tx: &TypedTransaction) -> Result<Signature, Self::Error> {
        // TODO: 实现实际的交易签名逻辑
        Err(FirmwareError::SigningError("未实现".to_string()))
    }

    fn address(&self) -> Address {
        // TODO: 实现实际的地址获取逻辑
        Address::zero()
    }

    async fn sign_typed_data<T: ethers::types::transaction::eip712::Eip712 + Send + Sync>(
        &self,
        payload: &T,
    ) -> Result<Signature, Self::Error> {
        let hash = payload
            .encode_eip712()
            .map_err(|e| FirmwareError::SigningError(e.to_string()))?;
        let message = ethers::utils::hash_message(hash);
        LedgerFirmwareTrait::sign_message(self, message.as_bytes()).await
    }

    fn chain_id(&self) -> u64 {
        self.chain_id
    }

    fn with_chain_id<T: Into<u64>>(mut self, chain_id: T) -> Self {
        self.chain_id = chain_id.into();
        self
    }
}

#[async_trait]
impl Signer for TrezorFirmware {
    type Error = FirmwareError;

    async fn sign_message<S: Send + Sync + AsRef<[u8]>>(
        &self,
        message: S,
    ) -> Result<Signature, Self::Error> {
        TrezorFirmwareTrait::sign_message(self, message.as_ref()).await
    }

    async fn sign_transaction(&self, _tx: &TypedTransaction) -> Result<Signature, Self::Error> {
        // TODO: 实现实际的交易签名逻辑
        Err(FirmwareError::SigningError("未实现".to_string()))
    }

    fn address(&self) -> Address {
        // TODO: 实现实际的地址获取逻辑
        Address::zero()
    }

    async fn sign_typed_data<T: ethers::types::transaction::eip712::Eip712 + Send + Sync>(
        &self,
        payload: &T,
    ) -> Result<Signature, Self::Error> {
        let hash = payload
            .encode_eip712()
            .map_err(|e| FirmwareError::SigningError(e.to_string()))?;
        let message = ethers::utils::hash_message(hash);
        TrezorFirmwareTrait::sign_message(self, message.as_bytes()).await
    }

    fn chain_id(&self) -> u64 {
        self.chain_id
    }

    fn with_chain_id<T: Into<u64>>(mut self, chain_id: T) -> Self {
        self.chain_id = chain_id.into();
        self
    }
}

impl Default for LedgerFirmware {
    fn default() -> Self {
        Self {
            nano_s_requirement: FirmwareRequirement::new("2.0.0", "2.1.0", &["1.9.0"]).unwrap(),
            nano_x_requirement: FirmwareRequirement::new("2.0.0", "2.1.0", &["1.9.0"]).unwrap(),
            device_model: None,
            firmware_version: None,
            derivation_path: "m/44'/60'/0'/0/0".to_string(),
            chain_id: 1,
            connected: false,
        }
    }
}

impl Default for TrezorFirmware {
    fn default() -> Self {
        Self {
            model_one_requirement: FirmwareRequirement::new("1.9.0", "1.10.0", &["1.8.0"]).unwrap(),
            model_t_requirement: FirmwareRequirement::new("2.3.0", "2.4.0", &["2.2.0"]).unwrap(),
            device_model: None,
            firmware_version: None,
            derivation_path: "m/44'/60'/0'/0/0".to_string(),
            chain_id: 1,
            connected: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_firmware_requirement() {
        let requirement = FirmwareRequirement::new("2.0.0", "2.1.0", &["1.9.0"]).unwrap();

        // 测试不兼容版本
        let incompatible = "1.9.0";
        assert!(requirement.check_version(incompatible).is_err());

        // 测试版本过低
        let too_low = "1.8.0";
        assert!(requirement.check_version(too_low).is_err());

        // 测试满足最低要求
        let minimum = "2.0.0";
        assert!(requirement.check_version(minimum).unwrap());

        // 测试推荐版本
        let current = "2.0.5";
        assert!(requirement.check_version(current).unwrap());

        // 测试最新版本
        let latest = "2.1.0";
        assert!(!requirement.check_version(latest).unwrap());
    }

    #[test]
    fn test_ledger_firmware() {
        let firmware = LedgerFirmware::default();

        // 测试 Nano S 版本检查
        let current = "2.1.0";
        assert!(firmware.nano_s_requirement.check_version(current).is_ok());
        assert!(firmware.nano_s_requirement.check_version(current).unwrap());

        // 测试 Nano X 版本检查
        let current = "2.0.0";
        assert!(firmware.nano_x_requirement.check_version(current).is_ok());
        assert!(firmware.nano_x_requirement.check_version(current).unwrap());
    }

    #[test]
    fn test_trezor_firmware() {
        let firmware = TrezorFirmware::default();

        // 测试 Model One 版本检查
        let current = "1.10.0";
        assert!(firmware
            .model_one_requirement
            .check_version(current)
            .is_ok());
        assert!(firmware
            .model_one_requirement
            .check_version(current)
            .unwrap());

        // 测试 Model T 版本检查
        let current = "2.4.0";
        assert!(firmware.model_t_requirement.check_version(current).is_ok());
        assert!(firmware.model_t_requirement.check_version(current).unwrap());
    }
}
