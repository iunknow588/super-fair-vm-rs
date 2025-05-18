use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// 配置类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// 数据目录
    pub data_dir: PathBuf,
    /// 网络监听地址
    pub listen_addr: String,
    /// 网络端口
    pub port: u16,
    /// 对等节点列表
    pub peers: Vec<String>,
    /// 最大对等节点数量
    pub max_peers: usize,
    /// 区块 gas 限制
    pub gas_limit: u64,
    /// 区块时间戳
    pub timestamp: u64,
    /// 区块难度
    pub difficulty: u64,
    /// 区块奖励
    pub block_reward: u64,
    /// 交易 gas 价格
    pub gas_price: u64,
    /// 交易 gas 限制
    pub tx_gas_limit: u64,
    /// 交易池大小
    pub tx_pool_size: usize,
    /// 区块池大小
    pub block_pool_size: usize,
    /// 日志级别
    pub log_level: String,
    /// 日志文件
    pub log_file: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            data_dir: PathBuf::from("data"),
            listen_addr: "127.0.0.1".to_string(),
            port: 8545,
            peers: Vec::new(),
            max_peers: 50,
            gas_limit: 8_000_000,
            timestamp: 0,
            difficulty: 1,
            block_reward: 5_000_000_000_000_000_000,
            gas_price: 1,
            tx_gas_limit: 2_100_000,
            tx_pool_size: 1000,
            block_pool_size: 100,
            log_level: "info".to_string(),
            log_file: None,
        }
    }
}

impl Config {
    /// 创建新的配置实例
    pub fn new() -> Self {
        Self::default()
    }

    /// 从文件加载配置
    pub fn load(path: &PathBuf) -> Result<Self, String> {
        let content =
            std::fs::read_to_string(path).map_err(|e| format!("读取配置文件失败: {}", e))?;
        serde_json::from_str(&content).map_err(|e| format!("解析配置文件失败: {}", e))
    }

    /// 保存配置到文件
    pub fn save(&self, path: &PathBuf) -> Result<(), String> {
        let content =
            serde_json::to_string_pretty(self).map_err(|e| format!("序列化配置失败: {}", e))?;
        std::fs::write(path, content).map_err(|e| format!("写入配置文件失败: {}", e))
    }

    /// 获取网络地址
    pub fn get_network_addr(&self) -> String {
        format!("{}:{}", self.listen_addr, self.port)
    }

    /// 设置数据目录
    pub fn set_data_dir(&mut self, data_dir: PathBuf) {
        self.data_dir = data_dir;
    }

    /// 设置网络监听地址
    pub fn set_listen_addr(&mut self, listen_addr: String) {
        self.listen_addr = listen_addr;
    }

    /// 设置网络端口
    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    /// 添加对等节点
    pub fn add_peer(&mut self, peer: String) {
        if !self.peers.contains(&peer) {
            self.peers.push(peer);
        }
    }

    /// 移除对等节点
    pub fn remove_peer(&mut self, peer: &str) {
        if let Some(pos) = self.peers.iter().position(|x| x == peer) {
            self.peers.remove(pos);
        }
    }

    /// 设置最大对等节点数量
    pub fn set_max_peers(&mut self, max_peers: usize) {
        self.max_peers = max_peers;
    }

    /// 设置区块 gas 限制
    pub fn set_gas_limit(&mut self, gas_limit: u64) {
        self.gas_limit = gas_limit;
    }

    /// 设置区块时间戳
    pub fn set_timestamp(&mut self, timestamp: u64) {
        self.timestamp = timestamp;
    }

    /// 设置区块难度
    pub fn set_difficulty(&mut self, difficulty: u64) {
        self.difficulty = difficulty;
    }

    /// 设置区块奖励
    pub fn set_block_reward(&mut self, block_reward: u64) {
        self.block_reward = block_reward;
    }

