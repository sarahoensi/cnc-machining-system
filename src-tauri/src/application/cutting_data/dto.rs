// application/cutting_data/dto.rs

/// Input where user provides a valid combination
#[derive(Default)]
pub struct SolveCuttingDataInput {
    pub cutting_speed_m_per_min: Option<f64>,
    pub rpm: Option<f64>,
    pub chip_load_mm_per_tooth: Option<f64>,
    pub feed_rate_mm_per_min: Option<f64>,
    pub teeth: Option<u32>,
    pub diameter_mm: Option<f64>,
}


pub struct SolveCuttingDataOutput {
    pub cutting_speed_m_per_min: Option<f64>,
    pub rpm: Option<f64>,
    pub chip_load_mm_per_tooth: Option<f64>,
    pub feed_rate_mm_per_min: Option<f64>,
}

