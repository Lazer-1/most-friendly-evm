/// Address represents the 20 byte address of an Ethereum account.
pub struct Address {
    inner: [u8; 20],
}

impl Address {
    pub fn new(inner: [u8; 20]) -> Self {
        Self { inner }
    }
}

impl AsRef<[u8]> for Address {
    fn as_ref(&self) -> &[u8] {
        &self.inner
    }
}
