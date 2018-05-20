//mod named;
//mod array;

//pub use self::named::NamedTypeParser;
//pub use self::array::ArrayTypeParser;

use lex::{Token, TokenType, Tokenizer};

use ast::Identifier;
use ast::types::{TypeExpression, NamedTypeExpression};
use parse::{Parser, ParseResult};
use parse::symbol::PrefixParser;

/// `Identifier` parser for type expressions.
///
/// Will be replaced when types become more complicated
#[derive(Debug)]
pub struct NamedTypeParser { }
impl<T: Tokenizer> PrefixParser<TypeExpression, T> for NamedTypeParser {
    fn parse(&self, parser: &mut Parser<T>, token: Token)
             -> ParseResult<TypeExpression> {
        debug_assert!(token.get_type() == TokenType::Ident,
            "NamedTypeParser called with non-name token {:?}", token);
        trace!("Parsing named type {}", token.get_text());
        Ok(TypeExpression::Named(NamedTypeExpression::new(
            Identifier::new(token)
        )))
    }
}
