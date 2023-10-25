use ethnum::U256;

/// A general purpose stack of 256-bit words. This was chosen because it is
/// convenient for Ethereum's core cryptographic operations such as Keccak-256
/// hashing and elliptic curve computations.
///
/// Stack is an object for basic stack operations. Items popped to the stack are
/// expected to be changed and modified. Stack does not take care of adding
/// newly initialised objects.
///
/// The max number of items the stack can have is 1024. If you try to push more
/// items than this, then execution will have to crash due to a stack overflow.
///
/// If you attempt to run an opcode that requires more parameters that currently
/// in the stack (e.g. running ADD with a stack size of zero or one), then
/// execution will have to crash due to a stack underflow.
pub struct Stack {
    /// The stack data. The element at the back of the vector is the topmost
    data: Vec<U256>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { data: Vec::new() }
    }

    /// Get a reference to the stack data
    pub fn data(&self) -> &Vec<U256> {
        &self.data
    }

    /// Push a value onto the stack
    /// TODO: The maximum size of the stack is 1024 bytes
    pub fn push(&mut self, value: U256) {
        self.data.push(value);
    }

    /// Pop the topmost element of the stack
    pub fn pop(&mut self) -> Option<U256> {
        self.data.pop()
    }

    /// Get the number of elements in the stack
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Swap the last element with the nth element from the top.
    /// Note that `n` starts from 1 (not 0)
    pub fn swap(&mut self, n: usize) {
        let len = self.len();
        let last = self.data[len - 1];
        let num = self.data[len - n];
        self.data[len - 1] = num;
        self.data[len - n] = last;
    }

    /// Duplicate the nth element from the top
    /// and push it onto the stack.
    /// Note that `n` starts from 1 (not 0)
    pub fn dup(&mut self, n: usize) {
        let len = self.len();
        let num = self.data[len - n];
        self.data.push(num);
    }

    /// Get the topmost element of the stack
    pub fn peek(&self) -> Option<&U256> {
        self.data.last()
    }

    /// Back returns the n'th item in stack.
    /// Unfortunately, unlike other functions, `n` starts from 0 (not 1).
    pub fn back(&self, n: usize) -> Option<&U256> {
        let len = self.len();
        self.data.get(len - n - 1)
    }
}

#[cfg(not(tarpaulin_include))]
impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethnum::uint;

    #[test]
    fn can_push() {
        let mut stack = Stack::new();
        stack.push(uint!("0o1"));
        assert_eq!(stack.len(), 1);
        assert_eq!(stack.data(), &[uint!("0o1")]);
    }

    #[test]
    fn can_pop() {
        let mut stack = Stack::new();
        stack.push(uint!("0o1"));
        assert_eq!(stack.len(), 1);
        assert_eq!(stack.data(), &[uint!("0o1")]);

        let value = stack.pop();
        assert_eq!(value, Some(uint!("0o1")));
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn can_swap_middle() {
        let mut stack = Stack::new();
        stack.push(uint!("0o1"));
        stack.push(uint!("0o2"));
        stack.push(uint!("0o3"));
        assert_eq!(stack.len(), 3);
        assert_eq!(stack.data(), &[uint!("0o1"), uint!("0o2"), uint!("0o3")]);

        stack.swap(2);
        assert_eq!(stack.len(), 3);
        assert_eq!(stack.data(), &[uint!("0o1"), uint!("0o3"), uint!("0o2")]);
    }

    #[test]
    fn can_swap_with_top() {
        let mut stack = Stack::new();
        stack.push(uint!("0o1"));
        stack.push(uint!("0o2"));
        stack.push(uint!("0o3"));
        assert_eq!(stack.len(), 3);
        assert_eq!(stack.data(), &[uint!("0o1"), uint!("0o2"), uint!("0o3")]);

        stack.swap(1);
        assert_eq!(stack.len(), 3);
        assert_eq!(stack.data(), &[uint!("0o1"), uint!("0o2"), uint!("0o3")]);
    }

    #[test]
    fn can_swap_with_bottom() {
        let mut stack = Stack::new();
        stack.push(uint!("0o1"));
        stack.push(uint!("0o2"));
        stack.push(uint!("0o3"));
        assert_eq!(stack.len(), 3);
        assert_eq!(stack.data(), &[uint!("0o1"), uint!("0o2"), uint!("0o3")]);

        stack.swap(3);
        assert_eq!(stack.len(), 3);
        assert_eq!(stack.data(), &[uint!("0o3"), uint!("0o2"), uint!("0o1")]);
    }

    #[test]
    fn can_dup_middle() {
        let mut stack = Stack::new();
        stack.push(uint!("0o1"));
        stack.push(uint!("0o2"));
        stack.push(uint!("0o3"));
        assert_eq!(stack.len(), 3);
        assert_eq!(stack.data(), &[uint!("0o1"), uint!("0o2"), uint!("0o3")]);

        stack.dup(2);
        assert_eq!(stack.len(), 4);
        assert_eq!(
            stack.data(),
            &[uint!("0o1"), uint!("0o2"), uint!("0o3"), uint!("0o2")]
        );
    }

    #[test]
    fn can_dup_top() {
        let mut stack = Stack::new();
        stack.push(uint!("0o1"));
        stack.push(uint!("0o2"));
        stack.push(uint!("0o3"));
        assert_eq!(stack.len(), 3);
        assert_eq!(stack.data(), &[uint!("0o1"), uint!("0o2"), uint!("0o3")]);

        stack.dup(1);
        assert_eq!(stack.len(), 4);
        assert_eq!(
            stack.data(),
            &[uint!("0o1"), uint!("0o2"), uint!("0o3"), uint!("0o3")]
        );
    }

    #[test]
    fn can_dup_bottom() {
        let mut stack = Stack::new();
        stack.push(uint!("0o1"));
        stack.push(uint!("0o2"));
        stack.push(uint!("0o3"));
        assert_eq!(stack.len(), 3);
        assert_eq!(stack.data(), &[uint!("0o1"), uint!("0o2"), uint!("0o3")]);

        stack.dup(3);
        assert_eq!(stack.len(), 4);
        assert_eq!(
            stack.data(),
            &[uint!("0o1"), uint!("0o2"), uint!("0o3"), uint!("0o1")]
        );
    }

    #[test]
    fn can_peek() {
        let mut stack = Stack::new();
        stack.push(uint!("0o1"));
        stack.push(uint!("0o2"));
        stack.push(uint!("0o3"));
        assert_eq!(stack.len(), 3);
        assert_eq!(stack.data(), &[uint!("0o1"), uint!("0o2"), uint!("0o3")]);

        let value = stack.peek();
        assert_eq!(value, Some(&uint!("0o3")));
        assert_eq!(stack.len(), 3);
    }

    #[test]
    fn can_back() {
        let mut stack = Stack::new();
        stack.push(uint!("0o1"));
        stack.push(uint!("0o2"));
        stack.push(uint!("0o3"));
        assert_eq!(stack.len(), 3);
        assert_eq!(stack.data(), &[uint!("0o1"), uint!("0o2"), uint!("0o3")]);

        let value = stack.back(1);
        assert_eq!(value, Some(&uint!("0o2")));
        assert_eq!(stack.len(), 3);
    }
}
