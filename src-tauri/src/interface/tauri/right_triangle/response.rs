// interface/tauri/right_triangle/response.rs

use serde::Serialize;

#[derive(Serialize)]
pub struct SolveRightTriangleResponse {
    pub a_mm: f64,
    pub b_mm: f64,
    pub c_mm: f64,
    pub alpha_deg: f64,
    pub beta_deg: f64,
}
