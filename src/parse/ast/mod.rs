mod expression;
mod item;
mod stmt;

pub use self::expression::*;
pub use self::item::*;
pub use self::stmt::*;

use lex::Token;

/// Basic identifier type
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

/// Collection of statements which may have an expression value
#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub statements: Vec<Statement>
}
impl Block {
    pub fn new(statements: Vec<Statement>) -> Block {
        Block { statements: statements }
    }
    pub fn has_value(&self) -> bool {
        if self.statements.len() == 0 {
            return false
        }
        let last_ix = self.statements.len() - 1;
        // TODO actual analysis
        for (ix, statement) in self.statements.iter().enumerate() {
            if ix == last_ix {
                return statement.has_value()
            }
            // else if stmt == return {
            //     return stmt.has_value()
            // }
        }
        return false
    }
}
