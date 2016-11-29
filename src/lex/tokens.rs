//! Contains lists of the default tokens in protosnirk

use std::borrow::Cow;
use std::collections::HashSet;

macro_rules! declare_tokens {
    (
        symbols { $($sym_name:ident : $sym_val:expr),* }
        keywords { $($kw_name:ident : $kw_val:expr),* }
    ) => {
        $(#[allow(non_upper_case_globals, dead_code)]
        pub const $kw_name : Cow<'static, str> = Cow::Borrowed($kw_val);)*
        $(#[allow(non_upper_case_globals, dead_code)]
        pub const $sym_name : Cow<'static, str> = Cow::Borrowed($sym_val);)*


        /// Gets the default set of keywords for protosnirk
        pub fn default_keywords() -> HashSet<Cow<'static, str>> {
            // Yo dawg, I heard you like macros
            hashset! [
                $(Cow::Borrowed($kw_val)),*
            ]
        }
        /// Gets the default set of symbols in protosnirk
        pub fn default_symbols() -> HashSet<Cow<'static, str>> {
            hashset! [
                $(Cow::Borrowed($sym_val)),*
            ]
        }
    }
}

declare_tokens! {
    symbols {
        Plus: "+",
        Minus: "-",
        Star: "*",
        Slash: "/",
        Equals: "=",
        Percent: "%",

        PlusEquals: "+=",
        MinusEquals: "-=",
        StarEquals: "*=",
        PercentEquals: "%=",

        LeftParen: "(",
        RightParen: ")",
        LeftBrace: "[",
        RighBrace: "]",
        LeftSquiggle: "{",
        RightSquiggle: "}",
        LeftAngle: "<",
        RightAngle: ">",
        GitMarker: "<<<<<<<"
    }
    keywords {
        Let: "let",
        Mut: "mut",
        Return: "return"
    }

}
