/// EVM操作码定义
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Opcode {
    // 0x0 范围 - 停止和算术运算
    STOP = 0x00,
    ADD = 0x01,
    MUL = 0x02,
    SUB = 0x03,
    DIV = 0x04,
    SDIV = 0x05,
    MOD = 0x06,
    SMOD = 0x07,
    ADDMOD = 0x08,
    MULMOD = 0x09,
    EXP = 0x0a,
    SIGNEXTEND = 0x0b,

    // 0x10 范围 - 比较运算
    LT = 0x10,
    GT = 0x11,
    SLT = 0x12,
    SGT = 0x13,
    EQ = 0x14,
    ISZERO = 0x15,
    AND = 0x16,
    OR = 0x17,
    XOR = 0x18,
    NOT = 0x19,
    BYTE = 0x1a,
    SHL = 0x1b,
    SHR = 0x1c,
    SAR = 0x1d,

    // 0x20 范围 - SHA3
    SHA3 = 0x20,

    // 0x30 范围 - 环境信息
    ADDRESS = 0x30,
    BALANCE = 0x31,
    ORIGIN = 0x32,
    CALLER = 0x33,
    CALLVALUE = 0x34,
    CALLDATALOAD = 0x35,
    CALLDATASIZE = 0x36,
    CALLDATACOPY = 0x37,
    CODESIZE = 0x38,
    CODECOPY = 0x39,
    GASPRICE = 0x3a,
    EXTCODESIZE = 0x3b,
    EXTCODECOPY = 0x3c,
    RETURNDATASIZE = 0x3d,
    RETURNDATACOPY = 0x3e,
    EXTCODEHASH = 0x3f,

    // 0x40 范围 - 区块信息
    BLOCKHASH = 0x40,
    COINBASE = 0x41,
    TIMESTAMP = 0x42,
    NUMBER = 0x43,
    DIFFICULTY = 0x44,
    GASLIMIT = 0x45,
    CHAINID = 0x46,
    SELFBALANCE = 0x47,
    BASEFEE = 0x48,

    // 0x50 范围 - 栈、内存、存储和流程操作
    POP = 0x50,
    MLOAD = 0x51,
    MSTORE = 0x52,
    MSTORE8 = 0x53,
    SLOAD = 0x54,
    SSTORE = 0x55,
    JUMP = 0x56,
    JUMPI = 0x57,
    PC = 0x58,
    MSIZE = 0x59,
    GAS = 0x5a,
    JUMPDEST = 0x5b,

    // 0x60 范围 - 入栈操作
    PUSH1 = 0x60,
    PUSH2 = 0x61,
    PUSH32 = 0x7f,

    // 0x80 范围 - 复制操作
    DUP1 = 0x80,
    DUP2 = 0x81,
    DUP16 = 0x8f,

    // 0x90 范围 - 交换操作
    SWAP1 = 0x90,
    SWAP2 = 0x91,
    SWAP16 = 0x9f,

    // 0xa0 范围 - 日志操作
    LOG0 = 0xa0,
    LOG1 = 0xa1,
    LOG2 = 0xa2,
    LOG3 = 0xa3,
    LOG4 = 0xa4,

    // 0xf0 范围 - 系统操作
    CREATE = 0xf0,
    CALL = 0xf1,
    CALLCODE = 0xf2,
    RETURN = 0xf3,
    DELEGATECALL = 0xf4,
    CREATE2 = 0xf5,
    STATICCALL = 0xfa,
    REVERT = 0xfd,
    INVALID = 0xfe,
    SELFDESTRUCT = 0xff,
}

