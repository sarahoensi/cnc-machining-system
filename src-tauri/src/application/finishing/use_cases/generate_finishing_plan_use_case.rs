use std::sync::Arc;

use crate::application::{ApplicationError, ValidationErrors};
use crate::application::finishing::finishing_execution_output::FinishingExecutionOutput;
use crate::application::finishing::generate_finishing_plan_input::GenerateFinishingPlanInput;
use crate::application::finishing::mapping::finishing_execution_mapper::to_execution_output;
use crate::application::shared::AppResult;

use crate::domain::{
    units::{Diameter, Length},
    FinishingExecution,
    FinishingExecutionId,
    FinishingExecutionRepository,
    FinishingPlanner,
    FinishingPlanning,
    FinishingRequest,
};

/// Generates and persists a new finishing execution plan.
pub struct GenerateFinishingPlanUseCase {
    repo: Arc<dyn FinishingExecutionRepository>,
}

impl GenerateFinishingPlanUseCase {
    pub fn new(repo: Arc<dyn FinishingExecutionRepository>) -> Self {
        Self { repo }
    }

    pub fn execute(&self, input: GenerateFinishingPlanInput) -> AppResult<FinishingExecutionOutput> {
        let request = Self::to_request_validated(input)?;

        let plan = FinishingPlanner::generate_plan(request)?;

        let id = FinishingExecutionId::new();
        let execution = FinishingExecution::new(id, plan)?;

        self.repo.save(execution.clone())?;

        Ok(to_execution_output(&execution))
    }

    // ---------------------------------------------------------
    // Validation + mapping
    // ---------------------------------------------------------

    fn to_request_validated(input: GenerateFinishingPlanInput) -> Result<FinishingRequest, ApplicationError> {
        let mut errors = ValidationErrors::new();

        match input {
            GenerateFinishingPlanInput::ByCuts {
                mode,
                start_diameter_mm,
                target_diameter_mm,
                cuts,
            } => {
                let start_diameter = match Diameter::mm(start_diameter_mm) {
                    Ok(v) => Some(v),
                    Err(e) => {
                        errors.push("start_diameter_mm", "invalid", e.to_string());
                        None
                    }
                };

                let target_diameter = match Diameter::mm(target_diameter_mm) {
                    Ok(v) => Some(v),
                    Err(e) => {
                        errors.push("target_diameter_mm", "invalid", e.to_string());
                        None
                    }
                };

                // (valgfritt, men nyttig) enkel app-validering på cuts
                if cuts == 0 {
                    errors.push("cuts", "non_positive", "cuts må være > 0");
                }

                if !errors.is_empty() {
                    return Err(ApplicationError::Validation(errors));
                }

                Ok(FinishingRequest {
                    mode,
                    start_diameter: start_diameter.unwrap(),
                    target_diameter: target_diameter.unwrap(),
                    planning: FinishingPlanning::ByCuts(cuts),
                })
            }

            GenerateFinishingPlanInput::ByRadialEngagement {
                mode,
                start_diameter_mm,
                target_diameter_mm,
                radial_engagement_mm,
            } => {
                let start_diameter = match Diameter::mm(start_diameter_mm) {
                    Ok(v) => Some(v),
                    Err(e) => {
                        errors.push("start_diameter_mm", "invalid", e.to_string());
                        None
                    }
                };

                let target_diameter = match Diameter::mm(target_diameter_mm) {
                    Ok(v) => Some(v),
                    Err(e) => {
                        errors.push("target_diameter_mm", "invalid", e.to_string());
                        None
                    }
                };

                let radial_engagement = match Length::mm_positive(radial_engagement_mm) {
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
                    start_diameter: start_diameter.unwrap(),
                    target_diameter: target_diameter.unwrap(),
                    planning: FinishingPlanning::ByRadialEngagement(radial_engagement.unwrap()),
                })
            }
        }
    }
}