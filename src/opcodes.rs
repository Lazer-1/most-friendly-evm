use thiserror::Error;

/// Abstraction over opcode. The full list of latest opcodes are available at https://github.com/ethereum/go-ethereum/blob/master/core/vm/opcodes.go.
///
/// Each opcode is a byte.
pub struct Opcode {
    byte: u8,
}

#[derive(Error, Debug)]
#[error("invalid opcode: {byte}")]
pub struct InvalidOpcode {
    byte: u8,
}

#[derive(Error, Debug)]
#[error("invalid opcode name: {name}")]
pub struct InvalidOpcodeName {
    name: String,
}

impl Opcode {
    // 0x0 range - arithmetic ops.
    pub const STOP: Self = Self { byte: 0x0 };
    pub const ADD: Self = Self { byte: 0x1 };
    pub const MUL: Self = Self { byte: 0x2 };
    pub const SUB: Self = Self { byte: 0x3 };
    pub const DIV: Self = Self { byte: 0x4 };
    pub const SDIV: Self = Self { byte: 0x5 };
    pub const MOD: Self = Self { byte: 0x6 };
    pub const SMOD: Self = Self { byte: 0x7 };
    pub const ADDMOD: Self = Self { byte: 0x8 };
    pub const MULMOD: Self = Self { byte: 0x9 };
    pub const EXP: Self = Self { byte: 0xa };
    pub const SIGNEXTEND: Self = Self { byte: 0xb };

    // 0x10 range - comparison ops.
    pub const LT: Self = Self { byte: 0x10 };
    pub const GT: Self = Self { byte: 0x11 };
    pub const SLT: Self = Self { byte: 0x12 };
    pub const SGT: Self = Self { byte: 0x13 };
    pub const EQ: Self = Self { byte: 0x14 };
    pub const ISZERO: Self = Self { byte: 0x15 };
    pub const AND: Self = Self { byte: 0x16 };
    pub const OR: Self = Self { byte: 0x17 };
    pub const XOR: Self = Self { byte: 0x18 };
    pub const NOT: Self = Self { byte: 0x19 };
    pub const BYTE: Self = Self { byte: 0x1a };
    pub const SHL: Self = Self { byte: 0x1b };
    pub const SHR: Self = Self { byte: 0x1c };
    pub const SAR: Self = Self { byte: 0x1d };

    // 0x20 range - crypto.
    pub const KECCAK256: Self = Self { byte: 0x20 };

    // 0x30 range - closure state.
    pub const ADDRESS: Self = Self { byte: 0x30 };
    pub const BALANCE: Self = Self { byte: 0x31 };
    pub const ORIGIN: Self = Self { byte: 0x32 };
    pub const CALLER: Self = Self { byte: 0x33 };
    pub const CALLVALUE: Self = Self { byte: 0x34 };
    pub const CALLDATALOAD: Self = Self { byte: 0x35 };
    pub const CALLDATASIZE: Self = Self { byte: 0x36 };
    pub const CALLDATACOPY: Self = Self { byte: 0x37 };
    pub const CODESIZE: Self = Self { byte: 0x38 };
    pub const CODECOPY: Self = Self { byte: 0x39 };
    pub const GASPRICE: Self = Self { byte: 0x3a };
    pub const EXTCODESIZE: Self = Self { byte: 0x3b };
    pub const EXTCODECOPY: Self = Self { byte: 0x3c };
    pub const RETURNDATASIZE: Self = Self { byte: 0x3d };
    pub const RETURNDATACOPY: Self = Self { byte: 0x3e };
    pub const EXTCODEHASH: Self = Self { byte: 0x3f };

    // 0x40 range - block operations.
    pub const BLOCKHASH: Self = Self { byte: 0x40 };
    pub const COINBASE: Self = Self { byte: 0x41 };
    pub const TIMESTAMP: Self = Self { byte: 0x42 };
    pub const NUMBER: Self = Self { byte: 0x43 };
    pub const DIFFICULTY: Self = Self { byte: 0x44 };
    pub const RANDOM: Self = Self { byte: 0x44 }; // Same as DIFFICULTY
    pub const PREVRANDAO: Self = Self { byte: 0x44 }; // Same as DIFFICULTY
    pub const GASLIMIT: Self = Self { byte: 0x45 };
    pub const CHAINID: Self = Self { byte: 0x46 };
    pub const SELFBALANCE: Self = Self { byte: 0x47 };
    pub const BASEFEE: Self = Self { byte: 0x48 };
    pub const BLOBHASH: Self = Self { byte: 0x49 };
    pub const BLOBBASEFEE: Self = Self { byte: 0x4a };