impl Opcode {
    /// 获取操作码的gas消耗
    pub fn gas_cost(&self) -> u64 {
        match self {
            // 零gas消耗操作
            Opcode::STOP | Opcode::JUMPDEST => 0,

            // 低gas消耗操作 (3)
            Opcode::ADD
            | Opcode::SUB
            | Opcode::NOT
            | Opcode::LT
            | Opcode::GT
            | Opcode::SLT
            | Opcode::SGT
            | Opcode::EQ
            | Opcode::ISZERO
            | Opcode::AND
            | Opcode::OR
            | Opcode::XOR
            | Opcode::BYTE
            | Opcode::SHL
            | Opcode::SHR
            | Opcode::SAR
            | Opcode::POP
            | Opcode::PUSH1
            | Opcode::PUSH2
            | Opcode::PUSH32
            | Opcode::DUP1
            | Opcode::DUP2
            | Opcode::DUP16
            | Opcode::SWAP1
            | Opcode::SWAP2
            | Opcode::SWAP16 => 3,

            // 中等gas消耗操作 (5)
            Opcode::MUL
            | Opcode::DIV
            | Opcode::SDIV
            | Opcode::MOD
            | Opcode::SMOD
            | Opcode::SIGNEXTEND => 5,

            // 内存操作 (3)
            Opcode::MLOAD | Opcode::MSTORE | Opcode::MSTORE8 => 3,

            // 存储操作 (较高gas消耗)
            Opcode::SLOAD => 200,    // 实际值在不同硬分叉中可能不同
            Opcode::SSTORE => 20000, // 实际值在不同硬分叉中可能不同

            // 环境操作
            Opcode::ADDRESS
            | Opcode::ORIGIN
            | Opcode::CALLER
            | Opcode::CALLVALUE
            | Opcode::CALLDATASIZE
            | Opcode::CODESIZE
            | Opcode::GASPRICE
            | Opcode::RETURNDATASIZE => 2,

            // 区块信息操作
            Opcode::BLOCKHASH => 20,
            Opcode::COINBASE
            | Opcode::TIMESTAMP
            | Opcode::NUMBER
            | Opcode::DIFFICULTY
            | Opcode::GASLIMIT => 2,

            // 日志操作
            Opcode::LOG0 => 375,
            Opcode::LOG1 => 750,
            Opcode::LOG2 => 1125,
            Opcode::LOG3 => 1500,
            Opcode::LOG4 => 1875,

            // 高gas消耗操作
            Opcode::CREATE => 32000,
            Opcode::CALL => 700,
            Opcode::CALLCODE => 700,
            Opcode::RETURN => 0,
            Opcode::DELEGATECALL => 700,
            Opcode::CREATE2 => 32000,
            Opcode::STATICCALL => 700,
            Opcode::REVERT => 0,
            Opcode::SELFDESTRUCT => 5000,

            // 其他操作使用默认gas消耗
            _ => 3,
        }
    }

    /// 从字节码创建操作码
    pub fn from_u8(value: u8) -> Option<Opcode> {
        // 0xff是u8的最大值，value始终在范围内
        Some(unsafe { std::mem::transmute::<u8, Opcode>(value) })
    }

    /// 获取操作码需要从代码中读取的额外字节数
    pub fn extra_bytes(&self) -> usize {
        match self {
            Opcode::PUSH1 => 1,
            Opcode::PUSH2 => 2,
            Opcode::PUSH32 => 32,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_from_u8() {
        assert_eq!(Opcode::from_u8(0x00), Some(Opcode::STOP));
        assert_eq!(Opcode::from_u8(0x01), Some(Opcode::ADD));
        assert_eq!(Opcode::from_u8(0x60), Some(Opcode::PUSH1));
    }

    #[test]
    fn test_gas_cost() {
        assert_eq!(Opcode::STOP.gas_cost(), 0);
        assert_eq!(Opcode::ADD.gas_cost(), 3);
        assert_eq!(Opcode::SSTORE.gas_cost(), 20000);
    }

    #[test]
    fn test_extra_bytes() {
        assert_eq!(Opcode::ADD.extra_bytes(), 0);
        assert_eq!(Opcode::PUSH1.extra_bytes(), 1);
        assert_eq!(Opcode::PUSH32.extra_bytes(), 32);
    }
}
