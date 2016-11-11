//! TDOP Operators
//!
//! Pratt parsing in protosnirk is implemented as a collection of symbols
//! whose role it is to offer the `parse` method which creates syntax nodes.

use lex::precedence::Precedence;
use lex::token::{Token, TokenType};

pub type SymbolResult<'a> = Result<SyntaxNode, ParseError<'a>>;

pub enum ParseError<'a> {
    Unexpected {
        exptected: TokenType,
        got: Token<'a>
    },
    Unimplemented
}

/// A parser which parses an operator that is a prefix operator.
///
/// A prefix operator (such as unary negate) is one which can be parsed
/// indifferent to what came before it.
///
/// Unary negate, for example, is implemented by registering a `PrefixSymbol`
/// with the parser at a higher precedence than infix -.
pub trait PrefixSymbol {
    fn parse<'a>(&self, parser: &'a mut Parser,
                 token: Token<'a>) -> SymbolResult<'a>;
}

/// A parser which parses an operator that is an infix or suffix operator.
///
/// As opposed to a `PrefixSymbol`, `InfixSymbol` can handle all other operators,
/// infix operators such as arithmetic and postfix operators like call
/// (i.e. the open paren in `foo()`).
pub trait InfixSymbol {
    fn parse<'a>(&self, parser: &'a mut Parser,
                 left: SyntaxNode, token: Token<'a>) -> SymbolResult<'a>;
    fn get_precedence(&self) -> Precedence;
}

pub trait StatementSymbol {
    fn parse<'a>(&self, )
}

/// A parser which parses symbols used for binary operators.
///
/// Instances of this parser return `BinaryExpression`s.
pub struct BinOpSymbol {
    precedence: Precedence
}
impl InfixSymbol for BinOpSymbol {
    /// Parses a binary operator expression.
    fn parse<'a>(&self, parser: &'a mut Parser, left: SyntaxNode, token: Token<'a>) -> SymbolResult<'a> {
        let right = try!(parser.parse_expression(self.precedence));
        Ok(SyntaxNode::Prefix(left, token.get_type(), right))
    }
    fn get_precedence(&self) -> Precedence {
        self.precedence
    }
}
impl BinOpSymbol {
    /// Creates a BinOpSymbol with the given type and precedence.
    pub fn with_precedence(precedence: Precedence) -> BinOpSymbol {
        BinOpSymbol { precedece: precedence }
    }
}

/// Unary operator parser.
///
/// Returns a unary operator with the given token type and following expression
pub struct UnaryOpSymbol {
    precedence: Precedence
}
impl PrefixSymbol for UnaryOpSymbol {
    fn parse<'a>(&self, parser: &'a mut Parser, token: Token<'a>) -> SymbolResult<'a> {
        let right = try!(parser.parse_expression(self.precedence));
        Ok(SyntaxNode::UnaryOp(token.get_type(), right))
    }
}
impl UnaryOpSymbol {
    /// Create a new BinaryOpSymbol parser with the given precedence
    pub fn with_precedence(precedence: Precedence) -> UnaryOpSymbol {
        UnaryOpSymbol { precedence: precedence }
    }
}

/// Returns an identifier
///
/// # Examples
/// ```
/// x
/// ^:name
/// ```
pub struct IdentifierParser { }
impl PrefixSymbol for IdentifierParser {
    fn parse<'a>(&self, parser: &'a mut Parser, token: Token<'a>) -> SymbolResult<'a> {
        Ok(SyntaxNode::Name(token))
    }
}

/// Parses a declaration
///
/// # Examples
/// ```
/// mut            x          =         6 + 3
///  ^:mutable  ->name:name (skip) ->value:expression
/// ```
pub struct DeclarationParser { }
impl PrefixSymbol for DeclarationParser {
    fn parse<'a>(&self, parser: &'a mut Parser, token: Token<'a>) -> SymbolResult<'a> {
        let mutable = token.get_type() == TokenType::Mut;
        let name_expr = try!(parsers::expect(ExprType::Name,
                                            parser.parse_expression()));
        try!(parser.advance(TokenType::Assign));
        let value = try!(parsers::expect(ExprType::Value,
                                         parser.parse_expression()));
        Ok(SyntaxNode::Declare(mutable, name_expr, value))
    }
}

/// Parses an assignment expresion.
///
/// # Examples
/// ```
///   x    =   y + 2
/// (left) ^ ->right:expression
/// ```
pub struct AssignmentParser { }
impl InfixSymbol for AssignmentParser {
    fn parse<'a>(&self, parser: &'a mut parser,
                 left: Expression, _token: Token<'a>) -> SymbolResult<'a> {
        debug_assert!(_token.get_type() == TokenType::Assign,
                      "Assign parser called with non-assign token {:?}", _token);
        try!(parsers::expect(ExprType::Name, left));
        let right = try!(parser.parse_expr_of(ExprType::Value, Precedence::Assign));
        Ok(SyntaxNode::Assign(left, right))
    }
}

/// Parses expressions wrapped in parentheses
///
/// # Examples
/// ```
/// (        x + 1          )
/// ^  ->right:expression (skip)
/// ```
pub struct ParensParser { }
impl PrefixSymbol for ParensParser {
    fn parse<'a>(&self, parser: &'a mut Parser, _token: Token<'a>) -> SymbolResult<'a> {
        debug_assert!(_token.get_type() == TokenTye::LeftParen,
                      "Parens parser called with non-left-paren {:?}", _token);
        let inner = try!(parser.parse_expr_of(ExprType::Expression, Precedence::Paren));
        try!(parser.advance(TokenType::RightParen));
        Ok(inner)
    }
}
