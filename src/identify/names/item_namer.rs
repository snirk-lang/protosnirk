//! AST visitor which assigns the ScopedIds of types on items.

use ast::{*, visit::*};
use check::{CheckerError, ErrorCollector};
use identify::NameScopeBuilder;

/// Identifies names of items that can be used in expressions,
/// namely function definitions.
pub struct ItemVarIdentifier<'err, 'builder> {
    errors: &'err mut ErrorCollector,
    builder: &'builder mut NameScopeBuilder,
    current_id: ScopedId
}

impl<'err, 'builder> ItemVarIdentifier<'err, 'builder> {
    pub fn new(errors: &'err mut ErrorCollector,
               builder: &'builder mut NameScopeBuilder,
               current_id: ScopedId)
               -> ItemVarIdentifier<'err, 'builder> {
        ItemVarIdentifier {
            errors,
            builder,
            current_id
        }
    }

    pub fn into_last_id(self) -> ScopedId {
        self.current_id
    }
}

impl<'err, 'builder> UnitVisitor for ItemVarIdentifier<'err, 'builder> {
    fn visit_unit(&mut self, unit: &Unit) {
        trace!("Visting a unit");
        // items are defined on the top level of the ScopedId.
        // We're passed in a ScopedId which is assumed to be non-default
        // so that the first item doesn't get a default scopedId
        self.builder.new_scope();

        visit::walk_unit(self, unit);

        self.current_id.increment();
    }
}

impl<'err, 'builder> ItemVisitor for ItemVarIdentifier<'err, 'builder> {
    fn visit_block_fn_decl(&mut self, block_fn: &BlockFnDeclaration) {
        trace!("Visiting fn definition {}", block_fn.get_name());
        if let Some(_previous_def_id) = self.builder.get(block_fn.get_name()) {
            // fn has been previously defined
            debug!("Emitting error: {} already declared", block_fn.get_name());
            self.errors.add_error(CheckerError::new(
                block_fn.get_token().clone(),
                vec![],
                format!("Function {} is already declared", block_fn.get_name())
            ));
            return
        }
        // If it was not in the builder its ID should be default.
        debug_assert!(block_fn.get_ident().get_id().is_default(),
            "Block fn {:?} already had an ID", block_fn);

        let fn_id = self.current_id.clone();
        trace!("Created id {:?} for block fn {}", fn_id, block_fn.get_name());
        self.builder.define_local(block_fn.get_name().to_string(),
                                  fn_id.clone());
        block_fn.set_id(fn_id);

        // Also name the params, in a new scope.
        self.current_id.push();
        self.builder.new_scope();

        // One of the consequences of setting params here is that we know the
        // parameter IDs of all items in scope. We could, for example, add these
        // to the global scope

        // https://github.com/immington-industries/protosnirk/issues/50

        for &(ref param, ref _param_type) in block_fn.get_params() {
            let param_name = param.get_name();
            if let Some(_previous_def_id) = self.builder.get(param_name) {
                debug!("Emitting error: {} in {} already declared",
                    param_name, block_fn.get_name());
                let error_text = format!(
                    "Parameter {} of function {} is already declared",
                    param.get_name(), block_fn.get_name());
                self.errors.add_error(CheckerError::new(
                    block_fn.get_token().clone(), vec![], error_text
                ));
                return // Stop checking params if there's a dupe.
            }

            trace!("Created id {:?} for {} param {}",
                self.current_id, block_fn.get_name(), param.get_name());
            self.builder.define_local(param_name.to_string(),
                                      self.current_id.clone());
            // We also put the param in the global scope as this is the only
            // scope visible outside the visitor.
            self.builder.define_global(
                    format!("{}::{}", block_fn.get_name(), param_name),
                    self.current_id.clone());
            param.set_id(self.current_id.clone());

            self.current_id.increment();
        }

        self.builder.pop();
        self.current_id.pop();
        self.current_id.increment();
    }
}
