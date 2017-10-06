mod item_namer;
mod expr_namer;

pub use self::item_namer::ItemTypeIdentifier;
pub use self::expr_namer::ExpressionTypeIdentifier;

use parse::ScopedId;
use identify::NameScopeBuilder;

/// So the checking of declaring types is gonna have to happen before the
/// items of a program in the future. We'll also have to consider things like
/// defintions from the standard library, etc.
///
/// We're not including the last default `ScopedId` for typedefs because we're
/// assuming no new types (struct, typedef, etc.) will be defined.
pub fn default_type_scope() -> NameScopeBuilder {
    // Can't do this with consts or statics because Rust
    // and also the internals of these structures are hidden.
    let mut id = ScopedId::default();
    let mut builder = NameScopeBuilder::new();
    let names = [
        "float",
        "bool"
    ];
    for name in names {
        id.increment();
        builder.define_local(name.to_string(), id.clone());
    }
    builder
}
