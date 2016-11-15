//! Contains the lexer which reads constable syntax.

mod precedence;
mod symbol;
mod token;
mod expression;
mod errors;
mod error_codes;
mod tokenizer;
mod grapheme_tokenizer;
mod parser;

pub use self::expression::{Expression, ExpressionType};
pub use self::errors::{ParseResult, ParseError, TokenResult};
pub use self::token::{Token, TokenType};
pub use self::tokenizer::Tokenizer;
pub use self::parser::{Parser};

/// All the keywords in the language.
pub const KEYWORDS: &'static [&'static str] = &[
    "and", "or", "not", "bitand", "bitor", "bitnot",
    "none", "true", "false",
    "case", "match", "switch",
    "for", "while", "loop", "if", "else",
    "break", "continue", "do",
    "let", "mut", "const", "static",
    "type", "class", "struct", "enum", "trait",
    "extends", "implements", "derive", "where", "of",
    "public", "module", "package", "use",
    "async", "await", "fixed", "send", "sync", "channel"
    ];
