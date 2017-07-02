use parse::ScopedId;
use parse::ast::*;

use check::{ASTVisitor, ErrorCollector};
use check::scope::scope_builder::ScopeBuilder;

/// Does the first pass of scope checking to ensure
/// items can be used before being declared.
#[derive(Debug, PartialEq, Clone)]
pub struct ItemChecker<'err, 'builder> {
    builder: &'builder mut ScopeBuilder,
    errors: &'err mut ErrorCollector,
    current_id: ScopedId
}

impl<'err, 'builder> ItemChecker<'err, 'builder> {
    pub fn new(errors: &'err mut ErrorCollector,
               builder: &'builder mut ScopeBuilder)
               -> ItemChecker<'err, 'builder> {
        ItemChecker {
            errors: errors,
            builder: builder,
            current_id: ScopedId::default()
        }
    }
}

impl<'err, 'builder> ASTVisitor for ItemChecker<'err, 'builder> {
    fn check_unit(&mut self, unit: &Unit) {
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

    fn check_item(&mut self, item: &Item) {
        ASTVisitor::check_item(&mut self, item);
        self.current_id.increment();
    }

    fn check_fn_declaration(&mut self, fn_decl: &mut FnDeclaration) {
        if let Some(prev_decl_ix) = self.builder.get(fn_decl.get_name()) {
            // fn was previously declared
            // add multiple def error, continue on
            let declared_at = self.symbol_table[&declared_index].get_declaration();
        }
        // Get an ID for the function - should start at [1, 0]
        let fn_id = self.current_id.clone();

        // Define the function in the builder and set the fn's ident.
        self.builder.define_global(fn_decl.get_name().clone(), fn_id.clone());
        fn_decl.get_ident().set_id(fn_id);
    }
}
