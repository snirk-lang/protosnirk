//! TDOP Operators
//!
//! Pratt parsing in protosnirk is implemented as a collection of symbols
//! whose role it is to offer the `parse` method which creates syntax nodes.

use std::rc::Rc;

use lex::{tokens, Token, TokenType, TokenData, Tokenizer};
use parse::{Parser, ParseResult, ParseError, Precedence};
use parse::expression::*;

/// A parser which parses an operator that is a prefix operator.
///
/// A prefix operator (such as unary negate) is one which can be parsed
/// indifferent to what came before it.
///
/// Unary negate, for example, is implemented by registering a `PrefixSymbol`
/// with the parser at a higher precedence than infix -.
pub trait PrefixSymbol<T: Tokenizer> {
    fn parse(&self, parser: &mut Parser<T>,
                 token: Token) -> ParseResult;
}

/// A parser which parses an operator that is an infix or suffix operator.
///
/// As opposed to a `PrefixSymbol`, `InfixSymbol` can handle all other operators,
/// infix operators such as arithmetic and postfix operators like call
/// (i.e. the open paren in `foo()`).
pub trait InfixSymbol<T: Tokenizer> {
    fn parse(&self, parser: &mut Parser<T>,
                 left: Expression, token: Token) -> ParseResult;
    fn get_precedence(&self) -> Precedence;
}

/// A parser which parses symbols used for binary operators.
///
/// Instances of this parser return `BinaryExpression`s.
pub struct BinOpSymbol {
    precedence: Precedence
}
impl<T: Tokenizer> InfixSymbol<T> for BinOpSymbol {
    /// Parses a binary operator expression.
    fn parse(&self, parser: &mut Parser<T>, left: Expression, token: Token) -> ParseResult {
        let right: Expression = try!(parser.expression(self.precedence));
        let bin_operator = try!(parser.operator(token.data.get_type(), &token.text));
        Ok(Expression::BinaryOp(
            BinaryOperation::new(bin_operator, token, Box::new(left), Box::new(right))))
    }
    fn get_precedence(&self) -> Precedence {
        self.precedence
    }
}
impl BinOpSymbol {
    /// Creates a BinOpSymbol with the given type and precedence.
    pub fn with_precedence<T: Tokenizer>(precedence: Precedence) -> Rc<InfixSymbol<T>> {
        Rc::new(BinOpSymbol { precedence: precedence }) as Rc<InfixSymbol<T>>
    }
}

/// Unary operator parser.
///
/// Returns a unary operator with the given token type and following expression
#[derive(Debug, PartialEq, Clone)]
pub struct UnaryOpSymbol {
    precedence: Precedence
}
impl<T: Tokenizer> PrefixSymbol<T> for UnaryOpSymbol {
    fn parse(&self, parser: &mut Parser<T>, token: Token) -> ParseResult {
        let right_expr = try!(parser.expression(self.precedence));
        let right_value = try!(right_expr.expect_value());
        let operator = try!(parser.operator(token.data.get_type(), &token.text));
        Ok(Expression::UnaryOp(UnaryOperation::new(operator, token, Box::new(right_value))))
    }
}
impl UnaryOpSymbol {
    /// Create a new BinaryOpSymbol parser with the given precedence
    pub fn with_precedence<T: Tokenizer>(precedence: Precedence) -> Rc<PrefixSymbol<T>> {
        Rc::new(UnaryOpSymbol { precedence: precedence }) as Rc<PrefixSymbol<T>>
    }
}

/// Returns a literal expression
///
/// # Examples
/// ```text
/// 34
/// ^literal
/// ```
pub struct LiteralParser { }
impl<T: Tokenizer> PrefixSymbol<T> for LiteralParser {
    fn parse(&self, _parser: &mut Parser<T>, token: Token) -> ParseResult {
        match token.data {
            TokenData::NumberLiteral(_) =>
                Ok(Expression::Literal(Literal::new(token))),
            _ => Err(ParseError::ExpectedToken {
                    expected: TokenType::Literal,
                    got: token
                })
        }
    }
}

/// Returns an identifier
///
/// # Examples
/// ```text
/// x
/// ^:name
/// ```
#[derive(Debug)]
pub struct IdentifierParser { }
impl<T: Tokenizer> PrefixSymbol<T> for IdentifierParser {
    fn parse(&self, _parser: &mut Parser<T>, token: Token) -> ParseResult {
        Ok(Expression::VariableRef(Identifier::new(token)))
    }
}

