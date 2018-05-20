//! Definition of data types in a compiled protosnirk program.

mod identify;

mod inference_source;
mod type_checker;

pub use self::inference_source::InferenceSource;
pub use self::identify::ASTTypeIdentifier;
