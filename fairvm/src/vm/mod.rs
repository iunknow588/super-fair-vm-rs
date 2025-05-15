// fairvm/src/vm/mod.rs

// 其他可能的功能
// 例如，定义一些与虚拟机相关的常量或辅助函数

use crate::{
    account::Account,
    account::Address,
    block::Block,
    config::Config,
    genesis::{FeesConfig, GasLimitConfig, Genesis},
    state::State,
    storage::Storage,
    transaction::Transaction,
};
use ethers::types::{
    Transaction as EthersTransaction, TransactionReceipt as EthersReceipt, H256 as EthersH256, U256,
};
use sha3::{Digest, Keccak256};
use std::sync::Arc;
use tokio::sync::RwLock;

/// VM配置
#[derive(Debug, Clone)]
pub struct VMConfig {
    /// Genesis配置
    pub genesis: Genesis,
    /// 当前区块基础费用
    pub current_base_fee: U256,
    /// 链ID
    pub chain_id: u64,
}

impl Default for VMConfig {
    fn default() -> Self {
        let genesis = Genesis::default();
        Self {
            chain_id: genesis.chain_id,
            current_base_fee: genesis.fees.gas_price_minimum,
            genesis,
        }
    }
}

/// 虚拟机实例
pub struct VM {
    /// 配置信息
    config: Config,
    /// 状态实例
    state: Arc<RwLock<State>>,
    current_block: Option<Block>,
    pending_transactions: Vec<Transaction>,
}

impl Default for VM {
    fn default() -> Self {
        Self {
            config: Config::default(),
            state: Arc::new(RwLock::new(State::new(None))),
            current_block: None,
            pending_transactions: Vec::new(),
        }
    }
}

impl VM {
    /// 创建新虚拟机实例
    pub fn new(storage: Option<Arc<RwLock<Box<dyn Storage + Send + Sync>>>>) -> Self {
        Self {
            config: Config::default(),
            state: Arc::new(RwLock::new(State::new(storage))),
            current_block: None,
            pending_transactions: Vec::new(),
        }
    }

    /// 使用配置创建虚拟机实例
    pub fn with_config(
        config: Config,
        storage: Option<Arc<RwLock<Box<dyn Storage + Send + Sync>>>>,
    ) -> Self {
        Self {
            config,
            state: Arc::new(RwLock::new(State::new(storage))),
            current_block: None,
            pending_transactions: Vec::new(),
        }
    }

    /// 从Genesis配置初始化VM
    pub async fn initialize_from_genesis(
        &mut self,
        genesis: Genesis,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 设置配置
        self.config = Config {
            chain_id: genesis.chain_id,
            current_base_fee: genesis.fees.gas_price_minimum,
            genesis: genesis.clone(),
        };

        // 初始化创世状态
        let mut state = self.state.write().await;

        // 应用初始账户分配
        for (address, account_data) in &genesis.alloc {
            let mut account = Account::new(Address(*address.as_fixed_bytes()));
            account.balance = account_data.balance;

            // 如果有合约代码，计算代码哈希
            if let Some(code_hex) = &account_data.code {
                if let Ok(code) = hex::decode(code_hex.trim_start_matches("0x")) {
                    let mut hasher = Keccak256::new();
                    hasher.update(&code);
                    account.code_hash = EthersH256(hasher.finalize().into());
                }
            }

            // 如果有初始存储，计算存储根
            if !account_data.storage.is_empty() {
                let mut hasher = Keccak256::new();
                for (key, value) in &account_data.storage {
                    if let (Ok(key_bytes), Ok(value_bytes)) = (
                        hex::decode(key.trim_start_matches("0x")),
                        hex::decode(value.trim_start_matches("0x")),
                    ) {
                        hasher.update(&key_bytes);
                        hasher.update(&value_bytes);
                    }
                }
                account.storage_root = EthersH256(hasher.finalize().into());
            }

            let _ = state.set_account(account);
        }

        // 创建创世区块
        self.current_block = Some(Block::new_with_fees(
            EthersH256([0; 32]),                  // 创世区块没有父区块
            0,                                    // 区块高度为0
            0,                                    // 时间戳为0
            Vec::new(),                           // 没有交易
            EthersH256(state.get_state_root().0), // 获取状态根
            genesis.gas_limit.block,
            0,                              // 没有gas使用
            genesis.fees.gas_price_minimum, // 初始基础费用
            U256::zero(),                   // 初始区块gas成本为0
        ));

        Ok(())
    }

