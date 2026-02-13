//! Frontend response DTOs for the cutting-data command.
//!
//! These serialized types are returned to the UI after application use-case
//! execution and represent the external API contract.

// interface/tauri/cutting_data/response.rs

use serde::Serialize;

/// UI response payload for `solve_cutting_data`.
///
/// Frontend representation:
/// - Serialized as a JSON object with stable field names.
///
/// Field stability:
/// - Field names are part of the external API surface for the frontend.
/// - Optional values indicate unresolved outputs from partial input.
#[derive(Serialize)]
pub struct SolveCuttingDataResponse {
    /// Solved cutting speed in meters per minute (`m/min`), when available.
    pub cutting_speed_m_per_min: Option<f64>,
    /// Solved spindle speed in revolutions per minute (`rpm`), when available.
    pub rpm: Option<f64>,
    /// Solved chip load in millimeters per tooth (`mm/tooth`), when available.
    pub chip_load_mm_per_tooth: Option<f64>,
    /// Solved feed rate in millimeters per minute (`mm/min`), when available.
    pub feed_rate_mm_per_min: Option<f64>,
}
