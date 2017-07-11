use check::ErrorCollector;

use check::types::environment::TypeEnvironment;

/// Collects type equations in expressions
pub struct ExpressionTypeCollector<'err, 'env> {
    errors: &'err mut ErrorCollector,
    equations: &'env mut TypeEnvironment
}
