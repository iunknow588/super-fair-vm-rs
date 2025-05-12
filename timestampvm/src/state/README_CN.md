# State 模块

本模块管理 timestampvm 的状态，包括区块存储和检索，以及链状态的维护。

## 文件结构

- `mod.rs` - 模块入口点，定义状态结构和实现

## 状态结构

`State` 结构包含以下字段：

- `db` - 区块链数据库，用于持久化存储
- `verified_blocks` - 已验证但尚未接受/拒绝的区块映射

## 辅助结构

- `BlockWithStatus` - 包装区块和其状态，用于持久化存储

## 主要功能

### 区块链状态管理

- `set_last_accepted_block` - 持久化最后接受的区块 ID
- `has_last_accepted_block` - 检查是否存在最后接受的区块
- `get_last_accepted_block_id` - 获取最后接受的区块 ID

### 已验证区块管理

- `add_verified` - 添加已验证的区块到内存映射
- `remove_verified` - 从内存映射中移除已验证的区块
- `has_verified` - 检查区块是否已验证

### 区块存储和检索

- `write_block` - 将区块写入持久化存储
- `get_block` - 从存储中检索区块

## 存储键格式

模块使用特定的键格式来组织数据库中的数据：

- 最后接受的区块键: `last_accepted_block`
- 区块状态键: `STATUS_PREFIX + DELIMITER + block_id`

## 并发控制

状态管理使用 `Arc<RwLock<>>` 来保护共享资源：

- `db` - 保护数据库访问
- `verified_blocks` - 保护已验证区块映射

这允许多个异步任务安全地访问和修改状态。

## 默认实现

默认状态使用内存数据库：

```rust
impl Default for State {
    fn default() -> State {
        Self {
            db: Arc::new(RwLock::new(
                subnet::rpc::database::memdb::Database::new_boxed(),
            )),
            verified_blocks: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
```

## 使用示例

```rust
// 创建状态实例
let state = state::State::default();

// 写入区块
state.write_block(&block).await?;

// 设置最后接受的区块
state.set_last_accepted_block(&block.id()).await?;

// 检查是否有最后接受的区块
let has_last_accepted = state.has_last_accepted_block().await?;

// 获取最后接受的区块 ID
let last_accepted_id = state.get_last_accepted_block_id().await?;

// 获取区块
let block = state.get_block(&block_id).await?;

// 添加已验证的区块
state.add_verified(&block).await;

// 检查区块是否已验证
let is_verified = state.has_verified(&block_id).await;

// 移除已验证的区块
state.remove_verified(&block_id).await;
```

## 与区块模块的集成

状态模块与区块模块紧密集成：

1. 区块包含对状态的引用
2. 区块验证、接受和拒绝操作更新状态
3. 状态管理区块的持久化存储和检索
