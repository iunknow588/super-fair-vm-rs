use ethers::types::U256;

/// EVM栈实现
#[derive(Debug, Clone, Default)]
pub struct Stack {
    /// 栈数据，使用Vec<U256>存储
    items: Vec<U256>,
    /// 栈的最大深度限制
    max_depth: usize,
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
        }
    }

    /// 创建指定最大深度的栈
    pub fn with_max_depth(max_depth: usize) -> Self {
        Self {
            items: Vec::with_capacity(max_depth),
            max_depth,
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
        Ok(())
    }

    /// 从栈顶弹出一个值
    ///
    /// # 错误
    /// 如果栈为空，返回StackError::Underflow
    pub fn pop(&mut self) -> Result<U256, StackError> {
        self.items.pop().ok_or(StackError::Underflow)
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
        Ok(())
    }

    /// 获取栈所有元素的克隆
    pub fn items(&self) -> Vec<U256> {
        self.items.clone()
    }

    /// 清空栈
    pub fn clear(&mut self) {
        self.items.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_push_and_pop() {
        let mut stack = Stack::new();

        // 测试推入和弹出
        stack.push(U256::from(1)).unwrap();
        stack.push(U256::from(2)).unwrap();
        stack.push(U256::from(3)).unwrap();

        assert_eq!(stack.len(), 3);
        assert_eq!(stack.pop().unwrap(), U256::from(3));
        assert_eq!(stack.pop().unwrap(), U256::from(2));
        assert_eq!(stack.pop().unwrap(), U256::from(1));
        assert!(stack.is_empty());

        // 测试空栈弹出错误
        assert!(stack.pop().is_err());
    }

    #[test]
    fn test_stack_peek() {
        let mut stack = Stack::new();

        // 测试空栈peek错误
        assert!(stack.peek().is_err());

        // 测试peek不影响栈内容
        stack.push(U256::from(42)).unwrap();
        assert_eq!(stack.peek().unwrap(), U256::from(42));
        assert_eq!(stack.len(), 1);
    }

    #[test]
    fn test_stack_dup() {
        let mut stack = Stack::new();

        // 设置初始栈
        stack.push(U256::from(1)).unwrap();
        stack.push(U256::from(2)).unwrap();
        stack.push(U256::from(3)).unwrap();

        // 测试DUP1（复制栈顶）
        stack.dup(0).unwrap();
        assert_eq!(stack.pop().unwrap(), U256::from(3));

        // 测试DUP2（复制次栈顶）
        stack.dup(1).unwrap();
        assert_eq!(stack.pop().unwrap(), U256::from(2));

        // 测试无效索引
        assert!(stack.dup(5).is_err());
    }

    #[test]
    fn test_stack_swap() {
        let mut stack = Stack::new();

        // 设置初始栈
        stack.push(U256::from(1)).unwrap();
        stack.push(U256::from(2)).unwrap();
        stack.push(U256::from(3)).unwrap();

        // 测试SWAP1（与次栈顶交换）
        stack.swap(1).unwrap();
        assert_eq!(stack.pop().unwrap(), U256::from(2));
        assert_eq!(stack.pop().unwrap(), U256::from(3));

        // 测试无效索引
        stack.push(U256::from(4)).unwrap();
        assert!(stack.swap(5).is_err());
        assert!(stack.swap(0).is_err()); // 与自己交换是无效的
    }

    #[test]
    fn test_stack_depth_limit() {
        let mut stack = Stack::with_max_depth(3);

        // 填满栈
        stack.push(U256::from(1)).unwrap();
        stack.push(U256::from(2)).unwrap();
        stack.push(U256::from(3)).unwrap();

        // 尝试超出限制
        assert!(stack.push(U256::from(4)).is_err());

        // 弹出后可以再次压入
        stack.pop().unwrap();
        assert!(stack.push(U256::from(4)).is_ok());
    }
}
