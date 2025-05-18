use crate::core::types::U256;
use crate::core::vm::stack::StackError;

/// EVM执行错误
#[derive(Debug, thiserror::Error)]
pub enum EvmError {
    /// 栈相关错误
    #[error("栈错误: {0}")]
    Stack(#[from] StackError),

    /// 内存错误
    #[error("内存错误: {0}")]
    Memory(String),

    /// gas不足
    #[error("gas不足: 需要 {required} gas, 剩余 {remaining} gas")]
    OutOfGas { required: u64, remaining: u64 },

    /// 无效的操作码
    #[error("无效的操作码: 0x{0:02x}")]
    InvalidOpcode(u8),

    /// 无效的跳转目标
    #[error("无效的跳转目标: {0}")]
    InvalidJumpdest(U256),

    /// 执行被还原
    #[error("执行被还原: {0}")]
    Reverted(String),

    /// 合约创建失败
    #[error("合约创建失败: {0}")]
    ContractCreationFailed(String),

    /// 只读调用修改状态
    #[error("静态调用中的状态修改")]
    StaticCallStateChange,

    /// 执行深度过深
    #[error("调用深度超过限制")]
    CallDepthExceeded,

    /// 算术错误
    #[error("算术错误: {0}")]
    Arithmetic(String),

    /// 存储访问错误
    #[error("存储访问错误: {0}")]
    Storage(String),

    /// 余额不足
    #[error("余额不足: 需要 {required}, 可用 {available}")]
    InsufficientBalance { required: U256, available: U256 },

    /// 不支持的操作
    #[error("不支持的操作: {0}")]
    Unsupported(String),

    /// 交易错误
    #[error("交易错误: {0}")]
    Transaction(String),

    /// 公平性错误
    #[error("公平性错误: {0}")]
    Fairness(String),

    /// 其他错误
    #[error("EVM错误: {0}")]
    Other(String),
}

impl EvmError {
    /// 从错误创建失败的执行结果
    pub fn into_result(self) -> crate::core::vm::ExecutionResult {
        crate::core::vm::ExecutionResult {
            success: false,
            gas_used: 0,
            return_data: Vec::new(),
            error: Some(self.to_string()),
            fairness_score: 0,
        }
    }

    /// 从错误消息创建失败的执行结果
    pub fn result_from_str(msg: impl Into<String>) -> crate::core::vm::ExecutionResult {
        crate::core::vm::ExecutionResult {
            success: false,
            gas_used: 0,
            return_data: Vec::new(),
            error: Some(msg.into()),
            fairness_score: 0,
        }
    }

    /// 从字符串创建其他错误
    pub fn other<S: Into<String>>(msg: S) -> Self {
        EvmError::Other(msg.into())
    }

    /// 从字符串创建公平性错误
    pub fn fairness<S: Into<String>>(msg: S) -> Self {
        EvmError::Fairness(msg.into())
    }
}

/// 使用字符串创建其他错误的简便方法
impl From<String> for EvmError {
    fn from(s: String) -> Self {
        EvmError::Other(s)
    }
}

/// 使用字符串切片创建其他错误的简便方法
impl From<&str> for EvmError {
    fn from(s: &str) -> Self {
        EvmError::Other(s.to_string())
    }
}

/// 状态访问相关错误
#[derive(Debug, thiserror::Error)]
pub enum StateError {
    /// 账户不存在
    #[error("账户不存在: {0:?}")]
    AccountNotFound(crate::core::types::Address),

    /// 代码不存在
    #[error("代码不存在: {0:?}")]
    CodeNotFound(crate::core::types::Address),

    /// 访问被拒绝
    #[error("访问被拒绝: {0}")]
    AccessDenied(String),

    /// 数据库错误
    #[error("数据库错误: {0}")]
    Database(String),

    /// 公平性错误
    #[error("公平性错误: {0}")]
    Fairness(String),

    /// 其他错误
    #[error("状态错误: {0}")]
    Other(String),
}

impl From<StateError> for EvmError {
    fn from(err: StateError) -> Self {
        match err {
            StateError::AccountNotFound(addr) => EvmError::Other(format!("账户不存在: {:?}", addr)),
            StateError::CodeNotFound(addr) => EvmError::Other(format!("代码不存在: {:?}", addr)),
            StateError::AccessDenied(msg) => EvmError::Other(format!("访问被拒绝: {}", msg)),
            StateError::Database(msg) => EvmError::Other(format!("数据库错误: {}", msg)),
            StateError::Fairness(msg) => EvmError::Fairness(msg),
            StateError::Other(msg) => EvmError::Other(format!("状态错误: {}", msg)),
        }
    }
}

/// 交易错误
#[derive(Debug, thiserror::Error)]
pub enum TransactionError {
    /// 无效的签名
    #[error("无效的签名")]
    InvalidSignature,

    /// Nonce错误
    #[error("无效的nonce: 预期 {expected}, 实际 {actual}")]
    InvalidNonce { expected: u64, actual: u64 },

