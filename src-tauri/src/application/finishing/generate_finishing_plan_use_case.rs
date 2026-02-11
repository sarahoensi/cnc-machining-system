// application/finishing/generate_finishing_plan_use_case.rs

use crate::application::shared::AppResult;

use crate::application::finishing::dto::{
    GenerateFinishingPlanInput,
    FinishingExecutionOutput,
};

use crate::application::finishing::finishing_output_mapper::to_output;


use crate::domain::{
    Diameter,
    Length,
    FinishingExecution,
    FinishingPlanner,
    FinishingPlanning,
    FinishingRequest,
};

use crate::domain::FinishingExecutionId;
use crate::domain::FinishingExecutionRepository;

pub struct GenerateFinishingPlanUseCase<R: FinishingExecutionRepository> {
    repo: R,
}

impl<R: FinishingExecutionRepository> GenerateFinishingPlanUseCase<R> {

    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub fn execute(
        &self,
        input: GenerateFinishingPlanInput,
    ) -> AppResult<FinishingExecutionOutput> {

        let request = Self::to_request(input)?;
        let plan = FinishingPlanner::generate_plan(request)?;
        let id = FinishingExecutionId::new();
        let execution = FinishingExecution::new(id, plan)?;

        self.repo.save(execution.clone())?;

        Ok(to_output(&execution))
    }

    // ---------------------------------------------------------
    // Mapping: DTO → Domain Request
    // ---------------------------------------------------------

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

