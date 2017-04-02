use parse::ScopedId;

use check::{ASTVisitor, ErrorCollector};
use check::scope::scope_builder::ScopeBuilder;

#[derive(Debug, PartialEq, Clone)]
pub struct ItemChecker<'err, 'builder> {
    builder: &'builder mut ScopeBuilder,
    errors: &'err mut ErrorCollector,
    current_id: ScopedId
}

impl<'err, 'builder> ItemChecker<'err, 'builder> {
    pub fn new(errors: &'err mut ErrorCollector,
               builder: &'builder mut ScopeBuilder)
               -> ItemsChecker<'err, 'builder> {
        ItemsChecker {
            errors: errors,
            builder: builder,
            current_id: ScopedId::default()
        }
    }
}

impl<'err, 'builder> ASTVisitor for ItemChecker<'err, 'builder> {
    pub fn check_unit(&mut self, unit: &Unit) {
        // Each time check_unit() is called, ensure we have unique IDs.
        // Incrementing first also ensures we don't use ID [0, 0] - the
        // first ID we use is [1, 0] - so checking id[0] != 0 should be
        // an invariant we can maintain throughout the process.
        self.current_id.increment();
        self.current_id.push();
        for item in unit.get_items() {
            self.check_item(item);
        }
        self.current_id.pop();
    }

    pub fn check_item(&mut self, item: &Item) {

    }
}
