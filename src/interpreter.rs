use crate::{memory::Memory, stack::Stack};

// Options for the Interpreter
pub struct Config {
    // Uncomment when EvmLogger is implemented
    // tracer: EvmLogger
    /// Forces the EIP-1559 baseFee to 0 (needed for 0 price calls)
    no_base_fee: bool,
    /// Enables recording of SHA3/keccak preimages
    enable_pre_image_recording: bool,
    /// Additional EIPS that are to be enabled
    extra_eips: Vec<u32>,
}

/// ScopeContext contains the things that are per-call, such as stack and
/// memory, but not transients like pc and gas
pub struct ScopeContext {
    memory: Memory,
    stack: Stack,
    // Uncomment when Contract is implemented
    // contract: Contract,
}
