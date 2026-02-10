// interface/tauri/cutting_data/mapping.rs

use crate::application::{
    SolveCuttingDataInput,
    SolveCuttingDataOutput,
};

use super::{
    SolveCuttingDataRequest,
    SolveCuttingDataResponse,
};

// ---------------------------------------------------------
// Request → Application Input
// ---------------------------------------------------------

impl From<SolveCuttingDataRequest> for SolveCuttingDataInput {
    fn from(req: SolveCuttingDataRequest) -> Self {

        match req {

            SolveCuttingDataRequest::FromCuttingSpeed {
                cutting_speed_m_per_min,
                diameter_mm,
                chip_load_mm_per_tooth,
                teeth,
            } => SolveCuttingDataInput::FromCuttingSpeed {
                cutting_speed_m_per_min,
                diameter_mm,
                chip_load_mm_per_tooth,
                teeth,
            },

            SolveCuttingDataRequest::FromRpm {
                rpm,
                chip_load_mm_per_tooth,
                teeth,
                diameter_mm,
            } => SolveCuttingDataInput::FromRpm {
                rpm,
                chip_load_mm_per_tooth,
                teeth,
                diameter_mm,
            },

            SolveCuttingDataRequest::FromFeedRate {
                feed_rate_mm_per_min,
                rpm,
                teeth,
                diameter_mm,
            } => SolveCuttingDataInput::FromFeedRate {
                feed_rate_mm_per_min,
                rpm,
                teeth,
                diameter_mm,
            },
        }
    }
}

// ---------------------------------------------------------
// Application Output → Response
// ---------------------------------------------------------

impl From<SolveCuttingDataOutput> for SolveCuttingDataResponse {
    fn from(out: SolveCuttingDataOutput) -> Self {

        Self {
            cutting_speed_m_per_min: out.cutting_speed_m_per_min,
            rpm: out.rpm,
            chip_load_mm_per_tooth: out.chip_load_mm_per_tooth,
            feed_rate_mm_per_min: out.feed_rate_mm_per_min,
        }
    }
}
