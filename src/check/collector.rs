//! The verifier verifies the initial parse tree, returning a list of errors and warnings

use check::CheckerError;

/// Structure to hold compiler errors, warnings, and lints.
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct ErrorCollector {
    errors: Vec<CheckerError>,
    warnings: Vec<CheckerError>,
    lints: Vec<CheckerError>
}
impl ErrorCollector {
    pub fn new() -> ErrorCollector {
        ErrorCollector {
            .. Default::default()
        }
    }
    pub fn add_error(&mut self, err: CheckerError) {
        self.errors.push(err);
    }
    pub fn add_warning(&mut self, warn: CheckerError) {
        self.warnings.push(warn);
    }
    pub fn add_lint(&mut self, lint: CheckerError) {
        self.lints.push(lint);
    }

    pub fn get_errors(&self) -> &[CheckerError] {
        &self.errors
    }
    pub fn get_warnings(&self) -> &[CheckerError] {
        &self.warnings
    }
    pub fn get_lints(&self) -> &[CheckerError] {
        &self.lints
    }

    pub fn get_errors_mut(&mut self) -> &mut [CheckerError] {
        &mut self.errors
    }
    pub fn get_warnings_mut(&mut self) -> &mut [CheckerError] {
        &mut self.warnings
    }
    pub fn get_lints_mut(&mut self) -> &mut [CheckerError] {
        &mut self.errors
    }

}
