use std::collections::HashMap;
use std::ops::Deref;

use lex::Token;

use parse::ScopedId;
use parse::ast::*;
use parse::ast::types::*;

use check::{ASTVisitor, ErrorCollector, CheckerError};
use check::symbol::SymbolTable;
use check::symbol::table_builder::SymbolTableBuilder;

/// Assigns unique `ScopedId`s to `Identifier`s in the AST and reports
/// missing variable and mutability errors.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SymbolChecker {
    symbol_table: SymbolTable,
    table_builder: SymbolTableBuilder,
    current_index: ScopedId,
    errors: ErrorCollector
}
impl SymbolChecker {
    pub fn new(errors: ErrorCollector) -> SymbolChecker {
        SymbolChecker {
            symbol_table: SymbolTable::new(),
            table_builder: SymbolTableBuilder::new(),
            current_index: ScopedId::default(),
            errors: errors
        }
    }
    pub fn decompose(self) -> (SymbolTable, ErrorCollector) {
        (self.symbol_table, self.errors)
    }
}
impl ASTVisitor for SymbolChecker {
    fn check_declaration(&mut self, decl: &Declaration) {
        // Check rvalue first to prevent use-before-declare
        self.check_expression(&decl.value);
        trace!("Checking declaration of {}", decl.get_name());
        // Check for name conflicts
        // TODO this prevents more shadowing than intended
        if let Some(declared_index) = self.table_builder.get(decl.get_name()).cloned() {
            let declared_at = self.symbol_table[&declared_index].get_declaration().clone();
            // Add previous declaration
            let references: Vec<Token> = vec![declared_at];
            let err_text = format!("Variable {} is already declared", decl.get_name());
            self.errors.add_error(CheckerError::new(decl.get_ident().get_token().clone(), references, err_text));
        } else {
            let var_index = self.current_index.clone();
            self.current_index.increment();
            trace!("Created index {:?} for declared var {}", var_index, decl.get_name());
            decl.get_ident().set_index(var_index.clone());
            self.table_builder.define_local(decl.get_name().to_string(), var_index.clone());
            self.symbol_table.insert(var_index.clone(),
                Symbol::from_declaration(decl, var_index));
        }
    }

    fn check_var_ref(&mut self, var_ref: &Identifier) {
        trace!("Checking reference to {}", var_ref.get_name());
        if let Some(index) = self.table_builder.get(var_ref.get_name()) {
            trace!("Found reference to {} at {:?}", var_ref.get_name(), index);
            var_ref.set_index(index.clone());
            self.symbol_table.get_mut(&var_ref.get_index())
                .map(Symbol::set_used);
        }
        else {
            let err_text = format!("Variable {} was not declared", var_ref.get_name());
            self.errors.add_error(CheckerError::new(var_ref.token.clone(), vec![], err_text));
        }
    }

    fn check_assignment(&mut self, assign: &Assignment) {
        trace!("Checking assignment to {}", assign.lvalue.get_name());
        if let Some(index) = self.table_builder.get(assign.lvalue.get_name()) {
            trace!("Found reference to {} at {:?}", assign.lvalue.get_name(), index);
            assign.lvalue.set_index(index.clone());
            if !self.symbol_table[index].is_mutable() {
                let err_text = format!("Variable {} was not declared mutable", assign.lvalue.get_name());
                let references = vec![
                    self.symbol_table[index].get_declaration().clone(),
                ];
                self.errors.add_error(CheckerError::new(assign.lvalue.token.clone(), references, err_text));
            }
            else {
                self.symbol_table.get_mut(index)
                    .map(Symbol::set_mutated);
            }
        }
        else {
            let err_text = format!("Variable {} was not declared", assign.lvalue.get_name());
            self.errors.add_error(CheckerError::new(assign.lvalue.token.clone(), vec![], err_text));
        }
        self.check_expression(&assign.rvalue);
    }

