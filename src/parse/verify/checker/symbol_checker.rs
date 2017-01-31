use std::collections::HashMap;

use lex::Token;
use parse::ASTVisitor;
use parse::ast::{Declaration, Identifier, Assignment};
use parse::verify::{ErrorCollector, VerifyError};
use parse::build::{SymbolTable, Symbol};

/// Builds up the symbol table for a parse tree
/// and reports variable declaration and mutability errors.
#[derive(Debug, Default)]
pub struct SymbolTableChecker {
    symbol_table: SymbolTable,
    errors: ErrorCollector
}
impl SymbolTableChecker {
    pub fn new(errors: ErrorCollector, symbol_table: SymbolTable) -> SymbolTableChecker {
        SymbolTableChecker {
            symbol_table: symbol_table,
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

        if let Some(declared_at) = self.symbol_table.get(decl.get_name()).map(|sym| sym.get_token().clone()) {
            // Add previous declaration
            let references: Vec<Token> = vec![declared_at];
            let err_text = format!("Variable {} is already declared", decl.get_name());
            self.errors.add_error(VerifyError::new(decl.token.clone(), references, err_text));
        } else {
            self.symbol_table.insert(decl.get_name().to_string(), Symbol::from_declaration(decl));
        }
    }
    fn check_var_ref(&mut self, var_ref: &Identifier) {
        if !self.symbol_table.contains_key(var_ref.get_name()) {
            let err_text = format!("Variable {} was not declared", var_ref.get_name());
            self.errors.add_error(VerifyError::new(var_ref.token.clone(), vec![], err_text));
        } else {
            self.symbol_table.get_mut(var_ref.get_name()).map(Symbol::set_used);
        }
    }
    fn check_assignment(&mut self, assign: &Assignment) {
        if !self.symbol_table.contains_key(assign.lvalue.get_name()) {
            let err_text = format!("Variable {} was not declared", assign.lvalue.get_name());
            self.errors.add_error(VerifyError::new(assign.lvalue.token.clone(), vec![], err_text));
        }
        else if !self.symbol_table[assign.lvalue.get_name()].is_mutable() {
            let err_text = format!("Variable {} was not declared mutable", assign.lvalue.get_name());
            let references = vec![self.symbol_table[assign.lvalue.get_name()].get_token().clone()];
            self.errors.add_error(VerifyError::new(assign.lvalue.token.clone(), references, err_text));
        }
        else {
            self.symbol_table.get_mut(assign.lvalue.get_name()).map(Symbol::set_mutated);
        }
        self.check_expression(&assign.rvalue);
    }
}
#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use lex::{Token, TokenData, TextLocation};
    use parse::tests::make_parser;
    use parse::verify::{ASTVisitor, ErrorCollector, VerifyError};
    use parse::build::SymbolTable;
    use super::SymbolTableChecker;

    #[test]
    fn it_finds_extra_declaration() {
        let mut parser = make_parser("let x = 0 let x = 1");
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
        let mut parser = make_parser("let mut y = 0 y = x + 1");
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
        let mut parser = make_parser("let x = 0 let mut x = 1");
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
        let mut parser = make_parser("let x = 0 return y");
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
        let mut parser = make_parser("let x = 0 x + y");
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
        let mut parser = make_parser("let x = 0 return -y");
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
        let mut parser = make_parser("let x = 0 y");
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
        let mut parser = make_parser("let x = 0 y = x");
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
        let mut parser = make_parser("let mut x = 0 x = y");
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
        let mut parser = make_parser("let x = x");
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
        let mut parser = make_parser("let x = 0 \n\
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
