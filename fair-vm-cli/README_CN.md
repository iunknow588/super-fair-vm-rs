# fairvm-cli

本目录为 FairVM 的命令行工具，提供与 FairVM 节点交互、管理和调试的 CLI 功能。

## 目录结构
- `Cargo.toml`：项目依赖和元数据配置文件。
- `src/`：CLI 源码目录。
  - `main.rs`：命令行主入口，实现参数解析、命令分发、与节点的交互逻辑。
  - `commands/`：命令实现目录
    - `node.rs`：节点管理命令
    - `account.rs`：账户管理命令
    - `transaction.rs`：交易管理命令
    - `contract.rs`：合约管理命令
    - `block.rs`：区块查询命令
    - `wallet.rs`：钱包管理命令

## 主要功能

### 1. 节点管理
- 启动节点
- 停止节点
- 查看节点状态
- 配置节点参数

### 2. 账户管理
- 创建账户
- 导入账户
- 查询余额
- 转账操作

### 3. 交易管理
- 发送交易
- 查询交易
- 交易签名
- 交易广播

### 4. 合约管理
- 部署合约
- 调用合约
- 查询合约
- 合约事件

### 5. 区块查询
- 查询区块
- 查询交易
- 查询状态
- 查询日志

### 6. 钱包管理
- 创建钱包
- 导入钱包
- 导出钱包
- 管理密钥

## 使用示例

### 1. 启动节点
```bash
fairvm-cli node start --config config.json
```

### 2. 创建账户
```bash
fairvm-cli account create --name myaccount
```

### 3. 发送交易
```bash
fairvm-cli transaction send --from myaccount --to 0x123... --value 1.0
```

### 4. 部署合约
```bash
fairvm-cli contract deploy --from myaccount --file contract.sol
```

## 设计模式
- **命令模式**：不同功能通过子命令实现，便于扩展和维护。
- **模块化设计**：各命令逻辑独立，主入口统一调度。
- **配置管理**：支持配置文件和环境变量。
- **错误处理**：统一的错误处理和提示。

## 开发指南

### 1. 环境要求
- Rust 1.70+
- Cargo
- Git

### 2. 构建步骤
```bash
cargo build
```

### 3. 运行测试
```bash
cargo test
```

## 适用场景
- 节点运维
- 开发调试
- 链上数据查询
- 批量操作
- 自动化脚本
- 系统监控

## 注意事项
1. 妥善保管私钥和密码
2. 注意网络连接状态
3. 合理设置超时时间
4. 定期备份重要数据 