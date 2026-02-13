//! Frontend request DTO for right-triangle command.
//!
//! This module defines the JSON payload contract consumed by the Tauri command
//! for machining geometry setup.

// interface/tauri/right_triangle/request.rs

use serde::Deserialize;

/// UI payload for `solve_right_triangle`.
///
/// Frontend representation:
/// - Tagged enum serialized/deserialized with `type`.
///
/// Validation expectations:
/// - Length values are expected in millimeters (`mm`) and must satisfy domain
///   constraints.
/// - Angle values are expected in degrees (`deg`).
#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum SolveRightTriangleRequest {
    /// Solve from two known legs.
    Legs {
        /// Leg `a` in millimeters (`mm`).
        a_mm: f64,
        /// Leg `b` in millimeters (`mm`).
        b_mm: f64,
    },
    /// Solve from one leg and the hypotenuse.
    LegAndHypotenuse {
        /// Known leg `a` in millimeters (`mm`).
        a_mm: f64,
        /// Hypotenuse `c` in millimeters (`mm`).
        c_mm: f64,
    },
    /// Solve from the other leg and the hypotenuse.
    OtherLegAndHypotenuse {
        /// Known leg `b` in millimeters (`mm`).
        b_mm: f64,
        /// Hypotenuse `c` in millimeters (`mm`).
        c_mm: f64,
    },
    /// Solve from hypotenuse and acute angle alpha.
    HypotenuseAndAngle {
        /// Hypotenuse `c` in millimeters (`mm`).
        c_mm: f64,
        /// Alpha angle in degrees (`deg`).
        alpha_deg: f64,
    },
}
