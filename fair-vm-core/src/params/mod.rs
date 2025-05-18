use primitive_types::U256;

pub struct ChainConfig {
    pub chain_id: U256,
    pub homestead_block: U256,
    pub eip150_block: U256,
    pub eip155_block: U256,
    pub eip158_block: U256,
    pub byzantium_block: U256,
    pub constantinople_block: U256,
    pub petersburg_block: U256,
    pub istanbul_block: U256,
    pub muir_glacier_block: U256,
    pub berlin_block: U256,
    pub london_block: U256,
    // ... 可根据需要继续扩展
}
