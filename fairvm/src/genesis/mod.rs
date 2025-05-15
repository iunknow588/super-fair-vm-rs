use ethers::types::{Address, U256};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Genesis配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Genesis {
    /// 链ID
    pub chain_id: u64,

    /// 初始代币分配
    pub alloc: HashMap<Address, GenesisAccount>,

    /// Gas限制相关配置
    pub gas_limit: GasLimitConfig,

    /// 费用相关配置
    pub fees: FeesConfig,
}

/// Genesis账户
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GenesisAccount {
    /// 账户余额
    pub balance: U256,

    /// 合约代码(如果有)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// 初始存储值
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub storage: HashMap<String, String>,
}

/// Gas限制配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GasLimitConfig {
    /// 区块gas限制
    pub block: u64,

    /// 合约创建gas限制
    pub contract_creation: u64,

    /// 交易gas限制
    pub tx: u64,
}

/// 手续费配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FeesConfig {
    /// 最小gas价格（单位：nanoFAIR）
    pub gas_price_minimum: U256,

    /// 目标gas用量（用于动态调整基础费用）
    pub gas_target: u64,

    /// 是否启用EIP-1559风格的费用机制
    pub enable_1559: bool,

    /// 基础费用变化分母（控制基础费用变化速度）
    pub base_fee_change_denominator: u64,
}

impl Default for Genesis {
    fn default() -> Self {
        Self {
            chain_id: 2023,
            alloc: HashMap::new(),
            gas_limit: GasLimitConfig::default(),
            fees: FeesConfig::default(),
        }
    }
}

impl Default for GasLimitConfig {
    fn default() -> Self {
        Self {
            block: 15_000_000,
            contract_creation: 8_000_000,
            tx: 8_000_000,
        }
    }
}

impl Default for FeesConfig {
    fn default() -> Self {
        Self {
            gas_price_minimum: U256::from(1_000_000_000), // 1 GWei
            gas_target: 8_000_000,
            enable_1559: true,
            base_fee_change_denominator: 8,
        }
    }
}

/// 从JSON字节解析Genesis配置
///
/// # 错误
/// 如果解析失败，返回错误
pub fn parse_genesis(bytes: &[u8]) -> Result<Genesis, serde_json::Error> {
    serde_json::from_slice(bytes)
}

/// 将Genesis配置序列化为JSON字节
///
/// # 错误
/// 如果序列化失败，返回错误
pub fn serialize_genesis(genesis: &Genesis) -> Result<Vec<u8>, serde_json::Error> {
    serde_json::to_vec(genesis)
}
