// application/shared/input_parser.rs

use crate::application::{
    ApplicationError,
    shared::ValidationErrors,
};

pub struct InputParser {
    errors: ValidationErrors,
}

impl InputParser {

    pub fn new() -> Self {
        Self {
            errors: ValidationErrors::new(),
        }
    }

    // ---------------------------
    // Parse primitive/domain values
    // ---------------------------

    pub fn value<T, E>(
        &mut self,
        field: &'static str,
        result: Result<T, E>,
    ) -> Option<T>
    where
        E: std::fmt::Display,
    {
        match result {
            Ok(v) => Some(v),
            Err(e) => {
                self.errors.push(field, "invalid", e.to_string());
                None
            }
        }
    }

    // ---------------------------
    // Domain rule validation
    // ---------------------------

    pub fn domain<T, E>(
        &mut self,
        field: &'static str,
        result: Result<T, E>,
    ) -> Option<T>
    where
        E: std::fmt::Display,
    {
        match result {
            Ok(v) => Some(v),
            Err(e) => {
                self.errors.push(field, "invalid_geometry", e.to_string());
                None
            }
        }
    }

    // ---------------------------
    // Push custom error
    // ---------------------------

    pub fn push(
        &mut self,
        field: &'static str,
        code: &'static str,
        message: impl Into<String>,
    ) {
        self.errors.push(field, code, message);
    }

    // ---------------------------
    // Finish validation
    // ---------------------------

    pub fn finish(self) -> Result<(), ApplicationError> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(ApplicationError::Validation(self.errors))
        }
    }

    // ---------------------------
    // Finish with value
    // ---------------------------

    pub fn finish_with<T>(self, value: Option<T>) -> Result<T, ApplicationError> {
        if self.errors.is_empty() {
            Ok(value.expect("validation succeeded but value missing"))
        } else {
            Err(ApplicationError::Validation(self.errors))
        }
    }
}