use crate::{
    account::Address as AccountAddress,
    api::VmExt,
    transaction::{Transaction, TransactionType},
    types::{Hash, U256},
};
use ethers::types::{H160, H256};
use fair_vm_core::types::Transaction as CoreTransaction;
use jsonrpc_core::{Error, Result};
use jsonrpc_derive::rpc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockResponse {
    pub number: u64,
    pub hash: String,
    pub parent_hash: String,
    pub timestamp: u64,
    pub transactions: Vec<TransactionResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub hash: String,
    pub from: String,
    pub to: Option<String>,
    pub value: String,
    pub data: String,
    pub nonce: u64,
    pub gas_price: String,
    pub gas_limit: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionRequest {
    pub from: String,
    pub to: Option<String>,
    pub value: String,
    pub data: String,
    pub nonce: Option<u64>,
    pub gas_price: Option<String>,
    pub gas_limit: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountResponse {
    pub address: String,
    pub balance: String,
    pub nonce: u64,
    pub code: String,
}

pub struct ChainHandlers {
    vm: Arc<RwLock<dyn VmExt>>,
}

impl ChainHandlers {
    pub fn new(vm: Arc<RwLock<dyn VmExt>>) -> Self {
        Self { vm }
    }

    fn parse_address(&self, address: String) -> Result<AccountAddress> {
        let address_bytes =
            hex::decode(&address).map_err(|_| Error::invalid_params("Invalid address"))?;
        Ok(AccountAddress::from(H160::from_slice(&address_bytes)))
    }
}

#[rpc]
pub trait ChainApi {
    #[rpc(name = "chain_getBlockByNumber")]
    fn get_block_by_number(&self, number: u64) -> Result<Option<BlockResponse>>;

    #[rpc(name = "chain_getBlockByHash")]
    fn get_block_by_hash(&self, hash: String) -> Result<Option<BlockResponse>>;

    #[rpc(name = "chain_getTransactionByHash")]
    fn get_transaction_by_hash(&self, hash: String) -> Result<Option<TransactionResponse>>;

    #[rpc(name = "chain_sendTransaction")]
    fn send_transaction(&self, transaction: TransactionRequest) -> Result<String>;

    #[rpc(name = "chain_getBalance")]
    fn get_balance(&self, address: String) -> Result<String>;

    #[rpc(name = "chain_getAccount")]
    fn get_account(&self, address: String) -> Result<AccountResponse>;
}

impl ChainApi for ChainHandlers {
    fn get_block_by_number(&self, _number: u64) -> Result<Option<BlockResponse>> {
        let vm = self.vm.clone();
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let result: Result<Option<BlockResponse>> = runtime.block_on(async {
            let _vm = vm.read().await;
            // TODO: 实现获取区块的逻辑
            Ok(None)
        });
        result.map_err(|_e| Error::internal_error())
    }

    fn get_block_by_hash(&self, _hash: String) -> Result<Option<BlockResponse>> {
        let vm = self.vm.clone();
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let result: Result<Option<BlockResponse>> = runtime.block_on(async {
            let _vm = vm.read().await;
            // TODO: 实现获取区块的逻辑
            Ok(None)
        });
        result.map_err(|_e| Error::internal_error())
    }

    fn get_transaction_by_hash(&self, _hash: String) -> Result<Option<TransactionResponse>> {
        let vm = self.vm.clone();
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let result: Result<Option<TransactionResponse>> = runtime.block_on(async {
            let _vm = vm.read().await;
            // TODO: 实现获取交易的逻辑
            Ok(None)
        });
        result.map_err(|_e| Error::internal_error())
    }

    fn send_transaction(&self, transaction: TransactionRequest) -> Result<String> {
        let vm = self.vm.clone();
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let result: Result<String> = runtime.block_on(async {
            let vm = vm.write().await;

            let from = self.parse_address(transaction.from)?;
            let to = if let Some(to_addr) = transaction.to {
                Some(self.parse_address(to_addr)?)
            } else {
                None
            };

            let value = U256::from_str_radix(&transaction.value, 16)
                .map_err(|_| Error::invalid_params("Invalid value"))?;

            let data = hex::decode(&transaction.data)
                .map_err(|_| Error::invalid_params("Invalid data"))?;

            let gas_price = match transaction.gas_price {
                Some(price) => U256::from_str_radix(&price, 16)
                    .map_err(|_| Error::invalid_params("Invalid gas price"))?,
                None => U256::from(1),
            };

            let tx = Transaction {
                hash: Hash::from(H256::from([0; 32])),
                from,
                to,
                value,
                nonce: transaction.nonce.unwrap_or(0),
                gas_limit: transaction.gas_limit.unwrap_or(21000),
                gas_price: Some(gas_price),
                data,
                signature: Vec::new(),
                transaction_type: TransactionType::Legacy,
                chain_id: 1,
                max_fee_per_gas: Some(gas_price * U256::from(2)),
                max_priority_fee_per_gas: Some(gas_price),
            };

            let state = vm.get_state().await;
            let state_guard = state.read().await;
            let core_tx = convert_transaction(&tx);
            let result = vm
                .execute_transaction(&core_tx, &*state_guard)
                .await
                .map_err(|e| {
                    let mut err = Error::internal_error();
                    err.data = Some(serde_json::Value::String(e.to_string()));
                    err
                })?;
            Ok(hex::encode(result.return_data))
        });
        result
    }

    fn get_balance(&self, address: String) -> Result<String> {
        let vm = self.vm.clone();
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let result: Result<String> = runtime.block_on(async {
            let vm = vm.read().await;
            let address = self.parse_address(address)?;
            let state = vm.get_state().await;
            let state_guard = state.read().await;
            let balance = state_guard.get_balance(&address).await;
            Ok(format!("0x{:x}", balance))
        });
        result
    }

    fn get_account(&self, address: String) -> Result<AccountResponse> {
        let vm = self.vm.clone();
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let result: Result<AccountResponse> = runtime.block_on(async {
            let vm = vm.read().await;
            let address = self.parse_address(address)?;
            let state = vm.get_state().await;
            let state_guard = state.read().await;
            let account = state_guard
                .get_account(&address)
                .await
                .ok_or_else(|| Error::invalid_params("Account not found"))?;

            Ok(AccountResponse {
                address: format!("0x{}", hex::encode(account.address.0)),
                balance: format!("0x{:x}", account.balance),
                nonce: account.nonce,
                code: "0x".to_string(),
            })
        });
        result
    }
}

fn convert_transaction(tx: &Transaction) -> CoreTransaction {
    CoreTransaction {
        hash: fair_vm_core::Hash::from_bytes(tx.hash.0),
        from: fair_vm_core::Address::from_bytes(tx.from.0),
        to: tx
            .to
            .as_ref()
            .map(|addr| fair_vm_core::Address::from_bytes(addr.0)),
        value: tx.value,
        nonce: tx.nonce,
        gas_limit: tx.gas_limit,
        gas_price: tx.gas_price.unwrap_or_default(),
        data: tx.data.clone(),
    }
}
