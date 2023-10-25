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
