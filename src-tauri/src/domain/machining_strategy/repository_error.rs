// domain/machining_strategy/repository_error.rs

use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum FinishingRepositoryError {

    #[error("Finishing execution not found")]
    NotFound,

    #[error("Persistence failure")]
    PersistenceFailure,
}