    /// 设置交易 gas 价格
    pub fn set_gas_price(&mut self, gas_price: u64) {
        self.gas_price = gas_price;
    }

    /// 设置交易 gas 限制
    pub fn set_tx_gas_limit(&mut self, tx_gas_limit: u64) {
        self.tx_gas_limit = tx_gas_limit;
    }

    /// 设置交易池大小
    pub fn set_tx_pool_size(&mut self, tx_pool_size: usize) {
        self.tx_pool_size = tx_pool_size;
    }

    /// 设置区块池大小
    pub fn set_block_pool_size(&mut self, block_pool_size: usize) {
        self.block_pool_size = block_pool_size;
    }

    /// 设置日志级别
    pub fn set_log_level(&mut self, log_level: String) {
        self.log_level = log_level;
    }

    /// 设置日志文件
    pub fn set_log_file(&mut self, log_file: Option<PathBuf>) {
        self.log_file = log_file;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.port, 8545);
        assert_eq!(config.max_peers, 50);
        assert_eq!(config.gas_limit, 8_000_000);
        assert_eq!(config.difficulty, 1);
        assert_eq!(config.gas_price, 1);
        assert_eq!(config.tx_gas_limit, 2_100_000);
        assert_eq!(config.tx_pool_size, 1000);
        assert_eq!(config.block_pool_size, 100);
        assert_eq!(config.log_level, "info");
    }

    #[test]
    fn test_config_new() {
        let config = Config::new();
        assert_eq!(config.port, 8545);
        assert_eq!(config.max_peers, 50);
        assert_eq!(config.gas_limit, 8_000_000);
        assert_eq!(config.difficulty, 1);
        assert_eq!(config.gas_price, 1);
        assert_eq!(config.tx_gas_limit, 2_100_000);
        assert_eq!(config.tx_pool_size, 1000);
        assert_eq!(config.block_pool_size, 100);
        assert_eq!(config.log_level, "info");
    }

    #[test]
    fn test_config_get_network_addr() {
        let config = Config::new();
        assert_eq!(config.get_network_addr(), "127.0.0.1:8545");
    }

    #[test]
    fn test_config_add_peer() {
        let mut config = Config::new();
        config.add_peer("127.0.0.1:8546".to_string());
        assert_eq!(config.peers.len(), 1);
        assert_eq!(config.peers[0], "127.0.0.1:8546");
    }

    #[test]
    fn test_config_remove_peer() {
        let mut config = Config::new();
        config.add_peer("127.0.0.1:8546".to_string());
        config.remove_peer("127.0.0.1:8546");
        assert_eq!(config.peers.len(), 0);
    }

    #[test]
    fn test_config_save_and_load() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("config.json");
        let mut config = Config::new();
        config.set_port(8546);
        config.set_max_peers(100);
        config.set_gas_limit(10_000_000);
        config.set_difficulty(2);
        config.set_gas_price(2);
        config.set_tx_gas_limit(3_000_000);
        config.set_tx_pool_size(2000);
        config.set_block_pool_size(200);
        config.set_log_level("debug".to_string());
        config.set_log_file(Some(PathBuf::from("debug.log")));

        assert!(config.save(&path).is_ok());
        let loaded_config = Config::load(&path).unwrap();
        assert_eq!(loaded_config.port, 8546);
        assert_eq!(loaded_config.max_peers, 100);
        assert_eq!(loaded_config.gas_limit, 10_000_000);
        assert_eq!(loaded_config.difficulty, 2);
        assert_eq!(loaded_config.gas_price, 2);
        assert_eq!(loaded_config.tx_gas_limit, 3_000_000);
        assert_eq!(loaded_config.tx_pool_size, 2000);
        assert_eq!(loaded_config.block_pool_size, 200);
        assert_eq!(loaded_config.log_level, "debug");
        assert_eq!(loaded_config.log_file, Some(PathBuf::from("debug.log")));

        fs::remove_file(path).unwrap();
    }
}
