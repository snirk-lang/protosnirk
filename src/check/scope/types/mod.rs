mod item_namer;
mod expr_namer;

pub use self::item_namer::ItemTypeIdentifier;
pub use self::expr_namer::ExpressionTypeIdentifier;

use parse::ScopedId;
use check::scope::NamedScopeBuilder;

/// So the checking of declaring types is gonna have to happen before the
/// items of a program in the future. We'll also have to consider things like
/// defintions from the standard library, etc.
/// For now, we'll just inject a collection of already-defined types plus the
/// last `ScopedId` needed
pub fn default_type_scope() -> NamedScopeBuilder {
    // Can't do this with consts or statics because Rust
    // and also the internals of these structures are hidden.
    let id = ScopedId::default();
    let mut builder = ScopeBuilder::new();
    let names = [
        "int", "float", "bool"
    ];
    for name in names {
        id.increment();
        builder.define_local(name.to_string(), id.clone());
    }
    builder
}
