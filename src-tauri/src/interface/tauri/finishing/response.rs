// interface/tauri/finishing/response.rs

use serde::Serialize;

#[derive(Serialize)]
pub struct FinishingExecutionResponse {
    pub execution_id: String,
    pub steps: Vec<FinishingStepResponse>,
}


#[derive(Serialize)]
pub struct FinishingStepResponse {
    pub index: u32,
    pub start_mm: f64,
    pub planned_delta_mm: f64,
    pub planned_end_mm: f64,
    pub measurement_mm: Option<f64>,
}
