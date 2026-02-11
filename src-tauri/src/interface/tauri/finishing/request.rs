// interface/tauri/finishing/request.rs

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum GenerateFinishingPlanRequest {

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

#[derive(Deserialize)]
pub struct RegisterFinishingMeasurementRequest {
    pub execution_id: String,
    pub step_number: u32,
    pub measurement_mm: f64,
}

#[derive(Deserialize)]
pub enum FinishingMode {
    Inner,
    Outer,
}
