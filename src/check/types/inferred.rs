//! Types during type inference process

use std::collections::HashMap;

use parse::ast::*;
use parse::ScopedId;

use check::ASTVisitor;
use check::types::Type;

/// Represents a unique type.
#[derive(PartialEq, Eq, Hash, Default)]
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
    /// Produce the next TypeSymbol.
    #[inline]
    pub fn next(&self) -> TypeSymbol {
        // I assume you won't get more than U64 types.
        TypeSymbol(self.0 + 1)
    }
    /// Increment this TypeSymbol.
    #[inline]
    pub fn increment(&mut self) -> TypeSymbol {
        self.0 += 1;
    }
    /// Whether this TypeSymbol has been assigned yet
    #[inline]
    pub fn is_known(&self) -> bool {
        self.0 == 0
    }
}

// The builtin types are given symbols starting from 1.
get_builtin_type_symbols! {
    UNIT,
    BOOL,
    FLOAT,
}

struct TypeEnvironment {
    types: HashMap<TypeSymbol, Type>,
    scope_map: HashMap<ScopedId, TypeSymbol>,
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
        self.check_unit(unit);
    }
}

impl ASTVisitor for TypeEnvironment {
    //fn check_unit(unit: &Unit) {
    //}

    pub fn check_fn_declaration(&mut self, fn_decl: &FnDeclaration) {
        
    }
}

enum TypeAscriptionRule {
    DeclaredParameter,
    DeclaredLetExplicit,
    MatchNamedArgCalled,
}
