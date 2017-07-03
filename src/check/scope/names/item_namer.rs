use parse::ast::*;
use parse::ScopedId;

use check::{CheckerError, ErrorCollector};
use check::scope::NameScopeBuilder;
use check::visitor::*;

/// Identifies names of items that can be used in expressions,
/// namely function definitions.
pub struct ItemVarIdentifier<'err, 'builder, 'id> {
    errors: &'err mut ErrorCollector,
    builder: &'builder mut NameScopeBuilder,
    current_id: ScopedId
}

impl<'err, 'builder> ItemVarIdentifier<'err, 'builder> {
    pub fn new(errors: &'err mut ErrorCollector,
               builder: &'builder mut NameScopeBuilder,
               scoped_id: ScopedId)
               -> ItemVarIdentifier<'err, 'builder> {
        ItemVarIdentifier {
            errors,
            builder,
            current_id: scoped_id
        }
    }

    pub fn into_last_id(self) -> ScopedId {
        self.current_id
    }
}

impl<'err, 'builder> DefaultUnitVisitor
    for ItemVarIdentifier<'err, 'builder> { }

impl<'err, 'builder> ItemVisitor for ItemVarIdentifier<'err, 'builder> {
    fn visit_inline_fn_decl(&mut self, inline_fn: &InlineFnDeclaration) {
        debug_assert!(inline_fn.get_ident().get_id().is_default(),
            "Inline fn {:?} already had an ID", inline_fn);
        if let Some(previous_def) = self.builder.get(inline_fn.get_name()) {
            // fn has been previously defined
        }
        else {
            self.current_id.increment();
            let fn_id = self.current_id.clone();
            trace!("Created id {:?} for inline fn {}",
                fn_id, inline_fn.get_name());
            self.builder.define_local(inline_fn.get_name().to_string(),
                                      fn_id.clone());
            inline_fn.get_ident().set_id(fn_id);
        }
    }

    fn visit_block_fn_decl(&mut self, block_fn: &BlockFnDeclaration) {
        debug_assert!(block_fn.get_ident().get_id().is_default(),
            "Block fn {:?} already had an ID", block_fn);
        if let Some(previous_def) = self.builder.get(block_fn.get_name()) {
            // fn has been previously defined
        }
        else {
            self.current_id.increment();
            let fn_id = self.current_id.clone();
            trace!("Created id {:?} for block fn {}",
                fn_id, block_fn.get_name());
            self.bilder.define_local(block_fn.get_name().to_string(),
                                     fn_id.clone());
            block_fn.get_ident().set_id(fn_id);
        }
    }
}
