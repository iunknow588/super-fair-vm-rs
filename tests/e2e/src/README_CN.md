# FairVM 端到端测试

本目录包含 FairVM 端到端测试的源代码。

## 目录结构

```
.
├── tests/           # 测试用例
│   ├── mod.rs      # 测试模块定义
│   └── README_CN.md # 测试说明文档
└── main.rs         # 测试入口
```

## 测试内容

- 虚拟机初始化
- 区块构建和验证
- 状态管理
- API 服务
- 客户端交互
- 创世区块处理

## 环境要求

- Rust 1.70.0 或更高版本
- [protoc](https://grpc.io/docs/protoc-installation/) 3.15.0 或更高版本
- AvalancheGo 节点

## 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_name

# 运行端到端测试
./scripts/tests.e2e.sh
```

## 注意事项

- 确保所有依赖都正确安装
- 建议在测试环境中运行
- 测试可能需要较长时间
- 确保有足够的系统资源
