mod expression;
mod item;
mod stmt;

pub use self::expression::*;
pub use self::item::*;
pub use self::stmt::*;

use lex::Token;
use parse::scope::ScopeIndex;

/// Basic identifier type
#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub token: Token,
    pub index: ScopeIndex
}
impl Identifier {
    pub fn new(token: Token) -> Self {
        Identifier { token: token, index: ScopeIndex::default() }
    }
    pub fn get_name(&self) -> &str {
        &self.token.text
    }

    pub fn get_index(&self) -> &ScopeIndex {
        &self.index
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
    pub statements: Vec<Statement>,
    pub partial_index: ScopeIndex
}
impl Block {
    pub fn new(statements: Vec<Statement>) -> Block {
        Block { statements: statements, partial_index: ScopeIndex::default() }
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
    pub fn get_index(&self) -> &ScopeIndex {
        &self.partial_index
    }
}
