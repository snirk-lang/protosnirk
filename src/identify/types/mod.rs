mod type_identifier;
use self::type_identifier::TypeIdentifier;

mod item_namer;
mod expr_namer;


pub use self::item_namer::ItemTypeIdentifier;
pub use self::expr_namer::ExprTypeIdentifier;
