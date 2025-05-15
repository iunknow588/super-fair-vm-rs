use crate::account::Address;
use crate::transaction::Transaction;
use crate::vm::VM;
use ethers::types::{H256, U256};
use jsonrpc_core::{Error, Result};
use jsonrpc_derive::rpc;
use serde::{Deserialize, Serialize};
use serde_json;
use std::sync::Arc;
use tokio::sync::RwLock;

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
}

pub struct WalletHandlers {
    vm: Arc<RwLock<VM>>,
}

impl WalletHandlers {
    pub fn new(vm: Arc<RwLock<VM>>) -> Self {
        Self { vm }
    }

    fn parse_address(&self, address: String) -> Result<Address> {
        let bytes = hex::decode(address.trim_start_matches("0x"))
            .map_err(|_| Error::invalid_params("无效的地址格式"))?;

        if bytes.len() != 20 {
            return Err(Error::invalid_params("地址长度必须为20字节"));
        }

        let mut addr = [0u8; 20];
        addr.copy_from_slice(&bytes);
        Ok(Address(addr))
    }

    fn parse_hash(&self, hash: String) -> Result<H256> {
        let bytes = hex::decode(hash.trim_start_matches("0x"))
            .map_err(|_| Error::invalid_params("无效的哈希格式"))?;

        if bytes.len() != 32 {
            return Err(Error::invalid_params("哈希长度必须为32字节"));
        }

        let mut h = [0u8; 32];
        h.copy_from_slice(&bytes);
        Ok(H256(h))
    }
}

impl WalletApi for WalletHandlers {
    fn get_account(&self, address: String) -> Result<AccountResponse> {
        let address = self.parse_address(address)?;
        let vm = self.vm.blocking_read();

        let account = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(vm.get_account(&address))
            .ok_or_else(|| Error::invalid_params("账户不存在"))?;

        Ok(AccountResponse {
            address: format!("0x{}", hex::encode(account.address.0)),
            nonce: account.nonce,
            balance: account.balance.to_string(),
            has_code: !account.code_hash.is_zero(),
        })
    }

    fn get_balance(&self, address: String) -> Result<String> {
        let address = self.parse_address(address)?;
        let vm = self.vm.blocking_read();

        let account = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(vm.get_account(&address))
            .ok_or_else(|| Error::invalid_params("账户不存在"))?;

        Ok(account.balance.to_string())
    }

    fn send_transaction(&self, transaction: TransactionRequest) -> Result<String> {
        let from = self.parse_address(transaction.from)?;
        let to = if let Some(to_addr) = transaction.to {
            Some(self.parse_address(to_addr)?)
        } else {
            None
        };

        let value = transaction
            .value
            .parse::<u64>()
            .map_err(|_| Error::invalid_params("无效的金额"))?;

        let tx = Transaction {
            hash: H256([0; 32]), // 使用 ethers::types::H256
            from,
            to,
            value: U256::from(value),
            nonce: transaction.nonce.unwrap_or(0),
            gas_limit: transaction.gas_limit.unwrap_or(21000),
            gas_price: Some(U256::from(transaction.gas_price.unwrap_or(1))),
            data: hex::decode(
                transaction
                    .data
                    .unwrap_or_default()
                    .trim_start_matches("0x"),
            )
            .map_err(|_| Error::invalid_params("无效的数据格式"))?,
            signature: Vec::new(),
            transaction_type: crate::transaction::TransactionType::Legacy,
            chain_id: 1,
            max_fee_per_gas: Some(U256::from(transaction.gas_price.unwrap_or(1) * 2)),
            max_priority_fee_per_gas: Some(U256::from(transaction.gas_price.unwrap_or(1))),
        };

        let mut vm = self.vm.blocking_write();
        let hash = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(vm.submit_transaction(tx))
            .map_err(|e| {
                let mut err = Error::internal_error();
                err.data = Some(serde_json::Value::String(e.to_string()));
                err
            })?;

        Ok(format!("0x{}", hex::encode(hash.0)))
    }

    fn get_transaction_history(&self, address: String) -> Result<Vec<TransactionInfo>> {
        let address = self.parse_address(address)?;
        let vm = self.vm.blocking_read();

        let transactions = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(vm.get_account_transactions(&address))
            .map_err(|e| {
                let mut err = Error::internal_error();
                err.data = Some(serde_json::Value::String(e.to_string()));
                err
            })?;

        Ok(transactions
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

        let receipt = match tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(vm.get_transaction_receipt(&hash))
        {
            Ok(r) => r,
            Err(_) => return Ok(None),
        };

        Ok(Some(TransactionReceiptResponse {
            transaction_hash: format!("0x{}", hex::encode(receipt.transaction_hash.0)),
            block_number: receipt.block_number.map(|n| n.as_u64()).unwrap_or(0),
            block_hash: format!(
                "0x{}",
                hex::encode(receipt.block_hash.unwrap_or_default().0)
            ),
            from: format!("0x{}", hex::encode(receipt.from.0)),
            to: receipt.to.map(|addr| format!("0x{}", hex::encode(addr.0))),
            gas_used: receipt.gas_used.map(|g| g.as_u64()).unwrap_or(0),
            status: receipt.status.map(|s| s.as_u64() == 1).unwrap_or(false),
            logs: receipt
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

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountResponse {
    pub address: String,
    pub nonce: u64,
    pub balance: String,
    pub has_code: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionRequest {
    pub from: String,
    pub to: Option<String>,
    pub value: String,
    pub data: Option<String>,
    pub gas_limit: Option<u64>,
    pub gas_price: Option<u64>,
    pub nonce: Option<u64>,
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
