//! Contains lists of the default tokens in protosnirk

use std::borrow::Cow;
use std::collections::{HashMap, HashSet};

use lex::{Token, TokenData, CowStr, TokenizerSymbolRule};
use lex::TokenizerSymbolRule::*;

macro_rules! declare_tokens {
    (
        symbols { $($sym_name:ident : $sym_val:expr; $sym_rule:expr,)* }
        symparts { $($part_val:expr; $part_rule:expr,)* }
        keywords { $($kw_name:ident : $kw_val:expr,)* }
        tynames { $($ty_name:ident : $ty_val:expr,)* }
    ) => {
        $(#[allow(non_upper_case_globals, dead_code)]
        pub const $kw_name : CowStr = Cow::Borrowed($kw_val);)*
        $(#[allow(non_upper_case_globals, dead_code)]
        pub const $sym_name : CowStr = Cow::Borrowed($sym_val);)*

        /// Gets the default set of keywords for protosnirk
        pub fn default_keywords() -> HashSet<CowStr> {
            // Yo dawg, I heard you like macros
            hashset! [
                $(Cow::Borrowed($kw_val)),*
            ]
        }

        /// Gets the default set of symbols in protosnirk
        pub fn default_symbols() -> HashMap<CowStr, TokenizerSymbolRule> {
            hashmap! [
                $(
                    Cow::Borrowed($sym_val) => $sym_rule,
                )*
                $(
                    Cow::Borrowed($part_val) => $part_rule
                ),*
            ]
        }

        /// Which type of token this is.
        ///
        /// Used by the parsers to expect keywords or symbols
        /// in the token stream
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum TokenType {
            /// Token is an identifier
            Ident,
            /// Token is a literal
            Literal,
            $(
                $sym_name,
            )*
            $(
                $kw_name,
            )*
            $(
                $ty_name,
            )*
            BeginBlock,
            EndBlock,
            EOF,
        }

        impl Token {
            pub fn get_type(&self) -> TokenType {
                match *self.get_data() {
                    TokenData::NumberLiteral(_)
                    | TokenData::UnitLiteral
                    | TokenData::BoolLiteral(_) => TokenType::Literal,
                    TokenData::Ident => TokenType::Ident,
                    TokenData::BeginBlock => TokenType::BeginBlock,
                    TokenData::EndBlock => TokenType::EndBlock,
                    TokenData::EOF => TokenType::EOF,
                    TokenData::Keyword => {
                        match self.get_text() {
                            $(
                                $kw_val => TokenType::$kw_name,
                            )*
                            _ => unreachable!("Invalid token text for kw")
                        }
                    },
                    TokenData::Symbol => {
                        match self.get_text() {
                            $(
                                $sym_val => TokenType::$sym_name,
                            )*
                            _ => unreachable!("Invalid token text for symbol")
                        }
                    },
                    TokenData::TypeName => {
                        match self.get_text() {
                            $(
                                $ty_val => TokenType::$ty_name,
                            )*
                            _ => unreachable!("Invalid token text for tyname")
                        }
                    }
                }
            }
        }
    }
}

declare_tokens! {
    symbols {
        Plus: "+"; CompletePrefix,
        Minus: "-"; CompletePrefix,
        Star: "*"; CompletePrefix,
        Slash: "/"; CompletePrefix,
        Equals: "="; CompletePrefix,
        Percent: "%"; CompletePrefix,
        LeftAngle: "<"; CompletePrefix,
        RightAngle: ">"; CompletePrefix,

        DoubleEquals: "=="; Complete,
        NotEquals: "!="; Complete,
        PlusEquals: "+="; Complete,
        MinusEquals: "-="; Complete,
        StarEquals: "*="; Complete,
        SlashEquals: "/="; Complete,
        PercentEquals: "%="; Complete,
        LessThanEquals: "<="; Complete,
        GreaterThanEquals: ">="; Complete,

        LeftParen: "("; Complete,
        RightParen: ")"; Complete,
        GitMarker: "<<<<<<<"; Complete,
        InlineArrow: "=>"; Complete,
        Arrow: "->"; Complete,
        Comma: ","; Complete,
        Colon: ":"; Complete,
    }
    symparts {
        "//"; CompletePrefix, // Comments hack, allows // and /// to be parsed.
        "<<"; Partial,
        "<<<"; Partial,
        "<<<<"; Partial,
        "<<<<<"; Partial,
        "!"; Partial,
    }
    keywords {
        Let: "let",
        Mut: "mut",
        Return: "return",
        Do: "do",
        If: "if",
        Else: "else",
        Fn: "fn",
    }
    tynames {
        Int: "int",
        Bool: "bool",
    }
}
