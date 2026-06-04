//! Application DTOs for cutting-data solving workflows.
//!
//! These types transport operator-supplied and computed machining parameters
//! between external layers and the cutting-data use case.

// application/cutting_data/dto.rs

/// Input where user provides a valid combination

/// Input DTO for cutting-data completion.
///
/// This type carries optional known machining values. The use case attempts to
/// derive missing values from valid combinations.
///
/// Validation expectations:
/// - Numeric values must satisfy domain constraints (for example positive units).
/// - `teeth` is required only for feed/chip-load relationships.
///
/// Unit expectations:
/// - Speeds in meters per minute and revolutions per minute.
/// - Length-related rates in millimeters-based units.
#[derive(Default)]
pub struct SolveCuttingDataInput {
    /// Cutting speed in meters per minute (`m/min`).
    pub cutting_speed_m_per_min: Option<f64>,
    /// Spindle speed in revolutions per minute (`rpm`).
    pub rpm: Option<f64>,
    /// Chip load in millimeters per tooth (`mm/tooth`).
    pub chip_load_mm_per_tooth: Option<f64>,
    /// Feed rate in millimeters per minute (`mm/min`).
    pub feed_rate_mm_per_min: Option<f64>,
    /// Tooth count for the active tool.
    pub teeth: Option<i32>,
    /// Tool diameter in millimeters (`mm`).
    pub diameter_mm: Option<f64>,
}

/// Output DTO containing the solved cutting-data state.
///
/// This output returns all values that could be validated or derived from the
/// provided input combination. Fields remain `None` when there is not enough
/// information to compute them safely.
pub struct SolveCuttingDataOutput {
    /// Resolved cutting speed in meters per minute (`m/min`), when available.
    pub cutting_speed_m_per_min: Option<f64>,
    /// Resolved spindle speed in revolutions per minute (`rpm`), when available.
    pub rpm: Option<f64>,
    /// Resolved chip load in millimeters per tooth (`mm/tooth`), when available.
    pub chip_load_mm_per_tooth: Option<f64>,
    /// Resolved feed rate in millimeters per minute (`mm/min`), when available.
    pub feed_rate_mm_per_min: Option<f64>,
}
