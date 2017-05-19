mod literal;
mod identifier;
mod parens;
mod assignment;
mod assign_op;
mod declaration;
mod if_expr;
mod fn_call;

pub use self::literal::LiteralParser;
pub use self::identifier::IdentifierParser;
pub use self::parens::ParensParser;
pub use self::assignment::AssignmentParser;
pub use self::assign_op::AssignOpParser;
pub use self::declaration::DeclarationParser;
pub use self::if_expr::IfExpressionParser;
pub use self::fn_call::FnCallParser;

use std::rc::Rc;

use lex::{Token, TokenType, Tokenizer};
use parse::{Parser, ParseResult};
use parse::ast::*;
use parse::symbol::{Precedence, InfixParser, PrefixParser};

/// A parser which parses symbols used for binary operators.
///
/// Instances of this parser return `BinaryExpression`s.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BinOpExprSymbol {
    precedence: Precedence
}
impl<T: Tokenizer> InfixParser<Expression, T> for BinOpExprSymbol {
    /// Parses a binary operator expression.
    fn parse(&self, parser: &mut Parser<T>,
             left: Expression, token: Token) -> ParseResult<Expression> {
        let right: Expression = try!(parser.expression(self.precedence));
        let bin_operator = try!(parser.operator(token.data.get_type(), &token.text));
        Ok(Expression::BinaryOp(
            BinaryOperation::new(bin_operator, token, Box::new(left), Box::new(right))))
    }
    fn get_precedence(&self) -> Precedence {
        self.precedence
    }
}
impl BinOpExprSymbol {
    /// Creates a BinOpSymbol with the given type and precedence.
    pub fn with_precedence<T: Tokenizer>(precedence: Precedence) -> Rc<InfixParser<Expression, T>> {
        Rc::new(BinOpExprSymbol { precedence: precedence }) as Rc<InfixParser<Expression, T>>
    }
}

/// Unary operator parser.
///
/// Returns a unary operator with the given token type and following expression
#[derive(Debug, PartialEq, Clone)]
pub struct UnaryOpExprSymbol {
    precedence: Precedence
}
impl<T: Tokenizer> PrefixParser<Expression, T> for UnaryOpExprSymbol {
    fn parse(&self,
             parser: &mut Parser<T>, token: Token) -> ParseResult<Expression> {
        let right_expr = try!(parser.expression(self.precedence));
        let right_value = try!(right_expr.expect_value());
        let operator = try!(parser.operator(token.data.get_type(), &token.text));
        Ok(Expression::UnaryOp(UnaryOperation::new(operator, token, Box::new(right_value))))
    }
}
impl UnaryOpExprSymbol {
    /// Create a new BinaryOpSymbol parser with the given precedence
    pub fn with_precedence<T: Tokenizer>(precedence: Precedence) -> Rc<PrefixParser<Expression, T>> {
        Rc::new(UnaryOpExprSymbol { precedence: precedence }) as Rc<PrefixParser<Expression, T>>
    }
}

#[cfg(test)]
mod tests {

}
