# fairvm-sdk

本目录为 FairVM 的 Rust SDK，提供与 FairVM 交互的核心功能模块，便于开发者集成和扩展。

## 目录结构

- `Cargo.toml`：项目依赖和元数据配置文件。
- `src/`：SDK 源码目录。
  - `wallet/`：钱包相关功能模块。
  - `client/`：与 FairVM 通信的客户端实现。
  - `contract/`：智能合约交互模块。
  - `transaction/`：交易构建和处理模块。
  - `types/`：基础类型定义。
  - `utils/`：工具函数集合。
  - `lib.rs`：SDK 入口，聚合各模块。

## 主要模块说明

### 1. wallet 模块
- `mod.rs`：钱包模块主入口，聚合子模块。
- `hardware.rs`：硬件钱包支持，实现与硬件设备的交互。
- `transaction.rs`：钱包交易相关逻辑。
- `message.rs`：钱包消息签名与验证。
- `firmware.rs`：硬件钱包固件管理。
- `mnemonic.rs`：助记词生成与管理。
- `keystore.rs`：密钥存储与加密管理。

### 2. client 模块
- `mod.rs`：客户端主入口，实现与 FairVM 节点的 RPC 通信。
- `rpc.rs`：RPC 请求和响应处理。
- `websocket.rs`：WebSocket 连接管理。
- `subscription.rs`：事件订阅处理。

### 3. contract 模块
- `mod.rs`：合约模块主入口。
- `abi.rs`：ABI 编码和解码。
- `deploy.rs`：合约部署功能。
- `call.rs`：合约调用功能。
- `event.rs`：合约事件处理。

### 4. transaction 模块
- `mod.rs`：交易模块主入口。
- `builder.rs`：交易构建器。
- `signer.rs`：交易签名器。
- `broadcast.rs`：交易广播器。

### 5. types 模块
- `mod.rs`：类型定义主入口。
- `block.rs`：区块相关类型。
- `account.rs`：账户相关类型。
- `transaction.rs`：交易相关类型。

### 6. utils 模块
- `mod.rs`：工具函数主入口。
- `crypto.rs`：加密相关工具。
- `format.rs`：格式化工具。
- `validation.rs`：验证工具。

## 使用示例

### 1. 创建钱包
```rust
use fairvm_sdk::wallet::Wallet;

let wallet = Wallet::new()?;
let address = wallet.get_address();
```

### 2. 发送交易
```rust
use fairvm_sdk::client::Client;
use fairvm_sdk::transaction::TransactionBuilder;

let client = Client::new("http://localhost:8545");
let tx = TransactionBuilder::new()
    .from(address)
    .to(recipient)
    .value(amount)
    .build()?;
client.send_transaction(tx).await?;
```

### 3. 部署合约
```rust
use fairvm_sdk::contract::Contract;

let contract = Contract::new(abi, bytecode);
let deployed = contract.deploy(&client, &wallet).await?;
```

## 设计模式
- **模块化设计**：各功能独立，便于维护和扩展。
- **接口抽象**：对外暴露统一接口，内部细节可替换。
- **安全性**：钱包相关实现注重密钥、助记词等敏感信息的安全管理。
- **异步支持**：基于 tokio 的异步实现。
- **错误处理**：统一的错误类型和处理机制。

## 开发指南

### 1. 环境要求
- Rust 1.70+
- Cargo
- Git

### 2. 构建步骤
```bash
cargo build
```

### 3. 运行测试
```bash
cargo test
```

## 适用场景
- 开发者集成 FairVM 功能
- 构建 DApp 应用
- 开发钱包应用
- 自动化交易系统
- 区块链监控工具

## 注意事项
1. 妥善保管私钥和助记词
2. 注意网络连接状态
3. 合理设置超时时间
4. 正确处理异步操作
5. 注意错误处理
6. 遵循安全最佳实践 