// interface/tauri/helix/request.rs

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum SolveCuttingDataRequest {

    FromCuttingSpeed {
        cutting_speed_m_per_min: f64,
        diameter_mm: f64,
        chip_load_mm_per_tooth: f64,
        teeth: u32,
    },

    FromRpm {
        rpm: f64,
        chip_load_mm_per_tooth: f64,
        teeth: u32,
        diameter_mm: f64,
    },

    FromFeedRate {
        feed_rate_mm_per_min: f64,
        rpm: f64,
        teeth: u32,
        diameter_mm: f64,
    },
}