/// Parses a declaration
///
/// # Examples
/// ```text
/// mut            x          =         6 + 3
///  ^:mutable  ->name:name (skip) ->value:expression
/// ```
#[derive(Debug)]
pub struct DeclarationParser { }
impl<T: Tokenizer> PrefixSymbol<T> for DeclarationParser {
    fn parse(&self, parser: &mut Parser<T>, token: Token) -> ParseResult {
        debug_assert!(token.text == tokens::Let,
                      "Let parser called with non-let token {:?}", token);
        println!("Parsing declaration for {}", token);
        let is_mutable = parser.look_ahead(1).text == tokens::Mut;
        if is_mutable {
            parser.consume();
        }
        println!("Found mutability: {}", is_mutable);
        let name = try!(parser.lvalue());
        println!("Got name {:?}", name);
        try!(parser.try_consume_name(TokenType::Symbol, tokens::Equals));
        println!("Consumed =");
        println!("Parsing an expression");
        let value_expr = try!(parser.expression(Precedence::Min));
        println!("Getting a value from {:?}", value_expr);
        let value = try!(value_expr.expect_value());
        println!("Got value");
        Ok(Expression::Declaration(Declaration::new(name.into(), is_mutable, Box::new(value))))
    }
}

/// Parses an assignment expresion.
///
/// # Examples
/// ```text
///   x    =   y + 2
/// (left) ^ ->right:expression
/// ```
#[derive(Debug)]
pub struct AssignmentParser { }
impl<T: Tokenizer> InfixSymbol<T> for AssignmentParser {
    fn parse(&self, parser: &mut Parser<T>, left: Expression, _token: Token) -> ParseResult {
        debug_assert!(_token.text == tokens::Equals,
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
/// ```text
/// (        x + 1          )
/// ^  ->right:expression (skip)
/// ```
#[derive(Debug)]
pub struct ParensParser { }
impl<T: Tokenizer> PrefixSymbol<T> for ParensParser {
    fn parse(&self, parser: &mut Parser<T>, _token: Token) -> ParseResult {
        debug_assert!(_token.text == tokens::LeftParen,
                      "Parens parser called with non-left-paren {:?}", _token);
        let inner_expr = try!(parser.expression(Precedence::Min));
        let inner = try!(inner_expr.expect_value());
        try!(parser.try_consume_name(TokenType::Symbol, tokens::RightParen));
        Ok(inner)
    }
}

/// Parses return statements
///
/// # Examples
/// ```text
/// return x + 1 + 3 * 4
///   ^    ->right:expression
/// ```
#[derive(Debug)]
pub struct ReturnParser { }
impl<T: Tokenizer> PrefixSymbol<T> for ReturnParser {
    fn parse(&self, parser: &mut Parser<T>, token: Token) -> ParseResult {
        debug_assert!(token.text == tokens::Return,
                      "Return parser called with non-return {:?}", token);
        let inner_expr = try!(parser.expression(Precedence::Return));
        let inner = try!(inner_expr.expect_value());
        Ok(Expression::Return(Return::new(token, Box::new(inner))))
    }
}

/// Parses block statements, ending with an `EndBlock` token. Not used.
///
/// # Examples
/// ```text
/// Not used.
/// ```
#[derive(Debug)]
pub struct BlockParser { }
impl<T: Tokenizer> PrefixSymbol<T> for BlockParser {
    fn parse(&self, parser: &mut Parser<T>, _token: Token) -> ParseResult {
        let mut stmts = Vec::new();
        while parser.look_ahead(1).data != TokenData::EndBlock {
            let expr = try!(parser.expression(Precedence::Min));
            stmts.push(expr);
        }
        parser.consume(); // Skip over the end block
        return Ok(Expression::Block(stmts))
    }
}

/// Parses expresisons using the expr/assign style operators.
///
/// This parser will actually generate a regular `Assignment` expression, desugaring
/// the assignment+operation
/// # Examples
/// ```text
/// x        +=   5
/// ^lvalue  ^op  ^rvalue
/// ```
/// This will be parsed as `Assignment { Var { 'x' }, BinaryOp { +, Var { x }, Literal(5) } }`
pub struct AssignOpParser { }
impl<T: Tokenizer> InfixSymbol<T> for AssignOpParser {
    fn parse(&self, parser: &mut Parser<T>, left: Expression, token: Token) -> ParseResult {
        let lvalue = try!(left.expect_identifier());
        let right_expr = try!(parser.expression(Precedence::Min));
        let right_value = try!(right_expr.expect_value());
        let operator = try!(parser.operator(token.data.get_type(), &token.text));
        // We parse it here into an expanded expression.
        let right_expr = Expression::BinaryOp(BinaryOperation::new(operator, token, Box::new(Expression::VariableRef(lvalue.clone())), Box::new(right_value)));
        Ok(Expression::Assignment(Assignment::new(lvalue, Box::new(right_expr))))
    }
    fn get_precedence(&self) -> Precedence {
        Precedence::Assign
    }
}
