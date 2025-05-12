# Block 模块

本模块实现了 [`snowman.Block`](https://pkg.go.dev/github.com/ava-labs/avalanchego/snow/consensus/snowman#Block) 接口，定义了 timestampvm 的区块结构和操作。

## 文件结构

- `mod.rs` - 模块入口点，定义区块结构和实现

## 区块结构

`Block` 结构包含以下字段：

- `parent_id` - 父区块的 ID
- `height` - 区块高度（创世区块高度为 0）
- `timestamp` - 区块提议的 Unix 时间戳
- `data` - 任意数据
- `status` - 当前区块状态（处理中、已接受、已拒绝）
- `bytes` - 区块的编码字节
- `id` - 生成的区块 ID
- `state` - 对 VM 状态管理器的引用

## 主要功能

### 构造和序列化

- `try_new` - 创建新区块
- `to_json_string` - 将区块序列化为 JSON 字符串
- `to_vec` - 将区块编码为 JSON 字节
- `from_slice` - 从 JSON 字节加载区块

### 访问器方法

- `parent_id` - 获取父区块 ID
- `height` - 获取区块高度
- `timestamp` - 获取区块时间戳
- `data` - 获取区块数据
- `status` - 获取区块状态
- `bytes` - 获取区块字节
- `id` - 获取区块 ID

### 状态管理

- `set_status` - 更新区块状态
- `set_state` - 更新区块的状态管理器引用

### 区块处理

- `verify` - 验证区块属性并记录到状态
- `accept` - 标记区块为已接受并更新状态
- `reject` - 标记区块为已拒绝并更新状态

## 接口实现

该模块实现了两个关键接口：

1. `snowman::Block` - Avalanche Snowman 共识引擎的区块接口
   - `bytes` - 获取区块字节
   - `height` - 获取区块高度
   - `timestamp` - 获取区块时间戳
   - `parent` - 获取父区块 ID
   - `verify` - 验证区块

2. `Decidable` - 表示可以被接受或拒绝的对象
   - `status` - 获取对象状态
   - `id` - 获取对象 ID
   - `accept` - 接受对象
   - `reject` - 拒绝对象

## 验证规则

区块验证包括以下规则：

1. 区块高度必须比父区块高度大 1
2. 区块时间戳必须晚于父区块时间戳
3. 区块时间戳不能超过当前时间 1 小时以上

## 使用示例

```rust
// 创建创世区块
let mut genesis_block = Block::try_new(
    ids::Id::empty(),
    0,
    current_timestamp,
    data,
    choices::status::Status::default(),
)?;

// 设置状态管理器
genesis_block.set_state(state.clone());

// 验证区块
genesis_block.verify().await?;

// 接受区块
genesis_block.accept().await?;

// 创建子区块
let mut block1 = Block::try_new(
    genesis_block.id(),
    genesis_block.height() + 1,
    genesis_block.timestamp() + 1,
    new_data,
    choices::status::Status::default(),
)?;
```