    fn check_block(&mut self, block: &Block) {
        trace!("Checking a block");
        self.current_index.push();
        self.table_builder.new_scope();
        for stmt in &block.statements {
            self.check_statement(&stmt);
        }
        self.table_builder.pop();
        self.current_index.pop();
        self.current_index.increment();
    }

    fn check_fn_declaration(&mut self, fn_declaration: &FnDeclaration) {
        trace!("Checking function declaration for {}", fn_declaration.get_name().get_name());
        if let Some(index) = self.table_builder.get(fn_declaration.get_name().get_name()).cloned() {
            let declared_at = self.symbol_table[&index].get_declaration().clone();
            // Add declaration to error
            let references = vec![declared_at];
            let err_text = format!("Function {} is already declared",
                fn_declaration.get_name().get_name());
            self.errors.add_error(CheckerError::new(fn_declaration.get_name().get_token().clone(),
                                                   references,
                                                   err_text));
            // _Should_ return here
            // but let's see if checking the function ends up being helpful
        }

        let fn_index = self.current_index.clone();
        self.current_index.push();
        self.table_builder.new_scope();
        // Declared function info
        let mut param_types = Vec::new();

        for param in fn_declaration.get_args() {
            trace!("Checking parameter {}", param.get_name());
            // All parameters are floats for now
            param_types.push((param.get_name().to_string(), Type::Float));
            // Check standard symbol table for any conflicts.
            // They're probably only present in other param names.
            if let Some(declared_index) = self.table_builder.get(param.get_name()).cloned() {
                let declared_at = self.symbol_table[&declared_index].get_declaration().clone();
                // Add previous declaration
                let references = vec![declared_at];
                let err_text = format!("Argument {} is already declared", param.get_name());
                self.errors.add_error(CheckerError::new(param.get_token().clone(), references, err_text));
                // We will keep parsing arg params after registering duplicate
                continue
            }
            let var_index = self.current_index.clone();
            self.current_index.increment();
            trace!("Created index {:?} for fn arg {}", var_index, param.get_name());
            param.set_index(var_index.clone());
            self.table_builder.define_local(param.get_name().to_string(), var_index.clone());
            self.symbol_table.insert(var_index.clone(),
                Symbol::from_parameter(param, var_index));
        }
        // Add the function to the symbol table
        let fn_type = Type::Fn(FnType::new(Box::new(Type::Float), param_types));
        self.table_builder.define_global(fn_declaration.get_name().get_name().into(),
                                         fn_index.clone());
        self.symbol_table.insert(fn_index.clone(),
            Symbol::from_fn_decl(fn_declaration.get_name(), fn_index.clone(), fn_type));
        fn_declaration.get_name().set_index(fn_index);

        // Inlined the check_block code here, didn't feel like having fn args be in a different
        // scope.
        for stmt in &fn_declaration.get_block().statements {
            self.check_statement(&stmt);
        }
        // Go back to global scope
        self.current_index.pop();
        self.table_builder.pop();
        // Go on to the next function
        self.current_index.increment();
    }

