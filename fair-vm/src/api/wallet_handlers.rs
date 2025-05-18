use crate::{
    account::Account,
    account::Address as AccountAddress,
    api::{LocalTransaction, VmExt},
    transaction::{Transaction, TransactionType},
    types::{Address, Hash, U256},
};
use ethers::types::{TransactionReceipt, H160, H256};
use fair_vm_core::types::Transaction as CoreTransaction;
use jsonrpc_core::{Error, Result};
use jsonrpc_derive::rpc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountResponse {
    pub address: String,
    pub balance: String,
    pub nonce: u64,
    pub code: String,
    pub has_code: bool,
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

pub struct WalletHandlers {
    vm: Arc<RwLock<dyn VmExt>>,
}

impl WalletHandlers {
    pub fn new(vm: Arc<RwLock<dyn VmExt>>) -> Self {
        Self { vm }
    }

    /// 获取账户信息
    pub async fn get_account(&self, address: &Address) -> Option<Account> {
        let vm = self.vm.read().await;
        vm.get_account(&AccountAddress::from(H160(address.0))).await
    }

    /// 获取账户交易列表
    pub async fn get_account_transactions(&self, address: &Address) -> Vec<LocalTransaction> {
        let vm = self.vm.read().await;
        vm.get_account_transactions(&AccountAddress::from(H160(address.0)))
            .await
    }

    /// 提交交易
    pub async fn submit_transaction(&self, tx: LocalTransaction) -> std::result::Result<(), Error> {
        let vm = self.vm.write().await;
        let core_tx = convert_transaction(&tx);
        let state = vm.get_state().await;
        let state_guard = state.read().await;
        let result = vm
            .execute_transaction(&core_tx, &*state_guard)
            .await
            .map_err(|e| {
                let mut err = Error::internal_error();
                err.data = Some(serde_json::Value::String(e.to_string()));
                err
            })?;
        if !result.status {
            return Err(Error::internal_error());
        }
        Ok(())
    }

    /// 获取交易收据
    pub async fn get_transaction_receipt(&self, tx_hash: &Hash) -> Option<TransactionReceipt> {
        let vm = self.vm.read().await;
        vm.get_transaction_receipt(tx_hash.as_bytes()).await
    }

    fn parse_address(&self, address: String) -> Result<AccountAddress> {
        let address_bytes =
            hex::decode(&address).map_err(|_| Error::invalid_params("Invalid address"))?;
        Ok(AccountAddress::from(H160::from_slice(&address_bytes)))
    }

    fn parse_hash(&self, hash: String) -> Result<Hash> {
        let bytes = hex::decode(hash.trim_start_matches("0x"))
            .map_err(|_| Error::invalid_params("无效的哈希格式"))?;

        if bytes.len() != 32 {
            return Err(Error::invalid_params("哈希长度必须为32字节"));
        }

        let mut h = [0u8; 32];
        h.copy_from_slice(&bytes);
        Ok(Hash::from(H256(h)))
    }
}

#[rpc]
pub trait WalletApi {
    #[rpc(name = "wallet_getAccount")]
    fn get_account(&self, address: String) -> Result<AccountResponse>;

    #[rpc(name = "wallet_getBalance")]
    fn get_balance(&self, address: String) -> Result<String>;

    #[rpc(name = "wallet_sendTransaction")]
    fn send_transaction(&self, transaction: TransactionRequest) -> Result<String>;

    #[rpc(name = "wallet_getTransactionHistory")]
    fn get_transaction_history(&self, address: String) -> Result<Vec<TransactionInfo>>;

    #[rpc(name = "wallet_getTransactionReceipt")]
    fn get_transaction_receipt(
        &self,
        tx_hash: String,
    ) -> Result<Option<TransactionReceiptResponse>>;

    #[rpc(name = "wallet_createAccount")]
    fn create_account(&self) -> Result<AccountResponse>;
}

impl WalletApi for WalletHandlers {
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
                has_code: false,
            })
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

    fn create_account(&self) -> Result<AccountResponse> {
        let vm = self.vm.clone();
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let result: Result<AccountResponse> = runtime.block_on(async {
            let vm = vm.write().await;
            let address = fair_vm_core::Address::random();
            let account = Account::new(AccountAddress::new(address.0 .0));
            let state = vm.get_state().await;
            let mut state_guard = state.write().await;
            state_guard.set_account(&account).await.map_err(|e| {
                let mut err = Error::internal_error();
                err.data = Some(serde_json::Value::String(e));
                err
            })?;
            Ok(AccountResponse {
                address: format!("0x{}", hex::encode(account.address.0)),
                balance: "0x0".to_string(),
                nonce: 0,
                code: "0x".to_string(),
                has_code: !account.code_hash.is_zero(),
            })
        });
        result
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
                hash: Hash::from([0; 32]),
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

    fn get_transaction_history(&self, address: String) -> Result<Vec<TransactionInfo>> {
        let address = self.parse_address(address)?;
        let vm = self.vm.blocking_read();
        let txs = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(vm.get_account_transactions(&address));
        Ok(txs
            .into_iter()
            .map(|tx| TransactionInfo {
                hash: format!("0x{}", hex::encode(tx.hash.0)),
                from: format!("0x{}", hex::encode(tx.from.0)),
                to: tx.to.map(|addr| format!("0x{}", hex::encode(addr.0))),
                value: tx.value.to_string(),
                nonce: tx.nonce,
                gas_price: tx.gas_price.unwrap_or_default().as_u64(),
                gas_limit: tx.gas_limit,
                data: format!("0x{}", hex::encode(tx.data)),
                transaction_type: tx.transaction_type as u8,
                max_fee_per_gas: tx.max_fee_per_gas.map(|f| f.to_string()),
                max_priority_fee_per_gas: tx.max_priority_fee_per_gas.map(|f| f.to_string()),
            })
            .collect())
    }

    fn get_transaction_receipt(
        &self,
        tx_hash: String,
    ) -> Result<Option<TransactionReceiptResponse>> {
        let hash = self.parse_hash(tx_hash)?;
        let vm = self.vm.blocking_read();
        let receipt = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(vm.get_transaction_receipt(&hash.0));
        Ok(receipt.map(|r| TransactionReceiptResponse {
            transaction_hash: format!("0x{}", hex::encode(r.transaction_hash.0)),
            block_number: r.block_number.map(|n| n.as_u64()).unwrap_or(0),
            block_hash: format!("0x{}", hex::encode(r.block_hash.unwrap_or_default().0)),
            from: format!("0x{}", hex::encode(r.from.0)),
            to: r.to.map(|addr| format!("0x{}", hex::encode(addr.0))),
            gas_used: r.gas_used.map(|g| g.as_u64()).unwrap_or(0),
            status: r.status.map(|s| s.as_u64() == 1).unwrap_or(false),
            logs: r
                .logs
                .into_iter()
                .map(|log| LogInfo {
                    address: format!("0x{}", hex::encode(log.address.0)),
                    topics: log
                        .topics
                        .into_iter()
                        .map(|topic| format!("0x{}", hex::encode(topic.0)))
                        .collect(),
                    data: format!("0x{}", hex::encode(log.data)),
                })
                .collect(),
        }))
    }
}

