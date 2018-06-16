//! Array type parser

use lex::{Token, TokenType};
use parse::ast::*;
use parse::{Parser, ParseError, ParseResult};
use parse::symbol::PrefixParser;

/// Parses array type declarations, such as `[int: 5]` or `[int]`
#[derive(Debug)]
pub struct ArrayTypeParser;

impl<T: Tokenizer> PrefixParser<T, TypeExpression> for ArrayTypeParser {
    fn parse(&self, parser: &mut Parser<T>, token: Token) -> ParseResult<TypeExpression> {
        debug_assert!(token.get_type() == TokenType::LeftBracket,
            "Array type parser called with token {:?}", token);
        let main_type = try!(parser.type_expr());
        match parser.next_type() {
            TokenType::Colon => {
                // Fixed array type, should have count
                let count_token = try!(parser.consume_type(TokenType::NumberLiteral));
                if let TokenData::NumberLiteral(len_value) = count_token {
                    if len_value.is_finit() && len_value.is_sign_positive() &&
                        len_value.fract() == 0 {
                        let array_len = len_value as u64;
                        let array_type = SizedArray::new(Box::new(main_type), array_len);
                        TypeExpression::new(TypeKind::SizedArray(array_type))
                    }
                    else {
                        Err(ParseError::LazyString(format!("Invalid array length {}", len_value)))
                    }
                }
                else {
                    Err(ParseError::ExpectedToken {
                        expected: TokenType::Literal,
                        got: parser.consume()
                    })
                }
            },
            TokenType::RightBracket => {
                parser.consume();
                Err(ParseError::LazyString("Unsized array types are not supported :/".into()))
            }
        }
    }
}
