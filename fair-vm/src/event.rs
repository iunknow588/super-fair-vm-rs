use crate::account::Address;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use ethers::types::{H256, U256};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::RwLock;

/// 事件类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    /// 区块事件
    Block {
        number: u64,
        hash: H256,
        timestamp: u64,
    },
    /// 交易事件
    Transaction {
        hash: H256,
        from: Address,
        to: Option<Address>,
        value: U256,
    },
    /// 账户事件
    Account {
        address: Address,
        balance: U256,
        nonce: u64,
    },
    /// NFT 事件
    NFT {
        contract: Address,
        token_id: U256,
        from: Option<Address>,
        to: Option<Address>,
    },
    /// 共识事件
    Consensus {
        height: u64,
        validators: Vec<Address>,
    },
    /// 错误事件
    Error {
        error: String,
    },
    BlockCreated,
    BlockFinalized,
    TransactionReceived,
    TransactionProcessed,
    StateChanged,
    ConsensusStateChanged,
    NetworkMessage,
}

/// 事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// 事件类型
    pub event_type: EventType,
    /// 事件数据
    pub data: Value,
    /// 事件时间戳
    pub timestamp: DateTime<Utc>,
}

/// 事件订阅者
pub type EventSubscriber = broadcast::Receiver<Event>;

/// 事件发布者
pub type EventPublisher = broadcast::Sender<Event>;

/// 事件处理器 trait
#[async_trait]
pub trait EventHandler: Send + Sync {
    fn handle_event(&self, event: &Event);
}

/// 事件管理器
pub struct EventManager {
    /// 事件发布者
    publisher: EventPublisher,
    /// 事件缓冲区大小
    buffer_size: usize,
    /// 事件处理器列表
    handlers: Vec<Arc<dyn EventHandler>>,
}

impl EventManager {
    /// 创建新的事件管理器
    pub fn new(buffer_size: usize) -> Self {
        let (publisher, _) = broadcast::channel(buffer_size);
        Self {
            publisher,
            buffer_size,
            handlers: Vec::new(),
        }
    }

    /// 订阅事件
    pub fn subscribe(&self) -> EventSubscriber {
        self.publisher.subscribe()
    }

    /// 发布事件
    pub fn publish(&self, event: Event) -> Result<(), String> {
        for handler in &self.handlers {
            handler.handle_event(&event);
        }
        self.publisher.send(event).map_err(|e| e.to_string())?;
        Ok(())
    }

    /// 获取事件缓冲区大小
    pub fn buffer_size(&self) -> usize {
        self.buffer_size
    }

    /// 添加事件处理器
    pub fn add_handler(&mut self, handler: Arc<dyn EventHandler>) {
        self.handlers.push(handler);
    }

    /// 移除事件处理器
    pub fn remove_handler(&mut self, index: usize) {
        if index < self.handlers.len() {
            self.handlers.remove(index);
        }
    }
}

impl Default for EventManager {
    fn default() -> Self {
        Self::new(1000)
    }
}

/// 事件处理器管理器
pub struct EventHandlerManager {
    event_manager: Arc<RwLock<EventManager>>,
}

impl Default for EventHandlerManager {
    fn default() -> Self {
        Self {
            event_manager: Arc::new(RwLock::new(EventManager::default())),
        }
    }
}

impl EventHandlerManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_handler(&mut self, handler: Arc<dyn EventHandler>) {
        let mut event_manager = self.event_manager.blocking_write();
        event_manager.add_handler(handler);
    }

    pub fn remove_handler(&mut self, index: usize) {
        let mut event_manager = self.event_manager.blocking_write();
        event_manager.remove_handler(index);
    }

    pub async fn start(&self) {
        // TODO: 实现事件处理启动逻辑
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[derive(Debug)]
    struct TestEventHandler {
        event_count: AtomicUsize,
    }

    impl TestEventHandler {
        fn new() -> Self {
            Self {
                event_count: AtomicUsize::new(0),
            }
        }

        fn count(&self) -> usize {
            self.event_count.load(Ordering::SeqCst)
        }
    }

    impl EventHandler for TestEventHandler {
        fn handle_event(&self, _event: &Event) {
            self.event_count.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[tokio::test]
    async fn test_event_manager() {
        let mut manager = EventManager::new(100);
        let handler = Arc::new(TestEventHandler::new());

        manager.add_handler(handler.clone());

        // 创建一个订阅者以保持通道打开
        let _subscriber = manager.subscribe();

        let event = Event {
            event_type: EventType::BlockCreated,
            data: json!({
                "height": 1,
                "hash": "0x123"
            }),
            timestamp: Utc::now(),
        };

        manager.publish(event).unwrap();
        assert_eq!(handler.count(), 1);
    }
}
