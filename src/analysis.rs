/// bitvec is a bit vector which maps bytes in a program.
/// An unset bit means the byte is an opcode, a set bit means
/// it's data (i.e. argument of PUSHxx).
pub struct BitVec {
    inner: Vec<u8>,
}

impl BitVec {
  const SET_2_BITS_MASK: u16 = 0b11;
  const SET_3_BITS_MASK: u16 = 0b111;
  const SET_4_BITS_MASK: u16 = 0b1111;
  const SET_5_BITS_MASK: u16 = 0b1111_1;
  const SET_6_BITS_MASK: u16 = 0b1111_11;
  const SET_7_BITS_MASK: u16 = 0b1111_111;

  fn set_1(&mut self, pos: usize) {
  	self.inner[pos / 8] |= 1 << (pos % 8)
  }
}