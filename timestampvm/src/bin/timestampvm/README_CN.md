# timestampvm 可执行文件

本目录包含 timestampvm 可执行文件的实现，用于作为 AvalancheGo 的插件运行，或生成创世区块和 VM ID。

## 文件结构

- `main.rs` - 主程序入口点，处理命令行参数和启动 VM
- `genesis.rs` - 创世区块命令实现
- `vm_id.rs` - VM ID 转换命令实现

## 功能概述

### main.rs

主程序入口点，提供以下功能：

- 解析命令行参数
- 初始化日志记录
- 处理子命令（genesis、vm-id）
- 启动 VM 服务器

### genesis.rs

实现创世区块命令，用于生成 timestampvm 的创世区块：

- 定义命令行参数
- 创建创世区块结构
- 输出 JSON 格式的创世区块

### vm_id.rs

实现 VM ID 转换命令，用于将 VM 名称转换为 VM ID：

- 定义命令行参数
- 使用 Avalanche 的哈希算法计算 VM ID
- 输出 VM ID

## 命令行用法

### 启动 VM

```bash
timestampvm
```

这将启动 VM 服务器，等待 AvalancheGo 的连接。

### 生成创世区块

```bash
timestampvm genesis "Custom genesis data"
```

这将输出 JSON 格式的创世区块。

### 获取 VM ID

```bash
timestampvm vm-id timestampvm
```

这将输出 VM 名称对应的 VM ID。

## 与 AvalancheGo 集成

当作为 AvalancheGo 的插件运行时，timestampvm 可执行文件：

1. 由 AvalancheGo 作为子进程启动
2. 通过 gRPC 与 AvalancheGo 通信
3. 实现 Avalanche 的 ChainVM 接口
4. 处理区块提议和验证
