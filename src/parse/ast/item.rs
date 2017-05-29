//! Item values
//!
//! An `Item` is a declaration made in the root context of a program
//! -- namely declarations such as `class`, `enum`, `struct`.

use lex::{Token};
use parse::ast::{Identifier, Block};
use parse::ast::types::TypeExpression;

/// A single "unit" of parsed code.
#[derive(Debug, Clone, PartialEq)]
pub struct Unit {
    items: Vec<Item>
}

/// Items exported from a protosnirk program
#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    /// Inline function declaration
    InlineFnDeclaration(InlineFnDeclaration),
    /// Block function declaration
    BlockFnDeclaration(BlockFnDeclaration)
}

impl Unit {
    /// Create a new unit with the given block
    pub fn new(items: Vec<Item>) -> Unit {
        Unit { items: items }
    }
    /// Gets the collection of exported items
    pub fn get_items(&self) -> &[Item] {
        &self.items
    }
}

/// Declared function argument
#[derive(Debug, Clone, PartialEq)]
pub struct FnParameter {
    ident: Identifier,
    declared_type: TypeExpression
    // default value, etc.
}
impl FnParameter {
    /// Creates a new `FnParameter` with the given identifier and declared type.
    pub fn new(ident: Identifier, declared_type: TypeExpression) -> FnParameter {
        FnParameter { ident, decared_type }
    }
    /// Gets the identifier of the parameter
    pub fn get_ident(&self) -> &Identifier {
        &self.ident
    }
    /// Gets the name of this parameter
    pub fn get_name(&self) -> &str {
        self.ident.get_name()
    }
    /// Gets the type of the parameter
    pub fn get_type(&self) -> &TypeExpression {
        &self.declared_type
    }
}

/// Inline fn declaration.
///
/// Functions can be declared inline using the inline arrow. Inline
/// fn declarations are only allowed one expression, but the return
/// type is inferred.
///
/// # Example
/// ```snirk
/// fn foo(arg: Type) => arg + 1
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct InlineFnDeclaration {
    fn_token: Token,
    ident: Identifier,
    params: Vec<FnParameter>,
    expr: Expression,
    type_id: Cell<Id>,
}
impl InlineFnDeclaration {
    /// Create a new `InlineFnDeclaration`
    pub fn new(fn_token: Token, ident: Identifier, params: Vec<FnParameter>, expr: Expression)
               -> BlockFnDeclaration {
        BlockFnDeclaration {
            fn_token, ident, params, expr,
            type_id: Cell::new(Id::default())
        }
    }
    /// Get the `fn` token
    pub fn get_token(&self) -> &Token {
        &self.fn_token
    }
    /// Get the identifier of the function
    pub fn get_ident(&self) -> &Identifier {
        &self.name
    }
    /// Get the textual name of the function
    pub fn get_name(&self) -> &str {
        &self.name.get_name()
    }
    /// Get the parameters of the function
    pub fn get_params(&self) -> &Vec<FnParameter> {
        &self.params
    }
    /// Get the expression of the function
    pub fn get_expr(&self) -> &Expression {
        &self.block
    }
    /// Get the typeid of this function.
    ///
    /// The typeid includes the parameter and return types.
    pub fn get_type_id(&self) -> Id {
        *self.type_id
    }

    /// Set the typeid of this function.
    pub fn set_type_id(&self, id: Id) {
        *self.type_id = id;
    }
}

/// Declaration of a function
#[derive(Debug, Clone, PartialEq)]
pub struct BlockFnDeclaration {
    fn_token: Token,
    name: Identifier,
    params: Vec<FnParameter>,
    return_type: TypeExpression,
    block: Block,
    type_id: Cell<Id>
}
impl BlockFnDeclaration {
    /// Create a new FnDeclaration
    pub fn new(fn_token: Token,
               ident: Identifier,
               params: Vec<FnParameter>,
               return_type: TypeExpression,
               block: Block)
               -> BlockFnDeclaration {
        BlockFnDeclaration {
            fn_token, name, params, return_type, block,
            type_id: Cell::new(Id::default())
        }
    }
    /// Get the `fn` token
    pub fn get_token(&self) -> &Token {
        &self.fn_token
    }
    /// Get the identifier of the function
    pub fn get_ident(&self) -> &Identifier {
        &self.name
    }
    /// Get the textual name of the function
    pub fn get_name(&self) -> &str {
        &self.name.get_name()
    }
    /// Get the parameters of the function
    pub fn get_params(&self) -> &Vec<FnParameter> {
        &self.params
    }
    /// Get the return type of the funciton
    pub fn get_return_type(&self) -> &TypeExpression {
        &self.return_type
    }
    /// Get the block inside the function
    pub fn get_block(&self) -> &Block {
        &self.block
    }

    /// Get the typeid of this function.
    pub fn get_type_id(&self) -> Id {
        *self.type_id
    }

    /// Set the typeid of this function.
    pub fn set_type_id(&self, id: Id) {
        *self.type_id = id;
    }
}
