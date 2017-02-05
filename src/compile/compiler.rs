use parse::{Program, ASTVisitor};
use run::{self, Chunk};
use super::chunk_creator::ChunkCreator;

pub struct Compiler { }
impl Compiler {
    pub fn compile(&mut self, program: Program) -> Chunk {
        let (unit, symbols, constants, _errors) = program.decompose();
        let mut creator = ChunkCreator::new(symbols, constants, run::MAX_REGISTERS);
        creator.check_unit(&unit);
        let (constants, instructions, max_registers) = creator.decompose();
        Chunk::new(constants, instructions, max_registers)
    }
}
