use std::collections::HashMap;
use std::ops::Deref;

use lex::Token;
use parse::ASTVisitor;
use parse::ast::{Declaration, Identifier, Assignment, Block};
use parse::verify::{ErrorCollector, VerifyError, Symbol};
use parse::verify::scope::{ScopeIndex, SymbolTable, SymbolTableBuilder};

/// Builds up the symbol table for a parse tree
/// and reports variable declaration and mutability errors.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SymbolTableChecker {
    symbol_table: SymbolTable,
    table_builder: SymbolTableBuilder,
    current_index: ScopeIndex,
    errors: ErrorCollector
}
impl SymbolTableChecker {
    pub fn new(errors: ErrorCollector) -> SymbolTableChecker {
        SymbolTableChecker {
            symbol_table: SymbolTable::new(),
            table_builder: SymbolTableBuilder::new(),
            current_index: ScopeIndex::new(vec![]),
            errors: errors
        }
    }
    pub fn decompose(self) -> (SymbolTable, ErrorCollector) {
        (self.symbol_table, self.errors)
    }
}
impl ASTVisitor for SymbolTableChecker {
    fn check_declaration(&mut self, decl: &Declaration) {
        // Check rvalue first to prevent use-before-declare
        self.check_expression(&decl.value);

        // Check for name conflicts
        // TODO this prevents more shadowing than intended
        if let Some(declared_index) = self.table_builder.get(decl.get_name()).cloned() {
            let declared_at = self.symbol_table[&declared_index].get_declaration().clone();
            // Add previous declaration
            let references: Vec<Token> = vec![declared_at];
            let err_text = format!("Variable {} is already declared", decl.get_name());
            self.errors.add_error(VerifyError::new(decl.token.clone(), references, err_text));
        } else {
            let var_index = self.current_index.clone();
            self.current_index.increment();
            decl.get_ident().set_index(var_index.clone());
            self.table_builder.define_local(decl.get_name().to_string(), var_index.clone());
            self.symbol_table.insert(var_index.clone(), Symbol::from_declaration(decl, var_index));
        }
    }
    fn check_var_ref(&mut self, var_ref: &Identifier) {
        if let Some(index) = self.table_builder.get(var_ref.get_name()) {
            var_ref.set_index(index.clone());
            self.symbol_table.get_mut(&var_ref.get_index())
                .map(Symbol::set_used);
        }
        else {
            let err_text = format!("Variable {} was not declared", var_ref.get_name());
            self.errors.add_error(VerifyError::new(var_ref.token.clone(), vec![], err_text));
        }
    }
    fn check_assignment(&mut self, assign: &Assignment) {
        if let Some(index) = self.table_builder.get(assign.lvalue.get_name()) {
            assign.lvalue.set_index(index.clone());
            if !self.symbol_table[index].is_mutable() {
                let err_text = format!("Variable {} was not declared mutable", assign.lvalue.get_name());
                let references = vec![
                    self.symbol_table[index].get_declaration().clone(),
                ];
                self.errors.add_error(VerifyError::new(assign.lvalue.token.clone(), references, err_text));
            }
            else {
                self.symbol_table.get_mut(index)
                    .map(Symbol::set_mutated);
            }
        }
        else {
            let err_text = format!("Variable {} was not declared", assign.lvalue.get_name());
            self.errors.add_error(VerifyError::new(assign.lvalue.token.clone(), vec![], err_text));
        }
        self.check_expression(&assign.rvalue);
    }
    fn check_block(&mut self, block: &Block) {
        self.current_index.push();
        for stmt in &block.statements {
            self.check_statement(&stmt);
        }
        self.current_index.pop();
        self.current_index.increment();
    }
}
#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use lex::{Token, TokenData, TextLocation};
    use parse::ASTVisitor;
    use parse::tests::parser;
    use parse::verify::{ErrorCollector, VerifyError};
    use parse::build::SymbolTable;
    use super::SymbolTableChecker;

    #[test]
    fn it_finds_extra_declaration() {
        let mut parser = parser("let x = 0 let x = 1");
        let block = parser.block().unwrap();
        let errors = ErrorCollector::new();
        let symbol_table = SymbolTable::new();
        let mut sym_checker = SymbolTableChecker::new(errors, symbol_table);
        sym_checker.check_block(&block);
        let (_table, verifier) = sym_checker.decompose();
        let expected = vec![
            VerifyError::new(Token {
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
        let symbol_table = SymbolTable::new();
        let mut sym_checker = SymbolTableChecker::new(errors, symbol_table);
        sym_checker.check_block(&block);
        let (_table, verifier) = sym_checker.decompose();
        let expected = vec![
            VerifyError::new(Token {
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
        let symbol_table = SymbolTable::new();
        let mut sym_checker = SymbolTableChecker::new(errors, symbol_table);
        sym_checker.check_block(&block);
        let (_table, verifier) = sym_checker.decompose();
        let expected = vec![
            VerifyError::new(Token {
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
        let symbol_table = SymbolTable::new();
        let mut sym_checker = SymbolTableChecker::new(errors, symbol_table);
        sym_checker.check_block(&block);
        let (_table, verifier) = sym_checker.decompose();
        let expected: Vec<VerifyError> = vec![
            VerifyError::new(Token {
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
        let symbol_table = SymbolTable::new();
        let mut sym_checker = SymbolTableChecker::new(errors, symbol_table);
        sym_checker.check_block(&block);
        let (_table, verifier) = sym_checker.decompose();
        let expected = vec![
            VerifyError::new(Token {
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
        let symbol_table = SymbolTable::new();
        let mut sym_checker = SymbolTableChecker::new(errors, symbol_table);
        sym_checker.check_block(&block);
        let (_table, verifier) = sym_checker.decompose();
        let expected: Vec<VerifyError> = vec![
            VerifyError::new(Token {
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
        let symbol_table = SymbolTable::new();
        let mut sym_checker = SymbolTableChecker::new(errors, symbol_table);
        sym_checker.check_block(&block);
        let (_table, verifier) = sym_checker.decompose();
        let expected: Vec<VerifyError> = vec![
            VerifyError::new(Token {
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
        let symbol_table = SymbolTable::new();
        let mut sym_checker = SymbolTableChecker::new(errors, symbol_table);
        sym_checker.check_block(&block);
        let (_table, verifier) = sym_checker.decompose();
        let expected: Vec<VerifyError> = vec![
            VerifyError::new(Token {
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
        let symbol_table = SymbolTable::new();
        let mut sym_checker = SymbolTableChecker::new(errors, symbol_table);
        sym_checker.check_block(&block);
        let (_table, verifier) = sym_checker.decompose();
        let expected = vec![
            VerifyError::new(Token {
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
        let symbol_table = SymbolTable::new();
        let mut sym_checker = SymbolTableChecker::new(errors, symbol_table);
        sym_checker.check_block(&block);
        let (_table, verifier) = sym_checker.decompose();
        let expected: Vec<VerifyError> = vec![
            VerifyError::new(Token {
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
        let symbol_table = SymbolTable::new();
        let mut sym_checker = SymbolTableChecker::new(errors, symbol_table);
        sym_checker.check_block(&block);
        let (_table, verifier) = sym_checker.decompose();
        let expected: Vec<VerifyError> = vec![
            VerifyError::new(
                Token {
                    location: TextLocation { index: 50, line: 4, column: 0 },
                    text: Cow::Borrowed("t"),
                    data: TokenData::Ident
                }, vec![],
                "Variable t was not declared".into()),
            VerifyError::new(
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
