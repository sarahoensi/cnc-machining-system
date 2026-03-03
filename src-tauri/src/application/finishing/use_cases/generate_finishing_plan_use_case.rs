//! Generates and persists a new finishing execution plan.
//!
//! - Parses raw input into validated domain value objects.
//! - Delegates planning to `FinishingPlanner`.
//! - Maps `StrategyError` into field-level `ValidationErrors`.
//! - Persists execution using repository abstraction.

use std::sync::Arc;

use crate::application::{ApplicationError, ValidationErrors};
use crate::application::shared::AppResult;

use crate::application::finishing::finishing_execution_output::FinishingExecutionOutput;
use crate::application::finishing::generate_finishing_plan_input::GenerateFinishingPlanInput;
use crate::application::finishing::mapping::finishing_execution_mapper::to_execution_output;

use crate::domain::units::PositiveLength;
use crate::domain::{
    units::{Diameter},
    FinishingExecution,
    FinishingExecutionId,
    FinishingExecutionRepository,
    FinishingPlanner,
    FinishingPlanning,
    FinishingRequest,
    StrategyError,
};

pub struct GenerateFinishingPlanUseCase {
    repo: Arc<dyn FinishingExecutionRepository>,
}

impl GenerateFinishingPlanUseCase {

    pub fn new(repo: Arc<dyn FinishingExecutionRepository>) -> Self {
        Self { repo }
    }

    pub fn execute(
        &self,
        input: GenerateFinishingPlanInput,
    ) -> AppResult<FinishingExecutionOutput> {

        let request = Self::to_request_validated(input)?;

        let plan =
            FinishingPlanner::generate_plan(request)
                .map_err(map_strategy_error)?;

        let id = FinishingExecutionId::new();

        let execution =
            FinishingExecution::new(id, plan)
                .map_err(map_strategy_error)?;

        self.repo
            .save(execution.clone())
            .map_err(|e| ApplicationError::Infrastructure(e.to_string()))?;

        Ok(to_execution_output(&execution))
    }

    // ---------------------------------------------------------
    // Validation + mapping
    // ---------------------------------------------------------

    fn to_request_validated(
        input: GenerateFinishingPlanInput,
    ) -> Result<FinishingRequest, ApplicationError> {

        let mut errors = ValidationErrors::new();

        match input {

            GenerateFinishingPlanInput::ByCuts {
                mode,
                start_diameter_mm,
                target_diameter_mm,
                cuts,
            } => {

                let start =
                    parse_diameter("start_diameter_mm", start_diameter_mm, &mut errors);

                let target =
                    parse_diameter("target_diameter_mm", target_diameter_mm, &mut errors);

                if cuts == 0 {
                    errors.push("cuts", "non_positive", "Cut count must be > 0");
                }

                if !errors.is_empty() {
                    return Err(ApplicationError::Validation(errors));
                }

                Ok(FinishingRequest {
                    mode,
                    start_diameter: start.unwrap(),
                    target_diameter: target.unwrap(),
                    planning: FinishingPlanning::ByCuts(cuts),
                })
            }

            GenerateFinishingPlanInput::ByRadialEngagement {
                mode,
                start_diameter_mm,
                target_diameter_mm,
                radial_engagement_mm,
            } => {

                let start =
                    parse_diameter("start_diameter_mm", start_diameter_mm, &mut errors);

                let target =
                    parse_diameter("target_diameter_mm", target_diameter_mm, &mut errors);

                let radial = match PositiveLength::mm(radial_engagement_mm) {
                    Ok(v) => Some(v),
                    Err(e) => {
                        errors.push("radial_engagement_mm", "invalid", e.to_string());
                        None
                    }
                };

                if !errors.is_empty() {
                    return Err(ApplicationError::Validation(errors));
                }

                Ok(FinishingRequest {
                    mode,
                    start_diameter: start.unwrap(),
                    target_diameter: target.unwrap(),
                    planning: FinishingPlanning::ByRadialEngagement(radial.unwrap()),
                })
            }
        }
    }
}

fn parse_diameter(
    field: &'static str,
    raw: f64,
    v: &mut ValidationErrors,
) -> Option<Diameter> {
    match Diameter::mm(raw) {
        Ok(val) => Some(val),
        Err(e) => {
            v.push(field, "invalid", e.to_string());
            None
        }
    }
}

fn map_strategy_error(err: StrategyError) -> ApplicationError {

    let mut v = ValidationErrors::new();

    match err {

        StrategyError::InvalidModeDirection { .. } => {
            v.push("mode", "invalid_combination", err.to_string());
        }

        StrategyError::DiametersMustDiffer => {
            v.push("target_diameter_mm", "invalid_combination", err.to_string());
        }

        StrategyError::InvalidCutCount { .. } => {
            v.push("cuts", "invalid", err.to_string());
        }

        StrategyError::InvalidRadialEngagement { .. } => {
            v.push("radial_engagement_mm", "invalid", err.to_string());
        }

        StrategyError::ComputedStepNotPositive { .. }
        | StrategyError::ImpossiblePlan { .. }
        | StrategyError::DivisionByZero => {
            v.push("finishing_plan", "invalid_combination", err.to_string());
        }

        // Execution-related errors
        StrategyError::StepNumberMustBeOneBased
        | StrategyError::StepNumberOutOfRange { .. }
        | StrategyError::StepLocked { .. }
        | StrategyError::MeasurementOutOfBounds { .. }
        | StrategyError::MeasurementBackwards { .. }
        | StrategyError::MeasurementExceedsTarget { .. }
        | StrategyError::RecalculationDidNotReachTarget { .. } => {
            v.push("execution", "invalid_state", err.to_string());
        }
    }

    ApplicationError::Validation(v)
}