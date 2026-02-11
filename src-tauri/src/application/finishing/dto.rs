// application/finishing/dto.rs

use crate::domain::FinishingMode;

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

pub struct FinishingStepOutput {
    pub index: u32,
    pub start_mm: f64,
    pub planned_delta_mm: f64,
    pub planned_end_mm: f64,
    pub measurement_mm: Option<f64>,
}

pub struct FinishingExecutionOutput {
    pub steps: Vec<FinishingStepOutput>,
}