    fn check_fn_call(&mut self, fn_call: &FnCall) {
        trace!("Checking function call of {}", fn_call.get_name().get_name());
        // Should also get some better naming conventions here
        if let Some(fn_index) = self.table_builder.get(fn_call.get_text()).cloned() {
            trace!("Found function info of {}", fn_call.get_text());
            fn_call.get_name().set_index(fn_index.clone());
            // TODO cloning the symbol here, may even switch to Rc to make this easier
            // across the checker.
            let fn_info = self.symbol_table[&fn_index].clone();
            if let &Type::Fn(ref fn_type) = fn_info.get_type() {
                let declared_len = fn_type.get_args().len();
                let invoked_len = fn_call.get_args().len();
                if declared_len != invoked_len {
                    // This is how Rust does it but we can do better.
                    let err_text = format!("Function {}: expected {} args, got {}",
                        fn_call.get_name().get_name(), declared_len, invoked_len);
                    let err = CheckerError::new(fn_call.get_token().clone(), vec![], err_text);
                    self.errors.add_error(err);
                }
                match *fn_call.get_args() {
                    FnCallArgs::SingleExpr(ref expr) => {
                        if declared_len != 1 {
                            // TODO could also provide references
                            let err_text = format!("Function {}: expected {} args, got 1",
                                fn_call.get_name().get_name(), declared_len);
                            let err = CheckerError::new(fn_call.get_token().clone(), vec![], err_text);
                            self.errors.add_error(err);
                        }
                        self.check_expression(expr);
                    }
                    FnCallArgs::Arguments(ref args) => {
                        for call_arg in args {
                            trace!("Checking arg {:#3?}", call_arg);
                            // Check values given to params first.
                            if let Some(expr) = call_arg.get_expr() {
                                self.check_expression(expr);
                            }
                            else {
                                // It's got a var ref
                                self.check_var_ref(call_arg.get_name());
                            }
                            if let Some((_ix, _declared_type)) = fn_type.get_arg(call_arg.get_text()) {
                                let _call_type = self.symbol_table[&call_arg.get_name().get_index()].get_type();

                                // We need to be able to do real type check here of the expr that
                                // is being passed into the arg. We don't have that - we don't even
                                // know if it's an attempt at a function reference or something.
                                /*
                                if &declared_type != call_type {
                                    let err_text = format!("Expected type {:?} for arg {} of {}, got {:?}",
                                        declared_type, call_arg.get_text(), fn_call.get_text(), call_type);
                                    let refs = vec![call_arg.get_name().get_token().clone(), fn_info.get_declaration().clone()];
                                    let err = CheckerError::new(fn_call.get_token().clone(), refs, err_text);
                                    self.errors.add_error(err);
                                }
                                */
                                // else the arg matches and don't need to do anything.
                            }
                            else {
                                let err_text = format!("Unknown parameter {}", call_arg.get_text());
                                let refs = vec![call_arg.get_name().get_token().clone()];
                                let err = CheckerError::new(fn_call.get_token().clone(), refs, err_text);
                                self.errors.add_error(err);
                            }
                        }
                    }
                }
            }
            self.symbol_table.get_mut(&fn_call.get_name().get_index())
                .map(Symbol::set_used);
        }
        else {
            let err_text = format!("Unknown function {}", fn_call.get_text());
            let err = CheckerError::new(fn_call.get_token().clone(), vec![], err_text);
            self.errors.add_error(err);
        }
    }

