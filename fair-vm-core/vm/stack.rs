use crate::core::types::U256;

/// EVM栈实现
#[derive(Debug, Clone, Default)]
pub struct Stack {
    /// 栈数据，使用Vec<U256>存储
    items: Vec<U256>,
    /// 栈的最大深度限制
    max_depth: usize,
    /// 公平性权重
    fairness_weight: u64,
}

/// 定义栈相关的错误类型
#[derive(Debug, thiserror::Error)]
pub enum StackError {
    #[error("栈溢出")]
    Overflow,
    #[error("栈下溢")]
    Underflow,
    #[error("栈深度超过限制")]
    DepthLimitExceeded,
    #[error("无效的索引: {0}")]
    InvalidIndex(usize),
}

impl Stack {
    /// 创建一个新的栈实例
    /// 默认最大深度为1024（以太坊规范）
    pub fn new() -> Self {
        Self {
            items: Vec::with_capacity(1024),
            max_depth: 1024,
            fairness_weight: 0,
        }
    }

    /// 创建指定最大深度的栈
    pub fn with_max_depth(max_depth: usize) -> Self {
        Self {
            items: Vec::with_capacity(max_depth),
            max_depth,
            fairness_weight: 0,
        }
    }

    /// 获取栈的当前深度
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// 判断栈是否为空
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// 压入一个值到栈顶
    ///
    /// # 错误
    /// 如果栈已满（达到最大深度），返回StackError::DepthLimitExceeded
    pub fn push(&mut self, value: U256) -> Result<(), StackError> {
        if self.items.len() >= self.max_depth {
            return Err(StackError::DepthLimitExceeded);
        }
        self.items.push(value);
        self.update_fairness_weight(1);
        Ok(())
    }

    /// 从栈顶弹出一个值
    ///
    /// # 错误
    /// 如果栈为空，返回StackError::Underflow
    pub fn pop(&mut self) -> Result<U256, StackError> {
        let value = self.items.pop().ok_or(StackError::Underflow)?;
        self.update_fairness_weight(1);
        Ok(value)
    }

    /// 查看栈顶元素但不弹出
    ///
    /// # 错误
    /// 如果栈为空，返回StackError::Underflow
    pub fn peek(&self) -> Result<U256, StackError> {
        self.items.last().copied().ok_or(StackError::Underflow)
    }

    /// 复制栈内指定位置的元素到栈顶
    /// index=0表示栈顶元素
    ///
    /// # 错误
    /// - 如果索引无效，返回StackError::InvalidIndex
    /// - 如果栈会溢出，返回StackError::DepthLimitExceeded
    pub fn dup(&mut self, index: usize) -> Result<(), StackError> {
        if index >= self.items.len() {
            return Err(StackError::InvalidIndex(index));
        }

        let value = self.items[self.items.len() - 1 - index];
        self.push(value)
    }

    /// 交换栈顶元素和指定位置的元素
    /// index=0表示与栈顶元素交换（无效操作）
    /// index=1表示与次栈顶元素交换
    ///
    /// # 错误
    /// 如果索引无效，返回StackError::InvalidIndex
    pub fn swap(&mut self, index: usize) -> Result<(), StackError> {
        if index == 0 || index >= self.items.len() {
            return Err(StackError::InvalidIndex(index));
        }

        let len = self.items.len();
        self.items.swap(len - 1, len - 1 - index);
        self.update_fairness_weight(2);
        Ok(())
    }

    /// 获取指定位置的元素引用
    /// index=0表示栈顶元素
    ///
    /// # 错误
    /// 如果索引无效，返回StackError::InvalidIndex
    pub fn get(&self, index: usize) -> Result<U256, StackError> {
        if index >= self.items.len() {
            return Err(StackError::InvalidIndex(index));
        }

        Ok(self.items[self.items.len() - 1 - index])
    }

    /// 设置指定位置的元素值
    /// index=0表示栈顶元素
    ///
    /// # 错误
    /// 如果索引无效，返回StackError::InvalidIndex
    pub fn set(&mut self, index: usize, value: U256) -> Result<(), StackError> {
        if index >= self.items.len() {
            return Err(StackError::InvalidIndex(index));
        }

        let actual_index = self.items.len() - 1 - index;
        self.items[actual_index] = value;
        self.update_fairness_weight(1);
        Ok(())
    }

    /// 获取栈所有元素的克隆
    pub fn items(&self) -> Vec<U256> {
        self.items.clone()
    }

    /// 清空栈
    pub fn clear(&mut self) {
        self.items.clear();
        self.fairness_weight = 0;
    }

    /// 获取公平性权重
    pub fn fairness_weight(&self) -> u64 {
        self.fairness_weight
    }

