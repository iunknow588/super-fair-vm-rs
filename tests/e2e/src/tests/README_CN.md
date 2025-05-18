# FairVM 端到端测试用例

本目录包含 FairVM 的端到端测试用例。

## 测试内容

`mod.rs` 文件实现了一个完整的端到端测试，验证 FairVM 与 Avalanche 网络的集成：

1. 初始化测试环境
   - 启动 AvalancheGo 节点
   - 配置网络参数
   - 准备测试数据

2. 虚拟机测试
   - 验证虚拟机初始化
   - 测试区块构建
   - 检查状态管理
   - 验证 API 服务

3. 客户端测试
   - 测试 RPC 调用
   - 验证区块提交
   - 检查状态同步

4. 清理测试环境
   - 关闭网络连接
   - 清理测试数据
   - 验证清理结果

## 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_name

# 运行端到端测试
cargo build --release \
&& VM_PLUGIN_PATH=$(pwd)/target/release/fairvm \
    ./scripts/tests.e2e.sh
```

## 注意事项

- 确保所有依赖都正确安装
- 建议在测试环境中运行
- 测试可能需要较长时间
- 确保有足够的系统资源
- 检查网络连接状态
