# Client 模块

本模块实现了与 timestampvm API 交互的客户端，提供了调用 VM 各种 RPC 方法的函数。

## 文件结构

- `mod.rs` - 模块入口点，定义客户端函数和结构

## 主要功能

### RPC 客户端函数

- `ping` - 发送 ping 请求，检查 VM 是否正常运行
- `propose_block` - 提议新区块，将数据提交到 VM
- `last_accepted` - 获取最后接受的区块 ID
- `get_block` - 获取特定区块的详细信息

### 响应结构

- `PingResponse` - ping 请求的响应
- `ProposeBlockResponse` - 提议区块的响应
- `LastAcceptedResponse` - 获取最后接受区块的响应
- `GetBlockResponse` - 获取区块的响应

## 实现细节

客户端使用 HTTP JSON-RPC 与 VM 通信：

1. 构造 JSON-RPC 请求
2. 发送 HTTP POST 请求到指定端点
3. 解析 JSON-RPC 响应
4. 返回类型化的响应结构

所有客户端函数都是异步的，返回 `Result` 类型，支持错误处理。

## 使用示例

```rust
// 发送 ping 请求
let resp = timestampvm::client::ping("http://127.0.0.1:9650", "/ext/bc/my-chain-id/rpc")
    .await
    .unwrap();
assert!(resp.result.unwrap().success);

// 提议新区块
let data = vec![0, 1, 2]; // 示例数据
let resp = timestampvm::client::propose_block(
    "http://127.0.0.1:9650", 
    "/ext/bc/my-chain-id/rpc", 
    data
)
.await
.unwrap();
assert!(resp.result.unwrap().success);

// 获取最后接受的区块
let resp = timestampvm::client::last_accepted(
    "http://127.0.0.1:9650", 
    "/ext/bc/my-chain-id/rpc"
)
.await
.unwrap();
let block_id = resp.result.unwrap().id;

// 获取区块详情
let resp = timestampvm::client::get_block(
    "http://127.0.0.1:9650", 
    "/ext/bc/my-chain-id/rpc", 
    &block_id
)
.await
.unwrap();
let block = resp.result.unwrap().block;
```

## 错误处理

客户端函数返回 `Result` 类型，可能的错误包括：

- 网络错误
- JSON 解析错误
- VM 返回的错误响应

示例错误处理：

```rust
match timestampvm::client::ping("http://127.0.0.1:9650", "/ext/bc/my-chain-id/rpc").await {
    Ok(resp) => {
        if let Some(result) = resp.result {
            println!("Ping success: {}", result.success);
        } else if let Some(error) = resp.error {
            println!("VM returned error: {}", error.message);
        }
    },
    Err(e) => {
        println!("Client error: {e}");
    }
}
```
