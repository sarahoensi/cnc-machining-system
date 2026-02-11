// application/finishing/generate_finishing_plan_use_case.rs
use crate::application::finishing::finishing_execution_output::FinishingExecutionOutput;
use crate::application::finishing::generate_finishing_plan_input::GenerateFinishingPlanInput;
use crate::application::shared::AppResult;

use crate::application::finishing::finishing_output_mapper::to_output;

use crate::domain::{
    Diameter,
    Length,
    FinishingExecution,
    FinishingPlanner,
    FinishingPlanning,
    FinishingRequest,
    FinishingExecutionId,
    FinishingExecutionRepository,
};

use std::sync::Arc;


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

        let request = Self::to_request(input)?;

        let plan = FinishingPlanner::generate_plan(request)?;

        let id = FinishingExecutionId::new();

        let execution = FinishingExecution::new(id, plan)?;

        self.repo.save(execution.clone())?;

        Ok(to_output(&execution))
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
