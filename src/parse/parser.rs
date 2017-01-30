//! Pratt parser
//!
//! The parser is a configurable object which parses a stream of tokens into a
//! source tree.

use std::borrow::{Cow, BorrowMut};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::Cell;

use lex::{CowStr, Token, TokenType, TokenData, Tokenizer};
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
    token_operators: HashMap<(TokenType, CowStr), Operator>,
    /// Allows the parser to skip over unneeded indentation
    indent_rules: Vec<IndentationRule>
}

impl<T: Tokenizer> Parser<T> {

    /// Peeks at the next available token
    pub fn peek(&mut self) -> &Token {
        self.look_ahead(1usize)
    }

    /// Peeks at the next available token
    pub fn peek_around_indent(&mut self) -> (bool, &Token) {
        let mut indent = false;
        for size in 1usize.. {
            let peeked_type = self.look_ahead(size).data.get_type();
            if peeked_type != TokenType::BeginBlock &&
                peeked_type != TokenType::EndBlock {
                return (indent, self.look_ahead(size))
            }
            else {
                indent = true;
            }
        }
        unreachable!()
    }

    /// Determines if the next token to be peeked at is on a different
    // line
    pub fn peek_is_newline(&mut self, current: &Token) -> bool {
        let peeked = self.look_ahead(1usize);
        peeked.location.line > current.location.line
    }

    /// Consumes the next token from the tokenizer.
    pub fn consume(&mut self) -> Token {
        self.look_ahead(1usize);
        self.lookahead.pop()
            .expect("Unable to queue token via lookahead for consume")
    }

    /// Consume the next token, returning whether the given rule has been
    /// applied.
    pub fn consume_with_rule(&mut self, rule: IndentationRule) -> (bool, Token) {
        let next = self.consume();
        if next.data.get_type() == TokenType::BeginBlock {
            self.indent_rules.push(rule);
            (true, self.consume())
        } else {
            (false, next)
        }
    }

    /// Grab `count` more tokens from the lexer and return the last one.
    pub fn look_ahead(&mut self, count: usize) -> &Token {
        debug_assert!(count != 0, "Cannot look ahead 0");
        while count > self.lookahead.len() {
            let next = self.tokenizer.next();
            if self.indent_rules.is_empty() {
                self.lookahead.push(next);
            }
            else {
                let indent_rule = self.indent_rules.last().cloned()
                    .expect("checked expect");
                match indent_rule {
                    // Ignore indentation until match found
                    IndentationRule::UntilToken(_indent_type) => {
                        unimplemented!()
                    },
                    // Negate the next EndBlock
                    IndentationRule::NegateDeindent => {
                        if next.data.get_type() == TokenType::EndBlock {
                            self.indent_rules.pop();
                            continue
                        }
                    },
                    // ????
                    IndentationRule::ResetIndentation => {

                    },
                    // ????
                    IndentationRule::ClearIndentation => {

                    },
                    // Negate all the indentation
                    IndentationRule::DisableIndentation => {
                        if next.data.get_type() == TokenType::BeginBlock
                            || next.data.get_type() == TokenType::EndBlock {
                            continue
                        }
                    },
                    IndentationRule::None | IndentationRule::ExpectNoIndent => {
                        unreachable!("Invalid IndentationRule found its way onto the stack")
                    }
                }
            }
        }
        &self.lookahead[count - 1]
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
        self.peek().data.get_type()
    }

    /// Parses any expression with the given precedence.
    pub fn expression(&mut self, precedence: Precedence) -> Result<Expression, ParseError> {
        let mut token = self.consume();
        trace!("Parsing expression(precedence={:?}) with {}", precedence, token);
        let prefix: Rc<PrefixSymbol<T> + 'static>;
        if token.data.get_type() == TokenType::EOF {
            trace!("Parsing received EOF!");
            return Err(ParseError::LazyString(format!("got eof?")));
        }
        else if token.data.get_type() == TokenType::EndBlock {
            trace!("Received end block mid-parse");
            return Err(ParseError::LazyString("Unexpected EndBlock".to_string()))
        }
        else if token.data.get_type() == TokenType::Ident {
            trace!("Parsing an identifier, using the identifier parser");
            prefix = Rc::new(IdentifierParser {});
        }
        else if token.data.get_type() == TokenType::Literal {
            trace!("Got a literal token");
            prefix = Rc::new(LiteralParser {});
        }
        else if let Some(found_parser) = self.prefix_parsers.get(&(token.data.get_type(), Cow::Borrowed(&*token.text))) {
            trace!("Found a parser to parse ({:?}, {:?})", token.data.get_type(), token.text);
            prefix = found_parser.clone();
        }
        else {
            trace!("Could not find a parser!");
            return Err(ParseError::LazyString(format!("Unexpected token {:?}", token)))
        }
        let mut left = try!(prefix.parse(self, token));
        trace!("Parsed left expression: {:?}", left);
        while precedence < self.current_precedence() {
            trace!("Checking that {:?} < {:?}", precedence, self.current_precedence());
            token = self.consume();
            trace!("Continuing with {}", token);
            if let Some(infix) = self.infix_parsers.get(&(token.data.get_type(), Cow::Borrowed(&*token.text))).map(Rc::clone) {
                trace!("Parsing via infix parser!");
                left = try!(infix.parse(self, left, token));
            }
        }
        trace!("Done parsing expression");
        Ok(left)
    }

    /// Parse a block of code. This is synonymous with a "program" as programs do not support
    /// nested blocks. Later on, this will be using the lexer's significant whitespace parsing
    /// to support `Indent` and `Outdent` tokens for begin/end blocks.
    pub fn block(&mut self) -> Result<Block, ParseError> {
        let mut found = Vec::new();
        loop {
            if self.next_type() == TokenType::EOF {
                break
            }
            let next_expr = try!(self.expression(Precedence::Min));
            found.push(next_expr);
        }
        return Ok(Block::new(found))
    }

    ///Grab an lvalue from the token stream
    pub fn lvalue(&mut self) -> Result<Identifier, ParseError> {
        let token = self.consume();
        trace!("Getting an lvalue from {}", token);
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
            token_operators: operator_map,
            indent_rules: Vec::new()
        }
    }

    /// Parse a block and verify it for errors
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
            lookup = (looked_ahead.data.get_type(),
                      Cow::Owned(looked_ahead.text.deref().into()));
        }
        if let Some(infix_parser) = self.infix_parsers.get(&lookup) {
            infix_parser.get_precedence()
        } else {
            Precedence::Min
        }
    }
}

#[derive(Debug, Clone)]
pub enum IndentationRule {
    /// Ignore indentation until a matching token is consumed
    UntilToken(TokenData),
    /// Remove the next indentation found
    NegateDeindent,
    /// Ignore all indent/deindent tokens
    DisableIndentation,
    /// Push back any saved indentation
    ResetIndentation,
    /// Clear any indentation encountered
    ClearIndentation,
    /// Compiler should emit indentation errors
    ExpectNoIndent,
    /// Receive all whitespace tokens
    None,
}
