# API 模块

本模块实现了 timestampvm 的 API 服务，包括链特定的 RPC 处理程序和 VM 特定的静态处理程序。

## 文件结构

- `mod.rs` - 模块入口点，定义通用结构和函数
- `chain_handlers.rs` - 实现链特定的 RPC 处理程序
- `static_handlers.rs` - 实现 VM 特定的静态处理程序

## 功能概述

### mod.rs

定义了 API 模块的通用结构和函数：

- `PingResponse` - ping 请求的响应结构
- `de_request` - 解析 HTTP 请求为 JSON-RPC 请求

### chain_handlers.rs

实现链特定的 RPC 处理程序，通过 `/ext/bc/[CHAIN ID]/rpc` 路径提供服务：

- `Rpc` trait - 定义链特定的 RPC 方法
  - `ping` - 检查 VM 是否正常运行
  - `proposeBlock` - 提议新区块
  - `lastAccepted` - 获取最后接受的区块
  - `getBlock` - 获取特定区块

- `ChainService` - 实现 `Rpc` trait 的服务
- `ChainHandler` - 处理 HTTP 请求并路由到相应的 RPC 方法

### static_handlers.rs

实现 VM 特定的静态处理程序，通过 `/ext/vm/[VM ID]/static` 路径提供服务：

- `Rpc` trait - 定义静态 RPC 方法
  - `ping` - 检查 VM 是否正常运行

- `StaticService` - 实现 `Rpc` trait 的服务
- `StaticHandler` - 处理 HTTP 请求并路由到相应的 RPC 方法

## 设计模式

该模块采用了以下设计模式：

1. **接口分离原则** - 将链处理程序和静态处理程序分开
2. **依赖注入** - 通过构造函数注入 VM 实例
3. **装饰器模式** - 使用 JSON-RPC 装饰器扩展基本功能
4. **异步编程** - 使用 `BoxFuture` 提供异步 RPC 处理

## 使用示例

```rust
// 创建链处理程序
let chain_service = ChainService::new(vm.clone());
let chain_handler = ChainHandler::new(chain_service);

// 创建静态处理程序
let static_service = StaticService::new();
let static_handler = StaticHandler::new(static_service);

// 注册处理程序
let mut handlers = HashMap::new();
handlers.insert(
    "/rpc".to_string(),
    HttpHandler {
        lock_option: LockOptions::WriteLock,
        handler: chain_handler,
        server_addr: None,
    },
);
```
