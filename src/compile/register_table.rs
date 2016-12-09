
use std::ops::Deref;

use run::Value;

/// Data stored about each register being used.
///
/// If we choose to recycle used registers
/// (an important problem for real compilers)
/// we'll need to know a little more. For now,
/// managing constants should work.
#[derive(Debug, PartialEq, Clone)]
enum RegisterData {
    /// Register is used to store a variable
    Variable(String),
    /// Register is used to store a temporary
    /// usize is which temporary stored
    Temproary(usize),
    /// Register was used to store a constant.
    ///
    /// When we have
    Constant(Value)
}

/// Contains data about the registers being used in the compiler
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RegisterTable {
    registers: Vec<RegisterData>, // We don't have clashing vars yet.
    current_temporary: usize,
    consts: Vec<Value>, // Keep track of constants separately
}
impl RegisterTable {
    pub fn new() -> RegisterTable {
        RegisterTable {
            registers: Vec::new(),
            current_temporary: 0,
            consts: Vec::new()
        }
    }
    /// Get the register index for the corresponding variable.
    ///
    /// If the variable doesn't already have a register, it will
    /// get one.
    pub fn variable(&mut self, name: &str) -> u8 {
        println!("Registers: checking for {}", name);
        for (ix, sym) in self.registers.iter().enumerate() {
            match sym {
                &RegisterData::Variable(ref sym_name) => {
                    if sym_name == name {
                        println!("Registers: {} has register {}", name, ix);
                        return ix as u8
                    }
                },
                _ => {}
            }
        }
        self.ensure_size();
        self.registers.push(RegisterData::Variable(name.to_string()));
        return (self.registers.len() - 1) as u8
    }

    /// Create a new temporary register.
    ///
    /// Using a new register for each temporary will quickly cause
    /// us to run out of space.
    pub fn temporary(&mut self) -> u8 {
        // Here is where we could be clever about "caching results",
        // but it would be more parse tree analysis to reduce reduntant
        // operations.
        self.ensure_size();
        self.registers.push(RegisterData::Temproary(self.current_temporary));
        self.current_temporary += 1;
        (self.registers.len() - 1) as u8
    }

    /// If we have already loaded a constant into a register,
    /// returns that register so the constant isn't loaded again.
    ///
    /// If this constant isn't in a register yet we return none.
    /// You can request a register for
    pub fn get_for_constant(&self, value: Value) -> Option<u8> {
        for (ix, sym) in self.registers.iter().enumerate() {
            match sym {
                &RegisterData::Constant(ref existing_val) => {
                    if value == *existing_val {
                        return Some(ix as u8)
                    }
                },
                _ => {}
            }
        }
        None
    }

    /// Add a new known constant register, which can be reused later.
    pub fn load_constant(&mut self, value: Value) -> u8 {
        self.ensure_size();
        self.registers.push(RegisterData::Constant(value));
        println!("Registers: added space for {:?} at ix {}, now have {:?}",
            value, self.registers.len() - 1, self.registers);
        (self.registers.len() - 1) as u8
    }

    pub fn len(&self) -> usize {
        self.registers.len()
    }

    #[inline]
    fn ensure_size(&self) {
        debug_assert!(self.registers.len() < ::std::u8::MAX as usize);
    }
}