fn convert_transaction(tx: &Transaction) -> CoreTransaction {
    CoreTransaction {
        hash: fair_vm_core::Hash(H256::from_slice(&tx.hash.0)),
        from: fair_vm_core::Address(H160::from_slice(&tx.from.0)),
        to: tx
            .to
            .as_ref()
            .map(|addr| fair_vm_core::Address(H160::from_slice(&addr.0))),
        value: tx.value,
        nonce: tx.nonce,
        gas_limit: tx.gas_limit,
        gas_price: tx.gas_price.unwrap_or_default(),
        data: tx.data.clone(),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionInfo {
    pub hash: String,
    pub from: String,
    pub to: Option<String>,
    pub value: String,
    pub nonce: u64,
    pub gas_price: u64,
    pub gas_limit: u64,
    pub data: String,
    pub transaction_type: u8,
    pub max_fee_per_gas: Option<String>,
    pub max_priority_fee_per_gas: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionReceiptResponse {
    pub transaction_hash: String,
    pub block_number: u64,
    pub block_hash: String,
    pub from: String,
    pub to: Option<String>,
    pub gas_used: u64,
    pub status: bool,
    pub logs: Vec<LogInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogInfo {
    pub address: String,
    pub topics: Vec<String>,
    pub data: String,
}
