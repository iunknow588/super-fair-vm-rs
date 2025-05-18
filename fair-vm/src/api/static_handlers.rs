use crate::account::{Account, Address};
use crate::api::VmExt;
use crate::state::State;
use crate::storage::Storage;
use async_trait::async_trait;
use ethers::types::{H160, H256, U256};
use hex;
use jsonrpc_core::{Error, Result};
use jsonrpc_derive::rpc;
use serde::{Deserialize, Serialize};
use serde_json;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageValue {
    pub value: String,
}

#[async_trait]
impl Storage for StorageValue {
    async fn get_account(&self, _address: &Address) -> Option<Account> {
        None
    }

    async fn set_account(&mut self, _account: &Account) {}

    async fn get_balance(&self, _address: &Address) -> U256 {
        U256::zero()
    }

    async fn set_balance(&mut self, _address: &Address, _balance: U256) {}

    async fn get_nonce(&self, _address: &Address) -> u64 {
        0
    }

    async fn set_nonce(&mut self, _address: &Address, _nonce: u64) {}

    async fn get_code_hash(&self, _address: &Address) -> H256 {
        H256::zero()
    }

    async fn set_code_hash(&mut self, _address: &Address, _code_hash: H256) {}

    async fn get_storage_root(&self, _address: &Address) -> H256 {
        H256::zero()
    }

    async fn set_storage_root(&mut self, _address: &Address, _storage_root: H256) {}

    async fn get_storage_value(&self, _address: &Address, _key: [u8; 32]) -> [u8; 32] {
        [0; 32]
    }

    async fn set_storage_value(&mut self, _address: &Address, _key: [u8; 32], _value: [u8; 32]) {}
}

#[rpc]
pub trait StaticApi {
    #[rpc(name = "static_getState")]
    fn get_state(&self) -> Result<StateResponse>;

    #[rpc(name = "static_getStorage")]
    fn get_storage(&self, address: String, key: String) -> Result<StorageResponse>;

    #[rpc(name = "static_getVM")]
    fn get_vm(&self) -> Result<VMResponse>;

    #[rpc(name = "static_getCode")]
    fn get_code(&self, address: String) -> Result<String>;
}

/// 静态处理器
pub struct StaticHandlers {
    vm: Arc<RwLock<dyn VmExt>>,
}

impl StaticHandlers {
    pub fn new(vm: Arc<RwLock<dyn VmExt>>) -> Self {
        Self { vm }
    }

    fn parse_address(&self, address: String) -> Result<H160> {
        let address = address.trim_start_matches("0x");
        let address = hex::decode(address).map_err(|e| {
            let mut err = Error::invalid_params("无效的地址格式");
            err.data = Some(serde_json::to_value(e.to_string()).unwrap());
            err
        })?;
        Ok(H160::from_slice(&address))
    }

    fn parse_hash(&self, hash: String) -> Result<H256> {
        let hash = hash.trim_start_matches("0x");
        let hash = hex::decode(hash).map_err(|e| {
            let mut err = Error::invalid_params("无效的哈希格式");
            err.data = Some(serde_json::to_value(e.to_string()).unwrap());
            err
        })?;
        Ok(H256::from_slice(&hash))
    }

    /// 获取状态
    pub async fn get_state(&self) -> Result<Arc<RwLock<State>>> {
        let vm = self.vm.read().await;
        Ok(vm.get_state().await)
    }

    /// 获取存储
    pub async fn get_storage(&self, address: &H160, key: &H256) -> Result<StorageValue> {
        let vm = self.vm.read().await;
        let storage = vm.get_storage_arc().await;
        let storage_guard = storage.read().await;
        let value = storage_guard
            .get_storage_value(&Address(address.0), key.0)
            .await;
        Ok(StorageValue {
            value: hex::encode(value),
        })
    }

    /// 获取虚拟机
    pub async fn get_vm(&self) -> Arc<RwLock<dyn VmExt>> {
        self.vm.clone()
    }
}

impl StaticApi for StaticHandlers {
    fn get_state(&self) -> Result<StateResponse> {
        let state = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(self.get_state())?;

        let address = Address([0; 20]);
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let state_guard = runtime.block_on(state.read());

        let balance = runtime.block_on(state_guard.get_balance(&address));
        let nonce = runtime.block_on(state_guard.get_nonce(&address));
        let code_hash = runtime.block_on(state_guard.get_code_hash(&address));

        Ok(StateResponse {
            balance: balance.to_string(),
            nonce,
            code: hex::encode(code_hash.as_bytes()),
            storage: vec![], // 这里需要实现存储的序列化
        })
    }

    fn get_storage(&self, address: String, key: String) -> Result<StorageResponse> {
        let address = self.parse_address(address)?;
        let key = self.parse_hash(key)?;

        let storage = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(self.get_storage(&address, &key))?;

        Ok(StorageResponse {
            root: storage.value,
            size: 32,
        })
    }

    fn get_vm(&self) -> Result<VMResponse> {
        Ok(VMResponse {
            version: "1.0.0".to_string(),
            chain_id: 1,
        })
    }

    fn get_code(&self, address: String) -> Result<String> {
        let vm = self.vm.clone();
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let result: Result<String> = runtime.block_on(async {
            let vm = vm.read().await;
            let address_bytes =
                hex::decode(&address).map_err(|_| Error::invalid_params("Invalid address"))?;
            let address = H160::from_slice(&address_bytes);
            let code = vm.get_code(&address).await.map_err(|e| {
                let mut err = Error::internal_error();
                err.data = Some(serde_json::Value::String(e.to_string()));
                err
            })?;
            Ok(format!("0x{}", hex::encode(code)))
        });
        result
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StateResponse {
    pub balance: String,
    pub nonce: u64,
    pub code: String,
    pub storage: Vec<(String, String)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageResponse {
    pub root: String,
    pub size: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VMResponse {
    pub version: String,
    pub chain_id: u64,
}
