// domain/machining_strategy/finishing/aggregate/finishing_execution_repository.rs

use crate::domain::machining_strategy::strategy_error::StrategyError;

use super::{FinishingExecution, FinishingExecutionId};

pub trait FinishingExecutionRepository {

    fn get(
        &self,
        id: FinishingExecutionId,
    ) -> Result<FinishingExecution, StrategyError>;

    fn save(
        &self,
        execution: FinishingExecution,
    ) -> Result<(), StrategyError>;
}
