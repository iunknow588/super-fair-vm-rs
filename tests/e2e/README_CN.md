# 端到端测试

本目录包含 FairVM 项目的端到端测试，用于验证 VM 与 Avalanche 网络的集成。

## 目录结构

```
tests/e2e/
├── Cargo.toml
├── README_CN.md
└── src/
    ├── lib.rs
    └── tests/
        ├── mod.rs
        └── README_CN.md
```

## 测试内容

1. **网络启动** - 启动包含 FairVM 的 Avalanche 网络
2. **API测试** - 测试 FairVM 的 RPC API
3. **区块操作** - 测试区块的创建、验证和接受
4. **状态管理** - 测试状态同步和持久化
5. **错误处理** - 测试各种错误情况的处理

## 运行测试

### 环境要求

- Rust 1.70+
- AvalancheGo
- protoc >= 3.15.0

### 运行命令

```bash
# 构建 FairVM 插件并运行端到端测试
cargo build --release \
&& VM_PLUGIN_PATH=$(pwd)/target/release/fairvm \
./scripts/tests.e2e.sh
```

### 环境变量

- `VM_PLUGIN_PATH` - FairVM 插件的路径
- `AVALANCHEGO_PATH` - AvalancheGo 可执行文件的路径
- `NETWORK_RUNNER_GRPC_ENDPOINT` - 网络运行器的 gRPC 端点
- `NETWORK_RUNNER_ENABLE_SHUTDOWN` - 是否在测试后关闭网络

## 测试组件

- `AvalancheGo` - Avalanche 网络节点
- `FairVM` - 被测试的 VM 实现
- `Network Runner` - 网络运行器，用于管理测试网络

## 注意事项

1. 确保所有依赖都已正确安装
2. 测试可能需要较长时间运行
3. 某些测试可能需要特定的网络配置
4. 建议在测试环境中运行，避免影响生产环境
