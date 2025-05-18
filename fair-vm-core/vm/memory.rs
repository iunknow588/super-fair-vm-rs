use crate::core::types::U256;
use std::cmp;

/// EVM内存实现
#[derive(Debug, Default, Clone)]
pub struct Memory {
    /// 内存数据存储
    data: Vec<u8>,
    /// 当前内存大小（以字节为单位）
    size: usize,
    /// 公平性权重
    fairness_weight: u64,
}

impl Memory {
    /// 创建新的内存实例
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            size: 0,
            fairness_weight: 0,
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

        let gas_cost = linear + quadratic;
        self.update_fairness_weight(gas_cost);
        gas_cost
    }

    /// 存储一个字（32字节）到内存
    ///
    /// # 参数
    /// * `offset` - 内存偏移量
    /// * `value` - 要存储的值
    pub fn store32(&mut self, offset: usize, value: U256) {
        self.expand(offset, 32);
        value.to_big_endian(&mut self.data[offset..offset + 32]);
        self.update_fairness_weight(1);
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
        self.update_fairness_weight(value.len() as u64);
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

    /// 获取公平性权重
    pub fn fairness_weight(&self) -> u64 {
        self.fairness_weight
    }

    /// 更新公平性权重
    fn update_fairness_weight(&mut self, weight: u64) {
        self.fairness_weight += weight;
    }

    /// 清空内存
    pub fn clear(&mut self) {
        self.data.clear();
        self.size = 0;
        self.fairness_weight = 0;
    }

    /// 复制内存区域
    ///
    /// # 参数
    /// * `src_offset` - 源偏移量
    /// * `dst_offset` - 目标偏移量
    /// * `size` - 要复制的字节数
    pub fn copy(&mut self, src_offset: usize, dst_offset: usize, size: usize) {
        if size == 0 {
            return;
        }

        self.expand(dst_offset, size);
        if src_offset + size <= self.data.len() {
            self.data.copy_within(src_offset..src_offset + size, dst_offset);
        } else {
            // 如果源区域超出范围，用0填充
            for i in 0..size {
                if src_offset + i < self.data.len() {
                    self.data[dst_offset + i] = self.data[src_offset + i];
                } else {
                    self.data[dst_offset + i] = 0;
                }
            }
        }
        self.update_fairness_weight(size as u64);
    }

    /// 调整内存大小
    ///
    /// # 参数
    /// * `new_size` - 新的内存大小
    pub fn resize(&mut self, new_size: usize) {
        if new_size > self.size {
            self.expand(0, new_size - self.size);
        } else {
            self.data.truncate(new_size);
            self.size = new_size;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_operations() {
        let mut memory = Memory::new();

        // 测试基本存储和加载
        memory.store(0, &[1, 2, 3, 4]);
        let data = memory.load(0, 4);
        assert_eq!(data, vec![1, 2, 3, 4]);

        // 测试32字节值操作
        let value = U256::from(0x1234567890abcdefu64);
        memory.store32(0, value);
        let loaded = memory.load32(0);
        assert_eq!(loaded, value);

        // 测试内存扩展
        memory.store(1000, &[1, 2, 3]);
        assert!(memory.size() >= 1003);

        // 测试内存清零
        memory.clear();
        assert_eq!(memory.size(), 0);
    }

    #[test]
    fn test_memory_boundaries() {
        let mut memory = Memory::new();

        // 测试边界存储
        memory.store(0, &[1, 2, 3]);
        memory.store(1, &[4, 5, 6]);
        let data = memory.load(0, 4);
        assert_eq!(data, vec![1, 4, 5, 6]);

        // 测试重叠存储
        memory.store(0, &[1, 2, 3, 4]);
        memory.store(2, &[5, 6]);
        let data = memory.load(0, 4);
        assert_eq!(data, vec![1, 2, 5, 6]);

        // 测试部分加载
        memory.store(0, &[1, 2, 3, 4, 5]);
        let data = memory.load(1, 3);
        assert_eq!(data, vec![2, 3, 4]);

        // 测试零长度操作
        memory.store(0, &[]);
        let data = memory.load(0, 0);
        assert_eq!(data, vec![]);
    }

    #[test]
    fn test_memory_alignment() {
        let mut memory = Memory::new();

        // 测试32字节对齐存储
        let value = U256::from(0x1234567890abcdefu64);
        memory.store32(0, value);
        memory.store32(32, value);
        let loaded1 = memory.load32(0);
        let loaded2 = memory.load32(32);
        assert_eq!(loaded1, value);
        assert_eq!(loaded2, value);

        // 测试非对齐存储
        memory.store(1, &[1, 2, 3, 4]);
        let data = memory.load(1, 4);
        assert_eq!(data, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_memory_overflow() {
        let mut memory = Memory::new();

        // 测试大偏移量存储
        let large_offset = usize::MAX - 10;
        memory.store(large_offset, &[1, 2, 3]);
        let data = memory.load(large_offset, 3);
        assert_eq!(data, vec![1, 2, 3]);

        // 测试大尺寸存储
        let large_size = 1024 * 1024; // 1MB
        let data = vec![1; large_size];
        memory.store(0, &data);
        assert_eq!(memory.size(), large_size);
    }

    #[test]
    fn test_memory_gas_calculation() {
        let mut memory = Memory::new();

        // 测试基本扩展gas计算
        let gas = memory.expand(0, 32);
        assert!(gas > 0);

        // 测试重叠扩展gas计算
        let gas = memory.expand(16, 32);
        assert!(gas > 0);

        // 测试零大小扩展gas计算
        let gas = memory.expand(0, 0);
        assert_eq!(gas, 0);
    }

    #[test]
    fn test_memory_copy() {
        let mut memory = Memory::new();

        // 测试基本复制
        memory.store(0, &[1, 2, 3, 4]);
        memory.copy(0, 4, 4);
        let data = memory.load(4, 4);
        assert_eq!(data, vec![1, 2, 3, 4]);

        // 测试重叠复制
        memory.store(0, &[1, 2, 3, 4, 5]);
        memory.copy(0, 2, 3);
        let data = memory.load(0, 5);
        assert_eq!(data, vec![1, 2, 1, 2, 3]);

        // 测试越界复制
        memory.copy(1000, 0, 4);
        let data = memory.load(0, 4);
        assert_eq!(data, vec![0, 0, 0, 0]);
    }

    #[test]
    fn test_memory_resize() {
        let mut memory = Memory::new();

        // 测试扩展
        memory.store(0, &[1, 2, 3]);
        memory.resize(6);
        assert_eq!(memory.size(), 6);
        let data = memory.load(0, 6);
        assert_eq!(data, vec![1, 2, 3, 0, 0, 0]);

        // 测试收缩
        memory.resize(2);
        assert_eq!(memory.size(), 2);
        let data = memory.load(0, 2);
        assert_eq!(data, vec![1, 2]);
    }

    #[test]
    fn test_memory_fairness_weight() {
        let mut memory = Memory::new();

        // 测试基本操作的公平性权重
        memory.store(0, &[1, 2, 3]);
        assert_eq!(memory.fairness_weight(), 3);

        memory.store32(0, U256::from(1));
        assert_eq!(memory.fairness_weight(), 4);

        memory.copy(0, 32, 4);
        assert_eq!(memory.fairness_weight(), 8);

        // 测试清空重置公平性权重
        memory.clear();
        assert_eq!(memory.fairness_weight(), 0);
    }
} 