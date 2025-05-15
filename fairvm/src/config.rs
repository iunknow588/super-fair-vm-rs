use crate::genesis::Genesis;
use ethers::types::U256;

/// VM 配置
#[derive(Debug, Clone)]
pub struct Config {
    /// 链 ID
    pub chain_id: u64,
    /// 当前区块基础费用
    pub current_base_fee: U256,
    /// Genesis 配置
    pub genesis: Genesis,
}

impl Default for Config {
    fn default() -> Self {
        let genesis = Genesis::default();
        Self {
            chain_id: genesis.chain_id,
            current_base_fee: genesis.fees.gas_price_minimum,
            genesis,
        }
    }
}

/// VM 配置构建器
#[derive(Debug, Default)]
pub struct ConfigBuilder {
    chain_id: Option<u64>,
    current_base_fee: Option<U256>,
    genesis: Option<Genesis>,
}

impl ConfigBuilder {
    /// 创建新的配置构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置链 ID
    pub fn chain_id(mut self, chain_id: u64) -> Self {
        self.chain_id = Some(chain_id);
        self
    }

    /// 设置当前区块基础费用
    pub fn current_base_fee(mut self, current_base_fee: U256) -> Self {
        self.current_base_fee = Some(current_base_fee);
        self
    }

    /// 设置 Genesis 配置
    pub fn genesis(mut self, genesis: Genesis) -> Self {
        self.genesis = Some(genesis);
        self
    }

    /// 构建配置
    pub fn build(self) -> Config {
        let genesis = self.genesis.unwrap_or_default();
        Config {
            chain_id: self.chain_id.unwrap_or(genesis.chain_id),
            current_base_fee: self
                .current_base_fee
                .unwrap_or(genesis.fees.gas_price_minimum),
            genesis,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        let genesis = Genesis::default();
        assert_eq!(config.chain_id, genesis.chain_id);
        assert_eq!(config.current_base_fee, genesis.fees.gas_price_minimum);
        assert_eq!(config.genesis, genesis);
    }

    #[test]
    fn test_config_builder() {
        let chain_id = 12345;
        let current_base_fee = U256::from(100);
        let genesis = Genesis::default();

        let config = ConfigBuilder::new()
            .chain_id(chain_id)
            .current_base_fee(current_base_fee)
            .genesis(genesis.clone())
            .build();

        assert_eq!(config.chain_id, chain_id);
        assert_eq!(config.current_base_fee, current_base_fee);
        assert_eq!(config.genesis, genesis);
    }
}
