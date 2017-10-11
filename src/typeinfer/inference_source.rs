//! Source of type inferences.

use lex::Token;
use parse::ScopedId;
use parse::ast::{Identifier, Literal};
use parse::ast::types::TypeExpression;

/// Type inference source, used for compiler errors.
#[derive(Debug, PartialEq, Clone)]
#[warn(dead_code)]
pub enum InferenceSource {
    /// Inference source is the signature of a function.
    FnSignature(Identifier),
    /// Inference source is the return type of a function.
    FnReturnType(Identifier),
    /// Inference source is the parameter of a function.
    FnParameter(Identifier),
    /// Inference source is the call argument of a function.
    CallArgument(Identifier),
    /// Inference source is the return type of a call.
    CallReturnType(Identifier),
    /// Inference source is the declaration of a variable with a given type.
    ExplicitDecl(Identifier),
    /// Inference source is from the rvalue of a variable declaration.
    Declaration(Identifier),
    /// Inference source is a literal.
    LiteralValue(Literal),
    /// Inference source is the conditional of an if being a bool.
    IfConditionalBool(Token),
    /// Inference source is the if branches being the same.
    IfBranchesSame(Token),
    /// Inference source is a `return` matching the fn return type.
    ExplicitReturn(Token),
    /// Inference source is an implicit return matching a block.
    ImplicitReturn(ScopedId),
    /// Inference source is from a variable (re)assignment.
    Assignment,
    /// Inference source is a numeric operator matching a number.
    NumericOperator,
    /// Inference source is a boolean operator matching a bool.
    BooleanOperator,
    /// Inference source is two types being on the same side of an
    /// equality operator.
    EqualityOperator,
}
