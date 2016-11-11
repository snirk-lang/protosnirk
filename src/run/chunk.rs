/// Represents a chunk of protosnirk code which has been compiled.

use super::value::Value;
use super::instruction::Instruction;

pub struct Chunk {
    pub stack_size: u8,
    pub constants: Vec<Value>,
    pub instructions: Vec<Instruction>
}
impl Chunk {
    pub fn new(consts: Vec<Value>, instructions: Vec<Instruction>, stack_size: u8) -> Chunk {
        Chunk {
            constants: consts,
            instructions: instructions,
            stack_size: stack_size
        }
    }
}

