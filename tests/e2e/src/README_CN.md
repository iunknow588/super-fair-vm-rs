# 端到端测试源代码

本目录包含 timestampvm 端到端测试的源代码。

## 文件结构

- `lib.rs` - 测试库入口点，提供辅助函数
- `tests/` - 包含测试用例
  - `mod.rs` - 端到端测试实现

## 功能概述

### lib.rs

提供端到端测试的辅助函数：

- `get_network_runner_grpc_endpoint` - 获取 network-runner 的 gRPC 端点
- `get_vm_plugin_path` - 获取 VM 插件的路径
- `get_avalanchego_path` - 获取 AvalancheGo 可执行文件的路径
- `get_network_runner_enable_shutdown` - 检查是否应在测试后关闭网络

这些函数从环境变量中读取配置，使测试更加灵活和可配置。
