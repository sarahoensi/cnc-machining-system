// src/application/shared/error.rs

use std::fmt;

use crate::application::shared::ValidationErrors;

/// Stable error boundary for application layer.
///
/// Domain errors are mapped into these variants.
/// UI/API should only depend on this type.
#[derive(Debug)]
pub enum ApplicationError {

    /// Input validation failure (field-level errors).
    Validation(ValidationErrors),

    /// Domain rule violation.
    ///
    /// `code` is stable and suitable for UI logic.
    Domain {
        code: &'static str,
        message: String,
    },

    /// Infrastructure failure (repository, IO, etc.)
    Infrastructure(String),
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApplicationError::Validation(v) => write!(f, "{}", v.message),
            ApplicationError::Domain { message, .. } => write!(f, "{message}"),
            ApplicationError::Infrastructure(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for ApplicationError {}