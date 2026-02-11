// interface/tauri/helix/request.rs

use serde::Serialize;

#[derive(Serialize)]
pub struct SolveHelixResponse {
    pub effective_diameter_mm: f64,
    pub pitch_mm_per_rev: f64,
    pub angle_deg: f64,
    pub circumference_mm: f64,
}
