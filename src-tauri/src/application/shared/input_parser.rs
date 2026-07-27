// application/shared/input_parser.rs

use crate::application::{shared::ValidationErrors, ApplicationError};

pub struct InputParser {
    errors: ValidationErrors,
}

impl Default for InputParser {
    fn default() -> Self {
        Self::new()
    }
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

    pub fn value<T, E>(&mut self, field: &'static str, result: Result<T, E>) -> Option<T>
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

    pub fn domain<T, E>(&mut self, field: &'static str, result: Result<T, E>) -> Option<T>
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

    pub fn push(&mut self, field: &'static str, code: &'static str, message: impl Into<String>) {
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

    pub fn combine<A, B, T, E>(
        &mut self,
        field: &'static str,
        a: Option<A>,
        b: Option<B>,
        f: impl FnOnce(A, B) -> Result<T, E>,
    ) -> Option<T>
    where
        E: std::fmt::Display,
    {
        match (a, b) {
            (Some(a), Some(b)) => self.domain(field, f(a, b)),
            _ => None,
        }
    }

    pub fn map2<A, B, T>(
        &mut self,
        a: Option<A>,
        b: Option<B>,
        f: impl FnOnce(A, B) -> T,
    ) -> Option<T> {
        match (a, b) {
            (Some(a), Some(b)) => Some(f(a, b)),
            _ => None,
        }
    }

    pub fn optional<T, Raw, E>(
        &mut self,
        field: &'static str,
        raw: Option<Raw>,
        f: impl FnOnce(Raw) -> Result<T, E>,
    ) -> Option<T>
    where
        E: std::fmt::Display,
    {
        raw.and_then(|v| self.value(field, f(v)))
    }
}
