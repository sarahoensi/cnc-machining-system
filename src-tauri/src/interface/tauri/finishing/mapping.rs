// interface/tauri/finishing/mapping.rs

use crate::application::finishing::finishing_execution_output::FinishingExecutionOutput;
use crate::application::finishing::finishing_step_output::FinishingStepOutput;
use crate::application::finishing::generate_finishing_plan_input::GenerateFinishingPlanInput;
use crate::domain::FinishingMode as AppFinishingMode;

use super::request::{
    GenerateFinishingPlanRequest,
    FinishingMode as UiFinishingMode,
};

use super::response::{
    FinishingExecutionResponse,
    FinishingStepResponse,
};


// -----------------------------
// Mode mapping
// -----------------------------

impl From<UiFinishingMode> for AppFinishingMode {
    fn from(m: UiFinishingMode) -> Self {
        match m {
            UiFinishingMode::Inner => AppFinishingMode::Inner,
            UiFinishingMode::Outer => AppFinishingMode::Outer,
        }
    }
}


// -----------------------------
// Request → Application input
// -----------------------------

impl From<GenerateFinishingPlanRequest> for GenerateFinishingPlanInput {
    fn from(req: GenerateFinishingPlanRequest) -> Self {

        match req {

            GenerateFinishingPlanRequest::ByCuts {
                mode,
                start_diameter_mm,
                target_diameter_mm,
                cuts,
            } => Self::ByCuts {
                mode: mode.into(),
                start_diameter_mm,
                target_diameter_mm,
                cuts,
            },

            GenerateFinishingPlanRequest::ByRadialEngagement {
                mode,
                start_diameter_mm,
                target_diameter_mm,
                radial_engagement_mm,
            } => Self::ByRadialEngagement {
                mode: mode.into(),
                start_diameter_mm,
                target_diameter_mm,
                radial_engagement_mm,
            },
        }
    }
}


// -----------------------------
// Application output → response
// -----------------------------

impl From<FinishingExecutionOutput> for FinishingExecutionResponse {
    fn from(out: FinishingExecutionOutput) -> Self {
        Self {
            execution_id: out.execution_id,
            steps: out.steps.into_iter().map(Into::into).collect(),
        }
    }
}


impl From<FinishingStepOutput> for FinishingStepResponse {
    fn from(s: FinishingStepOutput) -> Self {
        Self {
            index: s.index,
            start_mm: s.start_mm,
            planned_delta_mm: s.planned_delta_mm,
            planned_end_mm: s.planned_end_mm,
            measurement_mm: s.measurement_mm,
        }
    }
}
