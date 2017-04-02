mod scope_builder;
mod item_checker;
mod expression_checker;

use self::scope_builder::ScopeBuilder;
use self::item_checker::ItemChecker;
use self::expression_checker::ExpressionChecker;

use parse::ast::Unit;
use check::ErrorCollector;

#[derive(Debug, PartialEq, Eq)]
pub struct ScopeChecker<'err> {
    errors: &'err mut ErrorCollector
}

impl<'err> ScopeChecker<'err> {
    pub fn new(errors: &'err mut ErrorCollector) -> ScopeChecker {
        ScopeChecker {errors: errors }
    }

    /// Checks the scoping rules for a unit
    pub fn check_unit(&mut self, unit: &Unit) {
        let mut builder = ScopeBuilder::new();
        {
            let items_checker = ItemChecker::new(&mut self.errors, &mut builder);
            items_checker.check_unit(unit);
        }
        {
            let expression_checker = ExpressionChecker::new(&mut self.errors, &mut builder);
            expression_checker.check_unit(unit);
        }
        // we're done, the invariant is that all of the `ScopedId`s are set if there are no
        // errors.
    }
}
