use super::value::Value;
use super::chunk::Chunk;
use super::instruction::{OpCode, Instruction, RegisterIx, MAX_REGISTERS, MAX_CONSTANTS};

pub struct VM { }

impl VM {
    /// Evaluate a given chunk (from scratch)
    pub fn eval_chunk(&mut self, chunk: Chunk) -> Value {
        println!("Evaluate {:#?}", chunk);
        debug_assert!(chunk.register_count < MAX_REGISTERS,
                      "Requested compiling chunk with too many registers");
        debug_assert!(chunk.constants.len() < MAX_CONSTANTS as usize,
                      "Requested compiling chunk with too many constants");
        let mut registers = vec![Value(-999f64); chunk.register_count as usize];
        for inst in chunk.instructions {
            println!("Eval {:?}", inst); // Sure, let's debug-print every instruction we evaluate. That's how computers work.
            match inst.op {
                OpCode::LoadConst => {
                    debug_assert!(inst.left < chunk.constants.len() as u8);
                    debug_assert!(inst.right < registers.len() as u8);
                    registers[inst.dest as usize] = chunk.constants[inst.left as usize];
                },
                OpCode::Move => {
                    debug_assert!(inst.left < registers.len() as u8);
                    debug_assert!(inst.dest < registers.len() as u8);
                    registers[inst.dest as usize] = registers[inst.left as usize];
                },
                OpCode::Add => {
                    debug_assert!(inst.left < registers.len() as u8);
                    debug_assert!(inst.right < registers.len() as u8);
                    debug_assert!(inst.dest < registers.len() as u8);
                    registers[inst.dest as usize]
                        = registers[inst.left as usize] + registers[inst.right as usize];
                },
                OpCode::Sub => {
                    debug_assert!(inst.left < registers.len() as u8);
                    debug_assert!(inst.right < registers.len() as u8);
                    debug_assert!(inst.dest < registers.len() as u8);
                    registers[inst.dest as usize]
                        = registers[inst.left as usize] - registers[inst.right as usize];
                },
                OpCode::Div => {
                    debug_assert!(inst.left < registers.len() as u8);
                    debug_assert!(inst.right < registers.len() as u8);
                    debug_assert!(inst.dest < registers.len() as u8);
                    registers[inst.dest as usize]
                        = registers[inst.left as usize] / registers[inst.right as usize];
                },
                OpCode::Mul => {
                    debug_assert!(inst.left < registers.len() as u8);
                    debug_assert!(inst.right < registers.len() as u8);
                    debug_assert!(inst.dest < registers.len() as u8);
                    registers[inst.dest as usize]
                        = registers[inst.left as usize] * registers[inst.right as usize];
                },
                OpCode::Mod => {
                    debug_assert!(inst.left < registers.len() as u8);
                    debug_assert!(inst.right < registers.len() as u8);
                    debug_assert!(inst.dest < registers.len() as u8);
                    registers[inst.dest as usize]
                        = registers[inst.left as usize] % registers[inst.right as usize];
                },
                OpCode::Return => {
                    debug_assert!(inst.left < registers.len() as u8);
                    return registers[inst.left as usize];
                }
            }
        }
        panic!("Program did not return!"); // We're not gonna be able to assert this for long ;)
    }
}

#[cfg(test)]
mod test {
    use super::VM;
    use super::super::instruction::{Instruction, OpCode};
    use super::super::chunk::Chunk;
    use super::super::value::Value;

    #[test]
    fn it_executes_0_plus_1() {
        let consts = vec![Value(0f64), Value(1f64)];
        let instructions = vec![
            // Set register 0 to zero
            Instruction::load_const(0, 0),
            // Set register 1 to 1
            Instruction::load_const(1, 1),
            // Add register 0, 1 into 1
            Instruction::add(0, 1, 1),
            // Return value of register 1
            Instruction::return_(1)
        ];
        let program = Chunk::new(consts, instructions, 2);
        let mut machine = VM {};
        let returned = machine.eval_chunk(program);
        assert_eq!(returned, Value(1f64));
    }

    #[test]
    fn it_executes_1_plus_2() {
        let consts = vec![Value(1f64), Value(2f64)];
        let instructions = vec![
            // Register #0 to 1
            Instruction::load_const(0, 0),
            // Register #1 to 2
            Instruction::load_const(1, 1),
            // #2 = #0 + #1
            Instruction::add(0, 1, 2),
            // Return #2
            Instruction::return_(2)
        ];
        let program = Chunk::new(consts, instructions, 3);
        let mut machine = VM {};
        let returned = machine.eval_chunk(program);
        assert_eq!(returned, Value(3f64));
    }

    #[test]
    fn it_executes_1_minus_2() {
        let consts = vec![Value(1f64), Value(2f64)];
        let instructions = vec![
            // Register #0 to 1
            Instruction::load_const(0, 0),
            // Register #1 to 2
            Instruction::load_const(1, 1),
            // #2 = #0 - #1
            Instruction::sub(0, 1, 2),
            // Return #2
            Instruction::return_(2)
        ];
        let program = Chunk::new(consts, instructions, 3);
        let mut machine = VM {};
        let returned = machine.eval_chunk(program);
        assert_eq!(returned, Value(-1f64));
    }
    #[test]
    fn it_executes_3_times_2() {
        let consts = vec![Value(3f64), Value(2f64)];
        let instructions = vec![
            // Register 0 to 1
            Instruction::load_const(0, 0),
            // Register 1 to 2
            Instruction::load_const(1, 1),
            // #2 = #0 * #1
            Instruction::mul(0, 1, 2),
            // Return #2
            Instruction::return_(2)
        ];
        let program = Chunk::new(consts, instructions, 3);
        let mut machine = VM {};
        let returned = machine.eval_chunk(program);
        assert_eq!(returned, Value(6f64));
    }
    #[test]
    fn it_executes_10_mod_2() {
        let consts = vec![Value(10f64), Value(2f64)];
        let instructions = vec![
            // Register #0 to @0
            Instruction::load_const(0, 0),
            // Register #1 to @1
            Instruction::load_const(1, 1),
            // #2 = #0 % #1
            Instruction::modulo(0, 1, 2),
            // Return #2
            Instruction::return_(2)
        ];
        let program = Chunk::new(consts, instructions, 3);
        let mut machine = VM {};
        let returned = machine.eval_chunk(program);
        assert_eq!(returned, Value(0f64));
    }
    #[test]
    fn it_executes_6_div_2() {
        let consts = vec![Value(6f64), Value(2f64)];
        let instructions = vec![
            // load #0 to @0
            Instruction::load_const(0, 0),
            // load #1 to @1
            Instruction::load_const(1, 1),
            // #2 = #0 / #1
            Instruction::div(0, 1, 2),
            // Return #2
            Instruction::return_(2)
        ];
        let program = Chunk::new(consts, instructions, 3);
        let mut machine = VM {};
        let returned = machine.eval_chunk(program);
        assert_eq!(returned, Value(3f64));
    }
}
