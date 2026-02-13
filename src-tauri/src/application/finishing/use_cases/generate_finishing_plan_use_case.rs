//! Use case for starting a finishing execution lifecycle.
//!
//! The workflow builds a domain finishing request, delegates planning to the
//! domain planner, creates an execution aggregate, persists it, and returns an
//! application-facing execution snapshot.

// application/finishing/generate_finishing_plan_use_case.rs
use crate::application::finishing::finishing_execution_output::FinishingExecutionOutput;
use crate::application::finishing::generate_finishing_plan_input::GenerateFinishingPlanInput;
use crate::application::shared::AppResult;

use crate::application::finishing::mapping::finishing_execution_mapper::to_execution_output;


use crate::domain::{
    units::{Diameter, Length},
    FinishingExecution,
    FinishingPlanner,
    FinishingPlanning,
    FinishingRequest,
    FinishingExecutionId,
    FinishingExecutionRepository,
};

use std::sync::Arc;


/// Generates and persists a new finishing execution plan.
///
/// This use case orchestrates the start of a finishing workflow from operator
/// planning input.
pub struct GenerateFinishingPlanUseCase {
    repo: Arc<dyn FinishingExecutionRepository>,
}


impl GenerateFinishingPlanUseCase {

    /// Creates the use case with a finishing execution repository dependency.
    pub fn new(repo: Arc<dyn FinishingExecutionRepository>) -> Self {
        Self { repo }
    }


    /// Generates a finishing plan and opens a new execution lifecycle.
    ///
    /// Purpose:
    /// - Converts application input into a domain finishing request.
    /// - Delegates step planning to the domain planner.
    /// - Creates and persists a new finishing execution aggregate.
    ///
    /// Required inputs:
    /// - A [`GenerateFinishingPlanInput`] variant with valid diameter/planning
    ///   values in millimeters.
    ///
    /// Output meaning:
    /// - Returns a [`FinishingExecutionOutput`] snapshot with execution ID and
    ///   planned steps for downstream measurement registration.
    ///
    /// Domain invariants enforced:
    /// - Diameter, radial engagement, and planning constraints are validated by
    ///   domain value objects and planner rules.
    ///
    /// Side effects:
    /// - Persists a newly created finishing execution in the repository.
    ///
    /// Error scenarios:
    /// - Invalid input values rejected by domain constructors.
    /// - Planning failures produced by domain finishing strategy rules.
    /// - Repository save failures when persisting execution state.
    pub fn execute(
        &self,
        input: GenerateFinishingPlanInput,
    ) -> AppResult<FinishingExecutionOutput> {

        let request = Self::to_request(input)?;

        let plan = FinishingPlanner::generate_plan(request)?;

        let id = FinishingExecutionId::new();

        let execution = FinishingExecution::new(id, plan)?;

        self.repo.save(execution.clone())?;

        Ok(to_execution_output(&execution))
    }


    fn to_request(
        input: GenerateFinishingPlanInput,
    ) -> AppResult<FinishingRequest> {

        match input {

            GenerateFinishingPlanInput::ByCuts {
                mode,
                start_diameter_mm,
                target_diameter_mm,
                cuts,
            } => {
                Ok(FinishingRequest {
                    mode,
                    start_diameter: Diameter::mm(start_diameter_mm)?,
                    target_diameter: Diameter::mm(target_diameter_mm)?,
                    planning: FinishingPlanning::ByCuts(cuts),
                })
            }

            GenerateFinishingPlanInput::ByRadialEngagement {
                mode,
                start_diameter_mm,
                target_diameter_mm,
                radial_engagement_mm,
            } => {
                Ok(FinishingRequest {
                    mode,
                    start_diameter: Diameter::mm(start_diameter_mm)?,
                    target_diameter: Diameter::mm(target_diameter_mm)?,
                    planning: FinishingPlanning::ByRadialEngagement(
                        Length::mm_positive(radial_engagement_mm)?
                    ),
                })
            }
        }
    }
}
