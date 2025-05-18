use crate::core::types::{Address, U256};
use crate::core::vm::memory::Memory;
use crate::core::vm::opcodes::Opcode;
use crate::core::vm::stack::{Stack, StackError};
use crate::core::vm::errors::{EvmError, ExecutorError};
use crate::core::state::State;
use std::sync::Arc;
use tokio::sync::RwLock;

/// EVM执行上下文
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    /// 调用者地址
    pub caller: Address,
    /// 合约地址
    pub address: Address,
    /// 调用值
    pub value: U256,
    /// 输入数据
    pub data: Vec<u8>,
    /// gas限制
    pub gas_limit: u64,
    /// 代码
    pub code: Vec<u8>,
    /// 是否为静态调用
    pub is_static: bool,
    /// 公平性权重
    pub fairness_weight: u64,
}

/// 执行结果
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    /// 是否成功
    pub success: bool,
    /// 使用的gas
    pub gas_used: u64,
    /// 返回数据
    pub return_data: Vec<u8>,
    /// 错误信息
    pub error: Option<String>,
    /// 公平性得分
    pub fairness_score: u64,
}

/// EVM执行器
pub struct Executor {
    /// 执行上下文
    pub context: ExecutionContext,
    /// 内存
    pub memory: Memory,
    /// 栈
    pub stack: Stack,
    /// 程序计数器
    pub pc: usize,
    /// 已使用的gas
    pub gas_used: u64,
    /// 可用gas
    pub gas_limit: u64,
    /// 返回数据
    pub return_data: Vec<u8>,
    /// 状态
    pub state: Arc<RwLock<State>>,
    /// 公平性得分
    pub fairness_score: u64,
}

impl Executor {
    /// 创建新的执行器
    pub fn new(state: Arc<RwLock<State>>, context: ExecutionContext) -> Self {
        Self {
            gas_limit: context.gas_limit,
            context,
            memory: Memory::new(),
            stack: Stack::new(),
            pc: 0,
            gas_used: 0,
            return_data: Vec::new(),
            state,
            fairness_score: 0,
        }
    }

    /// 执行EVM代码
    pub async fn execute(&mut self, code: Vec<u8>) -> ExecutionResult {
        let jumpdests = Self::analyze_jumpdests(&code);

        while self.pc < code.len() {
            let op = code[self.pc];

            // 尝试执行操作码
            match self.execute_opcode(op, &code, &jumpdests).await {
                Ok(should_continue) => {
                    if should_continue {
                        self.pc += 1;
                    } else {
                        // 仅在STOP/RETURN等终止指令时break，JUMP/JUMPI跳转后应继续执行
                        if op == Opcode::STOP as u8 || op == Opcode::RETURN as u8 {
                            break;
                        }
                        // JUMP/JUMPI跳转后不break
                    }
                }
                Err(e) => {
                    return ExecutionResult {
                        success: false,
                        gas_used: self.gas_used,
                        return_data: Vec::new(),
                        error: Some(e.to_string()),
                        fairness_score: self.fairness_score,
                    }
                }
            }
        }

        ExecutionResult {
            success: true,
            gas_used: self.gas_used,
            return_data: self.return_data.clone(),
            error: None,
            fairness_score: self.fairness_score,
        }
    }

    /// 分析代码中的跳转目标
    fn analyze_jumpdests(code: &[u8]) -> Vec<bool> {
        let mut jumpdests = vec![false; code.len()];
        let mut i = 0;

        while i < code.len() {
            if code[i] == Opcode::JUMPDEST as u8 {
                jumpdests[i] = true;
            }
            if code[i] >= Opcode::PUSH1 as u8 && code[i] <= Opcode::PUSH32 as u8 {
                let bytes_to_push = (code[i] - Opcode::PUSH1 as u8 + 1) as usize;
                i += 1 + bytes_to_push;
            } else {
                i += 1;
            }
        }

        jumpdests
    }

    /// 增加gas使用量并检查是否超出限制
    fn use_gas(&mut self, amount: u64) -> Result<(), ExecutorError> {
        if self.gas_used + amount > self.gas_limit {
            return Err(ExecutorError::OutOfGas);
        }
        self.gas_used += amount;
        Ok(())
    }

    /// 更新公平性得分
    fn update_fairness_score(&mut self, weight: u64) {
        self.fairness_score += weight;
    }

