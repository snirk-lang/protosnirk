//! Pratt parser
//!
//! The parser is a configurable object which parses a stream of tokens into a
//! source tree.

use std::borrow::Cow;
use std::collections::HashMap;
use std::rc::Rc;

use lex::{CowStr, Token, TokenType, Tokenizer};
use parse::{Operator, Precedence, ParseError, ParseResult};
use parse::expression::*;
use parse::symbol::*;
use parse::verify::Verifier;
use parse::build::Program;

/// Parser object which parses things
pub struct Parser<T: Tokenizer> {
    /// Tokenizer which supplies tokens
    tokenizer: T,
    /// Lookahead stack for peeking
    lookahead: Vec<Token>,
    /// Parsers used for infix symbols
    infix_parsers: HashMap<(TokenType, CowStr), Rc<InfixSymbol<T> + 'static>>,
    /// Parsers used for prefix symbols
    prefix_parsers: HashMap<(TokenType, CowStr), Rc<PrefixSymbol<T> + 'static>>,
    /// Mapping of tokens to applied operators
    token_operators: HashMap<(TokenType, CowStr), Operator>
}

impl<T: Tokenizer> Parser<T> {
    /// Consumes the next token from the tokenizer.
    pub fn consume(&mut self) -> Token {
        self.look_ahead(1usize);
        self.lookahead.pop()
            .expect("Unable to queue token via lookahead for consume")
    }

    /// Grab `count` more tokens from the lexer and return the last one.
    ///
    /// Usually called with `0usize` to just peek at the next one.
    pub fn look_ahead(&mut self, count: usize) -> &Token {
        debug_assert!(count != 0, "Cannot look ahead 0");
        while count > self.lookahead.len() {
            let next = self.tokenizer.next();
            self.lookahead.push(next);
        }
        &self.lookahead[count - 1]
    }

    /// Peeks at the next available token
    pub fn peek(&mut self) -> &Token {
        self.look_ahead(1usize)
    }

    /// Attempts to match the next token from the tokenizer with the given type.
    pub fn try_consume_type(&mut self, expected_type: TokenType) -> Result<Token, ParseError> {
        let token = self.consume();
        if token.data.get_type() != expected_type {
            Err(ParseError::ExpectedToken {
                expected: expected_type,
                got: token.into()
            })
        }
        else {
            Ok(token)
        }
    }

