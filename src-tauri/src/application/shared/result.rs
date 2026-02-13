//! Shared result alias for application use cases.
//!
//! This alias standardizes success/error return signatures across the
//! application layer.

// src/application/shared/result.rs

use super::ApplicationError;

/// Standard application-layer result type.
///
/// Use cases return this alias to signal either successful workflow output or
/// an [`ApplicationError`] suitable for external handling.
pub type AppResult<T> = Result<T, ApplicationError>;
