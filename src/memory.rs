use std::usize;

use ethnum::U256;

/// The EVM has a volatile space called memory which is used to store data
/// during execution. This memory is organized into 32-byte words.
pub struct Memory {
    store: Vec<u8>,
    #[allow(dead_code)]
    last_gas_cost: u64,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            store: Vec::new(),
            last_gas_cost: 0,
        }
    }

    /// Set sets offset + size to value
    pub fn set(&mut self, offset: usize, size: usize, value: Vec<u8>) {
        // It's possible the offset is greater than 0 and size equals 0. This is because
        // the calcMemSize (common.go) could potentially return 0 when size is zero
        // (NO-OP)
        if size > 0 {
            // length of store may never be less than offset + size.
            // The store should be resized PRIOR to setting the memory
            if offset + size > self.store.len() {
                panic!("invalid memory: store size is {} but will need minimum size of {}; failed to write {} bytes from {} to {}",
                    self.store.len(), offset + size, size, offset, offset + size);
            }
        }

        // QS: why is size === value.len() not checked?
        // Though, this function will panic if the two slices have different lengths.
        self.store[offset..offset + size].copy_from_slice(&value);
    }

    // Set32 sets the 32 bytes starting at offset to the value of val, left-padded
    // with zeroes to 32 bytes.
    pub fn set_32(&mut self, offset: usize, val: U256) {
        // length of store may never be less than offset + size.
        // The store should be resized PRIOR to setting the memory
        if offset + 32 > self.store.len() {
            panic!("invalid memory: store size is {} but will need minimum size of {}; failed to write 32 bytes from {} to {}", self.store.len(), offset + 32, offset, offset + 32);
        }
        // Fill in relevant bits
        let b32 = val.to_be_bytes();
        // This function will panic if the two slices have different lengths.
        self.store[offset..offset + 32].copy_from_slice(&b32);
    }

    /// Resize resizes the memory to size
    ///
    /// NOTE: resizing can only grow the memory, not shrink it.
    ///
    /// NOTE: resizing must happen before setting memory using other methods
    pub fn resize(&mut self, size: usize) {
        if self.store.len() < size {
            self.store.resize(size - self.store.len(), 0)
        }
    }

    /// GetCopy returns offset + size as a new slice
    pub fn get_copy(&self, offset: usize, size: usize) -> Option<Vec<u8>> {
        if size == 0 {
            return None;
        }

        if self.len() > offset {
            return Some(self.store[offset..offset + size].to_vec());
        }

        None
    }

    /// Len returns the length of the backing slice
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.store.len()
    }

    /// Copy copies data from the src position slice into the dst position.
    /// The source and destination may overlap.
    // OBS: This operation assumes that any necessary memory expansion has already
    // been performed, and this method may panic otherwise.
    pub fn copy(&mut self, dst: usize, src: usize, len: usize) {
        if len == 0 {
            return;
        }
        // copy_within panics if the range is out of bounds
        self.store.copy_within(src..src + len, dst);
    }
}

#[cfg(not(tarpaulin_include))]
impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(not(tarpaulin_include))]
impl AsRef<[u8]> for Memory {
    fn as_ref(&self) -> &[u8] {
        &self.store
    }
}

#[cfg(test)]
mod tests {
    use ethnum::uint;

    use super::*;

    #[test]
    fn can_resize() {
        let mut mem = Memory::new();
        mem.resize(32);
        assert_eq!(mem.store.len(), 32);
    }

    #[test]
    fn does_not_resize_when_size_is_bigger_than_store_len() {
        let mut mem = Memory::new();
        mem.resize(32);
        mem.resize(16);
        assert_eq!(mem.store.len(), 32);
    }

    #[test]
    fn can_set() {
        let mut mem = Memory::new();
        mem.resize(32);
        mem.set(0, 32, vec![0x01; 32]);
        assert_eq!(mem.store, vec![0x01; 32]);
    }

    #[test]
    #[should_panic]
    fn cannot_set_when_memory_too_small() {
        let mut mem = Memory::new();
        mem.resize(32);
        mem.set(0, 64, vec![0x01; 64]);
    }

    #[test]
    fn can_set_32() {
        let mut mem = Memory::new();
        mem.resize(32);
        mem.set_32(0, uint!("0x01"));
        // 0x01 is converted to 32 bytes, left-padded with zeroes
        assert_eq!(
            mem.store,
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 1
            ]
        );
    }

    #[test]
    #[should_panic]
    fn cannot_set_32_when_memory_too_small() {
        let mut mem = Memory::new();
        mem.resize(30);
        let test = uint!("0x001122334455667788990011223344556677889900112233445566778899001122");
        mem.set_32(0, test);
    }

    #[test]
    #[should_panic]
    fn cannot_set_32_when_remaining_memory_too_small() {
        let mut mem = Memory::new();
        mem.resize(32);
        let test = uint!("0x001122334455667788990011223344556677889900112233445566778899001122");
        mem.set_32(2, test);
    }

    #[test]
    fn can_get_copy() {
        let mut mem = Memory::new();
        mem.resize(32);
        mem.set(0, 32, [vec![0x01; 16], vec![0x02; 16]].concat());
        let result = mem.get_copy(16, 16);
        assert_eq!(result, Some(vec![0x02; 16]));
    }

    #[test]
    fn can_get_copy_when_size_is_zero() {
        let mut mem = Memory::new();
        mem.resize(32);
        mem.set(0, 32, vec![0x01; 32]);
        let result = mem.get_copy(0, 0);
        assert_eq!(result, None);
    }

    #[test]
    fn can_get_copy_when_offset_greater_than_len() {
        let mut mem = Memory::new();
        mem.resize(32);
        mem.set(0, 32, vec![0x01; 32]);
        let result = mem.get_copy(32, 1);
        assert_eq!(result, None);
    }

    #[test]
    fn can_get_len() {
        let mut mem = Memory::new();
        mem.resize(10);
        assert_eq!(mem.len(), 10);
    }

    #[test]
    fn can_copy() {
        let mut mem = Memory::new();
        mem.resize(32);
        mem.set(0, 32, [vec![0x01; 16], vec![0x02; 16]].concat());
        mem.copy(0, 16, 16);
        assert_eq!(mem.store, vec![0x02; 32]);
    }

    #[test]
    fn can_copy_when_len_is_zero() {
        let mut mem = Memory::new();
        mem.resize(32);
        mem.set(0, 32, [vec![0x01; 16], vec![0x02; 16]].concat());
        mem.copy(0, 16, 0);
        assert_eq!(mem.store, [vec![0x01; 16], vec![0x02; 16]].concat());
    }
}
