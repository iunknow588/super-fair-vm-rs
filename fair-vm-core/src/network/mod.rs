use crate::blockchain::Blockchain;
use crate::types::{Hash, Transaction};
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 网络接口
#[async_trait::async_trait]
pub trait Network: Send + Sync {
    /// 启动网络
    async fn start(&self) -> Result<(), Box<dyn Error>>;

    /// 停止网络
    async fn stop(&self) -> Result<(), Box<dyn Error>>;

    /// 广播交易
    async fn broadcast_transaction(&self, transaction: &Transaction) -> Result<(), Box<dyn Error>>;

    /// 广播区块
    async fn broadcast_block(&self, block_hash: &Hash) -> Result<(), Box<dyn Error>>;

    /// 获取对等节点列表
    async fn get_peers(&self) -> Result<Vec<SocketAddr>, Box<dyn Error>>;

    /// 添加对等节点
    async fn add_peer(&self, addr: SocketAddr) -> Result<(), Box<dyn Error>>;

    /// 移除对等节点
    async fn remove_peer(&self, addr: SocketAddr) -> Result<(), Box<dyn Error>>;
}

/// 基本网络实现
pub struct BasicNetwork {
    #[allow(dead_code)]
    blockchain: Arc<RwLock<Box<dyn Blockchain>>>,
    peers: Arc<RwLock<Vec<SocketAddr>>>,
}

impl BasicNetwork {
    /// 创建新的网络实例
    pub fn new(blockchain: Arc<RwLock<Box<dyn Blockchain>>>) -> Self {
        Self {
            blockchain,
            peers: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

#[async_trait::async_trait]
impl Network for BasicNetwork {
    async fn start(&self) -> Result<(), Box<dyn Error>> {
        // 实现启动网络的逻辑
        Ok(())
    }

    async fn stop(&self) -> Result<(), Box<dyn Error>> {
        // 实现停止网络的逻辑
        Ok(())
    }

    async fn broadcast_transaction(
        &self,
        _transaction: &Transaction,
    ) -> Result<(), Box<dyn Error>> {
        // 实现广播交易的逻辑
        Ok(())
    }

    async fn broadcast_block(&self, _block_hash: &Hash) -> Result<(), Box<dyn Error>> {
        // 实现广播区块的逻辑
        Ok(())
    }

    async fn get_peers(&self) -> Result<Vec<SocketAddr>, Box<dyn Error>> {
        // 实现获取对等节点列表的逻辑
        Ok(self.peers.read().await.clone())
    }

    async fn add_peer(&self, addr: SocketAddr) -> Result<(), Box<dyn Error>> {
        // 实现添加对等节点的逻辑
        self.peers.write().await.push(addr);
        Ok(())
    }

    async fn remove_peer(&self, addr: SocketAddr) -> Result<(), Box<dyn Error>> {
        // 实现移除对等节点的逻辑
        self.peers.write().await.retain(|&peer| peer != addr);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain::BasicBlockchain;
    use crate::state::State;
    use crate::vm::BasicVm;

    #[tokio::test]
    async fn test_network_new() {
        let state = Box::new(State::new());
        let vm = Box::new(BasicVm);
        let blockchain = Arc::new(RwLock::new(
            Box::new(BasicBlockchain::new(vm, state)) as Box<dyn Blockchain>
        ));
        let network = BasicNetwork::new(blockchain);
        assert_eq!(network.get_peers().await.unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_network_add_peer() {
        let state = Box::new(State::new());
        let vm = Box::new(BasicVm);
        let blockchain = Arc::new(RwLock::new(
            Box::new(BasicBlockchain::new(vm, state)) as Box<dyn Blockchain>
        ));
        let network = BasicNetwork::new(blockchain);
        let address = SocketAddr::from(([127, 0, 0, 1], 12345));
        assert!(network.add_peer(address).await.is_ok());
        assert_eq!(network.get_peers().await.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_network_remove_peer() {
        let state = Box::new(State::new());
        let vm = Box::new(BasicVm);
        let blockchain = Arc::new(RwLock::new(
            Box::new(BasicBlockchain::new(vm, state)) as Box<dyn Blockchain>
        ));
        let network = BasicNetwork::new(blockchain);
        let address = SocketAddr::from(([127, 0, 0, 1], 12345));
        assert!(network.add_peer(address).await.is_ok());
        assert!(network.remove_peer(address).await.is_ok());
        assert_eq!(network.get_peers().await.unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_network_get_peers() {
        let state = Box::new(State::new());
        let vm = Box::new(BasicVm);
        let blockchain = Arc::new(RwLock::new(
            Box::new(BasicBlockchain::new(vm, state)) as Box<dyn Blockchain>
        ));
        let network = BasicNetwork::new(blockchain);
        let address1 = SocketAddr::from(([127, 0, 0, 1], 12345));
        let address2 = SocketAddr::from(([127, 0, 0, 1], 12346));
        assert!(network.add_peer(address1).await.is_ok());
        assert!(network.add_peer(address2).await.is_ok());
        let peers = network.get_peers().await.unwrap();
        assert_eq!(peers.len(), 2);
        assert!(peers.contains(&address1));
        assert!(peers.contains(&address2));
    }
}
