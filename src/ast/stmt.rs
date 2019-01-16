/// Statement values
///
/// Function bodies are usually made up of statements. They include complex blocks
/// such as loop constructs. They are usually not accepted in as many places as
/// `Expression`s are because of their ability to use indentation.

use lex::{Span, Location};
use ast::{Expression, Identifier, TypeExpression, Block, ScopedId};

use std::cell::{RefCell, Ref};

/// Statement representation
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Expression(Expression),
    Return(Return),
    Declaration(Declaration),
    DoBlock(DoBlock),
    IfBlock(IfBlock)
    // match, loop, while, for
}
impl Statement {
    pub fn has_value(&self) -> bool {
        match *self {
            Statement::Expression(ref inner) => inner.has_value(),
            Statement::Return(ref return_) => return_.has_value(),
            Statement::DoBlock(ref do_block) => do_block.has_source(),
            Statement::IfBlock(ref if_block) => if_block.has_source(),
            Statement::Declaration(_) => false
        }
    }

    pub fn span(&self) -> Span {
        match *self {
            thing => thing.span()
        }
    }
}

/// Explicit return statement
#[derive(Debug, PartialEq, Clone)]
pub struct Return {
    value: Option<Box<Expression>>,
    span: Span
}

impl Return {
    pub fn new<V: Into<Option<Box<Expression>>>>(start: Location,
                                                 value: V) -> Return {
        let value = value.into();
        let end = if let &Some(ref exp) = &value {
            exp.span().end()
        }
        else {
            start.offset(5)
        };
        Return {
            value: value.into(),
            span: Span::from(start ..= end)
        }
    }

    pub fn has_value(&self) -> bool {
        if let Some(ref val) = self.value {
            val.has_value()
        }
        else {
            false
        }
    }

    pub fn value(&self) -> Option<&Expression> {
        self.value.as_ref().map(|expr| expr.as_ref())
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

/// Variable declaration
#[derive(Debug, PartialEq, Clone)]
pub struct Declaration {
    mutable: bool,
    ident: Identifier,
    value: Box<Expression>,
    type_decl: Option<TypeExpression>,
    span: Span
}
impl Declaration {
    pub fn new(start: Location,
               ident: Identifier,
               mutable: bool,
               type_decl: Option<TypeExpression>,
               value: Box<Expression>) -> Declaration {
        Declaration {
            ident,
            mutable,
            type_decl,
            value,
            span: Span::from(start ..= value.span().end())
        }
    }

    pub fn name(&self) -> &str {
        &self.ident.name()
    }
    pub fn value(&self) -> &Expression {
        &self.value
    }
    pub fn is_mut(&self) -> bool {
        self.mutable
    }
    pub fn ident(&self) -> &Identifier {
        &self.ident
    }
    pub fn id<'a>(&'a self) -> Ref<'a, ScopedId> {
        self.ident().id()
    }
    pub fn set_id(&self, id: ScopedId) {
        self.ident().set_id(id);
    }
    pub fn type_decl(&self) -> Option<&TypeExpression> {
        self.type_decl.as_ref()
    }
    pub fn has_declared_type(&self) -> bool {
        self.type_decl.is_some()
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

/// Do <block> statement.
#[derive(Debug, PartialEq, Clone)]
pub struct DoBlock {
    block: Box<Block>,
    span: Span
}
impl DoBlock {
    pub fn new(start: Location, block: Box<Block>) -> DoBlock {
        DoBlock { block, span: Span::from(start ..= (*block).span().end()) }
    }

    pub fn block(&self) -> &Block {
        &self.block
    }

    pub fn id<'a>(&'a self) -> Ref<'a, ScopedId> {
        self.block.id()
    }

    pub fn set_id(&self, id: ScopedId) {
        self.block.set_id(id);
    }

    pub fn source<'a>(&'a self) -> Ref<'a, Option<ScopedId>> {
        self.block.source()
    }

    pub fn set_source(&self, source: ScopedId) {
        self.block.set_source(source)
    }

    pub fn has_source(&self) -> bool {
        self.block.has_source()
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

/// if <condition> <block>
///
/// At the moment I'll be keeping `if` as a block because I don't think the syntax
/// will work well as an expression: consider
/// ```protosnirk
/// let x = if someExpression() != someOtherThing() someValue else false
/// ```
/// If there isn't a newline separaing the conditional from the first block, it's not
/// going to work out well.
///
/// In a somewhat unusual approach, I'll be completely ignring the "danging else" problem
/// thanks to the handwritten parsers.
///
/// The conditionals are in a list instead of nested.
#[derive(Debug, PartialEq, Clone)]
pub struct IfBlock {
    conditionals: Vec<Conditional>,
    else_block: Option<Block>,
    scoped_id: RefCell<ScopedId>,
    source: RefCell<Option<ScopedId>>,
    span: Span
}

/// A basic conditional
#[derive(Debug, PartialEq, Clone)]
pub struct Conditional {
    condition: Expression,
    block: Block,
    span: Span
}

impl IfBlock {
    pub fn new(start: Location,
               conditionals: Vec<Conditional>,
               else_block: Option<Block>) -> IfBlock {
        debug_assert!(!conditionals.is_empty(),
                      "Attempted to create an `If` with 0 conditionals");
        let end = if let Some(else_block) = else_block {
            else_block.span().end()
        }
        else if let Some(last_cond) = conditionals.last() {
            last_cond.span().end()
        }
        else {
            unreachable!("Attempted to create an if with no conditionals")
        };
        IfBlock {
            conditionals: conditionals,
            else_block: else_block,
            scoped_id: RefCell::default(),
            source: RefCell::new(None),
            span: Span::from(start ..= end)
        }
    }
    pub fn has_else_if(&self) -> bool {
        self.conditionals.len() > 1
    }
    pub fn has_else(&self) -> bool {
        self.else_block.is_some()
    }
    pub fn conditionals(&self) -> &Vec<Conditional> {
        &self.conditionals
    }
    pub fn condition(&self) -> &Expression {
        &self.conditionals[0].condition
    }
    pub fn else_block(&self) -> Option<&Block> {
        self.else_block.as_ref()
    }
    pub fn id<'a>(&'a self) -> Ref<'a, ScopedId> {
        self.scoped_id.borrow()
    }
    pub fn set_id(&self, id: ScopedId) {
        *self.scoped_id.borrow_mut() = id;
    }

    pub fn source<'a>(&'a self) -> Ref<'a, Option<ScopedId>> {
        self.source.borrow()
    }
    pub fn set_source(&self, source: ScopedId) {
        *self.source.borrow_mut() = Some(source);
    }
    pub fn has_source(&self) -> bool {
        self.source.borrow().is_some()
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

impl Conditional {
    pub fn new(start: Location,
               condition: Expression,
               block: Block) -> Conditional {
        Conditional {
            condition,
            block,
            span: Span::from(start ..= block.span().end())
        }
    }
    pub fn condition(&self) -> &Expression {
        &self.condition
    }
    pub fn block(&self) -> &Block {
        &self.block
    }

    pub fn source<'a>(&'a self) -> Ref<'a, Option<ScopedId>> {
        self.block.source()
    }

    pub fn set_source(&self, source: ScopedId) {
        self.block.set_source(source)
    }

    pub fn has_source(&self) -> bool {
        self.block.has_source()
    }

    pub fn span(&self) -> Span {
        self.span
    }
}
