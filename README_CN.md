# `timestampvm-rs`

[<img alt="crates.io" src="https://img.shields.io/crates/v/timestampvm.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/timestampvm)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-timestampvm-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/timestampvm)
![Github Actions](https://github.com/ava-labs/timestampvm-rs/actions/workflows/test-and-release.yml/badge.svg)

`timestampvm-rs` 是一个虚拟机，可以从用户提供的任意数据构建区块。它是使用 Avalanche [Rust SDK](https://github.com/ava-labs/avalanche-types-rs) 实现的最小化 Avalanche 自定义虚拟机 (VM)。

## 项目概述

当前，Avalanche 自定义 VM 需要满足以下要求：

1. 编译为 `avalanchego` 可以作为子进程启动的二进制文件
2. 插件二进制路径的哈希为 32 字节
3. 实现 [`snowman.block.ChainVM`](https://pkg.go.dev/github.com/ava-labs/avalanchego/snow/engine/snowman/block#ChainVM) 接口，可以通过 [`rpcchainvm.Serve`](https://pkg.go.dev/github.com/ava-labs/avalanchego/vms/rpcchainvm#Serve) 注册
4. 实现可通过区块链 ID 的 URL 路径提供服务的 VM 特定服务
5. （可选）实现可通过 VM ID 的 URL 路径提供服务的 VM 特定静态处理程序

## 项目结构

项目采用工作区结构，包含以下主要组件：

### 根目录文件

- `Cargo.toml` - 工作区配置文件，定义了项目成员和解析器设置
- `Cargo.lock` - 依赖锁定文件，确保构建的一致性
- `README.md` - 项目主要文档（英文版）
- `.github/` - GitHub Actions 工作流配置和 Dependabot 设置

### 脚本目录 (`scripts/`)

- `build.release.sh` - 构建发布版本的脚本
- `tests.lint.sh` - 运行静态分析测试的脚本
- `tests.unit.sh` - 运行单元测试的脚本
- `tests.e2e.sh` - 运行端到端测试的脚本

### 主要代码库 (`timestampvm/`)

- `src/lib.rs` - 库入口点，定义了 VM 的主要组件和结构
- `src/bin/timestampvm/` - 命令行界面和插件服务器实现
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

### 测试目录 (`tests/e2e/`)

- 包含端到端测试，验证 VM 与 Avalanche 网络的集成

## 设计模式

该项目采用了以下设计模式：

1. **模块化设计** - 将功能分解为独立的模块，每个模块负责特定功能
2. **接口实现** - 实现 Avalanche 网络定义的标准接口
3. **RPC 服务** - 使用 JSON-RPC 提供 API 服务
4. **状态管理** - 使用专用模块管理 VM 状态
5. **命令模式** - 在 CLI 中使用命令模式处理不同的操作

## 使用示例

以下是使用 timestampvm 的简单示例：

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

## 构建和测试

# 代码格式化处理
```bash
./scripts/tests.lint.sh
```


```bash
# 构建 timestampvm 插件，运行 e2e 测试，并保持网络运行
./scripts/build.release.sh \
&& VM_PLUGIN_PATH=$(pwd)/target/release/timestampvm \
./scripts/tests.e2e.sh
```

## API 测试示例

```bash
# 测试 ping API
curl -X POST --data '{
    "jsonrpc": "2.0",
    "id"     : 1,
    "method" : "timestampvm.ping",
    "params" : []
}' -H 'content-type:application/json;' 127.0.0.1:9650/ext/vm/tGas3T58KzdjcJ2iKSyiYsWiqYctRXaPTqBCA11BqEkNg8kPc/static

# 提案新区块
curl -X POST --data '{
    "jsonrpc": "2.0",
    "id"     : 1,
    "method" : "timestampvm.proposeBlock",
    "params" : [{"data":"MQo="}]
}' -H 'content-type:application/json;' 127.0.0.1:9650/ext/bc/2wb1UXxAstB8ywwv4rU2rFCjLgXnhT44hbLPbwpQoGvFb2wRR7/rpc
```

## 依赖项

- 最新版本的稳定 Rust
- 构建和测试 timestampvm 需要 
[protoc](https://grpc.io/docs/protoc-installation/#install-pre-compiled-binaries-any-os) 版本 >= 3.15.0

## AvalancheGo 兼容性

| 版本 | AvalancheGo 版本 |
| --- | --- |
| v0.0.6  | v1.9.2,v1.9.3 |
| v0.0.7  | v1.9.4 |
| v0.0.8, v0.0.9  | v1.9.7 |
| v0.0.10 | v1.9.8, v1.9.9 |
| v0.0.11,12 | v1.9.10 - v1.9.16 |
| v0.0.13 | v1.10.0 |
| v0.0.14..17 | v1.10.1 |
| v0.0.18 | v1.10.9+ |