    fn check_unit(&mut self, unit: &Unit) {
        self.table_builder.new_scope();
        for item in unit.get_items() {
            self.check_item(item);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use lex::{Token, TokenData, TextLocation};
    use parse::tests::parser;
    use check::{ASTVisitor, ErrorCollector, CheckerError};
    use super::SymbolTableChecker;

    #[test]
    fn it_finds_extra_declaration() {
        let mut parser = parser("let x = 0 let x = 1");
        let block = parser.block().unwrap();
        let errors = ErrorCollector::new();
        let mut sym_checker = SymbolTableChecker::new(errors);
        sym_checker.check_block(&block);
        let (_table, verifier) = sym_checker.decompose();
        let expected = vec![
            CheckerError::new(Token {
                    location: TextLocation { index: 14, line: 0, column: 14 },
                    text: Cow::Borrowed("x"),
                    data: TokenData::Ident
                },
                vec![
                    Token {
                        location: TextLocation { index: 4, line: 0, column: 4 },
                        text: Cow::Borrowed("x"),
                        data: TokenData::Ident
                    }
                ],
                "Variable x is already declared".into())
        ];
        assert_eq!(verifier.get_errors(), &*expected);
    }

    #[test]
    fn it_finds_missing_declaration() {
        let mut parser = parser("let mut y = 0 y = x + 1");
        let block = parser.block().unwrap();
        let errors = ErrorCollector::new();
        let mut sym_checker = SymbolTableChecker::new(errors);
        sym_checker.check_block(&block);
        let (_table, verifier) = sym_checker.decompose();
        let expected = vec![
            CheckerError::new(Token {
                location: TextLocation { index: 18, line: 0, column: 18 },
                text: Cow::Borrowed("x"),
                data: TokenData::Ident
            },
            vec![],
            "Variable x was not declared".into())
        ];
        assert_eq!(verifier.get_errors(), &*expected);
    }

    #[test]
    fn it_finds_extra_declaration_of_different_type() {
        let mut parser = parser("let x = 0 let mut x = 1");
        let block = parser.block().unwrap();
        let errors = ErrorCollector::new();
        let mut sym_checker = SymbolTableChecker::new(errors);
        sym_checker.check_block(&block);
        let (_table, verifier) = sym_checker.decompose();
        let expected = vec![
            CheckerError::new(Token {
                location: TextLocation { index: 18, line: 0, column: 18 },
                text: Cow::Borrowed("x"),
                data: TokenData::Ident
            },
            vec![
                Token {
                    location: TextLocation { index: 4, line: 0, column: 4 },
                    text: Cow::Borrowed("x"),
                    data: TokenData::Ident
                }
            ],
            "Variable x is already declared".into())
        ];
        assert_eq!(verifier.get_errors(), &*expected);
    }

    #[test]
    fn it_finds_missing_declaration_in_return_stmt() {
        let mut parser = parser("let x = 0 return y");
        let block = parser.block().unwrap();
        let errors = ErrorCollector::new();
        let mut sym_checker = SymbolTableChecker::new(errors);
        sym_checker.check_block(&block);
        let (_table, verifier) = sym_checker.decompose();
        let expected: Vec<CheckerError> = vec![
            CheckerError::new(Token {
                location: TextLocation {
                    index: 17,
                    line: 0,
                    column: 17
                },
                text: Cow::Borrowed("y"),
                data: TokenData::Ident
            }, vec![],
            "Variable y was not declared".into())
        ];
        assert_eq!(verifier.get_errors(), &*expected);
    }

    #[test]
    fn it_finds_missing_declaration_in_binary_op() {
        let mut parser = parser("let x = 0 x + y");
        let block = parser.block().unwrap();
        let errors = ErrorCollector::new();
        let mut sym_checker = SymbolTableChecker::new(errors);
        sym_checker.check_block(&block);
        let (_table, verifier) = sym_checker.decompose();
        let expected = vec![
            CheckerError::new(Token {
                location: TextLocation { index: 14, line: 0, column: 14 },
                text: Cow::Borrowed("y"),
                data: TokenData::Ident
            },
            vec![],
            "Variable y was not declared".into())
        ];
        assert_eq!(verifier.get_errors(), &*expected);

    }

    #[test]
    fn it_finds_missing_declaration_in_unary_op() {
        let mut parser = parser("let x = 0 return -y");
        let block = parser.block().unwrap();
        let errors = ErrorCollector::new();
        let mut sym_checker = SymbolTableChecker::new(errors);
        sym_checker.check_block(&block);
        let (_table, verifier) = sym_checker.decompose();
        let expected: Vec<CheckerError> = vec![
            CheckerError::new(Token {
                location: TextLocation {
                    index: 18,
                    line: 0,
                    column: 18
                },
                text: Cow::Borrowed("y"),
                data: TokenData::Ident
            }, vec![],
            "Variable y was not declared".into())
        ];
        assert_eq!(verifier.get_errors(), &*expected);

    }

    #[test]
    fn it_finds_missing_declaration_in_var_ref_expr() {
        let mut parser = parser("let x = 0 y");
        let block = parser.block().unwrap();
        let errors = ErrorCollector::new();
        let mut sym_checker = SymbolTableChecker::new(errors);
        sym_checker.check_block(&block);
        let (_table, verifier) = sym_checker.decompose();
        let expected: Vec<CheckerError> = vec![
            CheckerError::new(Token {
                location: TextLocation {
                    index: 10,
                    line: 0,
                    column: 10
                },
                text: Cow::Borrowed("y"),
                data: TokenData::Ident
            }, vec![],
            "Variable y was not declared".into())
        ];
        assert_eq!(verifier.get_errors(), &*expected);
    }

    #[test]
    fn it_finds_missing_declaration_in_assignment_lvalue() {
        let mut parser = parser("let x = 0 y = x");
        let block = parser.block().unwrap();
        let errors = ErrorCollector::new();
        let mut sym_checker = SymbolTableChecker::new(errors);
        sym_checker.check_block(&block);
        let (_table, verifier) = sym_checker.decompose();
        let expected: Vec<CheckerError> = vec![
            CheckerError::new(Token {
                location: TextLocation {
                    index: 10,
                    line: 0,
                    column: 10
                },
                text: Cow::Borrowed("y"),
                data: TokenData::Ident
            }, vec![],
            "Variable y was not declared".into())
        ];
        assert_eq!(verifier.get_errors(), &*expected);
    }

    #[test]
    fn it_finds_missing_declaration_in_assignment_rvalue() {
        let mut parser = parser("let mut x = 0 x = y");
        let block = parser.block().unwrap();
        let errors = ErrorCollector::new();
        let mut sym_checker = SymbolTableChecker::new(errors);
        sym_checker.check_block(&block);
        let (_table, verifier) = sym_checker.decompose();
        let expected = vec![
            CheckerError::new(Token {
                location: TextLocation { index: 18, line: 0, column: 18 },
                text: Cow::Borrowed("y"),
                data: TokenData::Ident
            },
            vec![],
            "Variable y was not declared".into())
        ];
        assert_eq!(verifier.get_errors(), &*expected);
    }

    #[test]
    fn it_finds_circular_declaration() {
        let mut parser = parser("let x = x");
        let block = parser.block().unwrap();
        let errors = ErrorCollector::new();
        let mut sym_checker = SymbolTableChecker::new(errors);
        sym_checker.check_block(&block);
        let (_table, verifier) = sym_checker.decompose();
        let expected: Vec<CheckerError> = vec![
            CheckerError::new(Token {
                location: TextLocation {
                    index: 8,
                    line: 0,
                    column: 8
                },
                text: Cow::Borrowed("x"),
                data: TokenData::Ident
            }, vec![],
            "Variable x was not declared".into())
        ];
        assert_eq!(verifier.get_errors(), &*expected);
    }

    #[test]
    fn it_finds_missing_declaration_in_assignop_expression() {
        let mut parser = parser("let x = 0 \n\
        let mut y = -x - 1 \n\
        let z = 2 \n\
        y += z \n\
        t += z * (2 % 2) \n\
        return y - 2");
        let block = parser.block().unwrap();
        let errors = ErrorCollector::new();
        let mut sym_checker = SymbolTableChecker::new(errors);
        sym_checker.check_block(&block);
        let (_table, verifier) = sym_checker.decompose();
        let expected: Vec<CheckerError> = vec![
            CheckerError::new(
                Token {
                    location: TextLocation { index: 50, line: 4, column: 0 },
                    text: Cow::Borrowed("t"),
                    data: TokenData::Ident
                }, vec![],
                "Variable t was not declared".into()),
            CheckerError::new(
                Token {
                    location: TextLocation { index: 50, line: 4, column: 0 },
                    text: Cow::Borrowed("t"),
                    data: TokenData::Ident
                }, vec![],
                "Variable t was not declared".into())
            ];
        assert_eq!(verifier.get_errors(), &*expected);
    }

}
