//! Source of type inferences.

use lex::Token;
use ast::{Identifier, Literal, ScopedId};
use ast::types::TypeExpression;

use std::fmt::{self, Formatter};

/// Type inference source, used for compiler errors.
#[derive(PartialEq, Clone)]
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
    IfConditionalBool,
    /// Inference source is the if branches being the same.
    IfBranchesSame,
    /// Inference source is a `return` matching the fn return type.
    ExplicitReturn,
    /// Inference source is an implicit return matching a block.
    ImplicitReturn,
    /// Inference source is from a variable (re)assignment.
    Assignment,
    /// Inference source is a numeric operator matching a number.
    NumericOperator,
    /// Inference source is a boolean operator matching a bool.
    BooleanOperator,
    /// Inference source is two types being on the same side of an
    /// equality operator.
    EqualityOperator,
    /// Value is inferred to be of a given type based upon other connections.
    Inferred,
}

impl fmt::Debug for InferenceSource {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use self::InferenceSource::*;
        match *self {
            FnSignature(ref id) => f.debug_tuple("IsTheFn")
                                 .field(&id.get_name())
                                 .finish(),
            FnReturnType(ref id) => f.debug_tuple("FnReturn")
                                  .field(&id.get_name())
                                  .finish(),
            FnParameter(ref id) => f.debug_tuple("FnParam")
                                 .field(&id.get_name())
                                 .finish(),
            CallArgument(ref id) => f.debug_tuple("CallArg")
                                  .field(&id.get_name())
                                  .finish(),
            CallReturnType(ref id) => f.debug_tuple("CallReturn")
                                    .field(&id.get_name())
                                    .finish(),
            ExplicitDecl(ref id) => f.debug_tuple("ExplicitLet")
                                  .field(&id.get_name())
                                  .finish(),
            Declaration(ref id) => f.debug_tuple("Let")
                                 .field(&id.get_name())
                                 .finish(),
            LiteralValue(ref lit) => f.debug_tuple("Literal")
                                   .field(&lit.get_value())
                                   .finish(),
            IfConditionalBool => f.write_str("IfCond"),
            IfBranchesSame => f.write_str("IfBranchEq"),
            ExplicitReturn => f.write_str("ReturnStmt"),
            ImplicitReturn => f.write_str("ReturnExpr"),
            Assignment => f.write_str("Assign"),
            NumericOperator => f.write_str("NumOp"),
            BooleanOperator => f.write_str("BoolOp"),
            EqualityOperator => f.write_str("EqualOp"),
            Inferred => f.write_str("Infer")
         }
    }
}
