//! Pratt parser
//!
//! The parser is a configurable object which parses a stream of tokens into a
//! source tree.

use std::borrow::Cow;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

use lex::{CowStr, Token, TokenType, Tokenizer};
use parse::ParseError;
use ast::*;
use parse::symbol::*;

/// Parser object which parses things
pub struct Parser<T: Tokenizer> {
    /// Tokenizer which supplies tokens
    tokenizer: T,
    /// Lookahead queue for peeking
    lookahead: VecDeque<Token>,
    /// Parsers used for prefix symbols in statements (`return`, `do`)
    stmt_prefix_parsers: HashMap<TokenType, Rc<PrefixParser<Statement, T> + 'static>>,
    /// Parsers used for infix symbols in expressions (`+`, `<`)
    expr_infix_parsers: HashMap<TokenType, Rc<InfixParser<Expression, T> + 'static>>,
    /// Parsers used for prefix symbols in expressions (`not`, `let`)
    expr_prefix_parsers: HashMap<TokenType, Rc<PrefixParser<Expression, T> + 'static>>,
    /// Parses for parsing program items (struct/enum/fn declarations, etc.)
    item_parsers: HashMap<TokenType, Rc<PrefixParser<Item, T> + 'static>>,
    /// Allows the parser to skip over unneeded indentation
    indent_rules: Vec<IndentationRule>,
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
            let peeked_type = self.look_ahead(size).get_type();
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
        indent || peeked.location().line > current.location().line
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
        if next.get_type() == TokenType::BeginBlock {
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
                        if next.get_type() == indent_data {
                            self.indent_rules.pop();
                        }
                        // If indentation is found, skip it
                        else if next.get_type() == TokenType::BeginBlock
                                || next.get_type() == TokenType::EndBlock {
                            continue
                        }
                    },
                    // Negate the next EndBlock
                    IndentationRule::NegateDeindent => {
                        if next.get_type() == TokenType::EndBlock {
                            // Remove this rule so it won't trigger next time
                            // and go onto the next token.
                            self.indent_rules.pop();
                            continue
                        }
                    },
                    // Negate all the indentation
                    IndentationRule::DisableIndentation => {
                        if next.get_type() == TokenType::BeginBlock
                            || next.get_type() == TokenType::EndBlock {
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
        if token.get_type() != expected_type {
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
        if token.text() != expected_name {
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
        self.peek().get_type()
    }

    /// Push an indentation rule manually onto the stack
    pub fn push_rule(&mut self, rule: IndentationRule) {
        self.indent_rules.push(rule);
    }

    /// Pop an indentation rule manually from the stack
    pub fn pop_rule(&mut self) -> Option<IndentationRule> {
        self.indent_rules.pop()
    }

    /// Parses a type expression from the token stream.
    pub fn type_expr(&mut self) -> Result<TypeExpression, ParseError> {
        use parse::symbol::types::*;
        let next_type = self.next_type();
        trace!("Parsing type expression with {:?}", next_type);

        // Type expressions don't really have infixes so we just parse them -
        // the specifics of the array brackets/generic angles are handled by those
        // prefix parsers anyway.
        // Generic bounds (like `T: Managed + Cloneable`) will also have infix parsing.
        match next_type {
            TokenType::Ident => {
                trace!("Parsing named type expr");
                let consumed = self.consume();
                NamedTypeParser { }.parse(self, consumed)
            },
            _other => {
                trace!("Invalid token for type expr");
                // TODO this is also a bad error
                return Err(ParseError::LazyString(format!(
                    "Unexpected token {:?} for type expression", next_type)))
            }
        }
    }

    /// Parses any expression with the given precedence.
    ///
    /// This parser will push a `NegateDeindent` rule to the rule stack.
    pub fn expression(&mut self, precedence: Precedence) -> Result<Expression, ParseError> {
        let (_indented, mut token) = self.consume_indented(IndentationRule::NegateDeindent);
        trace!("Parsing expression(precedence={:?}) with {}", precedence, token);
        if _indented { trace!("Parsing indented expression"); }
        let token_type = token.get_type();
        let mut left = try!(match token_type {
            TokenType::EOF => {
                trace!("Unexpected EOF parsing expression");
                Err(ParseError::EOF)
            },
            TokenType::EndBlock => {
                trace!("Unexpected EndBlock parsing expression");
                Err(ParseError::EOF)
            },
            TokenType::If => {
                trace!("Parsing an if expression");
                IfExpressionParser { }.parse(self, token)
            },
            TokenType::Minus =>{
                trace!("Parsing a subtraction expression");
                UnaryOpExprSymbol::with_precedence(Precedence::NumericPrefix)
                    .parse(self, token)
            },
            TokenType::LeftParen => {
                trace!("Parsing a paren expression");
                ParensParser { }.parse(self, token)
            },
            TokenType::Ident => {
                trace!("Parsing an identifier reference");
                IdentifierParser { }.parse(self, token)
            },
            TokenType::Literal => {
                trace!("Parsing a literal expression");
                LiteralParser { }.parse(self, token)
            },
            _ => {
                trace!("Could not find parser");
                return Err(ParseError::LazyString(format!("Unexpected token {:?}", token)))
            }
        });
        trace!("Parsed left expression: {:?}", left);
        while precedence < self.current_precedence() {
            trace!("Consuming a token to determine if there's an infix");
            // We allow indentation before any infix operator in expression!
            let (_infix_indented, new_token) = self.consume_indented(IndentationRule::NegateDeindent);
            trace!("Consumed {:?}, indentation: {}", new_token, _infix_indented);
            token = new_token;
            let token_type = token.get_type();
            left = try!(match token_type {
                TokenType::Equals => {
                    trace!("Parsing an assignment");
                    AssignmentParser { }.parse(self, left, token)
                },
                TokenType::Plus => {
                    trace!("Parsing infix addition");
                    BinOpExprSymbol::with_precedence(Precedence::AddSub)
                        .parse(self, left, token)
                },
                TokenType::Minus => {
                    trace!("Parsing infix subtraction");
                    BinOpExprSymbol::with_precedence(Precedence::AddSub)
                        .parse(self, left, token)
                },
                TokenType::Star => {
                    trace!("Parsing with multiplication");
                    BinOpExprSymbol::with_precedence(Precedence::MulDiv)
                        .parse(self, left, token)
                },
                TokenType::Slash => {
                    trace!("Parsing with division");
                    BinOpExprSymbol::with_precedence(Precedence::MulDiv)
                        .parse(self, left, token)
                },
                TokenType::Percent => {
                    trace!("Parsing with modulo");
                    BinOpExprSymbol::with_precedence(Precedence::Modulo)
                        .parse(self, left, token)
                },
                TokenType::LeftParen => {
                    trace!("Parsing function call");
                    FnCallParser { }.parse(self, left, token)
                },
                TokenType::LeftAngle => {
                    trace!("Parsing less than expression");
                    BinOpExprSymbol::with_precedence(Precedence::EqualityCompare)
                        .parse(self, left, token)
                },
                TokenType::RightAngle => {
                    trace!("Parsing greater than expression");
                    BinOpExprSymbol::with_precedence(Precedence::EqualityCompare)
                        .parse(self, left, token)
                },
                TokenType::LessThanEquals => {
                    trace!("Parsing lte expression");
                    BinOpExprSymbol::with_precedence(Precedence::EqualityCompare)
                        .parse(self, left, token)
                },
                TokenType::GreaterThanEquals => {
                    trace!("Parsing gte expression");
                    BinOpExprSymbol::with_precedence(Precedence::EqualityCompare)
                        .parse(self, left, token)
                },
                TokenType::DoubleEquals => {
                    trace!("Parsing equality expression");
                    BinOpExprSymbol::with_precedence(Precedence::Equality)
                        .parse(self, left, token)
                },
                TokenType::NotEquals => {
                    trace!("Parsing not equals expression");
                    BinOpExprSymbol::with_precedence(Precedence::Equality)
                        .parse(self, left, token)
                },
                TokenType::PlusEquals
                | TokenType::MinusEquals
                | TokenType::StarEquals
                | TokenType::PercentEquals
                | TokenType::SlashEquals=> {
                    trace!("Parsing assign op");
                    AssignOpParser { }.parse(self, left, token)
                },
                _ => {
                    // If we can't match an infix then we need to parse the next
                    // expression.
                    break
                }
            });
            trace!("Checking that {:?} < {:?}", precedence, self.current_precedence());
            // ^ at the beginning of the loop
        }
        trace!("Done parsing expression");
        Ok(left)
    }

    /// Parse a single statement.
    pub fn statement(&mut self) -> Result<Statement, ParseError> {
        match self.next_type() {
            // This may be refactorable with NLL
            TokenType::Let => {
                trace!("Parsing let statement");
                let token = self.consume();
                DeclarationParser { }.parse(self, token)
            },
            TokenType::Return => {
                trace!("Parsing return statement");
                let token = self.consume();
                ReturnParser { }.parse(self, token)
            },
            TokenType::Do => {
                trace!("Parsing do block");
                let token = self.consume();
                DoBlockParser { }.parse(self, token)
            },
            TokenType::If => {
                trace!("Parsing an if block");
                let token = self.consume();
                IfBlockParser { }.parse(self, token)
            },
            _ => {
                trace!("Using expr parser for statement");
                self.expression(Precedence::Min)
                    .map(Expression::to_statement)
            }
        }
    }

    /// Parse a block of code.
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
        let token_type = self.next_type();
        let token = self.consume();
        match token_type {
            TokenType::Fn => {
                trace!("Parsing a fn");
                FnDeclarationParser { }.parse(self, token)
            },
            TokenType::Typedef => {
                trace!("Parsing a typedef");
                TypedefParser { }.parse(self, token)
            },
            _ => {
                Err(ParseError::LazyString(format!("Unexpected item token {:?}", token_type)))
            }
        }
    }

    /// Grab an lvalue from the token stream
    pub fn lvalue(&mut self) -> Result<Identifier, ParseError> {
        let token = self.consume();
        trace!("Getting an lvalue from {}", token);
        if token.get_type() == TokenType::Ident {
            IdentifierParser { }.parse(self, token)
                .and_then(|e| e.expect_identifier())
        } else {
            Err(ParseError::ExpectedToken {
                expected: TokenType::Ident,
                got: token
            })
        }
    }

    /// Gets the binary operator used for the given token.
    pub fn binary_operator(&self,
                           token_type: TokenType)
                           -> Result<BinaryOperator, ParseError> {
        use lex::TokenType::*;
        match token_type {
            Plus => Ok(BinaryOperator::Addition),
            Minus => Ok(BinaryOperator::Subtraction),
            Star => Ok(BinaryOperator::Multiplication),
            Slash => Ok(BinaryOperator::Division),
            Percent => Ok(BinaryOperator::Modulus),
            DoubleEquals => Ok(BinaryOperator::Equality),
            NotEquals => Ok(BinaryOperator::NonEquality),
            LeftAngle => Ok(BinaryOperator::LessThan),
            RightAngle => Ok(BinaryOperator::GreaterThan),
            LessThanEquals => Ok(BinaryOperator::LessThanEquals),
            GreaterThanEquals => Ok(BinaryOperator::GreaterThanEquals),
            _ => Err(ParseError::UnknownOperator {
                    text: Cow::from(format!("{:?}", token_type)),
                    token_type
                })
        }
    }

    pub fn unary_operator(&self,
                          token_type: TokenType)
                          -> Result<UnaryOperator, ParseError> {
        use lex::TokenType::*;
        match token_type {
            Minus => Ok(UnaryOperator::Negation),
            _ => Err(ParseError::UnknownOperator {
                    text: Cow::from(format!("{:?}", token_type)),
                    token_type
                })
        }
    }

    /// Create a new parser from the given tokenizer, initializing its fields to match
    pub fn new(tokenizer: T) -> Parser<T> {
        use parse::symbol::*;
        use lex::TokenType::*;
        let expr_infix_map: HashMap<TokenType, Rc<InfixParser<Expression, T> + 'static>> =
        hashmap![
            Equals => Rc::new(AssignmentParser { }) as Rc<InfixParser<Expression, T>>,

            Plus => BinOpExprSymbol::with_precedence(Precedence::AddSub),
            Minus => BinOpExprSymbol::with_precedence(Precedence::AddSub),
            Star => BinOpExprSymbol::with_precedence(Precedence::MulDiv),
            Slash => BinOpExprSymbol::with_precedence(Precedence::MulDiv),

            Percent => BinOpExprSymbol::with_precedence(Precedence::Modulo),

            LeftParen => Rc::new(FnCallParser { }) as Rc<InfixParser<Expression, T>>,

            LeftAngle => BinOpExprSymbol::with_precedence(Precedence::EqualityCompare),
            RightAngle => BinOpExprSymbol::with_precedence(Precedence::EqualityCompare),
            LessThanEquals => BinOpExprSymbol::with_precedence(Precedence::EqualityCompare),
            GreaterThanEquals => BinOpExprSymbol::with_precedence(Precedence::EqualityCompare),

            DoubleEquals => BinOpExprSymbol::with_precedence(Precedence::Equality),
            NotEquals => BinOpExprSymbol::with_precedence(Precedence::Equality),

            PlusEquals => Rc::new(AssignOpParser { }) as Rc<InfixParser<Expression, T>>,
            MinusEquals => Rc::new(AssignOpParser { }) as Rc<InfixParser<Expression, T>>,
            StarEquals => Rc::new(AssignOpParser { }) as Rc<InfixParser<Expression, T>>,
            PercentEquals => Rc::new(AssignOpParser { }) as Rc<InfixParser<Expression, T>>,
            SlashEquals => Rc::new(AssignOpParser { }) as Rc<InfixParser<Expression, T>>
        ];
        let expr_prefix_map: HashMap<TokenType, Rc<PrefixParser<Expression, T> + 'static>> =
        hashmap![
            If => Rc::new(IfExpressionParser { }) as Rc<PrefixParser<Expression, T>>,

            Minus => UnaryOpExprSymbol::with_precedence(Precedence::NumericPrefix),
            LeftParen => Rc::new(ParensParser { }) as Rc<PrefixParser<Expression, T>>,

            Ident => Rc::new(IdentifierParser { }) as Rc<PrefixParser<Expression, T>>,
            Literal => Rc::new(LiteralParser { }) as Rc<PrefixParser<Expression, T>>,
        ];
        let stmt_prefix_map: HashMap<TokenType, Rc<PrefixParser<Statement, T> + 'static>> =
        hashmap![
            Let => Rc::new(DeclarationParser { }) as Rc<PrefixParser<Statement, T>>,
            Return => Rc::new(ReturnParser { }) as Rc<PrefixParser<Statement, T>>,
            Do => Rc::new(DoBlockParser { }) as Rc<PrefixParser<Statement, T>>,
            If => Rc::new(IfBlockParser { }) as Rc<PrefixParser<Statement, T>>,
        ];
        let item_prefix_map: HashMap<TokenType, Rc<PrefixParser<Item, T> + 'static>> =
        hashmap![
            Fn => Rc::new(FnDeclarationParser { }) as Rc<PrefixParser<Item, T>>,
            Typedef => Rc::new(TypedefParser { }) as Rc<PrefixParser<Item, T>>,
        ];

        Parser {
            tokenizer: tokenizer,
            lookahead: VecDeque::new(),
            item_parsers: item_prefix_map,
            stmt_prefix_parsers: stmt_prefix_map,
            expr_prefix_parsers: expr_prefix_map,
            expr_infix_parsers: expr_infix_map,
            indent_rules: Vec::new(),
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
        Ok(unit)
    }

    /// Get the current precedence
    fn current_precedence(&mut self) -> Precedence {
        let looked_ahead = self.look_ahead(1).get_type();
 if let Some(infix_parser) = self.expr_infix_parsers.get(&looked_ahead).cloned() {
            infix_parser.precedence()
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
