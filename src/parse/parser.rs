//! Pratt parser
//!
//! The parser is a configurable object which parses a stream of tokens into a
//! source tree.

use std::collections::HashMap;
use std::rc::Rc;

use lex::{Token, TokenType, Tokenizer};
use parse::{Precedence, ParseError, ParseResult};
use parse::symbol::*;

/// Parser object which parses things
pub struct Parser {
    /// Tokenizer which supplies tokens
    tokenizer: Box<Tokenizer>,
    /// Lookahead stack for peeking
    lookahead: Vec<Token>,
    /// Parsers used for infix symbols
    infix_parsers: HashMap<TokenType, Rc<InfixSymbol + 'static>>,
    /// Parsers used for prefix symbols
    prefix_parsers: HashMap<TokenType, Rc<PrefixSymbol + 'static>>,
}

impl Parser {
    pub fn consume(&mut self) -> Token {
        self.look_ahead(0usize);
        self.lookahead.pop()
            .expect("Unable to queue token via lookahead for consume")
    }

    pub fn try_consume(&mut self, expected_type: TokenType) -> Result<Token, ParseError> {
        let token = self.consume();
        if token.get_type() != expected_type {
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
            .get_type()
    }

    /// Parses any expression with the given precedence.
    pub fn expression(&mut self, precedence: Precedence) -> ParseResult {
        let mut token = self.consume();
        if let Some(prefix) = self.prefix_parsers.get(&token.get_type()).map(Rc::clone) {
            let mut left = try!(prefix.parse(self, token));
            while precedence < self.current_precedence() {
                token = self.consume();
                if let Some(infix) = self.infix_parsers.get(&token.get_type()).map(Rc::clone) {
                    left = try!(infix.parse(self, left, token));
                }
            }
            Ok(left)
        } else {
            Err(ParseError::LazyString(format!("Unexpected token of type {:?}", token.get_type())))
        }
    }

    pub fn new(tokenizer: Box<Tokenizer>) -> Parser {
        use parse::symbol::*;
        let infix_map: HashMap<TokenType, Rc<InfixSymbol>> = hashmap![
            TokenType::Assign => Rc::new(AssignmentParser { }) as Rc<InfixSymbol>,

            TokenType::Plus => BinOpSymbol::with_precedence(Precedence::AddSub),
            TokenType::Minus => BinOpSymbol::with_precedence(Precedence::AddSub),
            TokenType::Star => BinOpSymbol::with_precedence(Precedence::MulDiv),
            TokenType::Slash => BinOpSymbol::with_precedence(Precedence::MulDiv),

            TokenType::Percent => BinOpSymbol::with_precedence(Precedence::Modulo),
        ];
        let prefix_map: HashMap<TokenType, Rc<PrefixSymbol>> = hashmap![
            TokenType::Let => Rc::new(DeclarationParser { }) as Rc<PrefixSymbol>,
            TokenType::Mut => Rc::new(DeclarationParser { }) as Rc<PrefixSymbol>,

            TokenType::Minus => UnaryOpSymbol::with_precedence(Precedence::NumericPrefix),
            TokenType::LeftParen => Rc::new(ParensParser { }) as Rc<PrefixSymbol>,

            TokenType::Return => Rc::new(ReturnParser { }) as Rc<PrefixSymbol>,

            TokenType::Identifier => Rc::new(IdentifierParser { }) as Rc<PrefixSymbol>
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
    fn look_ahead(&mut self, count: usize) {
        while count >= self.lookahead.len() {
            //self.lookahead.push(self.tokenizer.next());
        }
        //self.lookahead[count].clone()
    }

    /// Get the current precedence
    fn current_precedence(&mut self) -> Precedence {
        let next_type = self.next_type();
        if let Some(infix_parser) = self.infix_parsers.get(&next_type) {
            infix_parser.get_precedence()
        } else {
            Precedence::Min
        }
    }
}
