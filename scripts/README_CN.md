# 脚本目录

本目录包含用于构建、测试和运行 `timestampvm-rs` 项目的脚本。

## 脚本列表

### `build.release.sh`

构建 timestampvm 的发布版本。

```bash
./scripts/build.release.sh
```

这个脚本执行以下操作：
- 使用 `cargo build --release` 构建项目
- 输出构建的二进制文件路径

### `tests.e2e.sh`

运行端到端测试，验证 VM 与 Avalanche 网络的集成。

```bash
VM_PLUGIN_PATH=$(pwd)/target/release/timestampvm ./scripts/tests.e2e.sh
```

这个脚本执行以下操作：
- 启动 network-runner 服务
- 运行端到端测试
- 可选择在测试后保持网络运行

### `tests.lint.sh`

运行代码静态分析检查。

```bash
./scripts/tests.lint.sh
```

这个脚本执行以下操作：
- 运行 `cargo clippy` 进行代码质量检查
- 运行 `cargo fmt` 检查代码格式

### `tests.unit.sh`

运行单元测试。

```bash
./scripts/tests.unit.sh
```

这个脚本执行以下操作：
- 运行 `cargo test` 执行项目中的单元测试

### `tests.unused.sh`

检查项目中未使用的依赖项。

```bash
./scripts/tests.unused.sh
```

这个脚本执行以下操作：
- 使用 `cargo-udeps` 工具检查未使用的依赖项

## 环境变量

脚本使用以下环境变量：

- `VM_PLUGIN_PATH` - timestampvm 插件的路径（用于端到端测试）
- `AVALANCHEGO_PATH` - AvalancheGo 可执行文件的路径（可选，用于端到端测试）
- `NETWORK_RUNNER_GRPC_ENDPOINT` - network-runner 的 gRPC 端点（用于端到端测试）
- `NETWORK_RUNNER_ENABLE_SHUTDOWN` - 测试后是否关闭网络（用于端到端测试）
