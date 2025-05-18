use ethers::types::{H160, U256};

/// EVM 上下文
#[derive(Debug, Clone, Default)]
pub struct EvmContext {
    /// 当前区块时间戳
    pub timestamp: u64,
    /// 当前区块编号
    pub block_number: u64,
    /// 当前区块难度
    pub difficulty: U256,
    /// 当前区块矿工地址
    pub miner: H160,
    /// 当前区块 gas 限制
    pub gas_limit: u64,
}

impl EvmContext {
    /// 创建新的 EVM 上下文
    pub fn new() -> Self {
        Self::default()
    }
}
