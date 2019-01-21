//! Contains the lexer which reads protosnirk syntax.

mod span;
mod token;
pub mod tokens;
mod textiter;
pub mod tokenizer;
mod newtok;

pub use self::span::{Location, Span};
pub use self::token::{Token, TokenData};
pub use self::tokens::TokenType;
pub use self::textiter::{TextIter, PeekTextIter};
pub use self::tokenizer::{Tokenizer, IterTokenizer};

/// Fixed number of spaces to go before an indentation is emitted
pub const SPACES_PER_INDENT: u32 = 4;

/// Type representing a borrowed or owned string
pub type CowStr = ::std::borrow::Cow<'static, str>;

/// Rule for the symbol list indicating to the tokenizer
/// how to parse symbols.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TokenizerSymbolRule {
    /// The symbol is completely parsed.
    Complete,
    /// The symbol is partially parsed.
    /// More characters *must* match for a complete symbol.
    Partial,
    /// The parse is a complete symbol, but it could be part of
    /// another symbol depending on the characters after it.
    CompletePrefix
}

pub trait Tokenizer : std::fmt::Debug {
    fn next(&mut self) -> Result<Token, TokenizerError>;
}

/// Errors emitted by the tokenizer. Note that a recovery token can be
/// emitted by the lexer so that parsing can continue with recovery.
///
/// - `TabCharacterFound`: an indentation token is included.
/// - `UnrecognizedUnicode`: an ident token is included.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenizerError {
    /// A tab character was found in the whitespace of the source file.
    TabCharacterFound(Token),
    /// Unrecognized unicode was found in an identifier in the source file
    UnrecognizedUnicode(Token)
}
