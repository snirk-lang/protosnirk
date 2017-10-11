//! Definition of data types in a compiled protosnirk program.

mod identify;

mod concrete_type;
mod inferred_type;
mod inference_source;
mod type_checker;

use self::inferred_type::{InferredType, TypeEquation};

pub use self::inference_source::InferenceSource;
pub use self::concrete_type::*;
pub use self::identify::ASTTypeIdentifier;
