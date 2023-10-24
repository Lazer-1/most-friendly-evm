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
            if offset + size > value.len() {
                panic!("invalid memory: store empty");
            }
        }

        self.store[offset..offset + size].copy_from_slice(&value);
    }

    // Set32 sets the 32 bytes starting at offset to the value of val, left-padded
    // with zeroes to 32 bytes.
    pub fn set_32(&mut self, offset: usize, val: U256) {
        // length of store may never be less than offset + size.
        // The store should be resized PRIOR to setting the memory
        if offset + 32 > self.store.len() {
            panic!("invalid memory: store empty");
        }
        // Fill in relevant bits
        let b32 = val.to_be_bytes();
        self.store[offset..offset + 32].copy_from_slice(&b32);
    }

    /// Resize resizes the memory to size
    pub fn resize(&mut self, size: usize) {
        if self.store.len() < size {
            self.store.resize(size - self.store.len(), 0)
        }
    }

    /// GetCopy returns offset + size as a new slice
    pub fn get_copy(&self, offset: usize, size: usize) -> Option<&[u8]> {
        if size == 0 {
            return None;
        }

        if self.len() > offset {
            return Some(&self.store[offset..offset + size]);
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
        self.store.copy_within(src..src + len, dst);
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl AsRef<[u8]> for Memory {
    fn as_ref(&self) -> &[u8] {
        &self.store
    }
}
