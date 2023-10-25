/// Hash represents the 32 byte Keccak256 hash of arbitrary data.
pub struct Hash {
  inner: [u8; 32],
}

impl Hash {
  pub fn new(inner: [u8; 32]) -> Self {
    Self { inner }
  }
}

impl AsRef<[u8]> for Hash {
  fn as_ref(&self) -> &[u8] {
    &self.inner
  }
}
