# timestampvm 源代码

本目录包含 timestampvm 的源代码，实现了一个最小化的 Avalanche 自定义虚拟机 (VM)。

## 文件结构

- `lib.rs` - 库入口点，定义了 VM 的主要组件和结构
- `api/` - 实现 timestampvm API
- `bin/` - 命令行界面和插件服务器实现
- `block/` - 实现 `snowman.Block` 接口
- `client/` - 实现 timestampvm API 客户端
- `genesis/` - 定义 timestampvm 创世区块
- `state/` - 管理虚拟机状态
- `vm/` - 实现 `snowman.block.ChainVM` 接口

## 主要组件

### lib.rs

库入口点，定义了 VM 的主要组件和结构：

- 导出所有模块
- 提供库文档
- 设置代码质量检查

### api 模块

实现 timestampvm 的 API 服务：

- 链特定的 RPC 处理程序
- VM 特定的静态处理程序

### bin 目录

包含命令行界面和插件服务器实现：

- 主程序入口点
- 创世区块命令
- VM ID 转换命令

### block 模块

实现 `snowman.Block` 接口：

- 区块结构和操作
- 区块验证、接受和拒绝

### client 模块

实现 timestampvm API 客户端：

- API 调用函数
- 响应结构

### genesis 模块

定义 timestampvm 创世区块：

- 创世结构
- 序列化和持久化

### state 模块

管理虚拟机状态：

- 区块存储和检索
- 链状态维护

### vm 模块

实现 `snowman.block.ChainVM` 接口：

- VM 状态管理
- 区块构建
- 共识引擎接口

## 设计模式

该库采用了以下设计模式：

1. **模块化设计** - 将功能分解为独立的模块
2. **接口实现** - 实现 Avalanche 网络定义的标准接口
3. **RPC 服务** - 使用 JSON-RPC 提供 API 服务
4. **状态管理** - 使用专用模块管理 VM 状态
5. **异步编程** - 使用 Tokio 提供异步运行时支持
