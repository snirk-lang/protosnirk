//! Types during type inference process

use std::collections::HashMap;

use parse::ast::*;

use check::ASTVisitor;
use check::types::Type;
use check::ScopeIndex;

#[derive(PartialEq, Eq, Hash)]
struct TypeSymbol(u64);

macro_rules! gen_builtin_type_symbols {
    // Finally, produce the TypeSymbol that we should start with
    (@expand; $iota:expr;) => {
        /// The smallest "usable" TypeSymbol
        fn min() -> TypeSymbol { TypeSymbol($iota) }
    };
    // I'm not a fan of Go's `iota` but here it is in macro form
    // generate a unique number for each `TypeSymbol`
    (@expand; $iota:expr; $name:ident, $($rest:tt)*) => {
        #[inline]
        #[allow(non_snake_case)]
        pub fn $name() -> TypeSymbol { TypeSymbol($iota) }

        gen_builtin_type_symbols!(@expand; ($iota + 1u64); $($rest)*);
    };
}
macro_rules! get_builtin_type_symbols {
    ($name:ident, $($rest:tt)*) => {
        impl TypeSymbol {
            gen_builtin_type_symbols!(@expand; 1u64; $name, $($rest)*);
        }
    };
}

impl TypeSymbol {
    pub fn next(&self) -> TypeSymbol {
        // I assume you won't get more than U64 types.
        TypeSymbol(self.0 + 1)
    }
}

get_builtin_type_symbols! {
    UNIT,
    BOOL,
    FLOAT,
}

struct TypeEnvironment {
    types: HashMap<TypeSymbol, Type>,
    scope_map: HashMap<ScopeIndex, TypeSymbol>,
    curr_symbol: TypeSymbol,
}

impl TypeEnvironment {
    pub fn new() -> TypeEnvironment {
        TypeEnvironment {
            types: HashMap::new(),
            scope_map: HashMap::new(),
            curr_symbol: TypeSymbol::min()
        }
    }
    pub fn analyze(&mut self, unit: &Unit) {
        self.visit_unit(unit);
    }
}

struct Foo<'a> {
    bar: &'a mut String
}

impl ASTVisitor for TypeEnvironment {
    fn check_unit(unit: &Unit) {
        let mut text = "Hello world".into();
        let foo = Foo { bar: text };
        let text2 = foo.bar;
        let foo2 = Foo { bar: text};
    }
}

enum TypeAscriptionRule {
    
}
