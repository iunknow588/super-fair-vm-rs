use ethers::types::transaction::eip712::{
    EIP712Domain, Eip712, Eip712DomainType, TypedData as EthersTypedData,
};
use ethers::types::{Address, Signature};
use serde_json::Value;
use std::collections::BTreeMap;
use thiserror::Error;

/// 消息签名错误
#[derive(Debug, Error)]
pub enum MessageSignError {
    #[error("签名失败")]
    SigningFailed,
    #[error("恢复失败")]
    RecoveryFailed,
    #[error("验证失败")]
    VerificationFailed,
    #[error("其他错误: {0}")]
    Other(String),
}

/// 类型化数据
#[derive(Debug, Clone)]
pub struct TypedData {
    /// 域
    pub domain: EIP712Domain,
    /// 类型
    pub types: BTreeMap<String, Vec<Eip712DomainType>>,
    /// 主类型
    pub primary_type: String,
    /// 消息
    pub message: BTreeMap<String, Value>,
}

impl TypedData {
    /// 创建新的类型化数据
    pub fn new(
        domain: EIP712Domain,
        types: BTreeMap<String, Vec<Eip712DomainType>>,
        primary_type: String,
        message: BTreeMap<String, Value>,
    ) -> Self {
        Self {
            domain,
            types,
            primary_type,
            message,
        }
    }

    /// 编码数据
    pub fn encode(&self) -> Result<Vec<u8>, MessageSignError> {
        // TODO: 实现 EIP-712 编码逻辑
        Ok(Vec::new())
    }
}

impl From<TypedData> for EthersTypedData {
    fn from(data: TypedData) -> Self {
        let domain = EIP712Domain {
            name: data.domain.name,
            version: data.domain.version,
            chain_id: data.domain.chain_id,
            verifying_contract: data.domain.verifying_contract,
            salt: data.domain.salt,
        };

        let types = data
            .types
            .into_iter()
            .map(|(name, fields)| {
                (
                    name,
                    fields
                        .into_iter()
                        .map(|field| Eip712DomainType {
                            name: field.name,
                            r#type: field.r#type,
                        })
                        .collect(),
                )
            })
            .collect();

        Self {
            types,
            primary_type: data.primary_type,
            domain,
            message: data.message,
        }
    }
}

impl Eip712 for TypedData {
    type Error = MessageSignError;

    fn domain(&self) -> Result<EIP712Domain, Self::Error> {
        Ok(EIP712Domain {
            name: self.domain.name.clone(),
            version: self.domain.version.clone(),
            chain_id: self.domain.chain_id,
            verifying_contract: self.domain.verifying_contract,
            salt: self.domain.salt,
        })
    }

    fn type_hash() -> Result<[u8; 32], Self::Error> {
        // TODO: 实现类型哈希计算
        Ok([0u8; 32])
    }

    fn struct_hash(&self) -> Result<[u8; 32], Self::Error> {
        // TODO: 实现结构哈希计算
        Ok([0u8; 32])
    }
}

/// 消息签名器
#[derive(Debug, Clone)]
pub struct MessageSigner {
    /// 签名者地址
    pub address: Address,
}

impl MessageSigner {
    /// 创建新的消息签名器
    pub fn new(address: Address) -> Self {
        Self { address }
    }

    /// 签名消息
    pub fn sign_message(&self, _message: &[u8]) -> Result<Signature, MessageSignError> {
        // TODO: 实现消息签名逻辑
        Err(MessageSignError::SigningFailed)
    }

    /// 验证消息签名
    pub fn verify_message(
        &self,
        _message: &[u8],
        _signature: &Signature,
        _address: Address,
    ) -> Result<bool, MessageSignError> {
        // TODO: 实现消息验证逻辑
        Ok(false)
    }

    /// 签名类型化数据
    pub fn sign_typed_data(&self, _typed_data: &TypedData) -> Result<Signature, MessageSignError> {
        // TODO: 实现类型化数据签名逻辑
        Err(MessageSignError::SigningFailed)
    }

    /// 验证类型化数据签名
    pub fn verify_typed_data_signature(
        &self,
        _typed_data: &TypedData,
        _signature: &Signature,
        _address: Address,
    ) -> Result<bool, MessageSignError> {
        // TODO: 实现类型化数据验证逻辑
        Ok(false)
    }
}

#[derive(Debug, Clone)]
pub struct MessageTypedData {
    pub types: BTreeMap<String, Vec<ethers::types::transaction::eip712::Eip712DomainType>>,
    pub primary_type: String,
    pub domain: EIP712Domain,
    pub message: BTreeMap<String, Value>,
}

impl MessageTypedData {
    pub fn new(
        types: BTreeMap<String, Vec<ethers::types::transaction::eip712::Eip712DomainType>>,
        primary_type: String,
        domain: EIP712Domain,
        message: BTreeMap<String, Value>,
    ) -> Self {
        Self {
            types,
            primary_type,
            domain,
            message,
        }
    }
}

impl ethers::types::transaction::eip712::Eip712 for MessageTypedData {
    type Error = ethers::types::transaction::eip712::Eip712Error;

    fn domain(&self) -> Result<EIP712Domain, Self::Error> {
        Ok(self.domain.clone())
    }

    fn type_hash() -> Result<[u8; 32], Self::Error> {
        Ok([0u8; 32])
    }

    fn struct_hash(&self) -> Result<[u8; 32], Self::Error> {
        Ok([0u8; 32])
    }

    fn encode_eip712(&self) -> Result<[u8; 32], Self::Error> {
        Ok([0u8; 32])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::signers::{LocalWallet, Signer};
    use ethers::types::U256;
    use std::str::FromStr;

    #[tokio::test]
    async fn test_verify_message() {
        let wallet = LocalWallet::from_str(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )
        .unwrap();
        let message = b"Hello, FairVM!";
        let signature = wallet.sign_message(message).await.unwrap();
        let address = wallet.address();

        let signer = MessageSigner::new(address);
        assert!(signer.verify_message(message, &signature, address).unwrap());
    }

    #[tokio::test]
    async fn test_verify_typed_data() {
        let wallet = LocalWallet::from_str(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )
        .unwrap();

        let domain = EIP712Domain {
            name: Some("FairVM".to_string()),
            version: Some("1".to_string()),
            chain_id: Some(U256::from(1)),
            verifying_contract: None,
            salt: None,
        };

        let mut types = BTreeMap::new();
        types.insert(
            "Test".to_string(),
            vec![
                Eip712DomainType {
                    name: "name".to_string(),
                    r#type: "string".to_string(),
                },
                Eip712DomainType {
                    name: "wallet".to_string(),
                    r#type: "address".to_string(),
                },
            ],
        );

        let mut message = BTreeMap::new();
        message.insert("name".to_string(), serde_json::json!("Alice"));
        message.insert(
            "wallet".to_string(),
            serde_json::json!("0x0000000000000000000000000000000000000001"),
        );

        let typed_data = TypedData::new(domain, types, "Test".to_string(), message);

        let signature = wallet.sign_typed_data(&typed_data).await.unwrap();
        let address = wallet.address();

        let signer = MessageSigner::new(address);
        assert!(signer
            .verify_typed_data_signature(&typed_data, &signature, address)
            .unwrap());
    }
}
