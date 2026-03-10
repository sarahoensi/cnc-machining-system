//! Frontend response DTOs for finishing commands.
//!
//! These serialized types are returned by finishing Tauri commands and are
//! treated as the external API contract for finishing UI workflows.

// interface/tauri/finishing/response.rs

use serde::Serialize;

/// UI response payload representing finishing execution state.
///
/// Frontend representation:
/// - Serialized as JSON object with stable field names.
#[derive(Serialize)]
pub struct FinishingExecutionResponse {
    /// Stable identifier for subsequent measurement registration calls.
    pub execution_id: String,
    pub active_step: Option<u32>,
    pub finished: bool,
    /// Ordered execution steps with planned and optional measured values.
    pub steps: Vec<FinishingStepResponse>,
}


/// UI response payload representing one finishing step.
///
/// Frontend representation:
/// - Serialized as JSON object.
/// - `measurement_mm` is optional until recorded.
#[derive(Serialize)]
pub struct FinishingStepResponse {
    /// Step index in workflow order.
    pub index: u32,
    /// Start diameter in millimeters (`mm`).
    pub start_mm: f64,
    /// Planned diameter delta in millimeters (`mm`).
    pub planned_delta_mm: f64,
    /// Planned resulting diameter in millimeters (`mm`).
    pub planned_end_mm: f64,
    /// Optional measured diameter in millimeters (`mm`).
    pub measurement_mm: Option<f64>,
}