    /// 获取当前区块的gas限制
    pub fn get_block_gas_limit(&self) -> u64 {
        self.config.genesis.gas_limit.block
    }

    /// 获取当前区块的基础费用
    pub fn get_base_fee(&self) -> U256 {
        self.config.current_base_fee
    }

    /// 获取当前链ID
    pub fn get_chain_id(&self) -> u64 {
        self.config.chain_id
    }

    /// 获取当前费用配置
    pub fn get_fees_config(&self) -> &FeesConfig {
        &self.config.genesis.fees
    }

    /// 获取当前gas限制配置
    pub fn get_gas_limit_config(&self) -> &GasLimitConfig {
        &self.config.genesis.gas_limit
    }

    pub async fn submit_transaction(
        &mut self,
        transaction: Transaction,
    ) -> Result<EthersH256, Box<dyn std::error::Error>> {
        // 验证交易签名
        if !transaction.verify_signature() {
            return Err("无效的交易签名".into());
        }

        // 检查交易的gas价格是否满足最低要求
        let min_gas_price = self.config.genesis.fees.gas_price_minimum;
        let base_fee = self.config.current_base_fee;

        if !transaction.validate(base_fee, min_gas_price) {
            return Err("交易gas价格低于最低要求".into());
        }

        // 检查交易gas限制是否超过区块gas限制
        if transaction.gas_limit > self.config.genesis.gas_limit.tx {
            return Err(format!(
                "交易gas限制{}超过最大允许值{}",
                transaction.gas_limit, self.config.genesis.gas_limit.tx
            )
            .into());
        }

        // 添加到待处理交易池
        self.pending_transactions.push(transaction.clone());
        // 将account::H256转换为ethers::types::H256
        Ok(EthersH256(transaction.hash().0))
    }

    pub async fn create_block(
        &mut self,
        parent_hash: EthersH256,
        timestamp: u64,
    ) -> Result<Block, Box<dyn std::error::Error>> {
        let state = self.state.read().await;
        let number = if let Some(current) = &self.current_block {
            current.number + 1
        } else {
            0
        };

        // 使用配置中的区块gas限制
        let gas_limit = self.config.genesis.gas_limit.block;

        // 使用当前基础费用
        let base_fee = self.config.current_base_fee;

        // 创建新区块
        let block = Block::new_with_fees(
            parent_hash,
            number,
            timestamp,
            self.pending_transactions
                .iter()
                .map(|tx| {
                    // 将FairmVMTransaction转换为EthersTransaction
                    // 这里需要根据实际字段情况进行适当转换
                    EthersTransaction {
                        hash: EthersH256(tx.hash.0),
                        // 添加其他必要字段
                        ..Default::default()
                    }
                })
                .collect(),
            EthersH256(state.get_state_root().0), // 转换state_root类型
            gas_limit,
            0, // gas使用量会在执行交易后更新
            base_fee,
            U256::zero(), // 区块gas成本会在以后计算
        );

        // 更新当前区块和清空待处理交易
        self.current_block = Some(block.clone());
        self.pending_transactions.clear();

        // 如果启用了EIP-1559，根据上一个区块计算新的基础费用
        if self.config.genesis.fees.enable_1559 && number > 0 {
            if let Some(prev_block) = &self.current_block {
                let gas_target = self.config.genesis.fees.gas_target;
                let denominator = self.config.genesis.fees.base_fee_change_denominator;
                self.config.current_base_fee =
                    prev_block.calculate_next_base_fee(gas_target, denominator);
            }
        }

        Ok(block)
    }

    pub async fn get_account(&self, _address: &Address) -> Option<&Account> {
        // TODO: 实现账户获取逻辑
        None
    }

    pub async fn get_transaction_receipt(
        &self,
        _tx_hash: &EthersH256,
    ) -> Result<EthersReceipt, Box<dyn std::error::Error>> {
        // TODO: 实现交易收据查询
        Err("未实现".into())
    }

    pub async fn get_account_transactions(
        &self,
        _address: &Address,
    ) -> Result<Vec<Transaction>, Box<dyn std::error::Error>> {
        // TODO: 实现账户交易历史查询
        Ok(Vec::new())
    }

    /// 获取当前区块
    pub async fn current_block(&self) -> Option<Block> {
        self.current_block.clone()
    }
}
