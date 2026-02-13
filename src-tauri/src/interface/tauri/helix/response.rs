//! Frontend response DTO for helix command output.
//!
//! This serialized type defines the external response contract returned to UI
//! clients for helix-solving operations.

// interface/tauri/helix/request.rs

use serde::Serialize;

/// UI response payload for `solve_helix`.
///
/// Frontend representation:
/// - Serialized as a JSON object with stable field names.
#[derive(Serialize)]
pub struct SolveHelixResponse {
    /// Effective diameter in millimeters (`mm`).
    pub effective_diameter_mm: f64,
    /// Helix pitch in millimeters per revolution (`mm/rev`).
    pub pitch_mm_per_rev: f64,
    /// Helix angle in degrees (`deg`).
    pub angle_deg: f64,
    /// Circumference in millimeters (`mm`) at effective diameter.
    pub circumference_mm: f64,
}
