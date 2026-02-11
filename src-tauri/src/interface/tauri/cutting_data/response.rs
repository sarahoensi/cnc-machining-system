// interface/tauri/cutting_data/response.rs

use serde::Serialize;

#[derive(Serialize)]
pub struct SolveCuttingDataResponse {
    pub cutting_speed_m_per_min: Option<f64>,
    pub rpm: Option<f64>,
    pub chip_load_mm_per_tooth: Option<f64>,
    pub feed_rate_mm_per_min: Option<f64>,
}