    // 0x50 range - 'storage' and execution.
    pub const POP: Self = Self { byte: 0x50 };
    pub const MLOAD: Self = Self { byte: 0x51 };
    pub const MSTORE: Self = Self { byte: 0x52 };
    pub const MSTORE8: Self = Self { byte: 0x53 };
    pub const SLOAD: Self = Self { byte: 0x54 };
    pub const SSTORE: Self = Self { byte: 0x55 };
    pub const JUMP: Self = Self { byte: 0x56 };
    pub const JUMPI: Self = Self { byte: 0x57 };
    pub const PC: Self = Self { byte: 0x58 };
    pub const MSIZE: Self = Self { byte: 0x59 };
    pub const GAS: Self = Self { byte: 0x5a };
    pub const JUMPDEST: Self = Self { byte: 0x5b };
    pub const TLOAD: Self = Self { byte: 0x5c };
    pub const TSTORE: Self = Self { byte: 0x5d };
    pub const MCOPY: Self = Self { byte: 0x5e };
    pub const PUSH0: Self = Self { byte: 0x5f };

    // 0x60 range - pushes.
    pub const PUSH1: Self = Self { byte: 0x60 };
    pub const PUSH2: Self = Self { byte: 0x61 };
    pub const PUSH3: Self = Self { byte: 0x62 };
    pub const PUSH4: Self = Self { byte: 0x63 };
    pub const PUSH5: Self = Self { byte: 0x64 };
    pub const PUSH6: Self = Self { byte: 0x65 };
    pub const PUSH7: Self = Self { byte: 0x66 };
    pub const PUSH8: Self = Self { byte: 0x67 };
    pub const PUSH9: Self = Self { byte: 0x68 };
    pub const PUSH10: Self = Self { byte: 0x69 };
    pub const PUSH11: Self = Self { byte: 0x6a };
    pub const PUSH12: Self = Self { byte: 0x6b };
    pub const PUSH13: Self = Self { byte: 0x6c };
    pub const PUSH14: Self = Self { byte: 0x6d };
    pub const PUSH15: Self = Self { byte: 0x6e };
    pub const PUSH16: Self = Self { byte: 0x6f };
    pub const PUSH17: Self = Self { byte: 0x70 };
    pub const PUSH18: Self = Self { byte: 0x71 };
    pub const PUSH19: Self = Self { byte: 0x72 };
    pub const PUSH20: Self = Self { byte: 0x73 };
    pub const PUSH21: Self = Self { byte: 0x74 };
    pub const PUSH22: Self = Self { byte: 0x75 };
    pub const PUSH23: Self = Self { byte: 0x76 };
    pub const PUSH24: Self = Self { byte: 0x77 };
    pub const PUSH25: Self = Self { byte: 0x78 };
    pub const PUSH26: Self = Self { byte: 0x79 };
    pub const PUSH27: Self = Self { byte: 0x7a };
    pub const PUSH28: Self = Self { byte: 0x7b };
    pub const PUSH29: Self = Self { byte: 0x7c };
    pub const PUSH30: Self = Self { byte: 0x7d };
    pub const PUSH31: Self = Self { byte: 0x7e };
    pub const PUSH32: Self = Self { byte: 0x7f };

    // 0x80 range - dups.
    pub const DUP1: Self = Self { byte: 0x80 };
    pub const DUP2: Self = Self { byte: 0x81 };
    pub const DUP3: Self = Self { byte: 0x82 };
    pub const DUP4: Self = Self { byte: 0x83 };
    pub const DUP5: Self = Self { byte: 0x84 };
    pub const DUP6: Self = Self { byte: 0x85 };
    pub const DUP7: Self = Self { byte: 0x86 };
    pub const DUP8: Self = Self { byte: 0x87 };
    pub const DUP9: Self = Self { byte: 0x88 };
    pub const DUP10: Self = Self { byte: 0x89 };
    pub const DUP11: Self = Self { byte: 0x8A };
    pub const DUP12: Self = Self { byte: 0x8B };
    pub const DUP13: Self = Self { byte: 0x8C };
    pub const DUP14: Self = Self { byte: 0x8D };
    pub const DUP15: Self = Self { byte: 0x8E };
    pub const DUP16: Self = Self { byte: 0x8F };