    /// Gas价格过低
    #[error("gas价格过低: 最低 {minimum}, 提供 {provided}")]
    GasPriceTooLow { minimum: u64, provided: u64 },

    /// Gas限制超出区块限制
    #[error("gas限制超出区块限制: 区块限制 {block_gas_limit}, 交易限制 {tx_gas_limit}")]
    GasLimitExceedsBlockGasLimit {
        block_gas_limit: u64,
        tx_gas_limit: u64,
    },

    /// 余额不足支付预付费用
    #[error("余额不足: 需要 {required}, 可用 {available}")]
    InsufficientFundsForGas { required: U256, available: U256 },

    /// 无效的交易类型
    #[error("无效的交易类型: {0}")]
    InvalidTransactionType(u8),

    /// 公平性错误
    #[error("公平性错误: {0}")]
    Fairness(String),

    /// 其他交易错误
    #[error("交易错误: {0}")]
    Other(String),
}

impl From<TransactionError> for EvmError {
    fn from(err: TransactionError) -> Self {
        match err {
            TransactionError::Fairness(msg) => EvmError::Fairness(msg),
            _ => EvmError::Transaction(err.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evm_error_conversion() {
        // 测试字符串错误转换
        let err = EvmError::from("测试错误");
        assert!(matches!(err, EvmError::Other(_)));

        // 测试状态错误转换
        let state_err = StateError::AccountNotFound(crate::core::types::Address::from([1; 20]));
        let err = EvmError::from(state_err);
        assert!(matches!(err, EvmError::Other(_)));

        // 测试交易错误转换
        let tx_err = TransactionError::InvalidSignature;
        let err = EvmError::from(tx_err);
        assert!(matches!(err, EvmError::Transaction(_)));

        // 测试其他错误类型转换
        let err = EvmError::OutOfGas { required: 1000, remaining: 500 };
        assert!(matches!(err, EvmError::OutOfGas { .. }));

        let err = EvmError::InvalidOpcode(0xff);
        assert!(matches!(err, EvmError::InvalidOpcode(_)));

        let err = EvmError::InsufficientBalance { required: U256::from(1000), available: U256::from(500) };
        assert!(matches!(err, EvmError::InsufficientBalance { .. }));
    }

    #[test]
    fn test_evm_error_result() {
        // 测试错误结果
        let err = EvmError::OutOfGas { required: 1000, remaining: 500 };
        let result = err.into_result();
        assert!(!result.success);
        assert_eq!(result.gas_used, 0);
        assert!(result.error.is_some());
        assert_eq!(result.fairness_score, 0);

        // 测试从字符串创建结果
        let result = EvmError::result_from_str("测试错误");
        assert!(!result.success);
        assert_eq!(result.gas_used, 0);
        assert!(result.error.is_some());
        assert_eq!(result.fairness_score, 0);
    }

    #[test]
    fn test_state_error() {
        // 测试状态错误转换
        let addr = crate::core::types::Address::from([1; 20]);
        let err = StateError::AccountNotFound(addr);
        let evm_err = EvmError::from(err);
        assert!(matches!(evm_err, EvmError::Other(_)));

        // 测试公平性错误转换
        let err = StateError::Fairness("测试公平性错误".to_string());
        let evm_err = EvmError::from(err);
        assert!(matches!(evm_err, EvmError::Fairness(_)));
    }

    #[test]
    fn test_transaction_error() {
        // 测试交易错误转换
        let err = TransactionError::InvalidSignature;
        let evm_err = EvmError::from(err);
        assert!(matches!(evm_err, EvmError::Transaction(_)));

        // 测试公平性错误转换
        let err = TransactionError::Fairness("测试公平性错误".to_string());
        let evm_err = EvmError::from(err);
        assert!(matches!(evm_err, EvmError::Fairness(_)));

        // 测试其他交易错误转换
        let err = TransactionError::Other("测试其他错误".to_string());
        let evm_err = EvmError::from(err);
        assert!(matches!(evm_err, EvmError::Transaction(_)));
    }

    #[test]
    fn test_error_conversion_chains() {
        // 测试错误转换链
        let state_err = StateError::Fairness("状态公平性错误".to_string());
        let evm_err = EvmError::from(state_err);
        let result = evm_err.into_result();
        assert!(!result.success);
        assert_eq!(result.fairness_score, 0);

        let tx_err = TransactionError::Fairness("交易公平性错误".to_string());
        let evm_err = EvmError::from(tx_err);
        let result = evm_err.into_result();
        assert!(!result.success);
        assert_eq!(result.fairness_score, 0);
    }

    #[test]
    fn test_error_display() {
        // 测试错误显示
        let err = EvmError::OutOfGas { required: 1000, remaining: 500 };
        assert!(err.to_string().contains("gas不足"));

        let err = EvmError::InvalidOpcode(0xff);
        assert!(err.to_string().contains("无效的操作码"));

        let err = EvmError::Fairness("测试公平性错误".to_string());
        assert!(err.to_string().contains("公平性错误"));
    }
} 