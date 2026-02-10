// src/application/shared/result.rs

use super::ApplicationError;

pub type AppResult<T> = Result<T, ApplicationError>;
