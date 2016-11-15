//! TDOP Operators
//!
//! Pratt parsing in protosnirk is implemented as a collection of symbols
//! whose role it is to offer the `parse` method which creates syntax nodes.

use std::rc::Rc;

use lex::{Parser, ParseResult, ParseError, Token, TokenType};
use lex::precedence::Precedence;
use lex::expression::*;

/// A parser which parses an operator that is a prefix operator.
///
/// A prefix operator (such as unary negate) is one which can be parsed
/// indifferent to what came before it.
///
/// Unary negate, for example, is implemented by registering a `PrefixSymbol`
/// with the parser at a higher precedence than infix -.
pub trait PrefixSymbol {
    fn parse(&self, parser: &mut Parser,
                 token: Token) -> ParseResult;
}

/// A parser which parses an operator that is an infix or suffix operator.
///
/// As opposed to a `PrefixSymbol`, `InfixSymbol` can handle all other operators,
/// infix operators such as arithmetic and postfix operators like call
/// (i.e. the open paren in `foo()`).
pub trait InfixSymbol {
    fn parse(&self, parser: &mut Parser,
                 left: Expression, token: Token) -> ParseResult;
    fn get_precedence(&self) -> Precedence;
}

/// A parser which parses symbols used for binary operators.
///
/// Instances of this parser return `BinaryExpression`s.
pub struct BinOpSymbol {
    precedence: Precedence
}
impl InfixSymbol for BinOpSymbol {
    /// Parses a binary operator expression.
    fn parse(&self, parser: &mut Parser, left: Expression, token: Token) -> ParseResult {
        let right: Expression = try!(parser.expression(self.precedence));
        Ok(Expression::BinaryOp(
            BinaryOperation::new(token.get_type(), Box::new(left), Box::new(right))))
    }
    fn get_precedence(&self) -> Precedence {
        self.precedence
    }
}
impl BinOpSymbol {
    /// Creates a BinOpSymbol with the given type and precedence.
    pub fn with_precedence(precedence: Precedence) -> Rc<InfixSymbol> {
        Rc::new(BinOpSymbol { precedence: precedence }) as Rc<InfixSymbol>
    }
}

/// Unary operator parser.
///
/// Returns a unary operator with the given token type and following expression
#[derive(Debug, PartialEq, Clone)]
pub struct UnaryOpSymbol {
    precedence: Precedence
}
impl PrefixSymbol for UnaryOpSymbol {
    fn parse(&self, parser: &mut Parser, token: Token) -> ParseResult {
        let right_expr = try!(parser.expression(self.precedence));
        let right_value = try!(right_expr.expect_value());
        Ok(Expression::UnaryOp(UnaryOperation::new(token.get_type(), Box::new(right_value))))
    }
}
impl UnaryOpSymbol {
    /// Create a new BinaryOpSymbol parser with the given precedence
    pub fn with_precedence(precedence: Precedence) -> Rc<PrefixSymbol> {
        Rc::new(UnaryOpSymbol { precedence: precedence }) as Rc<PrefixSymbol>
    }
}

/// Returns an identifier
///
/// # Examples
/// ```
/// x
/// ^:name
/// ```
#[derive(Debug)]
pub struct IdentifierParser { }
impl PrefixSymbol for IdentifierParser {
    fn parse(&self, _parser: &mut Parser, token: Token) -> ParseResult {
        Ok(Expression::VariableRef(Identifier::new(token.into())))
    }
}

/// Parses a declaration
///
/// # Examples
/// ```
/// mut            x          =         6 + 3
///  ^:mutable  ->name:name (skip) ->value:expression
/// ```
#[derive(Debug)]
pub struct DeclarationParser { }
impl PrefixSymbol for DeclarationParser {
    fn parse(&self, parser: &mut Parser, token: Token) -> ParseResult {
        let mutable = token.get_type() == TokenType::Mut;
        let name_expr = try!(parser.expression(Precedence::Min));
        let name = try!(name_expr.expect_identifier());
        try!(parser.try_consume(TokenType::Assign));
        let value_expr = try!(parser.expression(Precedence::Min));
        let value = try!(value_expr.expect_value());
        Ok(Expression::Declaration(Declaration::new(name.into(), mutable, Box::new(value))))
    }
}

/// Parses an assignment expresion.
///
/// # Examples
/// ```
///   x    =   y + 2
/// (left) ^ ->right:expression
/// ```
#[derive(Debug)]
pub struct AssignmentParser { }
impl InfixSymbol for AssignmentParser {
    fn parse(&self, parser: &mut Parser, left: Expression, _token: Token) -> ParseResult {
        debug_assert!(_token.get_type() == TokenType::Assign,
                      "Assign parser called with non-assign token {:?}", _token);
        let ident = try!(left.expect_identifier());
        let right_expr = try!(parser.expression(Precedence::Assign));
        let right = try!(right_expr.expect_value());
        Ok(Expression::Assignment(Assignment::new(ident, Box::new(right))))
    }
    fn get_precedence(&self) -> Precedence {
        Precedence::Assign
    }
}

/// Parses expressions wrapped in parentheses
///
/// # Examples
/// ```
/// (        x + 1          )
/// ^  ->right:expression (skip)
/// ```
#[derive(Debug)]
pub struct ParensParser { }
impl PrefixSymbol for ParensParser {
    fn parse(&self, parser: &mut Parser, _token: Token) -> ParseResult {
        debug_assert!(_token.get_type() == TokenType::LeftParen,
                      "Parens parser called with non-left-paren {:?}", _token);
        let inner_expr = try!(parser.expression(Precedence::Paren));
        let inner = try!(inner_expr.expect_value());
        try!(parser.try_consume(TokenType::RightParen));
        Ok(inner)
    }
}

/// Parses return statements
///
/// # Examples
/// ```
/// return x + 1 + 3 * 4
///   ^    ->right:expression
/// ```
#[derive(Debug)]
pub struct ReturnParser { }
impl PrefixSymbol for ReturnParser {
    fn parse(&self, parser: &mut Parser, _token: Token) -> ParseResult {
        debug_assert!(_token.get_type() == TokenType::Return,
                      "Return parser called with non-return {:?}", _token);
        let inner_expr = try!(parser.expression(Precedence::Return));
        let inner = try!(inner_expr.expect_value());
        Ok(inner)
    }
}

/// Parses block statements, ending with an `EndBlock` token. Not used.
///
/// # Examples
/// ```
/// Not used.
/// ```
#[derive(Debug)]
pub struct BlockParser { }
impl PrefixSymbol for BlockParser {
    fn parse(&self, parser: &mut Parser, _token: Token) -> ParseResult {
        let mut stmts = Vec::new();
        while parser.next_type() != TokenType::EndBlock {
            let expr = try!(parser.expression(Precedence::Min));
            stmts.push(expr);
        }
        return Ok(Expression::Block(stmts))
    }
}
