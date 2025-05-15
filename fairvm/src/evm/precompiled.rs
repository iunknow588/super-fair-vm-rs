use crate::account::Address;
use ripemd::Ripemd160;
use secp256k1::{
    ecdsa::{RecoverableSignature, RecoveryId},
    Message, Secp256k1,
};
use sha2::{Digest, Sha256};

/// 预编译合约接口
pub trait PrecompiledContract {
    /// 执行合约
    fn execute(&self, input: &[u8], gas_limit: u64) -> Result<(Vec<u8>, u64), String>;

    /// 计算gas消耗
    fn gas_cost(&self, input: &[u8]) -> u64;
}

/// 合约地址到合约的映射
pub fn precompiled_contracts() -> Vec<(Address, Box<dyn PrecompiledContract + Send + Sync>)> {
    vec![
        (
            Address([0u8; 20]),
            Box::new(EcRecover) as Box<dyn PrecompiledContract + Send + Sync>,
        ),
        (
            Address([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2]),
            Box::new(Sha256Hash),
        ),
        (
            Address([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3]),
            Box::new(Ripemd160Hash),
        ),
        (
            Address([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4]),
            Box::new(Identity),
        ),
    ]
}

/// 1. 椭圆曲线签名恢复合约 (ecrecover)
#[allow(clippy::doc_lazy_continuation)]
/// 输入: hash (32 bytes) + v (1 byte) + r (32 bytes) + s (32 bytes)
/// 输出: 恢复的地址 (20 bytes)
pub struct EcRecover;

impl PrecompiledContract for EcRecover {
    fn execute(&self, input: &[u8], gas_limit: u64) -> Result<(Vec<u8>, u64), String> {
        let gas_cost = self.gas_cost(input);
        if gas_cost > gas_limit {
            return Err("Gas limit exceeded".to_string());
        }

        if input.len() < 128 {
            // 如果输入长度不够，返回空
            return Ok((vec![0u8; 32], gas_cost));
        }

        let hash = &input[0..32];
        let v = input[32];
        let r = &input[33..65];
        let s = &input[65..97];

        // 创建消息
        let message =
            Message::from_digest_slice(hash).map_err(|e| format!("Invalid message: {}", e))?;

        // v: 27 或 28 在以太坊中，对应secp256k1的0或1
        let recovery_id = if v == 27 {
            RecoveryId::from_i32(0).map_err(|e| format!("Invalid recovery ID: {}", e))?
        } else if v == 28 {
            RecoveryId::from_i32(1).map_err(|e| format!("Invalid recovery ID: {}", e))?
        } else {
            return Err(format!("Invalid recovery ID: {}", v));
        };

        // 解析 r 和 s
        let mut r_bytes = [0u8; 32];
        let mut s_bytes = [0u8; 32];
        r_bytes.copy_from_slice(r);
        s_bytes.copy_from_slice(s);

        // 创建可恢复签名
        let recoverable_sig =
            RecoverableSignature::from_compact(&[&r_bytes[..], &s_bytes[..]].concat(), recovery_id)
                .map_err(|e| format!("Invalid recoverable signature: {}", e))?;

        // 恢复公钥
        let secp = Secp256k1::new();
        let public_key = secp
            .recover_ecdsa(&message, &recoverable_sig)
            .map_err(|e| format!("Failed to recover public key: {}", e))?;

        // 获取以太坊地址 (Keccak-256 hash of the public key, then take last 20 bytes)
        let public_key_bytes = public_key.serialize_uncompressed();
        let hash = sha3::Keccak256::digest(&public_key_bytes[1..]);

        // 创建地址（最后20字节）
        let mut result = vec![0u8; 32];
        result[12..32].copy_from_slice(&hash[12..32]);

        Ok((result, gas_cost))
    }

    fn gas_cost(&self, _input: &[u8]) -> u64 {
        // ecrecover 固定消耗 3000 gas
        3000
    }
}

/// 2. SHA-256 哈希合约
#[allow(clippy::doc_lazy_continuation)]
/// 输入: 任意数据
/// 输出: 32字节 SHA-256 哈希
pub struct Sha256Hash;

impl PrecompiledContract for Sha256Hash {
    fn execute(&self, input: &[u8], gas_limit: u64) -> Result<(Vec<u8>, u64), String> {
        let gas_cost = self.gas_cost(input);
        if gas_cost > gas_limit {
            return Err("Gas limit exceeded".to_string());
        }

        let mut hasher = Sha256::new();
        hasher.update(input);
        let result = hasher.finalize().to_vec();

        Ok((result, gas_cost))
    }

    fn gas_cost(&self, input: &[u8]) -> u64 {
        // 基础消耗 60 gas + 每个字（32字节）12 gas
        let words = input.len().div_ceil(32);
        60 + (12 * words as u64)
    }
}

/// 3. RIPEMD-160 哈希合约
#[allow(clippy::doc_lazy_continuation)]
/// 输入: 任意数据
/// 输出: 32字节值，右对齐的RIPEMD-160哈希
pub struct Ripemd160Hash;

impl PrecompiledContract for Ripemd160Hash {
    fn execute(&self, input: &[u8], gas_limit: u64) -> Result<(Vec<u8>, u64), String> {
        let gas_cost = self.gas_cost(input);
        if gas_cost > gas_limit {
            return Err("Gas limit exceeded".to_string());
        }

        let mut hasher = Ripemd160::new();
        hasher.update(input);
        let hash = hasher.finalize();

        // RIPEMD-160 产生20字节的哈希，我们需要用0填充左边使其成为32字节
        let mut result = vec![0u8; 32];
        result[12..32].copy_from_slice(&hash);

        Ok((result, gas_cost))
    }

    fn gas_cost(&self, input: &[u8]) -> u64 {
        // 基础消耗 600 gas + 每个字（32字节）120 gas
        let words = input.len().div_ceil(32);
        600 + (120 * words as u64)
    }
}

/// 4. Identity (数据复制) 合约
#[allow(clippy::doc_lazy_continuation)]
/// 输入: 任意数据
/// 输出: 与输入相同的数据
pub struct Identity;

impl PrecompiledContract for Identity {
    fn execute(&self, input: &[u8], gas_limit: u64) -> Result<(Vec<u8>, u64), String> {
        let gas_cost = self.gas_cost(input);
        if gas_cost > gas_limit {
            return Err("Gas limit exceeded".to_string());
        }

        Ok((input.to_vec(), gas_cost))
    }

    fn gas_cost(&self, input: &[u8]) -> u64 {
        // 基础消耗 15 gas + 每个字（32字节）3 gas
        let words = input.len().div_ceil(32);
        15 + (3 * words as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256() {
        let contract = Sha256Hash;
        let input = b"hello world";
        let expected =
            hex::decode("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9")
                .unwrap();

        let (output, _) = contract.execute(input, 1000).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_ripemd160() {
        let contract = Ripemd160Hash;
        let input = b"hello world";

        let (output, _) = contract.execute(input, 1000).unwrap();

        // RIPEMD160("hello world") = 98c615784ccb5fe5936fbc0cbe9dfdb408d92f0f
        let expected_hash = hex::decode("98c615784ccb5fe5936fbc0cbe9dfdb408d92f0f").unwrap();

        // 检查输出的后20字节是否匹配RIPEMD160哈希
        assert_eq!(&output[12..32], expected_hash.as_slice());
    }

    #[test]
    fn test_identity() {
        let contract = Identity;
        let input = b"hello world";

        let (output, _) = contract.execute(input, 1000).unwrap();

        assert_eq!(output, input);
    }
}
