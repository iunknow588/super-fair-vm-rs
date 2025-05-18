# fairvm

本目录为 FairVM 的核心实现，包含虚拟机、共识、区块、账户、存储、API 等核心模块。

## 目录结构
- `Cargo.toml`：项目依赖和元数据配置文件。
- `genesis.json.example`：创世区块配置示例。
- `src/`：核心源码目录。
  - `lib.rs`：库主入口，聚合各核心模块。
  - `config.rs`：配置管理。
  - `state.rs`：全局状态管理。
  - `account.rs`：账户管理。
  - `event.rs`：事件处理。
  - `storage/`：存储抽象与实现。
  - `consensus/`：共识算法实现。
  - `nft/`：NFT 功能模块。
  - `transaction/`：交易相关逻辑。
  - `api/`：对外 API 服务。
  - `genesis/`：创世区块相关逻辑。
  - `evm/`：EVM 虚拟机兼容层。
  - `wallet/`：钱包相关功能。
  - `rpc/`：远程过程调用支持。
  - `mempool/`：交易内存池。
  - `block/`：区块结构与操作。
  - `vm/`：虚拟机主逻辑。
  - `contracts/`：合约相关实现。
  - 其他子模块：如 state、account、tests 等。
- `docs/`：文档与说明。
- `benches/`：性能基准测试。
- `tests/`：集成与单元测试。

## 设计模式
- **模块化设计**：各功能独立，便于维护和扩展。
- **接口抽象**：对外暴露统一接口，内部细节可替换。
- **分层架构**：区分存储、共识、虚拟机、API 等层次。
- **异步编程**：部分模块采用异步提升性能。

## 主要功能

### 1. 虚拟机功能
- EVM兼容性实现
- 智能合约执行
- 状态转换
- 内存管理

### 2. 共识机制
- 可插拔共识引擎
- 区块验证
- 状态同步
- 网络通信

### 3. 状态管理
- 账户状态
- 存储管理
- 状态转换
- 状态验证

### 4. 交易处理
- 交易验证
- 交易执行
- 交易池管理
- 交易广播

### 5. API服务
- JSON-RPC接口
- WebSocket支持
- 事件订阅
- 状态查询

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

### 4. 性能测试
```bash
cargo bench
```

## 使用示例
```rust
use fairvm::{FairVM, Config};

// 创建FairVM实例
let config = Config::default();
let fairvm = FairVM::with_config(config);

// 启动FairVM
fairvm.start().await?;

// 提交交易
fairvm.submit_transaction(tx).await?;

// 停止FairVM
fairvm.stop().await?;
```

## 适用场景
- 作为 FairVM 的核心实现，支撑区块链网络的运行、共识、账户、交易、合约等全流程。
- 可用于构建自定义区块链网络
- 支持智能合约开发
- 提供完整的区块链功能

## 注意事项
1. 确保正确配置创世区块
2. 注意网络参数设置
3. 合理配置共识参数
4. 定期备份状态数据 