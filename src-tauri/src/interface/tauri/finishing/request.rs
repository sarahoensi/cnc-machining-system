//! Frontend request DTOs for finishing commands.
//!
//! These deserialized types define the stable input contract consumed by Tauri
//! finishing endpoints.

// interface/tauri/finishing/request.rs

use serde::Deserialize;

//
// -----------------------------------------------------
// Generate plan
// -----------------------------------------------------

/// UI payload for `generate_finishing_plan`.
///
/// Frontend representation:
/// - Tagged enum serialized/deserialized with `type`.
///
/// Validation expectations:
/// - Diameter and planning values must satisfy domain/application constraints.
/// - Units are millimeters unless otherwise noted.
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum GenerateFinishingPlanRequest {
    /// Request planning by a fixed number of cuts.
    ByCuts {
        /// Finishing mode (`Inner` or `Outer`).
        mode: FinishingMode,
        /// Initial diameter in millimeters (`mm`).
        start_diameter_mm: f64,
        /// Target diameter in millimeters (`mm`).
        target_diameter_mm: f64,
        /// Number of planned cuts.
        cuts: u32,
    },

    /// Request planning by fixed radial engagement per step.
    ByRadialEngagement {
        /// Finishing mode (`Inner` or `Outer`).
        mode: FinishingMode,
        /// Initial diameter in millimeters (`mm`).
        start_diameter_mm: f64,
        /// Target diameter in millimeters (`mm`).
        target_diameter_mm: f64,
        /// Per-step radial engagement in millimeters (`mm`).
        radial_engagement_mm: f64,
    },
}

//
// -----------------------------------------------------
// Register measurement
// -----------------------------------------------------

/// UI payload for `register_finishing_measurement`.
///
/// Required fields:
/// - `step_number`: step index to update.
/// - `measurement_mm`: measured diameter in millimeters.
#[derive(Debug, Deserialize)]
pub struct RegisterFinishingMeasurementRequest {
    /// Step number to update.
    pub step_number: u32,

    /// Measured diameter in millimeters (`mm`).
    pub measurement_mm: f64,
}

//
// -----------------------------------------------------
// Finishing mode
// -----------------------------------------------------

/// Frontend finishing mode selector.
///
/// Frontend representation:
/// - Deserialized as enum string variant.
///
/// This enum is part of the Tauri interface contract.
#[derive(Debug, Deserialize)]
pub enum FinishingMode {
    /// Inner finishing path behavior.
    Inner,

    /// Outer finishing path behavior.
    Outer,
}
