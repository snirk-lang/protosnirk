use parse::{Operator, Program, SymbolTable, ExpressionChecker};
use parse::expression::*;
use parse::expression::Expression as BaseExpression;
use run::{Chunk, Value, Instruction, OpCode};
use super::register_table::RegisterTable;
use super::constant_table::ConstantTable;

pub struct ChunkCreator {
    symbols: SymbolTable,
    constants: Vec<Value>,
    registers: RegisterTable,
    instructions: Vec<Instruction>,
    max_registers: u8,
    curr_register: u8,
    found_register: u8,
}
impl ChunkCreator {
    pub fn new(symbols: SymbolTable, constants: Vec<Value>, max_registers: u8) -> ChunkCreator {
        debug_assert!(max_registers > 3, "Need at least 3 registers, got {}", max_registers);
        debug_assert!(symbols.len() < max_registers as usize,
            "Need at least {} registers, have {}", symbols.len(), max_registers);
        let registers = RegisterTable::new();
        // Creating a single chunk involves a fixed amount of registers.
        // This means a chunk (now a program) is fundamentally limited in the number of variables
        // (and operations in this case) it can do. This is fine - Lua enforces this per-function.
        // When functions are added, each will be treated as a chunk, where the register count
        // resets in the VM per-chunk.
        // However, we have no re-use of registers whatsoever - which will greatly limit the complexity
        // of individual blocks.
        let curr_register = registers.len() as u8;
        ChunkCreator {
            symbols: symbols,
            constants: constants,
            registers: registers,
            max_registers: max_registers,
            instructions: Vec::new(),
            curr_register: curr_register,
            found_register: 0u8
        }
    }
    pub fn decompose(self) -> (Vec<Value>, Vec<Instruction>, u8) {
        println!("Registers: {:?}", self.registers);
        (self.constants, self.instructions, self.registers.len() as u8)
    }
}
impl ExpressionChecker for ChunkCreator {
    fn check_literal(&mut self, literal: &Literal) {
        let src_register_ix: u8;
        let literal_value = Value(literal.get_value());
        // We cache which constants have been loaded. If the cache was hit,
        // we can directly use that register as its value will not have changed.
        if let Some(const_regiser_ix) = self.registers.get_for_constant(literal_value) {
            src_register_ix = const_regiser_ix;
            println!("Literal declaration: use register {}", src_register_ix);
        }
        else {
            let constant_ix = self.constants.iter()
                .position(|value| *value == Value(literal.get_value()))
                .expect("Missing literal value!") as u8;

            println!("Literal declaration: need to load constant #{}", constant_ix);

            let new_ix = self.registers.load_constant(literal_value);
            src_register_ix = new_ix;

            let load_const_instruction = Instruction {
                left: constant_ix,
                dest: new_ix,
                right: 0,
                op: OpCode::LoadConst
            };
            println!("Emitting {:?} for literal", load_const_instruction);
            self.instructions.push(load_const_instruction);
        }
        // The current register is either the new one loaded in by the `load_const`
        // or the old one we found.
        self.curr_register = src_register_ix;
    }

    fn check_var_ref(&mut self, var: &Identifier) {
        // Find the preallocated variable register
        self.curr_register = self.registers.variable(var.get_name());
    }

    fn check_declaration(&mut self, decl: &Declaration) {
        // #var = #expr
        // Assign preallocated variable register
        let var_register = self.registers.variable(decl.get_name());
        self.check_expression(decl.get_value());
        let expr_register = self.curr_register;

        let declaration_instruction = Instruction {
            dest: var_register,
            left: expr_register,
            right: 0,
            op: OpCode::Move
        };

        println!("Emitting {:?} for declaration", declaration_instruction);

        self.curr_register = var_register; // Again, not gonna be used next time
        self.instructions.push(declaration_instruction);
    }

    fn check_unary_op(&mut self, unary_op: &UnaryOperation) {
        debug_assert!(unary_op.operator == Operator::Subtraction,
            "Invalid unary operator {:?}", unary_op.operator);
        // #new = 0 - #inner
        // queue up the register for the inner expr
        self.check_expression(&*unary_op.expression);
        let inner_register = self.curr_register;

        let zero_register = self.registers.get_for_constant(Value(0f64))
            .expect("Did not create a zero register");
        let result_register = self.registers.temporary();
        let negate_instruction = Instruction {
            dest: result_register,
            left: zero_register,
            right: inner_register,
            op: OpCode::Sub
        };
        self.curr_register = result_register;
        self.instructions.push(negate_instruction);
    }

    fn check_binary_op(&mut self, binary_op: &BinaryOperation) {
        // #new = #left <op> #inner
        let opcode = binary_op.get_operator().get_opcode();
        self.check_expression(&*binary_op.left);
        let left_register = self.curr_register;
        println!("Left side of binary op {:?} is at {}", opcode, left_register);
        self.check_expression(&*binary_op.right);
        let right_register = self.curr_register;
        println!("Right side of binary op {:?} is at {}", opcode, right_register);

        let result_register = self.registers.temporary();
        let binary_op_instruction = Instruction {
            dest: result_register,
            left: left_register,
            right: right_register,
            op: opcode
        };
        println!("Emitting {:?} for binary operation", binary_op_instruction);

        self.curr_register = result_register;
        self.instructions.push(binary_op_instruction);
    }

    fn check_assignment(&mut self, assign: &Assignment) {
        let var_register = self.registers.variable(assign.lvalue.get_name());

        self.check_expression(&*assign.rvalue);
        let expr_register = self.curr_register;

        let assign_instruction = Instruction {
            dest: var_register,
            left: expr_register,
            right: 0,
            op: OpCode::Move
        };

        println!("Emitting {:?} for assignment", assign_instruction);

        self.curr_register = var_register; // Ideally not used; assignment returns nothing
        // In the future, this would be placed on the `()` register?
        self.instructions.push(assign_instruction);
    }

    fn check_return(&mut self, return_expr: &Return) {
        // return #expr
        return_expr.value.as_ref().map(|v| self.check_expression(&*v));
        let expr_register = self.curr_register;

        let return_instruction = Instruction::return_(expr_register);

        self.curr_register = expr_register;
        self.instructions.push(return_instruction);
    }
}

/*
use std::fmt::{Formatter, Display};
use std::fmt::Result as FmtResult;

impl Display for ChunkCreator {
    fn fmt(&self, format: &mut Formatter) -> FmtResult {
        /*
        Chunk: {} vars, {} consts
        Constants:
            0: {}
            1: {}
        Instructions:
            Load 0 [0] into 1
            Move 1 [Constant(0)] to 0 [Variable("x")]
            Move 2 [Variable("x")] to 1 [Variable("y")]
            Do Add l: 1 [Constant 0] r: 2 [Variable(3)], d: 4 [Temp(1)]
        */

    }
}


*/
