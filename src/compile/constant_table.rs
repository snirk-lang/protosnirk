use std::ops::{Deref, DerefMut};

use run::Value;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ConstantTable {
    values: Vec<Value>
}
impl Deref for ConstantTable {
    type Target = Vec<Value>;
    fn deref(&self) -> &Vec<Value> {
        &self.values
    }
}
impl DerefMut for ConstantTable {
    fn deref_mut(&mut self) -> &mut Vec<Value> {
        &mut self.values
    }
}
impl ConstantTable {
    pub fn search_or_add(&mut self, needle: Value) -> u8 {
        for (ix, value) in self.values.iter().enumerate() {
            if value == &needle {
                return ix as u8
            }
        }
        self.ensure_size();
        self.push(needle);
        return (self.len() - 1) as u8 // make as be a lower binding operator?
    }

    fn ensure_size(&self) {
        debug_assert!(self.values.len() < ::std::u8::MAX as usize);
    }
}
impl Into<Vec<Value>> for ConstantTable {
    fn into(self) -> Vec<Value> {
        self.values
    }
}
