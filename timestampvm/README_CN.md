# timestampvm

这是 `timestampvm-rs` 项目的核心库，实现了一个最小化的 Avalanche 自定义虚拟机 (VM)，可以从用户提供的任意数据构建区块。

## 目录结构

### 源代码 (`src/`)

- `lib.rs` - 库入口点，定义了 VM 的主要组件和结构
- `bin/timestampvm/` - 命令行界面和插件服务器实现
  - `main.rs` - 主程序入口点
  - `genesis.rs` - 创世区块命令实现
  - `vm_id.rs` - VM ID 转换命令实现

#### 核心模块

- `api/` - 实现 timestampvm API
  - `mod.rs` - API 定义和通用结构
  - `chain_handlers.rs` - 链特定 RPC 处理程序
  - `static_handlers.rs` - 静态 RPC 处理程序
- `block/` - 实现 `snowman.Block` 接口
- `client/` - 实现 timestampvm API 客户端
- `genesis/` - 定义 timestampvm 创世区块
- `state/` - 管理虚拟机状态
- `vm/` - 实现 `snowman.block.ChainVM` 接口

## 模块详解

### api 模块

该模块实现了 timestampvm 的 API 服务，包括：

- **chain_handlers.rs**: 实现链特定的 RPC 处理程序，通过 `/ext/bc/[CHAIN ID]/rpc` 路径提供服务
  - `ping` - 检查 VM 是否正常运行
  - `proposeBlock` - 提议新区块
  - `lastAccepted` - 获取最后接受的区块
  - `getBlock` - 获取特定区块

- **static_handlers.rs**: 实现 VM 特定的静态处理程序，通过 `/ext/vm/[VM ID]/static` 路径提供服务
  - `ping` - 检查 VM 是否正常运行

### block 模块

实现了 `snowman.Block` 接口，定义了区块的结构和操作：

- 区块结构包含：父区块 ID、高度、时间戳、数据、状态等
- 提供区块验证、接受和拒绝的方法
- 实现区块序列化和反序列化

### client 模块

提供与 timestampvm API 交互的客户端实现：

- `ping` - 发送 ping 请求
- `propose_block` - 提议新区块
- `last_accepted` - 获取最后接受的区块
- `get_block` - 获取特定区块

### genesis 模块

定义了 timestampvm 的创世区块：

- 包含创世数据的结构
- 提供序列化和反序列化方法
- 支持将创世信息持久化到文件

### state 模块

管理虚拟机的状态：

- 维护区块数据库
- 跟踪已验证但尚未接受/拒绝的区块
- 提供区块读写和状态查询方法

### vm 模块

实现了 `snowman.block.ChainVM` 接口：

- 管理 VM 特定状态
- 处理区块构建和验证
- 实现共识引擎接口
- 提供区块提议和通知功能

## 设计模式

该库采用了以下设计模式：

1. **模块化设计** - 将功能分解为独立的模块，每个模块负责特定功能
2. **接口实现** - 实现 Avalanche 网络定义的标准接口
3. **RPC 服务** - 使用 JSON-RPC 提供 API 服务
4. **状态管理** - 使用专用模块管理 VM 状态
5. **异步编程** - 使用 Tokio 提供异步运行时支持

## 使用示例

```rust
use avalanche_types::subnet;
use timestampvm::vm;
use tokio::sync::broadcast::{self, Receiver, Sender};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let (stop_ch_tx, stop_ch_rx): (Sender<()>, Receiver<()>) = broadcast::channel(1);
    let vm_server = subnet::rpc::vm::server::Server::new(vm::Vm::new(), stop_ch_tx);
    subnet::rpc::plugin::serve(vm_server, stop_ch_rx).await
}
```

## 依赖项

主要依赖包括：

- `avalanche-types` - Avalanche 网络的 Rust 类型定义
- `tokio` - 异步运行时
- `serde` - 序列化和反序列化
- `jsonrpc-core` - JSON-RPC 实现
- `clap` - 命令行参数解析
