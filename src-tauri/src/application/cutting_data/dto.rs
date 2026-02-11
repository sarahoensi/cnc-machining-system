// application/cutting_data/dto.rs

/// Input where user provides a valid combination
pub enum SolveCuttingDataInput {

    /// Known: Vc + Diameter → RPM
    /// Known: Chip load → Feed rate
    FromCuttingSpeed {
        cutting_speed_m_per_min: f64,
        diameter_mm: f64,
        chip_load_mm_per_tooth: f64,
        teeth: u32,
    },

    /// Known: RPM directly
    /// Known: Chip load → Feed rate
    FromRpm {
        rpm: f64,
        chip_load_mm_per_tooth: f64,
        teeth: u32,
        diameter_mm: f64,
    },

    /// Known: Feed rate → Chip load
    FromFeedRate {
        feed_rate_mm_per_min: f64,
        rpm: f64,
        teeth: u32,
        diameter_mm: f64,
    },
}

pub struct SolveCuttingDataOutput {
    pub cutting_speed_m_per_min: f64,
    pub rpm: f64,
    pub chip_load_mm_per_tooth: f64,
    pub feed_rate_mm_per_min: f64,
}
