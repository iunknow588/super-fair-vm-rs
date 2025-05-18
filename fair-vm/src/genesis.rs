use crate::types::{Address, Hash};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Genesis {
    pub chain_id: u64,
    pub timestamp: u64,
    pub gas_limit: GasLimitConfig,
    pub fees: FeesConfig,
    pub alloc: HashMap<Address, GenesisAccount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenesisAccount {
    pub balance: u64,
    pub code: Option<Vec<u8>>,
    pub storage: HashMap<Hash, Hash>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasLimitConfig {
    pub min: u64,
    pub max: u64,
    pub target: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeesConfig {
    pub base_fee: u64,
    pub max_priority_fee: u64,
    pub max_fee: u64,
}

impl Default for Genesis {
    fn default() -> Self {
        Self {
            chain_id: 1,
            timestamp: chrono::Utc::now().timestamp() as u64,
            gas_limit: GasLimitConfig {
                min: 21000,
                max: 8000000,
                target: 15000000,
            },
            fees: FeesConfig {
                base_fee: 1000000000,
                max_priority_fee: 2000000000,
                max_fee: 10000000000,
            },
            alloc: HashMap::new(),
        }
    }
}

impl Genesis {
    pub fn new(chain_id: u64) -> Self {
        Self {
            chain_id,
            ..Default::default()
        }
    }

    pub fn add_account(&mut self, address: Address, balance: u64) {
        self.alloc.insert(
            address,
            GenesisAccount {
                balance,
                code: None,
                storage: HashMap::new(),
            },
        );
    }

    pub fn add_contract(
        &mut self,
        address: Address,
        balance: u64,
        code: Vec<u8>,
        storage: HashMap<Hash, Hash>,
    ) {
        self.alloc.insert(
            address,
            GenesisAccount {
                balance,
                code: Some(code),
                storage,
            },
        );
    }
}
