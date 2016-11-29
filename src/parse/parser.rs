//! Pratt parser
//!
//! The parser is a configurable object which parses a stream of tokens into a
//! source tree.

use std::borrow::Cow;
use std::collections::HashMap;
use std::rc::Rc;

use lex::{CowStr, Token, TokenType, Tokenizer};
use parse::{Precedence, ParseError, ParseResult};
use parse::symbol::*;

/// Parser object which parses things
pub struct Parser {
    /// Tokenizer which supplies tokens
    tokenizer: Box<Tokenizer>,
    /// Lookahead stack for peeking
    lookahead: Vec<Token>,
    /// Parsers used for infix symbols
    infix_parsers: HashMap<(TokenType, CowStr), Rc<InfixSymbol + 'static>>,
    /// Parsers used for prefix symbols
    prefix_parsers: HashMap<(TokenType, CowStr), Rc<PrefixSymbol + 'static>>,
}

impl Parser {
    pub fn consume(&mut self) -> Token {
        self.look_ahead(0usize);
        self.lookahead.pop()
            .expect("Unable to queue token via lookahead for consume")
    }

    pub fn try_consume(&mut self, expected_type: TokenType, expected_name: CowStr)
            -> Result<Token, ParseError> {
        let token = self.consume();
        if token.data.get_type() != expected_type || token.text != expected_name {
            Err(ParseError::ExpectedToken {
                expected: expected_type,
                got: token.into()
            })
        } else {
            Ok(token)
        }
    }

    /// Peek at the next token without consuming it.
    pub fn next_type(&mut self) -> TokenType {
        self.look_ahead(0usize);
        self.lookahead.last()
            .expect("Unable to queue token via lookahead for peek")
            .data.get_type()
    }

    /// Parses any expression with the given precedence.
    pub fn expression(&mut self, precedence: Precedence) -> ParseResult {
        let mut token = self.consume();
        let prefix: Rc<PrefixSymbol + 'static>;
        if token.data.get_type() == TokenType::EOF {
            return Err(ParseError::LazyString(format!("got eof?")));
        }
        else if token.data.get_type() == TokenType::Ident {
            prefix = Rc::new(IdentifierParser {});
        }
        else if let Some(found_parser) = self.prefix_parsers.get(&(token.data.get_type(), Cow::Borrowed(&*token.text))) {
            prefix = found_parser.clone();
        }
        else {
            return Err(ParseError::LazyString(format!("Unexpected token {:?}", token)))
        }
        let mut left = try!(prefix.parse(self, token));
        while precedence < self.current_precedence() {
            token = self.consume();
            if let Some(infix) = self.infix_parsers.get(&(token.data.get_type(), Cow::Borrowed(&*token.text))).map(Rc::clone) {
                left = try!(infix.parse(self, left, token));
            }
        }
        Ok(left)
    }

    pub fn new(tokenizer: Box<Tokenizer>) -> Parser {
        use parse::symbol::*;
        use lex::tokens;
        use lex::TokenType::*;
        let infix_map: HashMap<(TokenType, CowStr), Rc<InfixSymbol + 'static>> = hashmap![
            (Symbol, tokens::Equals) => Rc::new(AssignmentParser { }) as Rc<InfixSymbol>,

            (Symbol, tokens::Plus) => BinOpSymbol::with_precedence(Precedence::AddSub),
            (Symbol, tokens::Minus) => BinOpSymbol::with_precedence(Precedence::AddSub),
            (Symbol, tokens::Star) => BinOpSymbol::with_precedence(Precedence::MulDiv),
            (Symbol, tokens::Slash) => BinOpSymbol::with_precedence(Precedence::MulDiv),

            (Symbol, tokens::Percent) => BinOpSymbol::with_precedence(Precedence::Modulo),
        ];
        let prefix_map: HashMap<(TokenType, CowStr), Rc<PrefixSymbol + 'static>> = hashmap![
            (Keyword, tokens::Let) => Rc::new(DeclarationParser { }) as Rc<PrefixSymbol>,

            (Symbol, tokens::Minus) => UnaryOpSymbol::with_precedence(Precedence::NumericPrefix),
            (Symbol, tokens::LeftParen) => Rc::new(ParensParser { }) as Rc<PrefixSymbol>,

            (Symbol, tokens::Return) => Rc::new(ReturnParser { }) as Rc<PrefixSymbol>,
        ];

        Parser {
            tokenizer: tokenizer,
            lookahead: Vec::with_capacity(2usize),
            infix_parsers: infix_map,
            prefix_parsers: prefix_map
        }
    }

    /// Grab `count` more tokens from the lexer and return the last one.
    ///
    /// Usually called with `0usize` to just peek at the next one.
    pub fn look_ahead(&mut self, count: usize) -> &Token {
        debug_assert!(count != 0, "Cannot look ahead 0");
        while count >= self.lookahead.len() {
            self.lookahead.push(self.tokenizer.next());
        }
        &self.lookahead[count - 1]
    }

    /// Get the current precedence
    fn current_precedence(&mut self) -> Precedence {
        use std::ops::Deref;
        let lookup: (TokenType, CowStr);
        {
            let looked_ahead = self.look_ahead(1);
            lookup = (looked_ahead.data.get_type(), Cow::Owned(looked_ahead.text.deref().into()));
        }
        if let Some(infix_parser) = self.infix_parsers.get(&lookup) {
            infix_parser.get_precedence()
        } else {
            Precedence::Min
        }
    }
}
