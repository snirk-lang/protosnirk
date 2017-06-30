//! Type environment which handles unification.

/// This object is responsible for doing type inference:
/// it maintains a list of solved and unsolved type equations
/// and, given symbol IDs that have been obtained from the
/// symbol and item define AST passes, handles the logic of
/// typechecking functions.
#[derive(Debug)]
pub struct TypeEnvironment {
    known_types: HashMap<Id, Type>,
    equations: Vec<TypeEquation>,
    scope_map: HashMap<ScopedId, TypeSymbol>,
    curr_type_id: Id
}
