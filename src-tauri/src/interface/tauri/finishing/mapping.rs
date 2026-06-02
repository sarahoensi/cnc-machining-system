// interface/tauri/finishing/mapping.rs
//! Mapping between finishing Tauri DTOs and application DTOs.
//!
//! This module performs boundary translation for request and response models
//! without changing workflow meaning.

use crate::application::finishing::{
    FinishingExecutionOutput, FinishingStepOutput, GenerateFinishingPlanInput,
};

use crate::domain::machining::finishing::FinishingMode as AppFinishingMode;

use super::request::{FinishingMode as UiFinishingMode, GenerateFinishingPlanRequest};

use super::response::{FinishingExecutionResponse, FinishingStepResponse};

//
// -----------------------------
// Mode mapping
// -----------------------------
//

impl From<UiFinishingMode> for AppFinishingMode {
    fn from(mode: UiFinishingMode) -> Self {
        match mode {
            UiFinishingMode::Inner => AppFinishingMode::Inner,
            UiFinishingMode::Outer => AppFinishingMode::Outer,
        }
    }
}

//
// -----------------------------
// Request → Application input
// -----------------------------
//

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

//
// -----------------------------
// Application output → response
// -----------------------------
//

impl From<FinishingExecutionOutput> for FinishingExecutionResponse {
    fn from(out: FinishingExecutionOutput) -> Self {
        Self {
            active_step: out.active_step,
            finished: out.finished,
            steps: out.steps.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<FinishingStepOutput> for FinishingStepResponse {
    fn from(step: FinishingStepOutput) -> Self {
        Self {
            index: step.index,
            start_mm: step.start_mm,
            planned_delta_mm: step.planned_delta_mm,
            planned_end_mm: step.planned_end_mm,
            measurement_mm: step.measurement_mm,
        }
    }
}
