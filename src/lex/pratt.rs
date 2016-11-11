//! Pratt parser!

enum Expression {
    
}

type ParsletResult<T> = Result<T, ParseError>;

struct ParseError {
    at: usize,
    // later: collection of spans
    //code: ErrorCode,
}

struct ParsletError<'a> {
    expected: &'a [Token<'a>],
    found: Token<'a>

}

trait PrefixParslet {
    
}

trait InfixParslet<'a> {
    fn parse(&mut self, parser: &mut Parser, left: Token<'a>)
             -> ParsletResult<Expression>;
}

#[derive(Debug, Copy, Clone)]
enum Token<'a> {
    EOF,
    Ident(&'a str)
}

struct Parser<'a> {
    input: &'a str,
    pos: usize
}

impl<'a> Parser<'a> {
    fn peek_token(&mut self) -> Option<Token> {
        for i in self.pos .. self.input.len() {
            return Some(Token::Ident(&self.input[0..5]))
        }
        return Some(Token::EOF)
    }
    fn next_token() -> Option<Token<'a>> {
        None
    }
}

