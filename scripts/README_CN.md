# 脚本目录

本目录包含用于构建、测试和运行 FairVM 项目的脚本。

## 脚本列表

- `build.release.sh` - 构建 FairVM 的发布版本
- `tests.lint.sh` - 运行代码格式检查和静态分析
- `tests.unit.sh` - 运行单元测试
- `tests.e2e.sh` - 运行端到端测试
- `tests.unused.sh` - 检查未使用的依赖

## 使用方法

### 构建发布版本

```bash
./scripts/build.release.sh
```

### 运行测试

```bash
# 运行代码检查
./scripts/tests.lint.sh

# 运行单元测试
./scripts/tests.unit.sh

# 运行端到端测试
VM_PLUGIN_PATH=$(pwd)/target/release/fairvm ./scripts/tests.e2e.sh
```

## 环境变量

- `VM_PLUGIN_PATH` - FairVM 插件的路径
- `AVALANCHEGO_PATH` - AvalancheGo 可执行文件的路径
- `NETWORK_RUNNER_GRPC_ENDPOINT` - 网络运行器的 gRPC 端点
- `NETWORK_RUNNER_ENABLE_SHUTDOWN` - 是否在测试后关闭网络

## 注意事项

1. 确保所有依赖都已正确安装
2. 脚本需要执行权限
3. 某些脚本可能需要特定的环境变量
4. 建议在测试环境中运行
