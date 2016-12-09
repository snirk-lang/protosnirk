//! Instructions written on a paper tape, interpreted by the runtime
//! to move electrons around.

/// Index of a local register in a chunk.
pub type RegisterIx = u8;
/// Max number of registers for a chunk/frame.
pub const MAX_REGISTERS: u8 = 64;
/// Max number of constants for chunk
pub const MAX_CONSTANTS: u8 = 128;

// loadconst dest: RegisterIx src: ConstIx
// move dest: RegisterIx src: RegisterIx
// add/sub/div/mul/mod dest: RegisterIx left: RegisterIx, right: RegisterIx

/// Opcodes
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum OpCode {
    LoadConst,
    Move,
    Add,
    Sub,
    Div,
    Mul,
    Mod,
    Return
}

/// A single instruction in the protosnirk VM
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Instruction {
    pub op: OpCode,
    pub left: RegisterIx,
    pub right: RegisterIx,
    pub dest: RegisterIx
}

impl Instruction {
    pub fn add(left: RegisterIx, right: RegisterIx, dest: RegisterIx) -> Instruction {
        Instruction {
            op: OpCode::Add,
            left: left,
            right: right,
            dest: dest
        }
    }
    pub fn sub(left: RegisterIx, right: RegisterIx, dest: RegisterIx) -> Instruction {
        Instruction {
            op: OpCode::Sub,
            left: left,
            right: right,
            dest: dest
        }
    }
    pub fn div(left: RegisterIx, right: RegisterIx, dest: RegisterIx) -> Instruction {
        Instruction {
            op: OpCode::Div,
            left: left,
            right: right,
            dest: dest
        }
    }
    pub fn mul(left: RegisterIx, right: RegisterIx, dest: RegisterIx) -> Instruction {
        Instruction {
            op: OpCode::Mul,
            left: left,
            right: right,
            dest: dest
        }
    }
    pub fn modulo(left: RegisterIx, right: RegisterIx, dest: RegisterIx) -> Instruction {
        Instruction {
            op: OpCode::Mod,
            left: left,
            right: right,
            dest: dest
        }
    }
    pub fn move_(src: RegisterIx, dest: RegisterIx) -> Instruction {
        Instruction {
            op: OpCode::Move,
            left: src,
            right: 0,
            dest: dest
        }
    }
    pub fn load_const(src: RegisterIx, dest: RegisterIx) -> Instruction {
        Instruction {
            op: OpCode::LoadConst,
            left: src,
            right: 0,
            dest: dest
        }
    }
    pub fn return_(value: RegisterIx) -> Instruction {
        Instruction {
            op: OpCode::Return,
            left: value,
            right: 0,
            dest: 0
        }
    }
}
