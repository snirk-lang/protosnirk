mod item_namer;
mod expr_namer;

pub use self::item_namer::ItemTypeIdentifier;
pub use self::expr_namer::ExpressionTypeIdentifier;

use parse::ScopedId;
use identify::NameScopeBuilder;
