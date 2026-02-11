// interface/tauri/cutting_data/request.rs

use serde::Deserialize;

#[derive(Default, Deserialize)]
pub struct SolveCuttingDataRequest {
    pub cutting_speed_m_per_min: Option<f64>,
    pub rpm: Option<f64>,
    pub chip_load_mm_per_tooth: Option<f64>,
    pub feed_rate_mm_per_min: Option<f64>,
    pub teeth: Option<u32>,
    pub diameter_mm: Option<f64>,
}
