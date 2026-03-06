//! Frontend request DTOs for the helix command.
//!
//! These deserialized types define the external input contract for helix
//! solving in machining workflows.

// interface/tauri/helix/request.rs

use serde::Deserialize;

/// UI payload for `solve_helix`.
///
/// Frontend representation:
/// - Tagged enum serialized/deserialized with `type`.
///
/// Validation expectations:
/// - Diameter values are expected in millimeters.
/// - Pitch is expected in `mm/rev`.
/// - Angle is expected in degrees.
#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum SolveHelixRequest {

    /// Solve helix values when pitch is known.
    Pitch {
        /// Path mode controlling effective-diameter offset direction.
        mode: HelixMode,
        /// Nominal diameter in millimeters (`mm`).
        diameter: f64,
        /// Tool diameter in millimeters (`mm`).
        tool_diameter: f64,
        /// Pitch in millimeters per revolution (`mm/rev`).
        pitch: f64,
    },

    /// Solve helix values when angle is known.
    Angle {
        /// Path mode controlling effective-diameter offset direction.
        mode: HelixMode,
        /// Nominal diameter in millimeters (`mm`).
        diameter: f64,
        /// Tool diameter in millimeters (`mm`).
        tool_diameter: f64,
        /// Helix angle in degrees (`deg`).
        angle: f64,
    },
}

/// UI mode for helix effective-diameter interpretation.
///
/// This enum is part of the stable command input surface.
#[derive(Deserialize)]
pub enum HelixMode {
    /// Inner path interpretation.
    Inner,
    /// Outer path interpretation.
    Outer,
}
