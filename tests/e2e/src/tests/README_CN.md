# 端到端测试用例

本目录包含 timestampvm 的端到端测试用例。

## 文件结构

- `mod.rs` - 端到端测试实现

## 测试内容

`mod.rs` 文件实现了一个完整的端到端测试，验证 timestampvm 与 Avalanche 网络的集成：

1. **网络启动**
   - 连接到 network-runner
   - 准备 VM 插件和 AvalancheGo
   - 创建随机创世文件
   - 启动 Avalanche 网络

2. **健康检查**
   - 等待网络就绪
   - 检查集群状态
   - 获取 RPC 端点和区块链 ID

3. **API 测试**
   - 测试链处理程序的 ping 功能
   - 获取最后接受的区块
   - 获取区块详情
   - 提议新区块
   - 验证区块高度增加
   - 测试超出限制的区块提议

4. **网络关闭**
   - 可选择关闭网络或保持运行

## 测试流程

1. 启动 network-runner 服务
2. 复制 VM 插件到 AvalancheGo 插件目录
3. 创建随机创世文件
4. 启动 AvalancheGo 集群
5. 等待集群健康
6. 检查集群状态
7. 执行 API 测试
8. 可选择关闭网络

## 使用方法

可以通过以下命令运行端到端测试：

```bash
cargo test --package e2e --lib -- tests::e2e --exact --nocapture
```

或者使用提供的脚本：

```bash
./scripts/build.release.sh \
&& VM_PLUGIN_PATH=$(pwd)/target/release/timestampvm \
./scripts/tests.e2e.sh
```