    // 0x90 range - swaps.
    pub const SWAP1: Self = Self { byte: 0x90 };
    pub const SWAP2: Self = Self { byte: 0x91 };
    pub const SWAP3: Self = Self { byte: 0x92 };
    pub const SWAP4: Self = Self { byte: 0x93 };
    pub const SWAP5: Self = Self { byte: 0x94 };
    pub const SWAP6: Self = Self { byte: 0x95 };
    pub const SWAP7: Self = Self { byte: 0x96 };
    pub const SWAP8: Self = Self { byte: 0x97 };
    pub const SWAP9: Self = Self { byte: 0x98 };
    pub const SWAP10: Self = Self { byte: 0x99 };
    pub const SWAP11: Self = Self { byte: 0x9A };
    pub const SWAP12: Self = Self { byte: 0x9B };
    pub const SWAP13: Self = Self { byte: 0x9C };
    pub const SWAP14: Self = Self { byte: 0x9D };
    pub const SWAP15: Self = Self { byte: 0x9E };
    pub const SWAP16: Self = Self { byte: 0x9F };

    // 0xa0 range - logging ops.
    pub const LOG0: Self = Self { byte: 0xa0 };
    pub const LOG1: Self = Self { byte: 0xa1 };
    pub const LOG2: Self = Self { byte: 0xa2 };
    pub const LOG3: Self = Self { byte: 0xa3 };
    pub const LOG4: Self = Self { byte: 0xa4 };

    // 0xf0 range - closures.
    pub const CREATE: Self = Self { byte: 0xf0 };
    pub const CALL: Self = Self { byte: 0xf1 };
    pub const CALLCODE: Self = Self { byte: 0xf2 };
    pub const RETURN: Self = Self { byte: 0xf3 };
    pub const DELEGATECALL: Self = Self { byte: 0xf4 };
    pub const CREATE2: Self = Self { byte: 0xf5 };
    pub const STATICCALL: Self = Self { byte: 0xfa };
    pub const REVERT: Self = Self { byte: 0xfd };
    pub const INVALID: Self = Self { byte: 0xfe };
    pub const SELFDESTRUCT: Self = Self { byte: 0xff };

    /// IsPush returns true if an opcode is a PUSH opcode.
    pub fn is_push(&self) -> bool {
        self.byte >= Opcode::PUSH0.byte && self.byte <= Opcode::PUSH32.byte
    }
}

impl TryFrom<u8> for Opcode {
    type Error = InvalidOpcode;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0x0C..0x0F
            | 0x1E..0x1F
            | 0x21..0x2F
            | 0x4b..0x4F
            | 0xA5..0xEF
            | 0xF6..0xF9
            | 0xFB..0xFC => Err(InvalidOpcode { byte }),
            _ => Ok(Opcode { byte }),
        }
    }
}

