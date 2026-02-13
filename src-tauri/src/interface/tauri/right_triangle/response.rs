//! Frontend response DTO for right-triangle command output.
//!
//! This serialized type defines the stable response contract returned to UI
//! clients after right-triangle solving.

// interface/tauri/right_triangle/response.rs

use serde::Serialize;

/// UI response payload for `solve_right_triangle`.
///
/// Frontend representation:
/// - Serialized as a JSON object with stable field names.
#[derive(Serialize)]
pub struct SolveRightTriangleResponse {
    /// Solved side `a` in millimeters (`mm`).
    pub a_mm: f64,
    /// Solved side `b` in millimeters (`mm`).
    pub b_mm: f64,
    /// Solved hypotenuse `c` in millimeters (`mm`).
    pub c_mm: f64,
    /// Solved alpha angle in degrees (`deg`).
    pub alpha_deg: f64,
    /// Solved beta angle in degrees (`deg`).
    pub beta_deg: f64,
}
