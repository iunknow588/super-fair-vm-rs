use ethers::types::U256;
use std::cmp;

/// EVM内存实现
#[derive(Debug, Default, Clone)]
pub struct Memory {
    /// 内存数据存储
    data: Vec<u8>,
    /// 当前内存大小（以字节为单位）
    size: usize,
}

impl Memory {
    /// 创建新的内存实例
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            size: 0,
        }
    }

    /// 扩展内存到指定大小
    ///
    /// # 参数
    /// * `offset` - 内存偏移量
    /// * `size` - 需要的内存大小
    ///
    /// # 返回值
    /// 返回扩展所需的gas消耗
    pub fn expand(&mut self, offset: usize, size: usize) -> u64 {
        if size == 0 {
            return 0;
        }

        let old_size = self.size;
        let new_size = cmp::max(old_size, offset + size);
        self.size = new_size;

        if new_size > self.data.len() {
            self.data.resize(new_size, 0);
        }

        // 计算gas消耗
        // 根据以太坊黄皮书，内存扩展的gas计算公式为：
        // Gmem * (new_words - old_words) + (new_words * new_words / 512) - (old_words * old_words / 512)
        let old_words = old_size.div_ceil(32);
        let new_words = new_size.div_ceil(32);

        if new_words <= old_words {
            return 0;
        }

        const GMEMORY: u64 = 3; // 内存扩展基础gas消耗
        const GMEMORY_QUADRATIC_DENOMINATOR: u64 = 512; // 二次项系数

        let linear = (new_words as u64 - old_words as u64) * GMEMORY;
        let quadratic = (new_words as u64 * new_words as u64) / GMEMORY_QUADRATIC_DENOMINATOR
            - (old_words as u64 * old_words as u64) / GMEMORY_QUADRATIC_DENOMINATOR;

        linear + quadratic
    }

    /// 存储一个字（32字节）到内存
    ///
    /// # 参数
    /// * `offset` - 内存偏移量
    /// * `value` - 要存储的值
    pub fn store32(&mut self, offset: usize, value: U256) {
        self.expand(offset, 32);
        value.to_big_endian(&mut self.data[offset..offset + 32]);
    }

    /// 从内存加载一个字（32字节）
    ///
    /// # 参数
    /// * `offset` - 内存偏移量
    pub fn load32(&self, offset: usize) -> U256 {
        if offset + 32 > self.data.len() {
            return U256::zero();
        }
        U256::from_big_endian(&self.data[offset..offset + 32])
    }

    /// 存储任意字节到内存
    ///
    /// # 参数
    /// * `offset` - 内存偏移量
    /// * `value` - 要存储的字节
    pub fn store(&mut self, offset: usize, value: &[u8]) {
        self.expand(offset, value.len());
        self.data[offset..offset + value.len()].copy_from_slice(value);
    }

    /// 从内存加载指定大小的字节
    ///
    /// # 参数
    /// * `offset` - 内存偏移量
    /// * `size` - 要加载的字节数
    pub fn load(&self, offset: usize, size: usize) -> Vec<u8> {
        if offset + size > self.data.len() {
            return vec![0; size];
        }
        self.data[offset..offset + size].to_vec()
    }

    /// 获取当前内存大小
    pub fn size(&self) -> usize {
        self.size
    }

    /// 获取内存数据的引用
    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_expansion() {
        let mut memory = Memory::new();

        // 测试基本扩展
        let gas = memory.expand(0, 32);
        assert!(gas > 0);
        assert_eq!(memory.size(), 32);

        // 测试重叠扩展
        let _gas = memory.expand(16, 32);
        assert_eq!(memory.size(), 48);

        // 测试零大小扩展
        let gas = memory.expand(0, 0);
        assert_eq!(gas, 0);
    }

    #[test]
    fn test_store_and_load() {
        let mut memory = Memory::new();

        // 测试32字节存储和加载
        let value = U256::from(0x1234);
        memory.store32(0, value);
        assert_eq!(memory.load32(0), value);

        // 测试任意字节存储和加载
        let bytes = vec![1, 2, 3, 4];
        memory.store(32, &bytes);
        assert_eq!(memory.load(32, 4), bytes);

        // 测试越界加载
        assert_eq!(memory.load(1000, 32), vec![0; 32]);
    }
}