impl TryFrom<String> for Opcode {
    type Error = InvalidOpcodeName;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        match name.as_str() {
            "STOP" => Ok(0x00.try_into().unwrap()),
            "ADD" => Ok(0x01.try_into().unwrap()),
            "MUL" => Ok(0x02.try_into().unwrap()),
            "SUB" => Ok(0x03.try_into().unwrap()),
            "DIV" => Ok(0x04.try_into().unwrap()),
            "SDIV" => Ok(0x05.try_into().unwrap()),
            "MOD" => Ok(0x06.try_into().unwrap()),
            "SMOD" => Ok(0x07.try_into().unwrap()),
            "ADDMOD" => Ok(0x08.try_into().unwrap()),
            "MULMOD" => Ok(0x09.try_into().unwrap()),
            "EXP" => Ok(0x0a.try_into().unwrap()),
            "SIGNEXTEND" => Ok(0x0b.try_into().unwrap()),

            "LT" => Ok(0x10.try_into().unwrap()),
            "GT" => Ok(0x11.try_into().unwrap()),
            "SLT" => Ok(0x12.try_into().unwrap()),
            "SGT" => Ok(0x13.try_into().unwrap()),
            "EQ" => Ok(0x14.try_into().unwrap()),
            "ISZERO" => Ok(0x15.try_into().unwrap()),
            "AND" => Ok(0x16.try_into().unwrap()),
            "OR" => Ok(0x17.try_into().unwrap()),
            "XOR" => Ok(0x18.try_into().unwrap()),
            "NOT" => Ok(0x19.try_into().unwrap()),
            "BYTE" => Ok(0x1a.try_into().unwrap()),
            "SHL" => Ok(0x1b.try_into().unwrap()),
            "SHR" => Ok(0x1c.try_into().unwrap()),
            "SAR" => Ok(0x1d.try_into().unwrap()),
            "KECCAK256" => Ok(0x20.try_into().unwrap()),

            "ADDRESS" => Ok(0x30.try_into().unwrap()),
            "BALANCE" => Ok(0x31.try_into().unwrap()),
            "ORIGIN" => Ok(0x32.try_into().unwrap()),
            "CALLER" => Ok(0x33.try_into().unwrap()),
            "CALLVALUE" => Ok(0x34.try_into().unwrap()),
            "CALLDATALOAD" => Ok(0x35.try_into().unwrap()),
            "CALLDATASIZE" => Ok(0x36.try_into().unwrap()),
            "CALLDATACOPY" => Ok(0x37.try_into().unwrap()),
            "CODESIZE" => Ok(0x38.try_into().unwrap()),
            "CODECOPY" => Ok(0x39.try_into().unwrap()),
            "GASPRICE" => Ok(0x3a.try_into().unwrap()),
            "EXTCODESIZE" => Ok(0x3b.try_into().unwrap()),
            "EXTCODECOPY" => Ok(0x3c.try_into().unwrap()),
            "RETURNDATASIZE" => Ok(0x3d.try_into().unwrap()),
            "RETURNDATACOPY" => Ok(0x3e.try_into().unwrap()),
            "EXTCODEHASH" => Ok(0x3f.try_into().unwrap()),

            "COINBASE" => Ok(0x41.try_into().unwrap()),
            "BLOCKHASH" => Ok(0x40.try_into().unwrap()),
            "TIMESTAMP" => Ok(0x42.try_into().unwrap()),
            "NUMBER" => Ok(0x43.try_into().unwrap()),
            "DIFFICULTY" => Ok(0x44.try_into().unwrap()),
            "RANDOM" => Ok(0x44.try_into().unwrap()),
            "PREVRANDAO" => Ok(0x44.try_into().unwrap()),
            "GASLIMIT" => Ok(0x45.try_into().unwrap()),
            "CHAINID" => Ok(0x46.try_into().unwrap()),
            "SELFBALANCE" => Ok(0x47.try_into().unwrap()),
            "BASEFEE" => Ok(0x48.try_into().unwrap()),
            "BLOBHASH" => Ok(0x49.try_into().unwrap()),
            "BLOBBASEFEE" => Ok(0x4a.try_into().unwrap()),

            "POP" => Ok(0x50.try_into().unwrap()),
            "MLOAD" => Ok(0x51.try_into().unwrap()),
            "MSTORE" => Ok(0x52.try_into().unwrap()),
            "MSTORE8" => Ok(0x53.try_into().unwrap()),
            "SLOAD" => Ok(0x54.try_into().unwrap()),
            "SSTORE" => Ok(0x55.try_into().unwrap()),
            "JUMP" => Ok(0x56.try_into().unwrap()),
            "JUMPI" => Ok(0x57.try_into().unwrap()),
            "PC" => Ok(0x58.try_into().unwrap()),
            "MSIZE" => Ok(0x59.try_into().unwrap()),
            "GAS" => Ok(0x5a.try_into().unwrap()),
            "JUMPDEST" => Ok(0x5b.try_into().unwrap()),
            "TLOAD" => Ok(0x5c.try_into().unwrap()),
            "TSTORE" => Ok(0x5d.try_into().unwrap()),
            "MCOPY" => Ok(0x5e.try_into().unwrap()),
            "PUSH0" => Ok(0x5f.try_into().unwrap()),

            "PUSH1" => Ok(0x60.try_into().unwrap()),
            "PUSH2" => Ok(0x61.try_into().unwrap()),
            "PUSH3" => Ok(0x62.try_into().unwrap()),
            "PUSH4" => Ok(0x63.try_into().unwrap()),
            "PUSH5" => Ok(0x64.try_into().unwrap()),
            "PUSH6" => Ok(0x65.try_into().unwrap()),
            "PUSH7" => Ok(0x66.try_into().unwrap()),
            "PUSH8" => Ok(0x67.try_into().unwrap()),
            "PUSH9" => Ok(0x68.try_into().unwrap()),
            "PUSH10" => Ok(0x69.try_into().unwrap()),
            "PUSH11" => Ok(0x6a.try_into().unwrap()),
            "PUSH12" => Ok(0x6b.try_into().unwrap()),
            "PUSH13" => Ok(0x6c.try_into().unwrap()),
            "PUSH14" => Ok(0x6d.try_into().unwrap()),
            "PUSH15" => Ok(0x6e.try_into().unwrap()),
            "PUSH16" => Ok(0x6f.try_into().unwrap()),
            "PUSH17" => Ok(0x70.try_into().unwrap()),
            "PUSH18" => Ok(0x71.try_into().unwrap()),
            "PUSH19" => Ok(0x72.try_into().unwrap()),
            "PUSH20" => Ok(0x73.try_into().unwrap()),
            "PUSH21" => Ok(0x74.try_into().unwrap()),
            "PUSH22" => Ok(0x75.try_into().unwrap()),
            "PUSH23" => Ok(0x76.try_into().unwrap()),
            "PUSH24" => Ok(0x77.try_into().unwrap()),
            "PUSH25" => Ok(0x78.try_into().unwrap()),
            "PUSH26" => Ok(0x79.try_into().unwrap()),
            "PUSH27" => Ok(0x7a.try_into().unwrap()),
            "PUSH28" => Ok(0x7b.try_into().unwrap()),
            "PUSH29" => Ok(0x7c.try_into().unwrap()),
            "PUSH30" => Ok(0x7d.try_into().unwrap()),
            "PUSH31" => Ok(0x7e.try_into().unwrap()),
            "PUSH32" => Ok(0x7f.try_into().unwrap()),

            "DUP1" => Ok(0x80.try_into().unwrap()),
            "DUP2" => Ok(0x81.try_into().unwrap()),
            "DUP3" => Ok(0x82.try_into().unwrap()),
            "DUP4" => Ok(0x83.try_into().unwrap()),
            "DUP5" => Ok(0x84.try_into().unwrap()),
            "DUP6" => Ok(0x85.try_into().unwrap()),
            "DUP7" => Ok(0x86.try_into().unwrap()),
            "DUP8" => Ok(0x87.try_into().unwrap()),
            "DUP9" => Ok(0x88.try_into().unwrap()),
            "DUP10" => Ok(0x89.try_into().unwrap()),
            "DUP11" => Ok(0x8A.try_into().unwrap()),
            "DUP12" => Ok(0x8B.try_into().unwrap()),
            "DUP13" => Ok(0x8C.try_into().unwrap()),
            "DUP14" => Ok(0x8D.try_into().unwrap()),
            "DUP15" => Ok(0x8E.try_into().unwrap()),
            "DUP16" => Ok(0x8F.try_into().unwrap()),

            "SWAP1" => Ok(0x90.try_into().unwrap()),
            "SWAP2" => Ok(0x91.try_into().unwrap()),
            "SWAP3" => Ok(0x92.try_into().unwrap()),
            "SWAP4" => Ok(0x93.try_into().unwrap()),
            "SWAP5" => Ok(0x94.try_into().unwrap()),
            "SWAP6" => Ok(0x95.try_into().unwrap()),
            "SWAP7" => Ok(0x96.try_into().unwrap()),
            "SWAP8" => Ok(0x97.try_into().unwrap()),
            "SWAP9" => Ok(0x98.try_into().unwrap()),
            "SWAP10" => Ok(0x99.try_into().unwrap()),
            "SWAP11" => Ok(0x9A.try_into().unwrap()),
            "SWAP12" => Ok(0x9B.try_into().unwrap()),
            "SWAP13" => Ok(0x9C.try_into().unwrap()),
            "SWAP14" => Ok(0x9D.try_into().unwrap()),
            "SWAP15" => Ok(0x9E.try_into().unwrap()),
            "SWAP16" => Ok(0x9F.try_into().unwrap()),

            "LOG0" => Ok(0xa0.try_into().unwrap()),
            "LOG1" => Ok(0xa1.try_into().unwrap()),
            "LOG2" => Ok(0xa2.try_into().unwrap()),
            "LOG3" => Ok(0xa3.try_into().unwrap()),
            "LOG4" => Ok(0xa4.try_into().unwrap()),

            "CREATE" => Ok(0xf0.try_into().unwrap()),
            "CALL" => Ok(0xf1.try_into().unwrap()),
            "RETURN" => Ok(0xf2.try_into().unwrap()),
            "CALLCODE" => Ok(0xf3.try_into().unwrap()),
            "DELEGATECALL" => Ok(0xf4.try_into().unwrap()),
            "CREATE2" => Ok(0xf5.try_into().unwrap()),
            "STATICCALL" => Ok(0xfa.try_into().unwrap()),
            "REVERT" => Ok(0xfd.try_into().unwrap()),
            "INVALID" => Ok(0xfe.try_into().unwrap()),
            "SELFDESTRUCT" => Ok(0xff.try_into().unwrap()),
            _ => Err(InvalidOpcodeName { name }),
        }
    }
}

