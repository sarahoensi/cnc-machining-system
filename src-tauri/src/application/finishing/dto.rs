// application/finishing/dto.rs

//! Finishing application DTOs.
//!
//! These types form the transport contracts used by finishing use cases.

use crate::domain::machining::finishing::{FinishingExecution, FinishingMode, FinishingStep};

//
// PLAN INPUT
//

pub enum GenerateFinishingPlanInput {
    ByCuts {
        mode: FinishingMode,
        start_diameter_mm: f64,
        target_diameter_mm: f64,
        cuts: u32,
    },

    ByRadialEngagement {
        mode: FinishingMode,
        start_diameter_mm: f64,
        target_diameter_mm: f64,
        radial_engagement_mm: f64,
    },
}

//
// EXECUTION INPUT
//

pub struct RegisterFinishingMeasurementInput {
    pub step_number: u32,
    pub measurement_mm: f64,
}

//
// EXECUTION OUTPUT
//

pub struct FinishingExecutionOutput {
    pub active_step: Option<u32>,
    pub finished: bool,
    pub steps: Vec<FinishingStepOutput>,
}

pub struct FinishingStepOutput {
    pub index: u32,
    pub start_mm: f64,
    pub planned_delta_mm: f64,
    pub planned_end_mm: f64,
    pub measurement_mm: Option<f64>,
}

//
// Domain → Application DTO mapping
//

impl From<&FinishingStep> for FinishingStepOutput {
    fn from(step: &FinishingStep) -> Self {
        Self {
            index: step.index(),
            start_mm: step.start().mm_value(),
            planned_delta_mm: step.planned_delta().mm_value(),
            planned_end_mm: step.planned_end().mm_value(),
            measurement_mm: step.measurement().map(|m| m.mm_value()),
        }
    }
}

impl From<&FinishingExecution> for FinishingExecutionOutput {
    fn from(exec: &FinishingExecution) -> Self {
        Self {
            active_step: exec.active_step(),
            finished: exec.finished(),
            steps: exec.steps().iter().map(Into::into).collect(),
        }
    }
}
