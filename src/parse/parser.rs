//! Pratt parser
//!
//! The parser is a configurable object which parses a stream of tokens into a
//! source tree.

use std::borrow::{Cow, BorrowMut};
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
use std::cell::Cell;

use lex::{CowStr, Token, TokenType, TokenData, Tokenizer};
use parse::{ParseError, ParseResult};
use parse::ast::*;
use parse::symbol::*;

/// Parser object which parses things
pub struct Parser<T: Tokenizer> {
    /// Tokenizer which supplies tokens
    tokenizer: T,
    /// Lookahead queue for peeking
    lookahead: VecDeque<Token>,
    /// Parsers used for prefix symbols in statements (`return`, `do`)
    stmt_prefix_parsers: HashMap<(TokenType, CowStr), Rc<PrefixParser<Statement, T> + 'static>>,
    /// Parsers used for infix symbols in expressions (`+`, `<`)
    expr_infix_parsers: HashMap<(TokenType, CowStr), Rc<InfixParser<Expression, T> + 'static>>,
    /// Parsers used for prefix symbols in expressions (`not`, `let`)
    expr_prefix_parsers: HashMap<(TokenType, CowStr), Rc<PrefixParser<Expression, T> + 'static>>,
    /// Parses for parsing program items (struct/enum/fn declarations, etc.)
    item_parsers: HashMap<(TokenType, CowStr), Rc<PrefixParser<Item, T> + 'static>>,
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
    pub fn peek_indented(&mut self) -> (bool, &Token) {
        let mut indent = false;
        for size in 1usize.. {
            let peeked_type = self.look_ahead(size).data.get_type();
            if peeked_type != TokenType::BeginBlock {
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
        let (indent, peeked) = self.peek_indented();
        indent || peeked.location.line > current.location.line
    }

    /// Consumes the next token from the tokenizer.
    pub fn consume(&mut self) -> Token {
        self.look_ahead(1usize);
        self.lookahead.pop_back()
            .expect("Unable to queue token via lookahead for consume")
    }

    /// Consume the next token, returning whether the given rule has been
    /// applied.
    pub fn consume_indented(&mut self, rule: IndentationRule) -> (bool, Token) {
        let next = self.consume();
        if next.data.get_type() == TokenType::BeginBlock {
            self.indent_rules.push(rule);
            (true, self.consume())
        } else {
            (false, next)
        }
    }

    /// If the next token is an indent, comsume it add the indentaiton rule to the stack.
    pub fn apply_indentation(&mut self, rule: IndentationRule) -> bool {
        if self.next_type() == TokenType::BeginBlock {
            trace!("Applying rule {:?} to BeginBlock", rule);
            self.consume();
            self.indent_rules.push(rule);
            true
        }
        else {
            false
        }
    }

    /// Grab `count` more tokens from the lexer and return the last one.
    ///
    /// This core method applies all indentation rules
    pub fn look_ahead(&mut self, count: usize) -> &Token {
        debug_assert!(count != 0, "Cannot look ahead 0");
        while count > self.lookahead.len() {
            let next = self.tokenizer.next();
            if let Some(indent_rule) = self.indent_rules.last().cloned() {
                match indent_rule {
                    // Ignore indentation until match found
                    IndentationRule::DisableUntil(indent_data) => {
                        // If match is found, disable this rule, return the match
                        if next.data.get_type() == indent_data {
                            self.indent_rules.pop();
                        }
                        // If indentation is found, skip it
                        else if next.data.get_type() == TokenType::BeginBlock
                                || next.data.get_type() == TokenType::EndBlock {
                            continue
                        }
                    },
                    // Negate the next EndBlock
                    IndentationRule::NegateDeindent => {
                        if next.data.get_type() == TokenType::EndBlock {
                            // Remove this rule so it won't trigger next time
                            // and go onto the next token.
                            self.indent_rules.pop();
                            continue
                        }
                    },
                    // Negate all the indentation
                    IndentationRule::DisableIndentation => {
                        if next.data.get_type() == TokenType::BeginBlock
                            || next.data.get_type() == TokenType::EndBlock {
                            continue
                        }
                    },
                }
            }
            self.lookahead.push_back(next)
        }
        &self.lookahead[count - 1]
    }

    /// Attempts to match the next token from the tokenizer with the given type.
    pub fn consume_type(&mut self, expected_type: TokenType) -> Result<Token, ParseError> {
        trace!("Consuming type {:?}", expected_type);
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

    /// Attempts to match the next token from the tokenizer with the given type.
    /// If the next token is an indentation, applies the rule and returns `(true, ...)``
    pub fn consume_type_indented(&mut self, expected_type: TokenType, rule: IndentationRule)
                                 -> Result<(bool, Token), ParseError> {
        let indented = self.apply_indentation(rule);
        self.consume_type(expected_type).map(|t| (indented, t))
    }

    /// Attempts to match the next token from the tokenizer with the given type and name.
    pub fn consume_name(&mut self, expected_type: TokenType, expected_name: CowStr)
            -> Result<Token, ParseError> {
        trace!("Consuming name {}", expected_name);
        let token = try!(self.consume_type(expected_type));
        if token.text != expected_name {
            Err(ParseError::ExpectedToken {
                expected: expected_type,
                got: token.into()
            })
        }
        else {
            Ok(token)
        }
    }

    /// Attempt to match the next token by name, applying the given rule
    /// if whitespace is found.
    pub fn consume_name_indented(&mut self,
                                 expected_type: TokenType,
                                 expected_name: CowStr,
                                 rule: IndentationRule) -> Result<(bool, Token), ParseError> {
        let indented = self.apply_indentation(rule);
        self.consume_name(expected_type, expected_name).map(|t| (indented, t))
    }

    /// Peek at the next token without consuming it.
    pub fn next_type(&mut self) -> TokenType {
        self.peek().data.get_type()
    }

    /// Push an indentation rule manually onto the stack
    pub fn push_rule(&mut self, rule: IndentationRule) {
        self.indent_rules.push(rule);
    }

    /// Pop an indentation rule manually from the stack
    pub fn pop_rule(&mut self) -> Option<IndentationRule> {
        self.indent_rules.pop()
    }

    /// Parses any expression with the given precedence.
    ///
    /// This parser will push a `NegateDeindent` rule to the rule stack.
    pub fn expression(&mut self, precedence: Precedence) -> Result<Expression, ParseError> {
        let (_indented, mut token) = self.consume_indented(IndentationRule::NegateDeindent);
        trace!("Parsing expression(precedence={:?}) with {}", precedence, token);
        if _indented { trace!("Parsing indented expression"); }
        let prefix: Rc<PrefixParser<Expression, T> + 'static>;
        if token.data.get_type() == TokenType::EOF {
            trace!("Parsing received EOF!");
            return Err(ParseError::EOF);
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
        else if let Some(found_parser) = self.expr_prefix_parsers.get(&(token.data.get_type(), Cow::Borrowed(&*token.text))) {
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
            trace!("Consuming a token to determine if there's an infix");
            // We allow indentation before any infix operator in expression!
            let (_infix_indented, new_token) = self.consume_indented(IndentationRule::NegateDeindent);
            trace!("Consumed {:?}, indentation: {}", new_token, _infix_indented);
            token = new_token;
            if let Some(infix) = self.expr_infix_parsers.get(&(token.data.get_type(), Cow::Borrowed(&*token.text))).map(Rc::clone) {
                trace!("Parsing via infix parser!");
                left = try!(infix.parse(self, left, token));
            }
            // consuming might be an issue here
            //else {
            //    break
            //}
            trace!("Checking that {:?} < {:?}", precedence, self.current_precedence());
        }
        trace!("Done parsing expression");
        Ok(left)
    }

    /// Parse a single statement.
    ///
    pub fn statement(&mut self) -> Result<Statement, ParseError> {
        let mut found_parser: Option<Rc<PrefixParser<Statement, T> + 'static>> = None;
        let peek_data = (self.next_type(), Cow::Owned(self.peek().text.to_string()));
        if let Some(stmt_parser) = self.stmt_prefix_parsers.get(&(peek_data.0, Cow::Borrowed(&*peek_data.1))) {
            trace!("Found statement parser for {}", &peek_data.1);
            found_parser = Some(stmt_parser.clone());
        }
        if found_parser.is_none() {
            trace!("Using expr parser for statement");
            return self.expression(Precedence::Min).map(Expression::to_statement)
        }
        let token = self.consume();
        return found_parser.expect("Checked expect").parse(self, token)
    }

    /// Parse a block of code.
    ///
    /// This is synonymous with a "program" as programs do not support
    /// nested blocks. Later on, this will be using the lexer's significant whitespace parsing
    /// to support `Indent` and `Outdent` tokens for begin/end blocks.
    ///
    /// Block parsing assumes the `BeginBlock` token has already been consumed.
    pub fn block(&mut self) -> Result<Block, ParseError> {
        let mut found = Vec::new();
        loop {
            let next_type = self.next_type();
            if next_type == TokenType::EOF {
                break
            }
            else if next_type == TokenType::EndBlock {
                self.consume();
                break
            }
            let next_stmt = try!(self.statement());
            found.push(next_stmt);
        }
        return Ok(Block::new(found))
    }

    /// Parse an item from a program (a function definition)
    pub fn item(&mut self) -> Result<Item, ParseError> {
        let mut found_parser: Option<Rc<PrefixParser<Item, T> + 'static>> = None;
        let peek_data = (self.next_type(), Cow::Owned(self.peek().text.to_string()));
        if let Some(item_parser) = self.item_parsers.get(&(peek_data.0, Cow::Borrowed(&*peek_data.1))) {
            trace!("Found item parser for {}", &peek_data.1);
            found_parser = Some(item_parser.clone());
        }
        match found_parser {
            Some(parser) => {
                let token = self.consume();
                parser.parse(self, token)
            },
            None =>
                Err(ParseError::LazyString(format!("Unexpeted item token `{}`", &peek_data.1)))
        }
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
        let expr_infix_map: HashMap<(TokenType, CowStr), Rc<InfixParser<Expression, T> + 'static>> =
        hashmap![
            (Symbol, tokens::Equals) => Rc::new(AssignmentParser { }) as Rc<InfixParser<Expression, T>>,

            (Symbol, tokens::Plus) => BinOpExprSymbol::with_precedence(Precedence::AddSub),
            (Symbol, tokens::Minus) => BinOpExprSymbol::with_precedence(Precedence::AddSub),
            (Symbol, tokens::Star) => BinOpExprSymbol::with_precedence(Precedence::MulDiv),
            (Symbol, tokens::Slash) => BinOpExprSymbol::with_precedence(Precedence::MulDiv),

            (Symbol, tokens::Percent) => BinOpExprSymbol::with_precedence(Precedence::Modulo),

            (Symbol, tokens::LeftParen) => Rc::new(FnCallParser { }) as Rc<InfixParser<Expression, T>>,

            (Symbol, tokens::LeftAngle) => BinOpExprSymbol::with_precedence(Precedence::EqualityCompare),
            (Symbol, tokens::RightAngle) => BinOpExprSymbol::with_precedence(Precedence::EqualityCompare),
            (Symbol, tokens::LessThanEquals) => BinOpExprSymbol::with_precedence(Precedence::EqualityCompare),
            (Symbol, tokens::GreaterThanEquals) => BinOpExprSymbol::with_precedence(Precedence::EqualityCompare),

            (Symbol, tokens::DoubleEquals) => BinOpExprSymbol::with_precedence(Precedence::Equality),
            (Symbol, tokens::NotEquals) => BinOpExprSymbol::with_precedence(Precedence::Equality),

            (Symbol, tokens::PlusEquals) => Rc::new(AssignOpParser { }) as Rc<InfixParser<Expression, T>>,
            (Symbol, tokens::MinusEquals) => Rc::new(AssignOpParser { }) as Rc<InfixParser<Expression, T>>,
            (Symbol, tokens::StarEquals) => Rc::new(AssignOpParser { }) as Rc<InfixParser<Expression, T>>,
            (Symbol, tokens::PercentEquals) => Rc::new(AssignOpParser { }) as Rc<InfixParser<Expression, T>>,
            (Symbol, tokens::SlashEquals) => Rc::new(AssignOpParser { }) as Rc<InfixParser<Expression, T>>
        ];
        let expr_prefix_map: HashMap<(TokenType, CowStr), Rc<PrefixParser<Expression, T> + 'static>> =
        hashmap![
            (Keyword, tokens::Let) => Rc::new(DeclarationParser { }) as Rc<PrefixParser<Expression, T>>,
            (Keyword, tokens::If) => Rc::new(IfExpressionParser { }) as Rc<PrefixParser<Expression, T>>,

            (Symbol, tokens::Minus) => UnaryOpExprSymbol::with_precedence(Precedence::NumericPrefix),
            (Symbol, tokens::LeftParen) => Rc::new(ParensParser { }) as Rc<PrefixParser<Expression, T>>,
        ];
        let stmt_prefix_map: HashMap<(TokenType, CowStr), Rc<PrefixParser<Statement, T> + 'static>> =
        hashmap![
            (Keyword, tokens::Return) => Rc::new(ReturnParser { }) as Rc<PrefixParser<Statement, T>>,
            (Keyword, tokens::Do) => Rc::new(DoBlockParser { }) as Rc<PrefixParser<Statement, T>>,
            (Keyword, tokens::If) => Rc::new(IfBlockParser { }) as Rc<PrefixParser<Statement, T>>,
        ];
        let item_prefix_map: HashMap<(TokenType, CowStr), Rc<PrefixParser<Item, T> + 'static>> =
        hashmap![
            (Keyword, tokens::Fn) => Rc::new(FnDeclarationParser { }) as Rc<PrefixParser<Item, T>>,
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
            (Symbol, tokens::LeftAngle) => Operator::LessThan,
            (Symbol, tokens::LessThanEquals) => Operator::LessThanEquals,
            (Symbol, tokens::RightAngle) => Operator::GreaterThan,
            (Symbol, tokens::GreaterThanEquals) => Operator::GreaterThan,
            (Symbol, tokens::DoubleEquals) => Operator::Equality,
            (Symbol, tokens::NotEquals) => Operator::NonEquality
        ];

        Parser {
            tokenizer: tokenizer,
            lookahead: VecDeque::new(),
            item_parsers: item_prefix_map,
            stmt_prefix_parsers: stmt_prefix_map,
            expr_prefix_parsers: expr_prefix_map,
            expr_infix_parsers: expr_infix_map,
            token_operators: operator_map,
            indent_rules: Vec::new()
        }
    }

    /// Parse a program and verify it for errors
    pub fn parse_unit(&mut self) -> Result<Unit, ParseError> {
        let mut items = Vec::with_capacity(10);
        while self.next_type() != TokenType::EOF {
            let item = try!(self.item());
            trace!("Parsed an item");
            items.push(item);
        }
        trace!("Parsed {} items", items.len());
        let unit = Unit::new(items);
        trace!("Parsed unit {:#?}", unit);
        unit
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
        if let Some(infix_parser) = self.expr_infix_parsers.get(&lookup) {
            infix_parser.get_precedence()
        } else {
            Precedence::Min
        }
    }
}

/// Rules for handling indentation when parsing
#[derive(Debug, Clone)]
pub enum IndentationRule {
    /// Ignore indentation until a matching token is consumed
    DisableUntil(TokenType),
    /// Remove the next indentation found
    NegateDeindent,
    /// Ignore all indent/deindent tokens
    DisableIndentation,
}