impl ToString for Opcode {
    fn to_string(&self) -> String {
        match self.byte {
            // 0x0 range
            0x00 => "STOP".to_string(),
            0x01 => "ADD".to_string(),
            0x02 => "MUL".to_string(),
            0x03 => "SUB".to_string(),
            0x04 => "DIV".to_string(),
            0x05 => "SDIV".to_string(),
            0x06 => "MOD".to_string(),
            0x07 => "SMOD".to_string(),
            0x08 => "ADDMOD".to_string(),
            0x09 => "MULMOD".to_string(),
            0x0a => "EXP".to_string(),
            0x0b => "SIGNEXTEND".to_string(),

            // 0x10 range
            0x10 => "LT".to_string(),
            0x11 => "GT".to_string(),
            0x12 => "SLT".to_string(),
            0x13 => "SGT".to_string(),
            0x14 => "EQ".to_string(),
            0x15 => "ISZERO".to_string(),
            0x16 => "AND".to_string(),
            0x17 => "OR".to_string(),
            0x18 => "XOR".to_string(),
            0x19 => "NOT".to_string(),
            0x1a => "BYTE".to_string(),
            0x1b => "SHL".to_string(),
            0x1c => "SHR".to_string(),
            0x1d => "SAR".to_string(),

            0x20 => "KECCAK256".to_string(),

            0x30 => "ADDRESS".to_string(),
            0x31 => "BALANCE".to_string(),
            0x32 => "ORIGIN".to_string(),
            0x33 => "CALLER".to_string(),
            0x34 => "CALLVALUE".to_string(),
            0x35 => "CALLDATALOAD".to_string(),
            0x36 => "CALLDATASIZE".to_string(),
            0x37 => "CALLDATACOPY".to_string(),
            0x38 => "CODESIZE".to_string(),
            0x39 => "CODECOPY".to_string(),
            0x3a => "GASPRICE".to_string(),
            0x3b => "EXTCODESIZE".to_string(),
            0x3c => "EXTCODECOPY".to_string(),
            0x3d => "RETURNDATASIZE".to_string(),
            0x3e => "RETURNDATACOPY".to_string(),
            0x3f => "EXTCODEHASH".to_string(),

            0x41 => "COINBASE".to_string(),
            0x40 => "BLOCKHASH".to_string(),
            0x42 => "TIMESTAMP".to_string(),
            0x43 => "NUMBER".to_string(),
            // TODO: Why is it not still renamed to PREVRANDAO after the merge?
            0x44 => "DIFFICULTY".to_string(),
            0x45 => "GASLIMIT".to_string(),
            0x46 => "CHAINID".to_string(),
            0x47 => "SELFBALANCE".to_string(),
            0x48 => "BASEFEE".to_string(),
            0x49 => "BLOBHASH".to_string(),
            0x4a => "BLOBBASEFEE".to_string(),

            0x50 => "POP".to_string(),
            0x51 => "MLOAD".to_string(),
            0x52 => "MSTORE".to_string(),
            0x53 => "MSTORE8".to_string(),
            0x54 => "SLOAD".to_string(),
            0x55 => "SSTORE".to_string(),
            0x56 => "JUMP".to_string(),
            0x57 => "JUMPI".to_string(),
            0x58 => "PC".to_string(),
            0x59 => "MSIZE".to_string(),
            0x5a => "GAS".to_string(),
            0x5b => "JUMPDEST".to_string(),
            0x5c => "TLOAD".to_string(),
            0x5d => "TSTORE".to_string(),
            0x5e => "MCOPY".to_string(),
            0x5f => "PUSH0".to_string(),

            0x60 => "PUSH1".to_string(),
            0x61 => "PUSH2".to_string(),
            0x62 => "PUSH3".to_string(),
            0x63 => "PUSH4".to_string(),
            0x64 => "PUSH5".to_string(),
            0x65 => "PUSH6".to_string(),
            0x66 => "PUSH7".to_string(),
            0x67 => "PUSH8".to_string(),
            0x68 => "PUSH9".to_string(),
            0x69 => "PUSH10".to_string(),
            0x6a => "PUSH11".to_string(),
            0x6b => "PUSH12".to_string(),
            0x6c => "PUSH13".to_string(),
            0x6d => "PUSH14".to_string(),
            0x6e => "PUSH15".to_string(),
            0x6f => "PUSH16".to_string(),
            0x70 => "PUSH17".to_string(),
            0x71 => "PUSH18".to_string(),
            0x72 => "PUSH19".to_string(),
            0x73 => "PUSH20".to_string(),
            0x74 => "PUSH21".to_string(),
            0x75 => "PUSH22".to_string(),
            0x76 => "PUSH23".to_string(),
            0x77 => "PUSH24".to_string(),
            0x78 => "PUSH25".to_string(),
            0x79 => "PUSH26".to_string(),
            0x7a => "PUSH27".to_string(),
            0x7b => "PUSH28".to_string(),
            0x7c => "PUSH29".to_string(),
            0x7d => "PUSH30".to_string(),
            0x7e => "PUSH31".to_string(),
            0x7f => "PUSH32".to_string(),

            0x80 => "DUP1".to_string(),
            0x81 => "DUP2".to_string(),
            0x82 => "DUP3".to_string(),
            0x83 => "DUP4".to_string(),
            0x84 => "DUP5".to_string(),
            0x85 => "DUP6".to_string(),
            0x86 => "DUP7".to_string(),
            0x87 => "DUP8".to_string(),
            0x88 => "DUP9".to_string(),
            0x89 => "DUP10".to_string(),
            0x8A => "DUP11".to_string(),
            0x8B => "DUP12".to_string(),
            0x8C => "DUP13".to_string(),
            0x8D => "DUP14".to_string(),
            0x8E => "DUP15".to_string(),
            0x8F => "DUP16".to_string(),

            0x90 => "SWAP1".to_string(),
            0x91 => "SWAP2".to_string(),
            0x92 => "SWAP3".to_string(),
            0x93 => "SWAP4".to_string(),
            0x94 => "SWAP5".to_string(),
            0x95 => "SWAP6".to_string(),
            0x96 => "SWAP7".to_string(),
            0x97 => "SWAP8".to_string(),
            0x98 => "SWAP9".to_string(),
            0x99 => "SWAP10".to_string(),
            0x9A => "SWAP11".to_string(),
            0x9B => "SWAP12".to_string(),
            0x9C => "SWAP13".to_string(),
            0x9D => "SWAP14".to_string(),
            0x9E => "SWAP15".to_string(),
            0x9F => "SWAP16".to_string(),

            0xa0 => "LOG0".to_string(),
            0xa1 => "LOG1".to_string(),
            0xa2 => "LOG2".to_string(),
            0xa3 => "LOG3".to_string(),
            0xa4 => "LOG4".to_string(),

            0xf0 => "CREATE".to_string(),
            0xf1 => "CALL".to_string(),
            0xf2 => "RETURN".to_string(),
            0xf3 => "CALLCODE".to_string(),
            0xf4 => "DELEGATECALL".to_string(),
            0xf5 => "CREATE2".to_string(),
            0xfa => "STATICCALL".to_string(),
            0xfd => "REVERT".to_string(),
            0xfe => "INVALID".to_string(),
            0xff => "SELFDESTRUCT".to_string(),

            // This should never happen as long as OpCode
            // constructors are correctly functioning.
            //
            // We make sure that the invariant is respected,
            // so we can safely panic here.
            _ => panic!("{}", InvalidOpcode { byte: self.byte }),
        }
    }
}
