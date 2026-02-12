// domain/machining_strategy/finishing/aggregate/finishing_execution_repository.rs

use crate::domain::machining_strategy::strategy_error::StrategyError;

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
pub trait FinishingExecutionRepository: Send + Sync {

    /// Retrieves a finishing execution by its identifier.
    ///
    /// Returns an error if the execution does not exist or
    /// cannot be retrieved.
    fn get(
        &self,
        id: FinishingExecutionId,
    ) -> Result<FinishingExecution, StrategyError>;

    /// Persists a finishing execution.
    ///
    /// Implementations should either:
    ///
    /// - Insert a new execution
    /// - Update an existing execution
    fn save(
        &self,
        execution: FinishingExecution,
    ) -> Result<(), StrategyError>;
}
