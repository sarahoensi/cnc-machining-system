// src/application/shared/error.rs

use thiserror::Error;

use crate::application::shared::ValidationErrors;
use crate::domain::DomainError;

/// Stable error boundary for application layer.
///
/// UI/API should only depend on this type.
#[derive(Debug, Error)]
pub enum ApplicationError {
    /// Input validation failure (field-level errors).
    #[error("Validation failed")]
    Validation(ValidationErrors),

    /// Domain rule violation.
    #[error(transparent)]
    Domain(#[from] DomainError),

    /// Infrastructure failure (repository, IO, etc.)
    #[error("Infrastructure error: {0}")]
    Infrastructure(String),
}
