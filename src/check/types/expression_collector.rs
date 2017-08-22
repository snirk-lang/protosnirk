use parse::ScopedId;
use parse::ast::*;
use parse::ast::types::*;

use check::ErrorCollector;
use check::types::environment::{TypeEnvironment,
                                TypeConstraint,
                                ConstraintSource};
use check::visitor::*;

/// Collects type equations in expressions
///
/// We can consider the `ItemTypeCollector` to have already visited the AST,
/// so the invariants described in its struct docs are assumed.
///
/// # Invariants
/// After the expression collector has visited an expression it will hopefully
/// have enough type information to pass checking. We ask the environment to
/// resolve rules one function at a time by unifying rules under the function's
/// top-level `ScopedId`.
#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionTypeCollector<'err, 'env> {
    errors: &'err mut ErrorCollector,
    environment: &'env mut TypeEnvironment,
    /// TypeId of expression rvalues
    current_id: TypeId,
    /// Keep a stack of the IDs of which block we're in.
    /// The top of the stack can be used for the return statement of a block.
    block_type_id_stack: Vec<TypeId>,
    /// ScopeId of the enclosing block or fn
    current_scope: ScopedId,
}

impl<'err, 'env> ExpressionTypeCollector<'err, 'env> {
    pub fn new(errors: &'err mut ErrorCollector,
               environment: &'env mut TypeEnvironment)
               -> ExpressionTypeCollector<'err, 'env> {
        ExpressionTypeCollector {
            errors,
            environment,
            current_id: TypeId::default(),
            expr_match_id: TypeId::default(),
            block_type_id_stack: Vec::new(),
            current_scope: ScopedId::default(),
        }
    }

    fn add_constraint(&mut self,
                      constraint: TypeConstraint,
                      source: ConstraintSource) {
        self.environment.add_constraint(self.current_scope, constraint, source);
    }
}

impl<'err, 'env> DefaultUnitVisitor for ExpressionTypeCollector<'err, 'env> { }

impl<'err, 'env> ItemVisitor for ExpressionTypeCollector<'err, 'env> {
    fn visit_inline_fn_decl(&mut self, inline_fn: &InlineFnDeclaration) {
        let top_scope = inline_fn.get_ident().get_id();
        if top_scope.is_default() {
            return
        }

        self.current_scope = top_scope.clone();
        self.block_type_id_stack.clear();
        self.block_type_id_stack.push(top_scope.clone());

        self.visit_expression(inline_fn.get_expression());
    }
    fn visit_block_fn_decl(&mut self, block_fn: &FnDeclaration) {
        let top_scope = block_fn.get_ident().get_id();
        if top_scope.is_default() {
            return
        }

        self.current_scope = top_scope.clone();
        self.block_type_id_stack.clear();
        self.block_type_id_stack.push(top_scope.clone());

        visit::walk_block(block_fn.get_block());
    }
}

impl<'err, 'env> BlockVisitor for ExpressionTypeCollector<'err, 'env> {
    fn visit_block(&mut self, block: &Block) {
        // declare block return type???

        // This is entirely possible if using a `// pass` comment.
        if block.statements().len() == 0 { return }

        // Correlate the last expr type with the block ID.
        // This information only matters if the block is being used as a value.
        let last_ix = block.statements.len() - 1usize;
        let last_stmt_id: TypeId = TypeId::default();
        for (ix, stmt) in block.statements().iter().enumerate() {
            self.visit_statement(stmt);
            if ix == last_ix {
                let last_stmt_ty_id = self.current_id;
                // Could still get the defaut `TypeId` if the last
                // stmt is an assignment or possibly if it's an unvalued block.
                if !last_stmt_ty_id.is_default() {
                    self.environment.add_constraint(
                        block.get_scope_id().clone(),
                        TypeConstraint::BlockHasType(block.get_scope_id.clone())
                    )
                }
            }
        }
    }
}

impl<'err, 'env> StatementVisitor for ExpressionTypeCollector<'err, 'env> {
    fn visit_do_block(&mut self, do_block: &DoBlock) {
        // No special handling here, it's just a regular block.
        visit::walk_do_block(self, do_block);
    }

    fn visit_if_block(&mut self, if_block: &IfBlock) {
        // conditional must be bool
        // branches must match up
    }

    fn visit_return_stmt(&mut self, return_: &Return) {
        // expr matches block's return
    }
}

impl<'err, 'env> ExpressionVisitor for ExpressionTypeCollector<'err, 'env> {
    fn visit_var_ref(&mut self, ident: &Identifier) {
        // Keep track of referred-to ident?
    }

    fn visit_if_expr(&mut self, if_expr: &IfExpression) {
        // condition must be boolean, exprs must match
    }

    fn visit_unary_op(&mut self, un_op: &UnaryOperation) {
        // expr must be numeric
    }

    fn visit_binary_op(&mut self, bin_up: &BinaryOperation) {
        // exprs must match (no coersion here)
    }

    fn visit_fn_call(&mut self, fn_call: &FnCall) {
        // exprs must match fn type.
    }

    fn visit_assignment(&mut self, assignment: &Assignment) {
        // var must match assignment type
    }

    fn visit_declaration(&mut self, decl: &Declaration) {
        // var must match decl type.
        // decl must match var declared type.
        let var_scope_id = decl.get_ident().get_id();
        if let Some(type_decl) = decl.get_type_decl() {
            // also restrict var_scope_id to the type decl.
        }
        self.visit_expression(decl.get_value());
        let expr_type_id = self.current_id;
        debug_assert!(!expr_type_id.is_default(),
            "No type ID from visiting an expression");

    }
}
