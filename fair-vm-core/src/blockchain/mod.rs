use crate::types::{Address, Hash, Header, Receipt, Transaction};
use crate::vm::{ExecutionResult, State, Vm};
use async_trait::async_trait;
use std::error::Error;

/// 区块链接口
#[async_trait]
pub trait Blockchain: Send + Sync {
    /// 获取当前区块头
    async fn current_header(&self) -> Result<Header, Box<dyn Error>>;

    /// 获取指定高度的区块头
    async fn get_header(&self, number: u64) -> Result<Header, Box<dyn Error>>;

    /// 获取账户余额
    async fn get_balance(&self, address: &Address) -> Result<u64, Box<dyn Error>>;

    /// 获取账户nonce
    async fn get_nonce(&self, address: &Address) -> Result<u64, Box<dyn Error>>;

    /// 获取账户代码
    async fn get_code(&self, address: &Address) -> Result<Vec<u8>, Box<dyn Error>>;

    /// 获取存储值
    async fn get_storage(&self, address: &Address, key: &Hash) -> Result<Hash, Box<dyn Error>>;

    /// 执行交易
    async fn execute_transaction(
        &self,
        transaction: &Transaction,
    ) -> Result<ExecutionResult, Box<dyn Error>>;

    /// 获取交易收据
    async fn get_receipt(&self, transaction_hash: &Hash) -> Result<Receipt, Box<dyn Error>>;
}

/// 基本区块链实现
pub struct BasicBlockchain {
    vm: Box<dyn Vm>,
    state: Box<dyn State>,
}

impl BasicBlockchain {
    /// 创建新的区块链实例
    pub fn new(vm: Box<dyn Vm>, state: Box<dyn State>) -> Self {
        Self { vm, state }
    }
}

#[async_trait]
impl Blockchain for BasicBlockchain {
    async fn current_header(&self) -> Result<Header, Box<dyn Error>> {
        // 实现获取当前区块头的逻辑
        Ok(Header::new(
            Hash::random(),
            0,
            0,
            Hash::random(),
            Hash::random(),
            Hash::random(),
        ))
    }

    async fn get_header(&self, number: u64) -> Result<Header, Box<dyn Error>> {
        // 实现获取指定高度区块头的逻辑
        Ok(Header::new(
            Hash::random(),
            number,
            0,
            Hash::random(),
            Hash::random(),
            Hash::random(),
        ))
    }

    async fn get_balance(&self, _address: &Address) -> Result<u64, Box<dyn Error>> {
        // 实现获取账户余额的逻辑
        Ok(0)
    }

    async fn get_nonce(&self, _address: &Address) -> Result<u64, Box<dyn Error>> {
        // 实现获取账户nonce的逻辑
        Ok(0)
    }

    async fn get_code(&self, _address: &Address) -> Result<Vec<u8>, Box<dyn Error>> {
        // 实现获取账户代码的逻辑
        Ok(vec![])
    }

    async fn get_storage(&self, _address: &Address, _key: &Hash) -> Result<Hash, Box<dyn Error>> {
        // 实现获取存储值的逻辑
        Ok(Hash::random())
    }

    async fn execute_transaction(
        &self,
        transaction: &Transaction,
    ) -> Result<ExecutionResult, Box<dyn Error>> {
        // 实现执行交易的逻辑
        self.vm
            .execute_transaction(transaction, self.state.as_ref())
            .await
    }

    async fn get_receipt(&self, transaction_hash: &Hash) -> Result<Receipt, Box<dyn Error>> {
        // 实现获取交易收据的逻辑
        Ok(Receipt {
            transaction_hash: *transaction_hash,
            block_hash: Hash::random(),
            block_number: 0,
            from: Address::random(),
            to: None,
            contract_address: None,
            gas_used: 0,
            status: true,
            logs: vec![],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::State;
    use crate::vm::BasicVm;

    #[tokio::test]
    async fn test_blockchain_new() {
        let state = Box::new(State::new());
        let vm = Box::new(BasicVm);
        let blockchain = BasicBlockchain::new(vm, state);
        assert!(blockchain.current_header().await.is_ok());
    }

    #[tokio::test]
    async fn test_blockchain_get_balance() {
        let state = Box::new(State::new());
        let vm = Box::new(BasicVm);
        let blockchain = BasicBlockchain::new(vm, state);
        let address = Address::random();
        assert!(blockchain.get_balance(&address).await.is_ok());
    }

    #[tokio::test]
    async fn test_blockchain_get_nonce() {
        let state = Box::new(State::new());
        let vm = Box::new(BasicVm);
        let blockchain = BasicBlockchain::new(vm, state);
        let address = Address::random();
        assert!(blockchain.get_nonce(&address).await.is_ok());
    }

    #[tokio::test]
    async fn test_blockchain_get_code() {
        let state = Box::new(State::new());
        let vm = Box::new(BasicVm);
        let blockchain = BasicBlockchain::new(vm, state);
        let address = Address::random();
        assert!(blockchain.get_code(&address).await.is_ok());
    }

    #[tokio::test]
    async fn test_blockchain_get_storage() {
        let state = Box::new(State::new());
        let vm = Box::new(BasicVm);
        let blockchain = BasicBlockchain::new(vm, state);
        let address = Address::random();
        let key = Hash::random();
        assert!(blockchain.get_storage(&address, &key).await.is_ok());
    }
}
