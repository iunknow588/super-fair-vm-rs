# FairVM手续费和Gas限制

本文档介绍FairVM中手续费和Gas限制的实现和配置方法。

## 概述

FairVM实现了可配置的手续费系统和Gas限制，支持：

1. EIP-1559风格的基础费用机制
2. 可配置的区块和交易Gas限制
3. 通过Genesis配置进行初始设置

## 费用机制

### EIP-1559支持

FairVM支持以太坊的EIP-1559费用机制，包括：

- 动态基础费用(base fee)，根据区块使用情况自动调整
- 优先费用(priority fee或tip)，作为矿工的额外奖励
- 最大费用(max fee)，设置交易愿意支付的最高燃料价格

### 费用类型

在交易中，支持以下费用类型：

- **传统交易(Legacy)**：使用固定的`gas_price`
- **EIP-1559交易(DynamicFee)**：使用`max_fee_per_gas`和`max_priority_fee_per_gas`

### 基础费用调整

基础费用根据以下规则自动调整：

- 如果区块Gas使用量超过目标值，基础费用增加
- 如果区块Gas使用量低于目标值，基础费用减少
- 调整幅度由`base_fee_change_denominator`控制

## Gas限制

FairVM支持以下可配置的Gas限制：

1. **区块Gas限制**：单个区块可使用的最大Gas总量
2. **交易Gas限制**：单个交易可使用的最大Gas量
3. **合约创建Gas限制**：创建新合约时可使用的最大Gas量

## 配置方法

### Genesis配置

在Genesis配置文件中设置初始手续费和Gas限制：

```json
{
    "chain_id": 2023,
    "gas_limit": {
        "block": 15000000,
        "contract_creation": 8000000,
        "tx": 8000000
    },
    "fees": {
        "gas_price_minimum": "0x3B9ACA00",
        "gas_target": 8000000,
        "enable_1559": true,
        "base_fee_change_denominator": 8
    }
}
```

### 配置参数说明

#### Gas限制配置

- `block`：区块Gas限制，默认15,000,000
- `contract_creation`：合约创建Gas限制，默认8,000,000
- `tx`：交易Gas限制，默认8,000,000

#### 费用配置

- `gas_price_minimum`：最低Gas价格，默认1 GWei (1,000,000,000 Wei)
- `gas_target`：目标区块Gas使用量，用于调整基础费用，默认8,000,000
- `enable_1559`：是否启用EIP-1559费用机制，默认`true`
- `base_fee_change_denominator`：基础费用变化分母，值越大变化越慢，默认8

## 代码API

### VM方法

获取当前费用和Gas配置：

```rust
// 获取当前区块Gas限制
pub fn get_block_gas_limit(&self) -> u64

// 获取当前基础费用
pub fn get_base_fee(&self) -> U256

// 获取当前费用配置
pub fn get_fees_config(&self) -> &FeesConfig

// 获取当前Gas限制配置
pub fn get_gas_limit_config(&self) -> &GasLimitConfig
```

### 交易验证

交易提交时会验证以下内容：

1. 交易签名是否有效
2. Gas价格是否满足最低要求
3. Gas限制是否超过配置的最大值

对于EIP-1559交易，还会验证：

1. `max_fee_per_gas`是否大于或等于当前区块的`base_fee`
2. `max_priority_fee_per_gas`是否小于或等于`max_fee_per_gas`

## 使用方法

### 创建自定义配置的VM

```rust
// 创建带有自定义配置的VM
let config = VMConfig::default();
let vm = VM::with_config(None, config);

// 或直接从Genesis配置初始化
let genesis_bytes = std::fs::read("genesis.json")?;
let genesis = parse_genesis(&genesis_bytes)?;
let mut vm = VM::default();
vm.initialize_from_genesis(genesis).await?;
```

### 创建交易

#### 传统交易

```rust
let tx = Transaction {
    hash: H256([0; 32]),
    from: sender_address,
    to: Some(receiver_address),
    value: U256::from(1000000000000000000u64), // 1 ETH
    nonce: 0,
    gas_limit: 21000,
    gas_price: Some(U256::from(2000000000u64)), // 2 GWei
    max_fee_per_gas: None,
    max_priority_fee_per_gas: None,
    data: Vec::new(),
    signature: Vec::new(), // 需要正确签名
    transaction_type: TransactionType::Legacy,
    chain_id: 2023,
};
```

#### EIP-1559交易

```rust
let tx = Transaction {
    hash: H256([0; 32]),
    from: sender_address,
    to: Some(receiver_address),
    value: U256::from(1000000000000000000u64), // 1 ETH
    nonce: 0,
    gas_limit: 21000,
    gas_price: None,
    max_fee_per_gas: Some(U256::from(3000000000u64)), // 最大3 GWei
    max_priority_fee_per_gas: Some(U256::from(1000000000u64)), // 优先费用1 GWei
    data: Vec::new(),
    signature: Vec::new(), // 需要正确签名
    transaction_type: TransactionType::DynamicFee,
    chain_id: 2023,
};
``` 