// domain/machining_strategy/finishing/aggregate/finishing_execution_repository.rs

use crate::domain::{FinishingRepositoryError,};

use super::{FinishingExecution, FinishingExecutionId};

/// Repository abstraction for storing and retrieving [`FinishingExecution`] aggregates.
///
/// # Purpose
///
/// Defines the persistence boundary for finishing executions.
/// Implementations may store executions in memory, database, file system,
/// or any external storage.
///
/// # Aggregate Boundary
///
/// The repository operates on complete [`FinishingExecution`] aggregates.
/// Partial loading or saving is not supported.
///
/// # Concurrency
///
/// Implementations must be safe for concurrent access (`Send + Sync`).
pub trait FinishingExecutionRepository:
    Send + Sync
{
    fn get(
        &self,
        id: FinishingExecutionId,
    ) -> Result<FinishingExecution, FinishingRepositoryError>;

    fn save(
        &self,
        execution: FinishingExecution,
    ) -> Result<(), FinishingRepositoryError>;
}