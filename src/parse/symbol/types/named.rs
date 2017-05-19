//! Parser for named types.
use lex::{Token, TokenType, Tokenizer};
use parse::ast::*;
use parse::{Parser, ParseError, ParseResult};
use parse::symbol::{PrefixParser, IdentifierParser};

/// Parses "named" type expressions, including generics.
/// # Examples
/// ```text
/// String
/// ^name:name
/// ```
/// ```text
/// List         <   T       >
/// ^ name:name,     ^params:genericparam
#[derive(Debug)]
pub struct NamedTypeParser;

impl<T: Tokenizer> PrefixParser<TypeExpression, T> for NamedTypeParser {
    fn parse(&self, parser: &mut Parser<T>, token: Token) -> ParseResult<TypeExpression> {
        debug_assert!(token.get_type() == TokenType::Identifier,
            "NamedTypeParser called with non-name token {:?}", token);
        trace!("Parsing named type {}", token.get_text());
        let ident = Identifier::new(token);
        if parser.next_type() == TokenType::LeftAngle {
            // TODO this should be split into its own parser.
            // Ideally we have "comma-separated" and "generic bounds".
            // However, the infrastructure is still pretty simple, but the AST is too.
            // Once generic parameter bounds begin to be added, they will need their
            // own expression grammar and parsers.
            trace!("Found an open bracket, parsing generic params.");
            let mut params = Vec::new();
            let mut expect_comma = false;
            loop {
                let next = parser.consume();
                match next.get_type() {
                    TokenType::RightAngle => {
                        if params.is_empty() {
                            return Err(ParseError::LazyString("No params specified".into()))
                        }
                        else if expect_comma {
                            return Err(ParseError::LazyString("Expected comma".into()))
                        }
                        else {
                            let kind = GenericType::new(ident, params);
                            return TypeExpression::new(TypeKind::Generic(kind))
                        }
                    },
                    TokenType::Comma => {
                        if !expect_comma {
                            return Err(ParseError::LazyString("Unexpected comma"))
                        }
                        else {
                            expect_comma = false;
                            // Continue with the next token
                        }
                    },
                    TokenType::Identifier => {
                        let generic_ident = IdentifierParser { }.parse(parser, next);
                        let param = GenericParameter::Named(NamedType::new(generic_ident));
                        params.push(param);
                        expect_comma = true;
                    },
                    other => {
                        return Err(ParseError::LazyString(
                            "Expected generic type parameter".into()))
                    }
                }
            }
        }
        else { // Not `LeftAngle`
            let kind = NamedType::new(ident);
            return TypeExpression::new(TypeKind::Named(kind))
        }
    }
}
