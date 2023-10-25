/// bitvec is a bit vector which maps bytes in a program.
/// An unset bit means the byte is an opcode, a set bit means
/// it's data (i.e. argument of PUSHxx).
pub struct BitVec {
    inner: Vec<u8>,
}
