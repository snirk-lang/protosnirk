/// Represents a chunk of protosnirk code which has been compiled.

use super::value::Value;
use super::instruction::Instruction;

/// Contains information for running a block or function
#[derive(Debug, Clone, PartialEq)]
pub struct Chunk {
    /// Max number of registers used
    pub register_count: u8,
    /// List of constants associated with this chunk
    pub constants: Vec<Value>,
    /// List of instructions compiled for the chunk
    pub instructions: Vec<Instruction>
}
impl Chunk {
    pub fn new(consts: Vec<Value>, instructions: Vec<Instruction>, register_count: u8) -> Chunk {
        Chunk {
            constants: consts,
            instructions: instructions,
            register_count: register_count
        }
    }
}
