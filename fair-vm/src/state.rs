use crate::account::Account;
use crate::account::Address;
use crate::evm::EvmContext;
use crate::storage::{MemoryStorage, Storage};
use crate::transaction::Transaction;
use async_trait::async_trait;
use ethers::types::{TransactionReceipt, H160, H256, U256};
use fair_vm_core::types::{Address as CoreAddress, Hash as CoreHash};
use fair_vm_core::vm::State as StateTrait;
use std::collections::HashMap;
use std::error::Error as StdError;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 状态类型
#[derive(Debug, Clone)]
pub struct State {
    /// 存储
    storage: Arc<RwLock<Box<dyn Storage + Send + Sync>>>,
    context: EvmContext,
    /// 账户交易列表
    account_transactions: Arc<RwLock<HashMap<Address, Vec<Transaction>>>>,
    /// 交易收据
    transaction_receipts: Arc<RwLock<HashMap<H256, TransactionReceipt>>>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            storage: Arc::new(RwLock::new(
                Box::new(MemoryStorage::default()) as Box<dyn Storage + Send + Sync>
            )),
            context: EvmContext::default(),
            account_transactions: Arc::new(RwLock::new(HashMap::new())),
            transaction_receipts: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl Storage for State {
    async fn get_account(&self, address: &Address) -> Option<Account> {
        let storage = self.storage.read().await;
        storage.get_account(address).await
    }

    async fn set_account(&mut self, account: &Account) {
        let mut storage = self.storage.write().await;
        storage.set_account(account).await;
    }

    async fn get_balance(&self, address: &Address) -> U256 {
        let storage = self.storage.read().await;
        storage.get_balance(address).await
    }

    async fn set_balance(&mut self, address: &Address, balance: U256) {
        let mut storage = self.storage.write().await;
        storage.set_balance(address, balance).await;
    }

    async fn get_nonce(&self, address: &Address) -> u64 {
        let storage = self.storage.read().await;
        storage.get_nonce(address).await
    }

    async fn set_nonce(&mut self, address: &Address, nonce: u64) {
        let mut storage = self.storage.write().await;
        storage.set_nonce(address, nonce).await;
    }

    async fn get_code_hash(&self, address: &Address) -> H256 {
        let storage = self.storage.read().await;
        storage.get_code_hash(address).await
    }

    async fn set_code_hash(&mut self, address: &Address, code_hash: H256) {
        let mut storage = self.storage.write().await;
        storage.set_code_hash(address, code_hash).await;
    }

    async fn get_storage_root(&self, address: &Address) -> H256 {
        let storage = self.storage.read().await;
        storage.get_storage_root(address).await
    }

    async fn set_storage_root(&mut self, address: &Address, storage_root: H256) {
        let mut storage = self.storage.write().await;
        storage.set_storage_root(address, storage_root).await;
    }

    async fn set_storage_value(&mut self, address: &Address, key: [u8; 32], value: [u8; 32]) {
        let mut storage = self.storage.write().await;
        storage.set_storage_value(address, key, value).await;
    }

    async fn get_storage_value(&self, address: &Address, key: [u8; 32]) -> [u8; 32] {
        let storage = self.storage.read().await;
        storage.get_storage_value(address, key).await
    }
}

impl State {
    /// 创建新状态实例
    pub fn new(storage: Arc<RwLock<Box<dyn Storage + Send + Sync>>>, context: EvmContext) -> Self {
        Self {
            storage,
            context,
            account_transactions: Arc::new(RwLock::new(HashMap::new())),
            transaction_receipts: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 获取账户信息
    pub async fn get_account(&self, address: &Address) -> Option<Account> {
        let storage = self.storage.read().await;
        storage.get_account(address).await
    }

    /// 设置账户信息
    pub async fn set_account(&mut self, account: &Account) -> Result<(), String> {
        let mut storage = self.storage.write().await;
        storage.set_account(account).await;
        Ok(())
    }

    /// 获取账户余额
    pub async fn get_balance(&self, address: &Address) -> U256 {
        let storage = self.storage.read().await;
        storage.get_balance(address).await
    }

    /// 设置账户余额
    pub async fn set_balance(&self, address: &Address, balance: U256) -> Result<(), String> {
        let mut storage = self.storage.write().await;
        storage.set_balance(address, balance).await;
        Ok(())
    }

    /// 获取账户 nonce
    pub async fn get_nonce(&self, address: &Address) -> u64 {
        let storage = self.storage.read().await;
        storage.get_nonce(address).await
    }

    /// 设置账户 nonce
    pub async fn set_nonce(&self, address: &Address, nonce: u64) -> Result<(), String> {
        let mut storage = self.storage.write().await;
        storage.set_nonce(address, nonce).await;
        Ok(())
    }

    /// 获取账户代码哈希
    pub async fn get_code_hash(&self, address: &Address) -> H256 {
        let storage = self.storage.read().await;
        storage.get_code_hash(address).await
    }

    /// 设置账户代码哈希
    pub async fn set_code_hash(&self, address: &Address, code_hash: H256) -> Result<(), String> {
        let mut storage = self.storage.write().await;
        storage.set_code_hash(address, code_hash).await;
        Ok(())
    }

    /// 获取账户存储根
    pub async fn get_storage_root(&self, address: &Address) -> H256 {
        let storage = self.storage.read().await;
        storage.get_storage_root(address).await
    }

    /// 设置账户存储根
    pub async fn set_storage_root(
        &self,
        address: &Address,
        storage_root: H256,
    ) -> Result<(), String> {
        let mut storage = self.storage.write().await;
        storage.set_storage_root(address, storage_root).await;
        Ok(())
    }

    /// 获取存储实例
    pub fn storage(&self) -> &Arc<RwLock<Box<dyn Storage + Send + Sync>>> {
        &self.storage
    }

    /// 获取状态根
    pub fn get_state_root(&self) -> H256 {
        H256::zero()
    }

    pub fn context(&self) -> &EvmContext {
        &self.context
    }

    /// 获取账户交易列表
    pub async fn get_account_transactions(&self, address: &Address) -> Vec<Transaction> {
        let transactions = self.account_transactions.read().await;
        transactions.get(address).cloned().unwrap_or_default()
    }

    /// 添加账户交易
    pub async fn add_account_transaction(&self, address: &Address, transaction: Transaction) {
        let mut transactions = self.account_transactions.write().await;
        let account_transactions = transactions.entry(*address).or_insert_with(Vec::new);
        account_transactions.push(transaction);
    }

    /// 获取交易收据
    pub async fn get_transaction_receipt(&self, tx_hash: &[u8]) -> Option<TransactionReceipt> {
        let mut hash = [0u8; 32];
        hash.copy_from_slice(tx_hash);
        let receipts = self.transaction_receipts.read().await;
        receipts.get(&H256::from_slice(&hash)).cloned()
    }

    /// 添加交易收据
    pub async fn add_transaction_receipt(&self, tx_hash: H256, receipt: TransactionReceipt) {
        let mut receipts = self.transaction_receipts.write().await;
        receipts.insert(tx_hash, receipt);
    }
}

#[async_trait]
impl StateTrait for State {
    async fn get_balance(&self, address: &CoreAddress) -> Result<U256, Box<dyn StdError>> {
        let bytes = address.as_bytes();
        let h160 = H160::from_slice(bytes);
        let local_address = Address::from(h160);
        Ok(self.get_balance(&local_address).await)
    }

    async fn get_nonce(&self, address: &CoreAddress) -> Result<u64, Box<dyn StdError>> {
        let bytes = address.as_bytes();
        let h160 = H160::from_slice(bytes);
        let local_address = Address::from(h160);
        Ok(self.get_nonce(&local_address).await)
    }

    async fn get_code(&self, address: &CoreAddress) -> Result<Vec<u8>, Box<dyn StdError>> {
        let bytes = address.as_bytes();
        let h160 = H160::from_slice(bytes);
        let local_address = Address::from(h160);
        let code_hash = self.get_code_hash(&local_address).await;
        if code_hash.is_zero() {
            Ok(Vec::new())
        } else {
            let storage = self.storage.read().await;
            let code_bytes = storage.get_storage_value(&local_address, [0u8; 32]).await;
            Ok(code_bytes.to_vec())
        }
    }

    async fn get_storage(
        &self,
        address: &CoreAddress,
        key: &CoreHash,
    ) -> Result<CoreHash, Box<dyn StdError>> {
        let bytes = address.as_bytes();
        let h160 = H160::from_slice(bytes);
        let local_address = Address::from(h160);

        let key_bytes = key.as_bytes();
        let mut key_array = [0u8; 32];
        key_array.copy_from_slice(key_bytes);

        let value = self.get_storage_value(&local_address, key_array).await;
        Ok(fair_vm_core::Hash(H256(value)))
    }

    async fn set_storage(
        &self,
        address: &CoreAddress,
        key: &CoreHash,
        value: &CoreHash,
    ) -> Result<(), Box<dyn StdError>> {
        let bytes = address.as_bytes();
        let h160 = H160::from_slice(bytes);
        let local_address = Address::from(h160);

        let key_bytes = key.as_bytes();
        let mut key_array = [0u8; 32];
        key_array.copy_from_slice(key_bytes);

        let value_bytes = value.as_bytes();
        let mut value_array = [0u8; 32];
        value_array.copy_from_slice(value_bytes);

        let mut state = self.clone();
        let _ = state
            .set_storage_value(&local_address, key_array, value_array)
            .await;
        Ok(())
    }

    async fn add_balance(
        &self,
        address: &CoreAddress,
        amount: U256,
    ) -> Result<(), Box<dyn StdError>> {
        let bytes = address.as_bytes();
        let h160 = H160::from_slice(bytes);
        let local_address = Address::from(h160);
        let current_balance = self.get_balance(&local_address).await;
        let _ = self
            .set_balance(&local_address, current_balance + amount)
            .await;
        Ok(())
    }

    async fn sub_balance(
        &self,
        address: &CoreAddress,
        amount: U256,
    ) -> Result<(), Box<dyn StdError>> {
        let bytes = address.as_bytes();
        let h160 = H160::from_slice(bytes);
        let local_address = Address::from(h160);
        let current_balance = self.get_balance(&local_address).await;
        if current_balance < amount {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Insufficient balance",
            )));
        }
        let _ = self
            .set_balance(&local_address, current_balance - amount)
            .await;
        Ok(())
    }

