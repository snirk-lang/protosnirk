//! Pratt parser
//!
//! The parser is a configurable object which parses a stream of tokens into a
//! source tree.

pub struct Parser<'a> {
    /// Tokenizer which supplies tokens
    tokenizer: Tokenizer<'a>,
    /// Lookahead stack for peeking
    lookahead: Vec<Token<'a>>,
    /// Parsers used for infix symbols
    infix_parsers: HashMap<TokenType, Box<InfixSymol>>,
    /// Parsers used for prefix symbols
    prefix_parsers: HashMap<TokenType, Box<PrefixSymbol>>
}

impl Parser {
    pub fn peek(&mut self) -> Token<'a> {
        
    }

    pub fn parse_expression(precedence: Precedence) -> SymbolResult {
        
    }

    pub fn parse_statement(precedence: Precedence) -> SymbolResult {
        
    }

    pub fn parse_program() -> SymbolResult {
        let mut expressions = Vec::new();
        while let Ok(expr) = self.parse_expression()
    }

    pub fn parse_expr_of(&mut self, expr_type: TokenType, precedence: Precedence) -> SymbolResult<'a> {
        let mut token = self.consume_token();
        let maybe_prefix = self.prefix_parsers.get(token.get_type());
        if let Some(prefix) = maybe_prefix {
            let mut left = try!(prefix.parse(self, token));

            while precedence < self.current_precedence() {
                token = self.consume_token();

                if let Some(infix) = infix_parsers.get(token.get_type()) {
                    left = try!(infix.parse(self, left, token));
                }
            }
            Ok(left)
        } else {
            Err(ParserError::Generic("Unexpected tokens n stuff".into()))
        }
    }

    pub fn parse_expr_of_any(&mut self, expr_types: &'static[TokenType], precedence: Precedence) -> SymbolResult<'a> {
        
    }

    pub fn with_protosnirk_parsers() -> Parser {
        let mut infix_map = hashmap![
            TokenType::Assign => Box::new(AssignParser),

            TokenType::Add => BinOpSymbol::with_precedence(Precedence::AddSub),
            TokenType::Sub => BinOpSymbol::with_precedence(Precedence::AddSub),
            TokenType::Mul => BinOpSymbol::with_precedence(Precedence::MulDiv),
            TokenType::Div => BinOpSymbol::with_precedence(Precedence::MulDiv),

            TokenType::Mod => BinOpSymbol::with_precedence(Precedence::Modulo),
        ];
        let mut prefix_map = hashmap![
            TokenType::Let => Box::new(DeclarationParser),
            TokenType::Mut => Box::new(DeclarationParser),

            TokenType::Negate => UnaryOpSymbol::with_precedence(Precedence::UnaryOp),
            TokenType::LeftParen => Box::new(ParensParser),

            TokenType::Return => Box::new(ReturnParser),

            TokenType::Name => Box::new(IdentifierParser)
        ];
    }
}

pub type SymbolResult<'a> = Result<Token<'a>, ParseError>;

/// Error types when parsing
pub enum ParseError {
    Expected {
        expected: TokenType,
        got: Token,
        info: String
    },
    Generic {
        info: String
    },
    EOF {
        expected: TokenType
    }
    Other
}
