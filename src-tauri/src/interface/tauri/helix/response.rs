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

    pub pitch: f64,

    pub angle: f64,
}