    /// 更新公平性权重
    fn update_fairness_weight(&mut self, weight: u64) {
        self.fairness_weight += weight;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_stack_operations() {
        let mut stack = Stack::new();

        // 测试基本入栈和出栈操作
        stack.push(U256::from(1)).unwrap();
        stack.push(U256::from(2)).unwrap();
        assert_eq!(stack.pop().unwrap(), U256::from(2));
        assert_eq!(stack.pop().unwrap(), U256::from(1));

        // 测试栈大小
        assert_eq!(stack.len(), 0);
        stack.push(U256::from(1)).unwrap();
        assert_eq!(stack.len(), 1);

        // 测试栈清空
        stack.clear();
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn test_stack_overflow() {
        let mut stack = Stack::new();
        let max_size = 1024;

        // 测试栈溢出
        for i in 0..max_size {
            stack.push(U256::from(i)).unwrap();
        }
        assert_eq!(stack.push(U256::from(max_size)), Err(StackError::DepthLimitExceeded));

        // 测试栈大小限制
        assert_eq!(stack.len(), max_size);
    }

    #[test]
    fn test_stack_underflow() {
        let mut stack = Stack::new();

        // 测试栈下溢
        assert_eq!(stack.pop(), Err(StackError::Underflow));
        assert_eq!(stack.peek(), Err(StackError::Underflow));

        // 测试空栈操作
        stack.push(U256::from(1)).unwrap();
        stack.pop().unwrap();
        assert_eq!(stack.pop(), Err(StackError::Underflow));
    }

    #[test]
    fn test_stack_peek() {
        let mut stack = Stack::new();

        // 测试查看栈顶元素
        stack.push(U256::from(1)).unwrap();
        assert_eq!(stack.peek().unwrap(), U256::from(1));
        assert_eq!(stack.len(), 1);

        // 测试查看多个元素
        stack.push(U256::from(2)).unwrap();
        assert_eq!(stack.get(0).unwrap(), U256::from(2));
        assert_eq!(stack.get(1).unwrap(), U256::from(1));
    }

    #[test]
    fn test_stack_swap() {
        let mut stack = Stack::new();

        // 测试交换操作
        stack.push(U256::from(1)).unwrap();
        stack.push(U256::from(2)).unwrap();
        stack.swap(1).unwrap();
        assert_eq!(stack.pop().unwrap(), U256::from(1));
        assert_eq!(stack.pop().unwrap(), U256::from(2));

        // 测试无效交换
        stack.push(U256::from(1)).unwrap();
        assert_eq!(stack.swap(0), Err(StackError::InvalidIndex(0)));
        assert_eq!(stack.swap(5), Err(StackError::InvalidIndex(5)));
    }

    #[test]
    fn test_stack_dup() {
        let mut stack = Stack::new();

        // 测试复制操作
        stack.push(U256::from(1)).unwrap();
        stack.push(U256::from(2)).unwrap();
        stack.dup(1).unwrap();
        assert_eq!(stack.pop().unwrap(), U256::from(1));
        assert_eq!(stack.pop().unwrap(), U256::from(2));
        assert_eq!(stack.pop().unwrap(), U256::from(1));

        // 测试无效复制
        assert_eq!(stack.dup(1), Err(StackError::InvalidIndex(1)));
    }

    #[test]
    fn test_stack_operations_with_large_values() {
        let mut stack = Stack::new();
        let large_value = U256::MAX;

        stack.push(large_value).unwrap();
        assert_eq!(stack.peek().unwrap(), large_value);
        assert_eq!(stack.pop().unwrap(), large_value);
    }

    #[test]
    fn test_stack_operations_with_zero() {
        let mut stack = Stack::new();
        let zero = U256::ZERO;

        stack.push(zero).unwrap();
        assert_eq!(stack.peek().unwrap(), zero);
        assert_eq!(stack.pop().unwrap(), zero);
    }

    #[test]
    fn test_stack_operations_with_multiple_values() {
        let mut stack = Stack::new();

        // 测试多个值的操作
        for i in 0..10 {
            stack.push(U256::from(i)).unwrap();
        }

        // 测试获取和设置
        assert_eq!(stack.get(5).unwrap(), U256::from(4));
        stack.set(5, U256::from(42)).unwrap();
        assert_eq!(stack.get(5).unwrap(), U256::from(42));

        // 测试交换
        stack.swap(9).unwrap();
        assert_eq!(stack.get(0).unwrap(), U256::from(0));
        assert_eq!(stack.get(9).unwrap(), U256::from(9));

        // 测试复制
        stack.dup(5).unwrap();
        assert_eq!(stack.get(0).unwrap(), U256::from(42));
    }

    #[test]
    fn test_stack_operations_with_invalid_indices() {
        let mut stack = Stack::new();
        stack.push(U256::from(1)).unwrap();

        // 测试无效索引
        assert_eq!(stack.get(1), Err(StackError::InvalidIndex(1)));
        assert_eq!(stack.set(1, U256::from(2)), Err(StackError::InvalidIndex(1)));
        assert_eq!(stack.swap(1), Err(StackError::InvalidIndex(1)));
        assert_eq!(stack.dup(1), Err(StackError::InvalidIndex(1)));
    }

    #[test]
    fn test_stack_operations_with_clear() {
        let mut stack = Stack::new();

        // 填充栈
        for i in 0..5 {
            stack.push(U256::from(i)).unwrap();
        }

        // 测试清空
        stack.clear();
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);

        // 测试清空后可以继续使用
        stack.push(U256::from(1)).unwrap();
        assert_eq!(stack.len(), 1);
    }

    #[test]
    fn test_stack_fairness_weight() {
        let mut stack = Stack::new();

        // 测试基本操作的公平性权重
        stack.push(U256::from(1)).unwrap();
        assert_eq!(stack.fairness_weight(), 1);

        stack.push(U256::from(2)).unwrap();
        assert_eq!(stack.fairness_weight(), 2);

        stack.pop().unwrap();
        assert_eq!(stack.fairness_weight(), 3);

        stack.swap(0).unwrap();
        assert_eq!(stack.fairness_weight(), 5);

        stack.set(0, U256::from(3)).unwrap();
        assert_eq!(stack.fairness_weight(), 6);

        // 测试清空重置公平性权重
        stack.clear();
        assert_eq!(stack.fairness_weight(), 0);
    }
} 