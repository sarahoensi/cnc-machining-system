
//! Frontend response DTOs for finishing commands.
//!
//! These types define the serialized response contract returned
//! by Tauri finishing commands.

use serde::Serialize;

//
// -----------------------------------------------------
// Execution response
// -----------------------------------------------------

/// Response payload describing the current finishing execution state.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FinishingExecutionResponse {

    /// Index of the next step expecting a measurement.
    pub active_step: Option<u32>,

    /// Indicates whether all finishing steps have been completed.
    pub finished: bool,

    /// Ordered list of finishing steps.
    pub steps: Vec<FinishingStepResponse>,
}

//
// -----------------------------------------------------
// Step response
// -----------------------------------------------------

/// Response payload representing a single finishing step.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FinishingStepResponse {

    /// Step index (1-based).
    pub index: u32,

    /// Diameter before the step.
    pub start_mm: f64,

    /// Planned diameter change.
    pub planned_delta_mm: f64,

    /// Planned resulting diameter.
    pub planned_end_mm: f64,

    /// Optional operator measurement.
    pub measurement_mm: Option<f64>,
}