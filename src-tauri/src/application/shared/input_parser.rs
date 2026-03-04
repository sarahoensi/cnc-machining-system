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

    pub fn parse<T, E>(
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

    pub fn check<E>(
        &mut self,
        field: &'static str,
        result: Result<(), E>,
    )
    where
        E: std::fmt::Display,
    {
        if let Err(e) = result {
            self.errors.push(field, "invalid_combination", e.to_string());
        }
    }

    pub fn push_error(
        &mut self,
        field: &'static str,
        code: &'static str,
        message: impl Into<String>,
    ) {
        self.errors.push(field, code, message);
    }

    pub fn finish(self) -> Result<(), ApplicationError> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(ApplicationError::Validation(self.errors))
        }
    }

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
            self.push_error(field, "invalid_geometry", e.to_string());
            None
        }
    }
}
pub fn pair<A, B>(a: Option<A>, b: Option<B>) -> Option<(A, B)> {
    match (a, b) {
        (Some(a), Some(b)) => Some((a, b)),
        _ => None,
    }
}
}