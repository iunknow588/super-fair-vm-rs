# 测试目录

本目录包含项目的测试用例，用于验证系统的功能和性能。

## 目录结构
```
tests/
├── e2e/          # 端到端测试
└── unit/         # 单元测试
```

## 测试类型

### 1. 端到端测试 (e2e/)
- 完整的系统集成测试
- 模拟真实用户场景
- 验证系统整体功能
- 测试网络交互
- 性能基准测试

### 2. 单元测试 (unit/)
- 模块级别的测试
- 功能验证
- 边界条件测试
- 错误处理测试

## 测试规范

### 1. 命名规范
- 测试文件以`_test.rs`结尾
- 测试函数以`test_`开头
- 测试模块以`tests`命名

### 2. 测试结构
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        // 测试代码
    }
}
```

### 3. 异步测试
```rust
#[tokio::test]
async fn test_async_function() {
    // 异步测试代码
}
```

## 运行测试

### 1. 运行所有测试
```bash
cargo test
```

### 2. 运行特定测试
```bash
cargo test test_name
```

### 3. 运行端到端测试
```bash
cargo test --test e2e
```

### 4. 显示测试输出
```bash
cargo test -- --nocapture
```

## 测试覆盖率

### 1. 安装工具
```bash
cargo install cargo-tarpaulin
```

### 2. 生成覆盖率报告
```bash
cargo tarpaulin
```

## 注意事项

1. 测试数据
   - 使用测试夹具
   - 避免硬编码数据
   - 清理测试环境

2. 测试隔离
   - 每个测试独立运行
   - 避免测试间依赖
   - 使用测试数据库

3. 性能考虑
   - 避免不必要的等待
   - 合理设置超时
   - 控制资源使用

4. 错误处理
   - 测试错误情况
   - 验证错误信息
   - 检查错误恢复 