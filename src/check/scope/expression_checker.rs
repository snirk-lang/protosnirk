use parse::ScopedId;
use parse::ast::*;

use check::{ASTVisitor, ErrorCollector};
use check::scope::scope_builder::ScopeBuilder;

/// Establishes variable scopes.
///
/// The `ExpressionChecker` is the first verify pass.
/// Having been given only the parsed AST, its job
/// is to give meaningful unique IDs to each `Ident`
/// token - whether variable reference, function call
/// reference, type name, etc.
///
/// This is no small task - a C compiler can't do this
/// because of C's famous type/name ambiguity. Using this design
/// could also run into the same problem, even for a less
/// ambiguous language - I don't think this would work as
/// well in Rust.
///
/// Right now, we're just checking variables - types are
/// established in a later pass.
#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionChecker<'err> {
    /// Current unique ID to give to the next variable.
    current_id: ScopedId,
    /// Build up the map of all name declarations.
    name_map: ScopeBuilder,
    /// Mutably borrow an ErrorCollector to push to while we're running.
    errors: &'err mut ErrorCollector
}

impl<'err> ExpressionChecker<'err> {
    pub fn new() -> ExpressionChecker<'err> {
        // We start with `default() + 1` so that default
        // may be used as an empty value. We do rely on
        // iterating the AST in the same order as the
        // `ItemChecker`, though.
        let default_id = ScopedId::default().incremented();
    }
}

impl<'err> ASTVisitor for ExpressionChecker<'err> {
    fn check_unit(&mut self, unit: &Unit) {
        // Checking a unit involves first passing over each item,
        // to add the global functions/vars to the scope map,
        // then going back and
        self.current_id.increment();
        self.current_id.push();
        for item in unit.get_items() {
            self.check_item(item);
        }
        self.current_id.pop();
    }

    
}
