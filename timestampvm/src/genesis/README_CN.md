# Genesis 模块

本模块定义了 timestampvm 的创世区块结构和操作，用于初始化区块链。

## 文件结构

- `mod.rs` - 模块入口点，定义创世结构和实现

## 创世结构

`Genesis` 结构包含以下字段：

- `data` - 创世区块的数据（字符串）

## 主要功能

### 构造和序列化

- `to_vec` - 将创世信息编码为 JSON 字节
- `from_slice` - 从 JSON 字节加载创世信息

### 持久化

- `sync` - 将创世信息持久化到文件

## 默认实现

默认的创世数据为 "Hello from Rust VM!"：

```rust
impl Default for Genesis {
    fn default() -> Self {
        Self {
            data: String::from("Hello from Rust VM!"),
        }
    }
}
```

## 使用示例

### 创建自定义创世信息

```rust
let genesis = timestampvm::genesis::Genesis {
    data: String::from("Custom genesis data"),
};
```

### 序列化和反序列化

```rust
// 序列化为 JSON 字节
let bytes = genesis.to_vec()?;

// 从 JSON 字节反序列化
let loaded_genesis = Genesis::from_slice(&bytes)?;
```

### 持久化到文件

```rust
// 将创世信息保存到文件
genesis.sync("/path/to/genesis.json")?;
```

### 在命令行中使用

timestampvm 提供了命令行工具来生成创世信息：

```bash
timestampvm genesis "Custom genesis data"
```

这将输出 JSON 格式的创世信息。

## 与 VM 的集成

创世信息在 VM 初始化时使用：

1. 从文件或命令行参数加载创世信息
2. 在 VM 初始化过程中传递创世字节
3. VM 使用创世信息创建创世区块

在 VM 初始化过程中的使用示例：

```rust
// 在 VM 初始化中
let genesis = Genesis::from_slice(genesis_bytes)?;
vm_state.genesis = genesis;

// 创建创世区块
let mut genesis_block = Block::try_new(
    ids::Id::empty(),
    0,
    0,
    vm_state.genesis.data.as_bytes().to_vec(),
    choices::status::Status::default(),
)?;
```
