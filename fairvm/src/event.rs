use crate::account::Address;
use ethers::types::{H256, U256};
use serde::{Deserialize, Serialize};
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
    Error { error: String },
}

/// 事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// 事件类型
    pub event_type: EventType,
    /// 事件时间戳
    pub timestamp: u64,
}

/// 事件订阅者
pub type EventSubscriber = broadcast::Receiver<Event>;

/// 事件发布者
pub type EventPublisher = broadcast::Sender<Event>;

/// 事件管理器
#[derive(Clone)]
pub struct EventManager {
    /// 事件发布者
    publisher: EventPublisher,
    /// 事件缓冲区大小
    buffer_size: usize,
}

impl EventManager {
    /// 创建新的事件管理器
    pub fn new(buffer_size: usize) -> Self {
        let (publisher, _) = broadcast::channel(buffer_size);
        Self {
            publisher,
            buffer_size,
        }
    }

    /// 订阅事件
    pub fn subscribe(&self) -> EventSubscriber {
        self.publisher.subscribe()
    }

    /// 发布事件
    pub fn publish(&self, event: Event) -> Result<(), broadcast::error::SendError<Event>> {
        self.publisher.send(event).map(|_| ())
    }

    /// 获取事件缓冲区大小
    pub fn buffer_size(&self) -> usize {
        self.buffer_size
    }
}

impl Default for EventManager {
    fn default() -> Self {
        Self::new(1000)
    }
}

/// 事件处理器
pub trait EventHandler: Send + Sync {
    /// 处理事件
    fn handle_event(&self, event: &Event);
}

/// 事件处理器管理器
pub struct EventHandlerManager {
    /// 事件管理器
    event_manager: Arc<RwLock<EventManager>>,
    /// 事件处理器列表
    handlers: Vec<Arc<dyn EventHandler>>,
}

impl EventHandlerManager {
    /// 创建新的事件处理器管理器
    pub fn new(event_manager: Arc<RwLock<EventManager>>) -> Self {
        Self {
            event_manager,
            handlers: Vec::new(),
        }
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

    /// 启动事件处理循环
    pub async fn start(&self) {
        let mut subscriber = self.event_manager.read().await.subscribe();
        let handlers = self.handlers.clone();

        tokio::spawn(async move {
            while let Ok(event) = subscriber.recv().await {
                for handler in &handlers {
                    handler.handle_event(&event);
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

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
        let event_manager = Arc::new(RwLock::new(EventManager::new(100)));
        let mut handler_manager = EventHandlerManager::new(event_manager.clone());

        // 添加测试处理器
        let handler = Arc::new(TestEventHandler::new());
        handler_manager.add_handler(handler.clone());

        // 启动事件处理
        handler_manager.start().await;

        // 发布测试事件
        let event = Event {
            event_type: EventType::Block {
                number: 1,
                hash: H256([0; 32]),
                timestamp: 0,
            },
            timestamp: 0,
        };

        event_manager.write().await.publish(event).unwrap();

        // 等待事件处理
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        assert_eq!(handler.count(), 1);
    }
}