    async fn increment_nonce(&self, address: &CoreAddress) -> Result<(), Box<dyn StdError>> {
        let bytes = address.as_bytes();
        let h160 = H160::from_slice(bytes);
        let local_address = Address::from(h160);
        let current_nonce = self.get_nonce(&local_address).await;
        let _ = self.set_nonce(&local_address, current_nonce + 1).await;
        Ok(())
    }

    async fn set_code(
        &self,
        address: &CoreAddress,
        code: Vec<u8>,
    ) -> Result<(), Box<dyn StdError>> {
        let bytes = address.as_bytes();
        let h160 = H160::from_slice(bytes);
        let local_address = Address::from(h160);
        let code_hash = H256::from_slice(&code);
        let _ = self.set_code_hash(&local_address, code_hash).await;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::MemoryStorage;

    #[tokio::test]
    async fn test_state_new() {
        let storage = Arc::new(RwLock::new(
            Box::new(MemoryStorage::default()) as Box<dyn Storage + Send + Sync>
        ));
        let state = State::new(storage.clone(), EvmContext::default());
        let _guard = state.storage().read().await;
    }

    #[tokio::test]
    async fn test_state_account_operations() {
        let storage = Arc::new(RwLock::new(
            Box::new(MemoryStorage::default()) as Box<dyn Storage + Send + Sync>
        ));
        let mut state = State::new(storage.clone(), EvmContext::default());
        let address = Address::from(H160::zero());
        let account = Account::new(address);

        // 测试设置账户
        assert!(state.set_account(&account).await.is_ok());
        assert_eq!(state.get_account(&address).await, Some(account));

        // 测试设置余额
        let balance = U256::from(100);
        assert!(state.set_balance(&address, balance).await.is_ok());
        assert_eq!(state.get_balance(&address).await, balance);

        // 测试设置 nonce
        let nonce = 42;
        assert!(state.set_nonce(&address, nonce).await.is_ok());
        assert_eq!(state.get_nonce(&address).await, nonce);

        // 测试设置代码哈希
        let code_hash = H256::random();
        assert!(state.set_code_hash(&address, code_hash).await.is_ok());
        assert_eq!(state.get_code_hash(&address).await, code_hash);
    }
}
