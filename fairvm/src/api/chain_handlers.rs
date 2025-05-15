use crate::account::Address;
use crate::transaction::Transaction;
use crate::vm::VM;
use ethers::types::{H256, U256};
use jsonrpc_core::{Error, Result};
use jsonrpc_derive::rpc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

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

    #[rpc(name = "chain_getNonce")]
    fn get_nonce(&self, address: String) -> Result<u64>;
}

pub struct ChainHandlers {
    vm: Arc<RwLock<VM>>,
}

impl ChainHandlers {
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

    #[allow(dead_code)]
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

impl ChainApi for ChainHandlers {
    fn get_block_by_number(&self, _number: u64) -> Result<Option<BlockResponse>> {
        // TODO: 实现按区块号查询
        Ok(None)
    }

    fn get_block_by_hash(&self, _hash: String) -> Result<Option<BlockResponse>> {
        // TODO: 实现按哈希查询
        Ok(None)
    }

    fn get_transaction_by_hash(&self, _hash: String) -> Result<Option<TransactionResponse>> {
        // TODO: 实现交易查询
        Ok(None)
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
            hash: H256([0; 32]), // 临时哈希，后续会更新
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
            signature: Vec::new(), // 需要客户端签名
            transaction_type: crate::transaction::TransactionType::Legacy,
            chain_id: 1, // 默认为主网链ID
            max_fee_per_gas: Some(U256::from(transaction.gas_price.unwrap_or(1) * 2)), // 默认为gas_price的两倍
            max_priority_fee_per_gas: Some(U256::from(transaction.gas_price.unwrap_or(1))), // 默认为gas_price
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

    fn get_balance(&self, address: String) -> Result<String> {
        let address = self.parse_address(address)?;
        let vm = self.vm.blocking_read();

        let account = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(vm.get_account(&address))
            .ok_or_else(|| Error::invalid_params("账户不存在"))?;

        Ok(account.balance.to_string())
    }

    fn get_nonce(&self, address: String) -> Result<u64> {
        let address = self.parse_address(address)?;
        let vm = self.vm.blocking_read();

        let account = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(vm.get_account(&address))
            .ok_or_else(|| Error::invalid_params("账户不存在"))?;

        Ok(account.nonce)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockResponse {
    pub hash: String,
    pub parent_hash: String,
    pub number: u64,
    pub timestamp: u64,
    pub transactions: Vec<String>,
    pub gas_used: u64,
    pub gas_limit: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub hash: String,
    pub from: String,
    pub to: Option<String>,
    pub value: String,
    pub nonce: u64,
    pub gas_price: u64,
    pub gas_limit: u64,
    pub data: String,
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
