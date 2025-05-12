# VM 模块

本模块实现了 [`snowman.block.ChainVM`](https://pkg.go.dev/github.com/ava-labs/avalanchego/snow/engine/snowman/block#ChainVM) 接口，是 timestampvm 的核心组件。

## 文件结构

- `mod.rs` - 模块入口点，定义 VM 结构和实现

## 常量

- `VERSION` - VM 版本，从 Cargo.toml 中获取
- `PROPOSE_LIMIT_BYTES` - 限制用户可以提议的数据大小（1MB）

## 结构定义

### State

表示 VM 特定状态：

- `ctx` - VM 上下文
- `version` - VM 版本
- `genesis` - 创世信息
- `state` - 持久化 VM 状态
- `preferred` - 当前首选区块 ID
- `to_engine` - 向共识引擎发送消息的通道
- `bootstrapped` - 指示 VM 是否已完成引导

### Vm

实现 `ChainVM` 接口：

- `state` - 维护 VM 特定状态
- `app_sender` - 应用消息发送器
- `mempool` - 尚未放入区块的数据队列

## 主要功能

### VM 状态管理

- `is_bootstrapped` - 检查 VM 是否已引导
- `notify_block_ready` - 通知共识引擎新区块准备就绪
- `propose_block` - 将任意数据提议到内存池
- `set_state` - 设置 VM 状态
- `last_accepted` - 返回最后接受的区块 ID

### CommonVm 接口实现

- `initialize` - 初始化 VM
- `shutdown` - 关闭 VM
- `create_static_handlers` - 创建静态处理程序
- `create_handlers` - 创建 VM 特定处理程序

### ChainVm 接口实现

- `build_block` - 从内存池数据构建区块
- `set_preference` - 设置首选区块
- `last_accepted` - 获取最后接受的区块 ID
- `issue_tx` - 发布交易（未实现）
- `verify_height_index` - 验证高度索引
- `get_block_id_at_height` - 获取指定高度的区块 ID
- `state_sync_enabled` - 检查状态同步是否启用

### BatchedChainVm 接口实现

- `get_ancestors` - 获取祖先区块（未实现）
- `batched_parse_block` - 批量解析区块（未实现）

### 网络和应用处理程序接口

- `NetworkAppHandler` - 处理应用特定消息
- `CrossChainAppHandler` - 处理跨链应用消息
- `AppHandler` - 应用处理程序
- `Connector` - 处理节点连接和断开连接
- `Checkable` - 提供健康检查
- `Getter` - 获取区块
- `Parser` - 解析区块

## 内存池管理

VM 维护一个内存池，存储尚未放入区块的数据：

1. `propose_block` 将数据添加到内存池
2. `build_block` 从内存池中取出数据构建区块
3. 通过 `notify_block_ready` 通知共识引擎新区块准备就绪

## 区块构建流程

1. 检查内存池是否有数据
2. 获取首选区块作为父区块
3. 从内存池中取出第一个数据项
4. 创建新区块，设置父区块 ID、高度、时间戳和数据
5. 验证新区块
6. 返回构建的区块

## 使用示例

```rust
// 创建 VM 实例
let vm = vm::Vm::new();

// 初始化 VM
vm.initialize(
    ctx,
    db_manager,
    genesis_bytes,
    upgrade_bytes,
    config_bytes,
    to_engine,
    fxs,
    app_sender,
).await?;

// 提议区块
vm.propose_block(data).await?;

// 构建区块
let block = vm.build_block().await?;

// 获取最后接受的区块
let last_accepted = vm.last_accepted().await?;
```

## 与其他模块的集成

VM 模块与其他模块紧密集成：

1. 使用 `state` 模块管理持久化状态
2. 使用 `block` 模块创建和验证区块
3. 使用 `genesis` 模块初始化创世区块
4. 使用 `api` 模块提供 RPC 服务
