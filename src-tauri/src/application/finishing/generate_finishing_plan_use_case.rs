// application/finishing/generate_finishing_plan_use_case.rs

use crate::application::shared::AppResult;

use crate::application::finishing::dto::{
    GenerateFinishingPlanInput,
    FinishingExecutionOutput,
    FinishingStepOutput,
};

use crate::domain::{
    Diameter,
    Length,
    FinishingExecution,
    FinishingPlanner,
    FinishingPlanning,
    FinishingRequest,
};

pub struct GenerateFinishingPlanUseCase;

impl GenerateFinishingPlanUseCase {

    pub fn execute(
        &self,
        input: GenerateFinishingPlanInput,
    ) -> AppResult<FinishingExecutionOutput> {

        let request = Self::to_request(input)?;
        let plan = FinishingPlanner::generate_plan(request)?;
        let execution = FinishingExecution::new(plan)?;

        Ok(Self::to_output(&execution))
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

    // ---------------------------------------------------------
    // Mapping: Domain Execution → DTO Output
    // ---------------------------------------------------------

    pub(crate) fn to_output(
        exec: &FinishingExecution,
    ) -> FinishingExecutionOutput {

        let steps = exec
            .steps()
            .iter()
            .map(|s| FinishingStepOutput {
                index: s.index(),
                start_mm: s.start().mm_value(),
                planned_delta_mm: s.planned_delta().mm_value(),
                planned_end_mm: s.planned_end().mm_value(),
                measurement_mm: s.measurement().map(|m| m.mm_value()),
            })
            .collect();

        FinishingExecutionOutput { steps }
    }
}

