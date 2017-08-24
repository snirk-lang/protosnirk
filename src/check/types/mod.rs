//! Definition of data types in a compiled protosnirk program.

mod types;
mod environment;

mod expression_collector;
mod item_collector;
mod type_checker;

pub use self::type_checker::TypeChecker;
pub use self::environment::*;
pub use self::types::*; // We're defining parts of the post-AST IR glue here.

use parse::ScopedId;

thread_local! {
    static TYPE_IDENT_INT: ScopedId
        = ScopedId::default().incremented();
    static TYPE_IDENT_BOOL: ScopedId
        = ScopedId::default().incremented().incremented();

    static TYPE_ID_INT: TypeId
        = TypeId::default().next();
    static TYPE_ID_BOOL: TypeId
        = TypeId::default().next().next();
}