    /// Attempts to match the next token from the tokenizer with the given type and name.
    pub fn try_consume_name(&mut self, expected_type: TokenType, expected_name: CowStr)
            -> Result<Token, ParseError> {
        let token = try!(self.try_consume_type(expected_type));
        if token.text != expected_name {
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
        self.look_ahead(1usize);
        self.lookahead[0]
            .data.get_type()
    }

    /// Parses any expression with the given precedence.
    pub fn expression(&mut self, precedence: Precedence) -> Result<Expression, ParseError> {
        let mut token = self.consume();
        println!("Parsing expression(precedence={:?}) with {}", precedence, token);
        let prefix: Rc<PrefixSymbol<T> + 'static>;
        if token.data.get_type() == TokenType::EOF {
            println!("Parsing received EOF!");
            return Err(ParseError::LazyString(format!("got eof?")));
        }
        else if token.data.get_type() == TokenType::Ident {
            println!("Parsing an identifier, using the identifier parser");
            prefix = Rc::new(IdentifierParser {});
        }
        else if token.data.get_type() == TokenType::Literal {
            println!("Got a literal token");
            prefix = Rc::new(LiteralParser {});
        }
        else if let Some(found_parser) = self.prefix_parsers.get(&(token.data.get_type(), Cow::Borrowed(&*token.text))) {
            println!("Found a parser to parse ({:?}, {:?})", token.data.get_type(), token.text);
            prefix = found_parser.clone();
        }
        else {
            println!("Could not find a parser!");
            return Err(ParseError::LazyString(format!("Unexpected token {:?}", token)))
        }
        let mut left = try!(prefix.parse(self, token));
        println!("Parsed left expression: {:?}", left);
        while precedence < self.current_precedence() {
            println!("Checking thatn {:?} < {:?}", precedence, self.current_precedence());
            token = self.consume();
            println!("Continuing with {}", token);
            if let Some(infix) = self.infix_parsers.get(&(token.data.get_type(), Cow::Borrowed(&*token.text))).map(Rc::clone) {
                println!("Parsing via infix parser!");
                left = try!(infix.parse(self, left, token));
            }
        }
        println!("Done parsing expression");
        Ok(left)
    }

    /// Parse a block of code. This is synonymous with a "program" as programs do not support
    /// nested blocks. Later on, this will be using the lexer's significant whitespace parsing
    /// to support `Indent` and `Outdent` tokens for begin/end blocks.
    pub fn block(&mut self) -> Result<Vec<Expression>, ParseError> {
        let mut found = Vec::new();
        loop {
            if self.next_type() == TokenType::EOF {
                break
            }
            let next_expr = try!(self.expression(Precedence::Min));
            found.push(next_expr);
        }
        return Ok(found)
    }

    ///Grab an lvalue from the token stream
    pub fn lvalue(&mut self) -> Result<Identifier, ParseError> {
        let token = self.consume();
        println!("Getting an lvalue from {}", token);
        if token.data.get_type() == TokenType::Ident {
            IdentifierParser { }.parse(self, token)
                .and_then(|e| e.expect_identifier())
        } else {
            Err(ParseError::ExpectedToken {
                expected: TokenType::Ident,
                got: token
            })
        }
    }

    /// Gets the operator registered for the given token.
    pub fn operator(&self, token_type: TokenType, text: &CowStr) -> Result<Operator, ParseError> {
        use std::ops::Deref;
        if let Some(op) = self.token_operators.get(&(token_type, Cow::Borrowed(text.deref()))) {
            Ok(*op)
        } else {
            Err(ParseError::UnknownOperator { text: text.clone(), token_type: token_type })
        }
    }

    /// Create a new parser from the given tokenizer, initializing its fields to match
    pub fn new(tokenizer: T) -> Parser<T> {
        use parse::symbol::*;
        use lex::tokens;
        use lex::TokenType::*;
        let infix_map: HashMap<(TokenType, CowStr), Rc<InfixSymbol<T> + 'static>> = hashmap![
            (Symbol, tokens::Equals) => Rc::new(AssignmentParser { }) as Rc<InfixSymbol<T>>,

            (Symbol, tokens::Plus) => BinOpSymbol::with_precedence(Precedence::AddSub),
            (Symbol, tokens::Minus) => BinOpSymbol::with_precedence(Precedence::AddSub),
            (Symbol, tokens::Star) => BinOpSymbol::with_precedence(Precedence::MulDiv),
            (Symbol, tokens::Slash) => BinOpSymbol::with_precedence(Precedence::MulDiv),

            (Symbol, tokens::Percent) => BinOpSymbol::with_precedence(Precedence::Modulo),

            (Symbol, tokens::PlusEquals) => Rc::new(AssignOpParser { }) as Rc<InfixSymbol<T>>,
            (Symbol, tokens::MinusEquals) => Rc::new(AssignOpParser { }) as Rc<InfixSymbol<T>>,
            (Symbol, tokens::StarEquals) => Rc::new(AssignOpParser { }) as Rc<InfixSymbol<T>>,
            (Symbol, tokens::PercentEquals) => Rc::new(AssignOpParser { }) as Rc<InfixSymbol<T>>,
            (Symbol, tokens::SlashEquals) => Rc::new(AssignOpParser { }) as Rc<InfixSymbol<T>>
        ];
        let prefix_map: HashMap<(TokenType, CowStr), Rc<PrefixSymbol<T> + 'static>> = hashmap![
            (Keyword, tokens::Let) => Rc::new(DeclarationParser { }) as Rc<PrefixSymbol<T>>,

            (Symbol, tokens::Minus) => UnaryOpSymbol::with_precedence(Precedence::NumericPrefix),
            (Symbol, tokens::LeftParen) => Rc::new(ParensParser { }) as Rc<PrefixSymbol<T>>,

            (Keyword, tokens::Return) => Rc::new(ReturnParser { }) as Rc<PrefixSymbol<T>>,
        ];
        let operator_map: HashMap<(TokenType, CowStr), Operator> = hashmap![
            (Symbol, tokens::Plus) => Operator::Addition,
            (Symbol, tokens::PlusEquals) => Operator::Addition,
            (Symbol, tokens::Minus) => Operator::Subtraction,
            (Symbol, tokens::MinusEquals) => Operator::Subtraction,
            (Symbol, tokens::Star) => Operator::Multiplication,
            (Symbol, tokens::StarEquals) => Operator::Multiplication,
            (Symbol, tokens::Slash) => Operator::Division,
            (Symbol, tokens::SlashEquals) => Operator::Division,
            (Symbol, tokens::Percent) => Operator::Modulus,
            (Symbol, tokens::PercentEquals) => Operator::Modulus,
        ];

        Parser {
            tokenizer: tokenizer,
            lookahead: Vec::with_capacity(2usize),
            infix_parsers: infix_map,
            prefix_parsers: prefix_map,
            token_operators: operator_map
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let block = try!(self.block());
        let program = Verifier { }.verify_program(block);
        if let Err(errors) = program {
            Err(ParseError::VerifierError { collection: errors })
        }
        else {
            Ok(program.expect("Checked expect"))
        }
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
