mod block;
mod expression;
mod item;
mod stmt;

pub use self::block::*;
pub use self::expression::*;
pub use self::item::*;
pub use self::stmt::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub token: Token
}
impl Identifier {
    pub fn new(token: Token) -> Self {
        Identifier { token: token }
    }
    pub fn get_name(&self) -> &str {
        &self.token.text
    }
}
impl Into<Token> for Identifier {
    fn into(self) -> Token {
        self.token
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub statements: Vec<Expression>
}
impl Block {
    pub fn new(statements: Vec<Expression>) -> Block {
        Block { statements: statements }
    }
}
