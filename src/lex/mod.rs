//! Contains the lexer which reads constable syntax.

mod identifier;

mod token;

mod lexer;

/// All the keywords in the language.
pub const KEYWORDS: &'static[&'static str] = &[
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
