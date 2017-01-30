
/// Prefix symbol for expressions.
///
/// Parses tokens which signify the beginning of expressions.
pub trait ExpressionPrefixSymbol<T: Tokenizer> {
    fn parse(&mut self, parser: &mut Parser<T>, token: Token) -> ParseResult<Expression>;
}
