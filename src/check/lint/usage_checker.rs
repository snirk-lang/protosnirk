//! Checks that variables have been used in a symbol table.

use check::{CheckerError, ErrorCollector, SymbolTable};

/// Reports warnings for unused variables and un-mutated mutable variables.
#[derive(Debug)]
pub struct UsageChecker { }
impl UsageChecker {
    pub fn warn_for_unsused(&self, warns: &mut ErrorCollector, table: &SymbolTable) {
        trace!("Found symbol table {:#?}", table);
        for (_name, sym) in table.iter() {
            debug_assert!(!(!sym.is_mutable() && sym.is_mutated()),
                "Did not expect immutable {:?} to be mutated", sym);
            if !sym.is_used() {
                let err_message = format!("{} {} is declared but never used",
                    sym.get_source().get_name(),
                    sym.get_declaration().text);
                warns.add_warning(
                    VerifyError::new(sym.get_declaration().clone(), vec![], err_message));
            }
            if sym.is_mutable() && !sym.is_mutated() {
                let err_message = format!("{} {} is declared mutable but never mutated",
                    sym.get_source().get_name(),
                    sym.get_declaration().text);
                warns.add_warning(
                    VerifyError::new(sym.get_declaration().clone(), vec![], err_message));
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use lex::{Token, TokenData, TextLocation};
    use parse::ASTVisitor;
    use parse::tests::parser;
    use parse::verify::scope::SymbolTable;
    use parse::verify::{ErrorCollector, VerifyError};
    use parse::verify::checker::*;

    #[test]
    fn it_detects_unused_declared() {
        let mut parser = parser("let x = 0");
        let block = parser.block().unwrap();
        let errors = ErrorCollector::new();
        let mut sym_checker = SymbolTableChecker::new(errors);
        sym_checker.check_block(&block);
        let (sym_table, mut errors) = sym_checker.decompose();
        UsageChecker { }.warn_for_unsused(&mut errors, &sym_table);
        let expected = vec![
            VerifyError::new(Token {
                location: TextLocation { index: 4, line: 0, column: 4 },
                text: Cow::Borrowed("x"),
                data: TokenData::Ident
            },
            vec![],
            "Variable x is declared but never used".to_string())
        ];
        assert_eq!(errors.get_warnings(), &*expected);
    }

    #[test]
    fn it_detects_usage_in_assignment_rvalue() {
        let mut parser = parser("let x = 0 let mut y = 0 y = x y");
        let block = parser.block().unwrap();
        let errors = ErrorCollector::new();
        let mut sym_checker = SymbolTableChecker::new(errors);
        sym_checker.check_block(&block);
        let (sym_table, mut errors) = sym_checker.decompose();
        UsageChecker { }.warn_for_unsused(&mut errors, &sym_table);
        assert_eq!(errors.get_warnings(), &*vec![]);
    }

    #[test]
    fn it_detects_usage_in_return_value() {
        let mut parser = parser("let x = 0 return x");
        let block = parser.block().unwrap();
        let errors = ErrorCollector::new();
        let mut sym_checker = SymbolTableChecker::new(errors);
        sym_checker.check_block(&block);
        let (sym_table, mut errors) = sym_checker.decompose();
        UsageChecker { }.warn_for_unsused(&mut errors, &sym_table);
        assert_eq!(errors.get_warnings(), &*vec![]);
    }

    #[test]
    fn it_detects_usage_in_implicit_return_expr() {
        let mut parser = parser("let x = 0 x");
        let block = parser.block().unwrap();
        let errors = ErrorCollector::new();
        let mut sym_checker = SymbolTableChecker::new(errors);
        sym_checker.check_block(&block);
        let (sym_table, mut errors) = sym_checker.decompose();
        UsageChecker { }.warn_for_unsused(&mut errors, &sym_table);
        assert_eq!(errors.get_warnings(), &*vec![]);
    }

    #[test]
    fn it_detects_usage_in_binary_operator_expr() {
        let mut parser = parser("let x = 0 x + 1");
        let block = parser.block().unwrap();
        let errors = ErrorCollector::new();
        let mut sym_checker = SymbolTableChecker::new(errors);
        sym_checker.check_block(&block);
        let (sym_table, mut errors) = sym_checker.decompose();
        UsageChecker { }.warn_for_unsused(&mut errors, &sym_table);
        assert_eq!(errors.get_warnings(), &*vec![]);
    }

    #[test]
    fn it_detects_usage_in_declaration_rvalue() {
        let mut parser = parser("let x = 0 let y = x");
        let block = parser.block().unwrap();
        let errors = ErrorCollector::new();
        let mut sym_checker = SymbolTableChecker::new(errors);
        sym_checker.check_block(&block);
        let (sym_table, mut errors) = sym_checker.decompose();
        UsageChecker { }.warn_for_unsused(&mut errors, &sym_table);
        let expected = vec![
            VerifyError::new(Token {
                location: TextLocation { index: 14, line: 0, column: 14 },
                text: Cow::Borrowed("y"),
                data: TokenData::Ident
            },
            vec![],
            "Variable y is declared but never used".to_string())
        ];
        assert_eq!(errors.get_warnings(), &*expected);
    }

    #[test]
    fn it_detects_unused_declared_with_similar_variables_used() {
        let mut parser = parser("let x = 0 let mut y = 0 let Y = 0 y += Y");
        let block = parser.block().unwrap();
        let errors = ErrorCollector::new();
        let mut sym_checker = SymbolTableChecker::new(errors);
        sym_checker.check_block(&block);
        let (sym_table, mut errors) = sym_checker.decompose();
        UsageChecker { }.warn_for_unsused(&mut errors, &sym_table);
        let expected = vec![
            VerifyError::new(Token {
                location: TextLocation { index: 4, line: 0, column: 4 },
                text: Cow::Borrowed("x"),
                data: TokenData::Ident
            },
            vec![],
            "Variable x is declared but never used".to_string())
        ];
        assert_eq!(errors.get_warnings(), &*expected);
    }

}
