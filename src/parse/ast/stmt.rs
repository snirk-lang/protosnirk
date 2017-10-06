/// Statement values
///
/// Function bodies are usually made up of statements. They include complex blocks
/// such as loop constructs. They are usually not accepted in as many places as
/// `Expression`s are because of their ability to use indentation.

use lex::{CowStr, Token, TokenData, TokenType};
use parse::{ScopedId, TypeId};
use parse::ast::{Expression, Block, Identifier};

use std::cell::{Cell, RefCell};

/// Statement representation
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Expression(Expression),
    Return(Return),
    DoBlock(DoBlock),
    IfBlock(IfBlock)
    // match, loop, while, for
}
impl Statement {
    pub fn has_value(&self) -> bool {
        match *self {
            Statement::Expression(ref inner) => inner.has_value(),
            Statement::DoBlock(ref inner) => inner.has_value(),
            Statement::Return(ref return_) => return_.has_value(),
            Statement::IfBlock(ref if_) => if_.has_value()
        }
    }
}

/// Explicit return statement
#[derive(Debug, PartialEq, Clone)]
pub struct Return {
    pub token: Token,
    pub value: Option<Box<Expression>>
}
impl Return {
    pub fn new<V: Into<Option<Box<Expression>>>>(token: Token,
                                                 value: V) -> Return {
        Return { token: token, value: value.into() }
    }
    pub fn has_value(&self) -> bool {
        if let Some(ref val) = self.value {
            val.has_value()
        }
        else {
            false
        }
    }
    pub fn get_value(&self) -> &Option<Box<Expression>> {
        &self.value
    }
}

/// Do <block> statement.
#[derive(Debug, PartialEq, Clone)]
pub struct DoBlock {
    pub do_token: Token,
    pub block: Box<Block>
}
impl DoBlock {
    pub fn new(token: Token, block: Box<Block>) -> DoBlock {
        DoBlock { do_token: token, block: block }
    }
    pub fn has_value(&self) -> bool {
        self.block.has_value()
    }
    pub fn get_block(&self) -> &Block {
        &self.block
    }
}

/// if <condition> <block>
///
/// At the moment I'll be keeping `if` as a block because I don't think the syntax
/// will work well as an expression: consider
/// ```protosnirk
/// let x = if someExpression() != someOtherThing() someValue else false
/// ```
/// If there isn't a newline separaing the conditional from the first block, it's not
/// going to work out well.
///
/// In a somewhat unusual approach, I'll be completely ignring the "danging else" problem
/// thanks to the handwritten parsers.
///
/// The conditionals are in a list instead of nested.
#[derive(Debug, PartialEq, Clone)]
pub struct IfBlock {
    pub conditionals: Vec<Conditional>,
    pub else_block: Option<(Token, Block)>,
    scoped_id: RefCell<ScopedId>,
    type_id: Cell<TypeId>
}

/// A basic conditional
#[derive(Debug, PartialEq, Clone)]
pub struct Conditional {
    pub if_token: Token,
    pub condition: Expression,
    pub block: Block
}

impl IfBlock {
    pub fn new(conditionals: Vec<Conditional>,
               else_block: Option<(Token, Block)>) -> IfBlock {
        debug_assert!(conditionals.len() >= 1,
            "Attempted to create an `If` with 0 conditionals");
        IfBlock {
            conditionals: conditionals,
            else_block: else_block,
            scoped_id: RefCell::new(ScopedId::default())
        }
    }
    pub fn has_else_if(&self) -> bool {
        self.conditionals.len() > 1
    }
    pub fn has_else(&self) -> bool {
        self.else_block.is_some()
    }
    pub fn get_conditionals(&self) -> &Vec<Conditional> {
        &self.conditionals
    }
    pub fn get_condition(&self) -> &Expression {
        &self.conditionals[0].condition
    }
    pub fn get_else(&self) -> Option<&(Token, Block)> {
        self.else_block.as_ref()
    }
    pub fn has_value(&self) -> bool {
        if !self.has_else() {
            return false
        }
        for ref cond in (&self.conditionals).iter() {
            if !cond.has_value() {
                return false
            }
        }
        self.else_block.as_ref().unwrap().1.has_value()
    }
    pub fn get_id(&self) -> ScopedId {
        *self.id
    }
    pub fn set_id(&self, id: ScopedId) {
        self.id = id;
    }
}

impl Conditional {
    pub fn new(if_token: Token,
               condition: Expression,
               block: Block) -> Conditional {
        Conditional {
            if_token: if_token,
            condition: condition,
            block: block
        }
    }
    pub fn get_condition(&self) -> &Expression {
        &self.condition
    }
    pub fn get_block(&self) -> &Block {
        &self.block
    }
    pub fn has_value(&self) -> bool {
        self.block.has_value()
    }
}