    /// 执行单个操作码
    async fn execute_opcode(
        &mut self,
        opcode: u8,
        code: &[u8],
        jumpdests: &[bool],
    ) -> Result<bool, ExecutorError> {
        // 尝试将字节转换为操作码
        let op = Opcode::from_u8(opcode).ok_or(ExecutorError::InvalidOpcode(opcode))?;

        // 使用基本gas
        self.use_gas(op.gas_cost())?;

        // 更新公平性得分
        self.update_fairness_score(op.fairness_weight());

        match op {
            // 0x00: STOP
            Opcode::STOP => Ok(false),

            // 0x01: ADD
            Opcode::ADD => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push(a.overflowing_add(b).0)?;
                Ok(true)
            }

            // 0x02: MUL
            Opcode::MUL => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push(a.overflowing_mul(b).0)?;
                Ok(true)
            }

            // 0x03: SUB
            Opcode::SUB => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push(a.overflowing_sub(b).0)?;
                Ok(true)
            }

            // 0x04: DIV
            Opcode::DIV => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;

                let result = if b.is_zero() { U256::zero() } else { a / b };

                self.stack.push(result)?;
                Ok(true)
            }

            // 0x06: MOD
            Opcode::MOD => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;

                let result = if b.is_zero() { U256::zero() } else { a % b };

                self.stack.push(result)?;
                Ok(true)
            }

            // 0x10: LT
            Opcode::LT => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push(if a < b { U256::from(1) } else { U256::zero() })?;
                Ok(true)
            }

            // 0x11: GT
            Opcode::GT => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push(if a > b { U256::from(1) } else { U256::zero() })?;
                Ok(true)
            }

            // 0x14: EQ
            Opcode::EQ => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push(if a == b { U256::from(1) } else { U256::zero() })?;
                Ok(true)
            }

            // 0x15: ISZERO
            Opcode::ISZERO => {
                let a = self.stack.pop()?;
                self.stack.push(if a.is_zero() { U256::from(1) } else { U256::zero() })?;
                Ok(true)
            }

            // 0x16: AND
            Opcode::AND => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push(a & b)?;
                Ok(true)
            }

            // 0x17: OR
            Opcode::OR => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push(a | b)?;
                Ok(true)
            }

            // 0x18: XOR
            Opcode::XOR => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push(a ^ b)?;
                Ok(true)
            }

            // 0x19: NOT
            Opcode::NOT => {
                let a = self.stack.pop()?;
                self.stack.push(!a)?;
                Ok(true)
            }

            // 0x1a: BYTE
            Opcode::BYTE => {
                let i = self.stack.pop()?;
                let x = self.stack.pop()?;

                let result = if i >= U256::from(32) {
                    U256::zero()
                } else {
                    let byte = (x >> ((31 - i.as_u32()) * 8)) & U256::from(0xff);
                    byte
                };

                self.stack.push(result)?;
                Ok(true)
            }

            // 0x50: POP
            Opcode::POP => {
                self.stack.pop()?;
                Ok(true)
            }

            // 0x51: MLOAD
            Opcode::MLOAD => {
                let offset = self.stack.pop()?;
                let value = self.memory.load32(offset.as_u64() as usize)?;
                self.stack.push(value)?;
                Ok(true)
            }

            // 0x52: MSTORE
            Opcode::MSTORE => {
                let offset = self.stack.pop()?;
                let value = self.stack.pop()?;
                self.memory.store32(offset.as_u64() as usize, value)?;
                Ok(true)
            }

            // 0x54: SLOAD
            Opcode::SLOAD => {
                let key = self.stack.pop()?;
                let value = {
                    let state = self.state.read().await;
                    state.get_storage(self.context.address, key)?
                };
                self.stack.push(value)?;
                Ok(true)
            }

            // 0x55: SSTORE
            Opcode::SSTORE => {
                if self.context.is_static {
                    return Err(ExecutorError::State("静态调用中不允许修改存储".into()));
                }

                let key = self.stack.pop()?;
                let value = self.stack.pop()?;

                {
                    let mut state = self.state.write().await;
                    state.set_storage(self.context.address, key, value)?;
                }

                Ok(true)
            }

            // 0x56: JUMP
            Opcode::JUMP => {
                let dest = self.stack.pop()?;
                let dest = dest.as_u64() as usize;

                if dest >= code.len() || !jumpdests[dest] {
                    return Err(ExecutorError::InvalidJumpdest);
                }

                self.pc = dest;
                Ok(false)
            }

            // 0x57: JUMPI
            Opcode::JUMPI => {
                let dest = self.stack.pop()?;
                let condition = self.stack.pop()?;

                if !condition.is_zero() {
                    let dest = dest.as_u64() as usize;

                    if dest >= code.len() || !jumpdests[dest] {
                        return Err(ExecutorError::InvalidJumpdest);
                    }

                    self.pc = dest;
                    Ok(false)
                } else {
                    Ok(true)
                }
            }

            // 0x58: PC
            Opcode::PC => {
                self.stack.push(U256::from(self.pc))?;
                Ok(true)
            }

            // 0x59: MSIZE
            Opcode::MSIZE => {
                self.stack.push(U256::from(self.memory.size()))?;
                Ok(true)
            }

            // 0x5a: GAS
            Opcode::GAS => {
                self.stack.push(U256::from(self.gas_limit - self.gas_used))?;
                Ok(true)
            }

            // 0x5b: JUMPDEST
            Opcode::JUMPDEST => Ok(true),

            // 0xf3: RETURN
            Opcode::RETURN => {
                let offset = self.stack.pop()?;
                let size = self.stack.pop()?;

                self.return_data = self.memory.load(offset.as_u64() as usize, size.as_u64() as usize)?;
                Ok(false)
            }

            // 0xfd: REVERT
            Opcode::REVERT => {
                let offset = self.stack.pop()?;
                let size = self.stack.pop()?;

                self.return_data = self.memory.load(offset.as_u64() as usize, size.as_u64() as usize)?;
                Ok(false)
            }

            // 其他操作码
            _ => Err(ExecutorError::InvalidOpcode(opcode)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    #[tokio::test]
    async fn test_basic_arithmetic_operations() {
        let state = Arc::new(RwLock::new(State::new()));
        let context = ExecutionContext {
            caller: Address::zero(),
            address: Address::zero(),
            value: U256::zero(),
            data: Vec::new(),
            gas_limit: 1000000,
            code: Vec::new(),
            is_static: false,
            fairness_weight: 0,
        };

        let mut executor = Executor::new(state, context);

        // 测试ADD
        let code = vec![0x60, 0x01, 0x60, 0x02, 0x01]; // PUSH1 1, PUSH1 2, ADD
        let result = executor.execute(code).await;
        assert!(result.success);
        assert_eq!(executor.stack.pop().unwrap(), U256::from(3));

        // 测试MUL
        let code = vec![0x60, 0x02, 0x60, 0x03, 0x02]; // PUSH1 2, PUSH1 3, MUL
        let result = executor.execute(code).await;
        assert!(result.success);
        assert_eq!(executor.stack.pop().unwrap(), U256::from(6));

        // 测试SUB
        let code = vec![0x60, 0x05, 0x60, 0x03, 0x03]; // PUSH1 5, PUSH1 3, SUB
        let result = executor.execute(code).await;
        assert!(result.success);
        assert_eq!(executor.stack.pop().unwrap(), U256::from(2));
    }

    #[tokio::test]
    async fn test_comparison_operations() {
        let state = Arc::new(RwLock::new(State::new()));
        let context = ExecutionContext {
            caller: Address::zero(),
            address: Address::zero(),
            value: U256::zero(),
            data: Vec::new(),
            gas_limit: 1000000,
            code: Vec::new(),
            is_static: false,
            fairness_weight: 0,
        };

        let mut executor = Executor::new(state, context);

        // 测试LT
        let code = vec![0x60, 0x02, 0x60, 0x03, 0x10]; // PUSH1 2, PUSH1 3, LT
        let result = executor.execute(code).await;
        assert!(result.success);
        assert_eq!(executor.stack.pop().unwrap(), U256::from(1));

        // 测试GT
        let code = vec![0x60, 0x03, 0x60, 0x02, 0x11]; // PUSH1 3, PUSH1 2, GT
        let result = executor.execute(code).await;
        assert!(result.success);
        assert_eq!(executor.stack.pop().unwrap(), U256::from(1));

        // 测试EQ
        let code = vec![0x60, 0x02, 0x60, 0x02, 0x14]; // PUSH1 2, PUSH1 2, EQ
        let result = executor.execute(code).await;
        assert!(result.success);
        assert_eq!(executor.stack.pop().unwrap(), U256::from(1));
    }

    #[tokio::test]
    async fn test_jump_operations() {
        let state = Arc::new(RwLock::new(State::new()));
        let context = ExecutionContext {
            caller: Address::zero(),
            address: Address::zero(),
            value: U256::zero(),
            data: Vec::new(),
            gas_limit: 1000000,
            code: Vec::new(),
            is_static: false,
            fairness_weight: 0,
        };

        let mut executor = Executor::new(state, context);

        // 测试JUMP
        let code = vec![
            0x60, 0x05, // PUSH1 5
            0x56,       // JUMP
            0x00,       // STOP
            0x5b,       // JUMPDEST
            0x60, 0x01, // PUSH1 1
        ];
        let result = executor.execute(code).await;
        assert!(result.success);
        assert_eq!(executor.stack.pop().unwrap(), U256::from(1));

        // 测试JUMPI
        let code = vec![
            0x60, 0x01, // PUSH1 1
            0x60, 0x05, // PUSH1 5
            0x57,       // JUMPI
            0x00,       // STOP
            0x5b,       // JUMPDEST
            0x60, 0x02, // PUSH1 2
        ];
        let result = executor.execute(code).await;
        assert!(result.success);
        assert_eq!(executor.stack.pop().unwrap(), U256::from(2));
    }

    #[tokio::test]
    async fn test_memory_operations() {
        let state = Arc::new(RwLock::new(State::new()));
        let context = ExecutionContext {
            caller: Address::zero(),
            address: Address::zero(),
            value: U256::zero(),
            data: Vec::new(),
            gas_limit: 1000000,
            code: Vec::new(),
            is_static: false,
            fairness_weight: 0,
        };

        let mut executor = Executor::new(state, context);

        // 测试MSTORE和MLOAD
        let code = vec![
            0x60, 0x00, // PUSH1 0
            0x60, 0x01, // PUSH1 1
            0x52,       // MSTORE
            0x60, 0x00, // PUSH1 0
            0x51,       // MLOAD
        ];
        let result = executor.execute(code).await;
        assert!(result.success);
        assert_eq!(executor.stack.pop().unwrap(), U256::from(1));
    }

    #[tokio::test]
    async fn test_storage_operations() {
        let state = Arc::new(RwLock::new(State::new()));
        let context = ExecutionContext {
            caller: Address::zero(),
            address: Address::zero(),
            value: U256::zero(),
            data: Vec::new(),
            gas_limit: 1000000,
            code: Vec::new(),
            is_static: false,
            fairness_weight: 0,
        };

        let mut executor = Executor::new(state, context);

        // 测试SSTORE和SLOAD
        let code = vec![
            0x60, 0x00, // PUSH1 0
            0x60, 0x01, // PUSH1 1
            0x55,       // SSTORE
            0x60, 0x00, // PUSH1 0
            0x54,       // SLOAD
        ];
        let result = executor.execute(code).await;
        assert!(result.success);
        assert_eq!(executor.stack.pop().unwrap(), U256::from(1));
    }

    #[tokio::test]
    async fn test_gas_operations() {
        let state = Arc::new(RwLock::new(State::new()));
        let context = ExecutionContext {
            caller: Address::zero(),
            address: Address::zero(),
            value: U256::zero(),
            data: Vec::new(),
            gas_limit: 1000000,
            code: Vec::new(),
            is_static: false,
            fairness_weight: 0,
        };

        let mut executor = Executor::new(state, context);

        // 测试GAS
        let code = vec![0x5a]; // GAS
        let result = executor.execute(code).await;
        assert!(result.success);
        assert_eq!(executor.stack.pop().unwrap(), U256::from(1000000));
    }

    #[tokio::test]
    async fn test_fairness_score() {
        let state = Arc::new(RwLock::new(State::new()));
        let context = ExecutionContext {
            caller: Address::zero(),
            address: Address::zero(),
            value: U256::zero(),
            data: Vec::new(),
            gas_limit: 1000000,
            code: Vec::new(),
            is_static: false,
            fairness_weight: 0,
        };

        let mut executor = Executor::new(state, context);

        // 测试公平性相关操作
        let code = vec![
            0x60, 0x00, // PUSH1 0
            0x60, 0x01, // PUSH1 1
            0x55,       // SSTORE
        ];
        let result = executor.execute(code).await;
        assert!(result.success);
        assert!(result.fairness_score > 0);
    }
} 