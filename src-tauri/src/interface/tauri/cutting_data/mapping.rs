//! Mapping between Tauri cutting-data DTOs and application DTOs.
//!
//! This module translates request/response types across the interface boundary
//! while preserving field semantics and units.

// interface/tauri/cutting_data/mapping.rs

use crate::application::{SolveCuttingDataInput, SolveCuttingDataOutput};

use super::{SolveCuttingDataRequest, SolveCuttingDataResponse};

// ---------------------------------------------------------
// Request → Application Input
// ---------------------------------------------------------

impl From<SolveCuttingDataRequest> for SolveCuttingDataInput {
    fn from(req: SolveCuttingDataRequest) -> Self {
        SolveCuttingDataInput {
            cutting_speed_m_per_min: req.cutting_speed_m_per_min,
            rpm: req.rpm,
            chip_load_mm_per_tooth: req.chip_load_mm_per_tooth,
            feed_rate_mm_per_min: req.feed_rate_mm_per_min,
            teeth: req.teeth,
            diameter_mm: req.diameter_mm,
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
