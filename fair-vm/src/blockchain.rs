use crate::transaction::Transaction;
use ethers::types::H256;
use serde::{Deserialize, Serialize};

/// 区块头
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    /// 父区块哈希
    pub parent_hash: H256,
    /// 区块高度
    pub number: u64,
    /// 时间戳
    pub timestamp: u64,
    /// 交易根
    pub transactions_root: H256,
    /// 状态根
    pub state_root: H256,
    /// 区块难度
    pub difficulty: u64,
    /// 区块奖励
    pub block_reward: u64,
}

/// 区块
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    /// 区块头
    pub header: BlockHeader,
    /// 交易列表
    pub transactions: Vec<Transaction>,
}

/// 区块链配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainConfig {
    /// 创世区块
    pub genesis_block: Block,
    /// 区块时间（秒）
    pub block_time: u64,
    /// 最大区块大小
    pub max_block_size: usize,
    /// 最小区块大小
    pub min_block_size: usize,
    /// 最大交易数
    pub max_transactions: usize,
    /// 最小交易数
    pub min_transactions: usize,
}

/// 区块链
#[derive(Debug)]
pub struct Blockchain {
    /// 配置
    #[allow(dead_code)]
    config: BlockchainConfig,
    /// 当前区块
    current_block: Option<Block>,
    /// 区块历史
    blocks: Vec<Block>,
}

impl Blockchain {
    /// 创建新的区块链实例
    pub fn new(config: BlockchainConfig) -> Self {
        Self {
            config,
            current_block: None,
            blocks: Vec::new(),
        }
    }

    /// 获取当前区块
    pub fn current_block(&self) -> Option<&Block> {
        self.current_block.as_ref()
    }

    /// 获取区块历史
    pub fn blocks(&self) -> &[Block] {
        &self.blocks
    }

    /// 添加新区块
    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block.clone());
        self.current_block = Some(block);
    }

    /// 获取指定高度的区块
    pub fn get_block(&self, height: u64) -> Option<&Block> {
        self.blocks.iter().find(|b| b.header.number == height)
    }

    /// 获取最新区块
    pub fn latest_block(&self) -> Option<&Block> {
        self.blocks.last()
    }
}

impl Default for Blockchain {
    fn default() -> Self {
        Self {
            config: BlockchainConfig {
                genesis_block: Block {
                    header: BlockHeader {
                        parent_hash: H256::zero(),
                        number: 0,
                        timestamp: 0,
                        transactions_root: H256::zero(),
                        state_root: H256::zero(),
                        difficulty: 0,
                        block_reward: 0,
                    },
                    transactions: Vec::new(),
                },
                block_time: 1,
                max_block_size: 1024 * 1024,
                min_block_size: 0,
                max_transactions: 1000,
                min_transactions: 0,
            },
            current_block: None,
            blocks: Vec::new(),
        }
    }
}
