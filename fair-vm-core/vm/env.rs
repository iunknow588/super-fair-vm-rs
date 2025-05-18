use crate::core::vm::runtime::Config;
use crate::core::types::{Address, U256};

pub struct Env {
    pub origin: Address,
    pub coinbase: Address,
    pub block_number: U256,
    pub time: u64,
    pub gas_limit: u64,
    pub difficulty: U256,
    // ... 其它区块/交易上下文字段
}

impl Env {
    pub fn new(cfg: &Config) -> Self {
        Self {
            origin: cfg.origin.clone(),
            coinbase: cfg.coinbase.clone(),
            block_number: cfg.block_number.clone(),
            time: cfg.time,
            gas_limit: cfg.gas_limit,
            difficulty: cfg.difficulty.clone(),
            // ...
        }
    }
}
