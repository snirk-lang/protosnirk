use parse::ScopedId;
use parse::ast::*;

use check::ASTVisitor;

pub struct ScopeChecker {
    current_id: ScopedId
}

impl ASTVisitor for ScopeChecker {

}
