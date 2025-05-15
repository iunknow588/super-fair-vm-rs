use ethers::{
    prelude::*,
    signers::{coins_bip39::English, MnemonicBuilder},
};
use rand::rngs::OsRng;
use thiserror::Error;

use crate::wallet::{WalletError, WalletType};

/// 助记词错误
#[derive(Debug, Error)]
pub enum MnemonicError {
    #[error("无效的助记词: {0}")]
    InvalidMnemonic(String),

    #[error("生成助记词失败: {0}")]
    GenerationFailed(String),

    #[error("创建钱包失败: {0}")]
    WalletCreationFailed(String),
}

/// 助记词管理器
pub struct MnemonicManager {
    phrase: String,
}

impl MnemonicManager {
    /// 生成新的助记词
    pub fn generate() -> Result<Self, WalletError> {
        let mut rng = OsRng;
        let mnemonic =
            ethers::signers::coins_bip39::Mnemonic::<English>::new_with_count(&mut rng, 12)
                .map_err(|e| WalletError::MnemonicError(e.to_string()))?;

        Ok(Self {
            phrase: mnemonic.to_phrase(),
        })
    }

    /// 从助记词短语创建管理器
    pub fn from_phrase(phrase: &str) -> Result<Self, WalletError> {
        // 验证助记词
        ethers::signers::coins_bip39::Mnemonic::<English>::new_from_phrase(phrase)
            .map_err(|e| WalletError::MnemonicError(e.to_string()))?;

        Ok(Self {
            phrase: phrase.to_string(),
        })
    }

    /// 获取助记词短语
    pub fn get_phrase(&self) -> &str {
        &self.phrase
    }

    /// 创建钱包
    pub fn create_wallet(
        &self,
        path: Option<&str>,
        chain_id: u64,
    ) -> Result<WalletType, WalletError> {
        let derivation_path = path.unwrap_or("m/44'/60'/0'/0/0");
        let wallet = MnemonicBuilder::<English>::default()
            .phrase(self.phrase.as_str())
            .derivation_path(derivation_path)
            .map_err(|e| WalletError::MnemonicError(e.to_string()))?
            .build()
            .map_err(|e| WalletError::MnemonicError(e.to_string()))?
            .with_chain_id(chain_id);

        Ok(WalletType::Local(wallet))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_mnemonic() {
        let mnemonic = MnemonicManager::generate().unwrap();
        let phrase = mnemonic.get_phrase();
        assert_eq!(phrase.split_whitespace().count(), 12);
    }

    #[test]
    fn test_from_phrase() {
        let phrase = "test vote clock valid group comfort iron tobacco field duty ice flash";
        let mnemonic = MnemonicManager::from_phrase(phrase).unwrap();
        assert_eq!(mnemonic.get_phrase(), phrase);
    }

    #[test]
    fn test_create_wallet() {
        let mnemonic = MnemonicManager::generate().unwrap();
        let wallet = mnemonic.create_wallet(None, 1).unwrap();
        match wallet {
            WalletType::Local(_) => (),
            _ => unreachable!(),
        }
    }
}
