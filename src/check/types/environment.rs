//! Type environment which handles unification.
use std::collections::HashMap;

use parse::ScopedId;
use check::types::*;

/// This object is responsible for doing type inference:
/// it maintains a list of solved and unsolved type constraints
/// and, given symbol IDs that have been obtained from the
/// symbol and item define AST passes, handles the logic of
/// typechecking functions.
#[derive(Debug, PartialEq, Clone)]
pub struct TypeEnvironment {
    /// Known type variables: the resulting `ScopedId` is correlated to the
    /// AST type definition for use in emitting.
    known_type_names: HashMap<TypeId, ScopedId>,
    /// `TypeId`s of given type identifiers.
    known_var_types: HashMap<ScopedId, TypeId>,
    /// Classical table of type defintions.
    ///
    /// For now, this is a lookup for fn signatures. In the future, this
    /// would have information for identifying fields of types, or
    type_defintions: HashMap<ScopedId, i32>,
    /// Bounds on types. Each is mapped to a top-level scope in which it may
    /// apply, i.e. the scope of its declared function.
    constraints: HashMap<ScopedId, (TypeConstraint, ConstraintSource)>,
    /// Current `TypeId` for creating new relations.
    curr_type_id: TypeId
}

impl TypeEnvironment {
    /// Creates a new `TypeEnvironment`, populating definitions for
    /// "standad library" types `float` and `bool`.
    pub fn new() -> TypeEnvironment {
        // This is copied from check/scope/types/mod.rs
        // The names have to be kept in the same order so that the
        // `ScopedId`s match.
        // This is where we inject knowledge of the standard library
        // into `check`.
        let mut scope_id = ScopedId::default();
        let mut type_id = TypeId::default();
        let mut known_type_defs = HashMap::new();
        let mut known_types = HashMap::new();
        for _ in 0..2 { // This has to match the number of
            scope_id.increment();
            type_id.increment();
            known_type_defs.insert(scope_id.clone(), type_id);
            known_types.insert(type_id, scope_id.clone());
        }
        // Unit type is weird right now. Basically, we're just gonna treat it
        // like C-likes treat `void` and make it an error to set a var to type
        // `()` (which by the way can't be parsed), or the result of a fun which
        // does not declare a return type. Inline fns which are a call to an
        // undeclared fn will also return ().
        debug_assert_eq!(known_type_defs.len() >= 2,
            "Expected to create TypeEnvironment with at least 2 std types");
        TypeEnvironment {
            known_type_names: known_types,
            known_var_types: known_type_defs,
            constraints: Vec::new(),
            curr_type_id: type_id
        }
    }

    /// Get a new fresh `TypeId` from the environment
    pub fn declare_new_type(&mut self) -> TypeId {
        self.curr_type_id.increment();
        self.curr_type_id
    }

    /// Declare a new `TypeId` for a given var identifier.
    pub fn declare_var_new_type(&mut self, var_id: ScopedId) -> TypeId {
        let type_id = self.declare_new_type();
        self.constraints.push(
            TypeConstraint::VarIdentKnownType(var_id, type_id)
        );
        type_id
    }

    /// Declare a new `TypeId` for a given type ident.
    pub fn declare_new_type_def(&mut self, type_ident: ScopedId) -> TypeId {
        let type_id = self.declare_new_type();
        self.known_type_defs.insert(type_id, type_id);
        type_id
    }

    /// Add a new `TypeConstraint` to the type environment.
    pub fn add_constraint(&mut self,
                          top_scope: ScopedId,
                          constraint: TypeConstraint,
                          source: ConstraintSource) {
        // TODO do some auto-replace/optimizing of the constraint, i.e.
        // check for ids in `known_defs`/`known_types`?
        self.constraints.insert(top_scope, (constraint, source));
    }

    /// Get the TypeId of a given variable, if known.
    pub fn get_type_id_of_var(&self, var_id: &ScopedId) -> Option<TypeId> {
        self.known_var_types.get(var_id)
    }

    /// Get a
    pub fn get_constraint_by_scope(&self, scope_id: &ScopedId)
                                   -> Option<TypeConstraint> {
        self.constraints.get(scope_id)
    }
}

/// Represents a unique type.
#[derive(PartialEq, Eq, Clone, Copy, Hash, Default)]
pub struct TypeId(u64);

impl TypeId {
    /// Produce the next TypeId.
    #[inline]
    pub fn next(&self) -> TypeId {
        // I assume you won't get more than U64 types.
        TypeId(self.0 + 1)
    }
    /// Increment this TypeId.
    #[inline]
    pub fn increment(&mut self) -> TypeId {
        self.0 += 1;
    }
    /// Whether this TypeId has been assigned yet
    #[inline]
    pub fn is_default(&self) -> bool {
        self.0 == 0
    }
}

/// A constraint that was found about types in a program.
#[derive(Debug, PartialEq, Clone)]
pub enum TypeConstraint {
    /// Two `TypeId`s must be the same type.
    TypesAreSame(TypeId, TypeId),
    /// A variable `ScopedId` has a known type.
    VarIdentKnownType(ScopedId, TypeId),
    // /// Two identifiers have the same type.
    // IdentsSameType(ScopedId, ScopedId),
    /// The identifier corresponds to a declared fn with named params.
    /// This would need a layer of indirection `(TypeId, Vec<ScopedId>)`
    /// once the AST starts working with first-class fns.
    DeclaredFn(ScopedId, Vec<ScopedId>),
    /// The return type of an identifier is known.
    FnReturnType(ScopedId, TypeId),
    /// The return type of an identified fn is unit.
    FnReturnsUnit(ScopedId),
    /// The given block may have the given type.
    BlockHasType(ScopedId, TypeId),
    /// The type of the expression is constrained by being the single argument
    /// in a fn call.
    ValueForSingleFnArg(ScopedId, TypeId)
}

/// Source of a type constraint.
#[derive(Debug, PartialEq, Clone, Hash)]
pub enum ConstraintSource {
    /// The constraint source is unknown :/
    Unknown,
    /// This type constraint was a result of a fn signature
    FnSignature,
    /// This type constraint is known because a variable is declared as the
    /// parameter of a funcion.
    ParamDecl,
    /// A variable was declared with an explicit type.
    ExplicitVarDecl,
    /// A variable was assigned to a value of a specific type.
    VarAssignment,
    /// The type of a value was inferred because of a fn call.
    CalledFnReturnType,
    /// The type of a value was inferred because it is the conditional
    /// of an `if` block or expression.
    IfConditionalBool,
    /// The type of a value was inferred because it must match another
    /// branch of an `if` statment or expression.
    IfBranchesSame,
    /// The type of a value was inferred because it was used in a `return`
    /// statment and must match the signature of its declaring fn.
    ExplicitReturnMatch,
    /// The type of a value was inferred becuase it was used in an implicit
    /// return exoression and must match the signature of its declaring fn.
    ImplicitReturnMatch,
    /// The type of a value needs to be a number in order to be used in a
    /// numeric operation.
    NumericOperator,
    /// The type of a value needs to be `bool` in order to be used in a
    /// boolean operation.
    BooleanOperator,
    /// An equality or non-equality operator needs the operatees to be the same
    /// type.
    EqualityOperator,
}
