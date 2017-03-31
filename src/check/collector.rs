//! The verifier verifies the initial parse tree, returning a list of errors and warnings

use super::VerifyError;

/// Structure to hold compiler errors, warnings, and lints.
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct ErrorCollector {
    errors: Vec<VerifyError>,
    warnings: Vec<VerifyError>,
    lints: Vec<VerifyError>
}
impl ErrorCollector {
    pub fn new() -> ErrorCollector {
        ErrorCollector {
            .. Default::default()
        }
    }
    pub fn add_error(&mut self, err: VerifyError) {
        self.errors.push(err);
    }
    pub fn add_warning(&mut self, warn: VerifyError) {
        self.warnings.push(warn);
    }
    pub fn add_lint(&mut self, lint: VerifyError) {
        self.lints.push(lint);
    }

    pub fn get_errors(&self) -> &[VerifyError] {
        &self.errors
    }
    pub fn get_warnings(&self) -> &[VerifyError] {
        &self.warnings
    }
    pub fn get_lints(&self) -> &[VerifyError] {
        &self.lints
    }

    pub fn get_errors_mut(&mut self) -> &mut [VerifyError] {
        &mut self.errors
    }
    pub fn get_warnings_mut(&mut self) -> &mut [VerifyError] {
        &mut self.warnings
    }
    pub fn get_lints_mut(&mut self) -> &mut [VerifyError] {
        &mut self.errors
    }

}
