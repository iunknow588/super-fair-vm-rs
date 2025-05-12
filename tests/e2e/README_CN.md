# 端到端测试

本目录包含 `timestampvm-rs` 项目的端到端测试，用于验证 VM 与 Avalanche 网络的集成。

## 目录结构

- `src/` - 测试源代码
  - `lib.rs` - 测试库入口点，提供辅助函数
  - `tests/mod.rs` - 包含端到端测试用例

## 测试内容

端到端测试验证了以下功能：

1. **网络启动** - 启动包含 timestampvm 的 Avalanche 网络
2. **VM 插件加载** - 验证 VM 插件正确加载
3. **API 调用** - 测试 VM 的 API 功能：
   - `ping` - 检查 VM 是否正常运行
   - `lastAccepted` - 获取最后接受的区块
   - `getBlock` - 获取特定区块
   - `proposeBlock` - 提议新区块
4. **错误处理** - 测试超出限制的区块提议

## 测试流程

1. 启动 network-runner 服务
2. 复制 VM 插件到 AvalancheGo 插件目录
3. 创建随机创世文件
4. 启动 AvalancheGo 集群
5. 检查集群健康状态
6. 执行 API 测试
7. 可选择关闭网络

## 运行测试

可以使用以下命令运行端到端测试：

```bash
# 构建 timestampvm 插件并运行端到端测试
./scripts/build.release.sh \
&& VM_PLUGIN_PATH=$(pwd)/target/release/timestampvm \
./scripts/tests.e2e.sh
```

## 环境变量

测试使用以下环境变量：

- `VM_PLUGIN_PATH` - timestampvm 插件的路径
- `AVALANCHEGO_PATH` - AvalancheGo 可执行文件的路径（可选）
- `NETWORK_RUNNER_GRPC_ENDPOINT` - network-runner 的 gRPC 端点
- `NETWORK_RUNNER_ENABLE_SHUTDOWN` - 测试后是否关闭网络

## 依赖项

- `avalanche-network-runner-sdk` - 与 Avalanche 网络运行器交互
- `avalanche-types` - Avalanche 网络的 Rust 类型定义
- `timestampvm` - 被测试的 VM 实现
