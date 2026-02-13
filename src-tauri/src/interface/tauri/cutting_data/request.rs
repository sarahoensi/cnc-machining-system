//! Frontend request DTOs for the cutting-data command.
//!
//! These types represent JSON payloads received from the UI and form a stable
//! interface contract for cutting-data solving.

// interface/tauri/cutting_data/request.rs

use serde::Deserialize;

/// UI request payload for the `solve_cutting_data` command.
///
/// Frontend representation:
/// - Deserialized from a JSON object with matching field names.
///
/// Field expectations:
/// - All fields are optional to support partial-input workflows.
/// - Numeric values are expected in machining units shown by each field name.
/// - Validation is enforced by application/domain layers.
#[derive(Default, Deserialize)]
pub struct SolveCuttingDataRequest {
    /// Cutting speed in meters per minute (`m/min`), optional.
    pub cutting_speed_m_per_min: Option<f64>,
    /// Spindle speed in revolutions per minute (`rpm`), optional.
    pub rpm: Option<f64>,
    /// Chip load in millimeters per tooth (`mm/tooth`), optional.
    pub chip_load_mm_per_tooth: Option<f64>,
    /// Feed rate in millimeters per minute (`mm/min`), optional.
    pub feed_rate_mm_per_min: Option<f64>,
    /// Tool tooth count, optional.
    pub teeth: Option<u32>,
    /// Tool diameter in millimeters (`mm`), optional.
    pub diameter_mm: Option<f64>,
}
