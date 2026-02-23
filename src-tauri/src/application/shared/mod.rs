//! Shared application-layer error and result contracts.
//!
//! These types provide a stable boundary for use cases to surface validation
//! and domain-originated failures to external interfaces.

mod error;
mod result;
mod validation;
mod domain_error_mapper;

pub use error::ApplicationError;
pub use result::AppResult;
pub use validation::{ValidationErrors};

pub use domain_error_mapper::*;