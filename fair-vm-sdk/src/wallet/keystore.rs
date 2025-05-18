use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};
use rand::{rngs::OsRng, RngCore};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use thiserror::Error;

use crate::wallet::WalletError;

const SALT_LENGTH: usize = 32;
const NONCE_LENGTH: usize = 12;
const MAC_LENGTH: usize = 16;

/// 密钥库错误
#[derive(Debug, Error)]
pub enum KeystoreError {
    #[error("加密错误: {0}")]
    EncryptionError(String),

    #[error("解密错误: {0}")]
    DecryptionError(String),

    #[error("IO错误: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON错误: {0}")]
    JsonError(#[from] serde_json::Error),
}

/// 密钥库
#[derive(Debug, Serialize, Deserialize)]
pub struct KeyStore {
    /// 加密后的私钥
    encrypted_key: Vec<u8>,
    /// 盐值
    salt: Vec<u8>,
    /// 随机数
    nonce: Vec<u8>,
    /// MAC
    mac: Vec<u8>,
}

impl KeyStore {
    /// 创建新的密钥库
    pub fn new(private_key: &[u8], password: &str) -> Result<Self, WalletError> {
        let mut salt = vec![0u8; SALT_LENGTH];
        OsRng.fill_bytes(&mut salt);

        let mut nonce = vec![0u8; NONCE_LENGTH];
        OsRng.fill_bytes(&mut nonce);

        // 使用 Argon2id 派生密钥
        let salt_string =
            SaltString::encode_b64(&salt).map_err(|e| WalletError::StorageError(e.to_string()))?;
        let argon2 = Argon2::default();
        let key = argon2
            .hash_password(password.as_bytes(), &salt_string)
            .map_err(|e| WalletError::StorageError(e.to_string()))?
            .hash
            .ok_or_else(|| WalletError::StorageError("Failed to derive key".to_string()))?
            .as_bytes()
            .to_vec();

        // 使用 AES-256-GCM 加密
        let cipher = Aes256Gcm::new_from_slice(&key)
            .map_err(|e| WalletError::StorageError(e.to_string()))?;
        let nonce = Nonce::from_slice(&nonce);
        let encrypted_key = cipher
            .encrypt(nonce, private_key)
            .map_err(|e| WalletError::StorageError(e.to_string()))?;

        // 计算 MAC
        let mut mac = vec![0u8; MAC_LENGTH];
        mac.copy_from_slice(&encrypted_key[encrypted_key.len() - MAC_LENGTH..]);

        Ok(Self {
            encrypted_key,
            salt,
            nonce: nonce.to_vec(),
            mac,
        })
    }

    /// 解密私钥
    pub fn decrypt(&self, password: &str) -> Result<Vec<u8>, WalletError> {
        // 使用 Argon2id 派生密钥
        let salt_string = SaltString::encode_b64(&self.salt)
            .map_err(|e| WalletError::StorageError(e.to_string()))?;
        let argon2 = Argon2::default();
        let key = argon2
            .hash_password(password.as_bytes(), &salt_string)
            .map_err(|e| WalletError::StorageError(e.to_string()))?
            .hash
            .ok_or_else(|| WalletError::StorageError("Failed to derive key".to_string()))?
            .as_bytes()
            .to_vec();

        // 使用 AES-256-GCM 解密
        let cipher = Aes256Gcm::new_from_slice(&key)
            .map_err(|e| WalletError::StorageError(e.to_string()))?;
        let nonce = Nonce::from_slice(&self.nonce);

        cipher
            .decrypt(nonce, self.encrypted_key.as_ref())
            .map_err(|e| WalletError::StorageError(e.to_string()))
    }

    /// 保存到文件
    pub fn save_to_file(&self, path: impl AsRef<Path>) -> Result<(), WalletError> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| WalletError::StorageError(e.to_string()))?;
        fs::write(path, json).map_err(|e| WalletError::StorageError(e.to_string()))
    }

    /// 从文件加载
    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self, WalletError> {
        let json =
            fs::read_to_string(path).map_err(|e| WalletError::StorageError(e.to_string()))?;
        serde_json::from_str(&json).map_err(|e| WalletError::StorageError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_keystore() {
        let private_key = b"test private key";
        let password = "test password";

        // 创建密钥库
        let keystore = KeyStore::new(private_key, password).unwrap();

        // 解密私钥
        let decrypted = keystore.decrypt(password).unwrap();
        assert_eq!(decrypted, private_key);

        // 使用错误密码解密
        assert!(keystore.decrypt("wrong password").is_err());
    }

    #[test]
    fn test_save_and_load() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("keystore.json");

        let private_key = b"test private key";
        let password = "test password";

        // 创建并保存密钥库
        let keystore = KeyStore::new(private_key, password).unwrap();
        keystore.save_to_file(&file_path).unwrap();

        // 加载密钥库并验证
        let loaded = KeyStore::load_from_file(&file_path).unwrap();
        let decrypted = loaded.decrypt(password).unwrap();
        assert_eq!(decrypted, private_key);
    }
}
