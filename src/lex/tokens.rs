//! Contains lists of the default tokens in protosnirk

use std::borrow::Cow;
use std::collections::{HashMap, HashSet};

use lex::{CowStr, TokenizerSymbolRule};
use lex::TokenizerSymbolRule::*;

macro_rules! declare_tokens {
    (
        symbols { $($sym_name:ident : $sym_val:expr; $sym_rule:expr),* }
        symparts { $($part_val:expr; $part_rule:expr),* }
        keywords { $($kw_name:ident : $kw_val:expr),* }
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

        PlusEquals: "+="; Complete,
        MinusEquals: "-="; Complete,
        StarEquals: "*="; Complete,
        SlashEquals: "/="; Complete,
        PercentEquals: "%="; Complete,

        LeftParen: "("; Complete,
        RightParen: ")"; Complete,
        LeftAngle: "<"; CompletePrefix,
        RightAngle: ">"; Complete,
        GitMarker: "<<<<<<<"; Complete,
        InlineArrow: "=>"; Complete
    }
    symparts {
        "//"; CompletePrefix, // Comments hack, allows // and /// to be parsed.
        "<<"; Partial,
        "<<<"; Partial,
        "<<<<"; Partial,
        "<<<<<"; Partial
    }
    keywords {
        Let: "let",
        Mut: "mut",
        Return: "return",
        Do: "do",
        If: "if"
    }
}
