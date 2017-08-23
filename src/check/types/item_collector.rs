use parse::ScopedId;
use parse::ast::*;
use parse::ast::types::*;

use check::{ErrorCollector};
use check::types::environment::{TypeEnvironment,
                                TypeConstraint,
                                ConstraintSource};
use check::visitor::*;

/// Collects type information from items.
///
/// # Invariants
/// In order for type inference to work, two invariants are upheld:
///
/// 1. Declared types (in this case `bool` and `float`) are mapped in the
/// environment. This means the `ScopedId`s of their identifiers are mapped
/// to `TypeId`s, as well as being assembled into known concrete types. It is
/// required, then, that the shape or at least name of types be understood
/// before checking the parameters of fns.
///
/// 2. Function signatures of block functions are fully understood.
/// This means the `ScopedId`s of the fn identifiers are known to have a
/// certain return type when called, and the `ScopedId` of each parameter is
/// known to have a specific `ScopedId`.
///
/// # Inline Fns
/// Inline functions (with no declared return type) are an obvious hole in type
/// inference. Determining their return type (i.e. in an item pass) requires
/// understanding an expression. Fortunately, this is simpler than understanding
/// a _block_ of code, although for now this is not done in the first pass now.
///
/// There are plenty of languages that do type inference and also have versions
/// of inline fns which cannot be fully typechecked in an items pass, such as
/// Kotlin (or Scala). However, inline fns may complicate type inference.
#[derive(Debug, PartialEq, Clone)]
pub struct ItemTypeCollector<'err, 'env> {
    errors: &'err mut ErrorCollector,
    environment: &'env mut TypeEnvironment,
    // `TypeId` obtained by visiting a type.
    current_id: TypeId,
    // ScopedId of the current item that is being type collected.
    current_scope: ScopedId
}
impl<'err, 'env> ItemTypeCollector<'err, 'env> {
    pub fn new(errors: &'err mut ErrorCollector,
               environment: &'env mut TypeEnvironment)
               -> ItemTypeCollector<'err, 'env> {
        ItemTypeCollector {
            errors,
            environment,
            current_id: TypeId::default()
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum CollectorState {
    /// Default collector state.
    Default,
    /// Just created a `TypeConstraint` with the given `TypeId`.
    VisitedType(TypeId),
    /// Just visited an `InlineFnTypeExpression`, need to create a
    /// constraint with the given
    VisitedInlineSignature(Vec<(ScopedId, TypeId)>),
    VisitedFnSignature(Vec<(ScopedId, TypeId)>)

}

impl<'err, 'env> DefaultUnitVisitor for ItemTypeCollector<'err, 'env> { }

impl<'err, 'env> ItemVisitor for ItemTypeCollector<'err, 'env> {
    fn visit_block_fn_decl(&mut self, block_fn: &BlockFnDeclaration) {
        // block fn:
        // fn foo(x: Type, y: Type) -> RetType
        //     block
        // - constrain args: ident(`x`) -> TypeId
        // - constrain `foo` to declared: DeclaredFn(args)
        // - constrain `foo` to return its type.
        let fn_scope_id = block_fn.get_ident().get_id();
        self.current_scope = fn_scope_id.clone();
        self.environment.add_constraint(
            fn_scope_id.clone(),
            TypeConstraint::DeclaredFn(
                fn_scope_id,
                block_fn.get_params()
                    .map(|(ident, _)| ident.get_id().clone())
                    .collect()
            ),
            ConstraintSource::FnSignature
        );
        // Declare params to have known types.
        let fn_type = block_fn.get_type_expr();
        for (param_ident, param_type) in fn_type.get_params() {
            let param_id = param_ident.get_id();
            self.visit_type_expression(param_type);
            let param_type = self.current_id;
            self.environment.add_constraint(
                TypeConstraint::VarIdentKnownType(param_id, param_type),
                ConstraintSource::ParamDecl
            );
        }
        // Declare return type of function.
        self.visit_type_expression(fn_type.get_return_type());
        let return_type = self.current_id;
        self.environment.add_constraint(
            TypeConstraint::FnReturnType(fn_scope_id, return_type)
        );
    }
}

impl<'err, 'env> TypeVisitor for ItemTypeCollector<'err, 'env> {
    fn visit_named_type_expr(&mut self, named_ty: &NamedTypeExpression) {
        // This is basically the only `leaf` type.
        // We're just looking at an identified type and adding a
        // `scoped = type` bound.
        let scoped_id = named_ty.get_ident().get_id();
        // If the type ident isn't scoped, it was redundantly declared.
        // For now we use `TypeId::default` and assume the errors will be
        // revealed and the invalid state propagation doesn't get too much
        // worse.
        // This is the main reason for having more semantic error objects;
        // to allow different compiler stages to better work around errors.
        if scoped_id.is_default() {
            self.found_id = TypeId::default();
            return
        }
        if let Some(type_id) = self.environment.get_type_def(scoped_id) {
            // we have a `TypeId` corresponding to this type.
            // save that so we can associate it with a variable.
            self.found_id = type_id;
        }
        else {
            // We _should_ have ecountered the type earlier.
            // (in this case it needs to be injected in `TypeEnvironment::new`)
            // scoped_id is probably also default at this point, as this has
            // probably been reported at the scope phase.

        }
    }

    fn visit_fn_type_expr(&mut self, fn_ty: &FnTypeExpression) {
        // This is where first class fns would go if the AST supported them.
        unreachable!("ItemTypeCollector cannot visit FnTypeExpressions");
    }
}
