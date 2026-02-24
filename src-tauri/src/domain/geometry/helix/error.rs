// domain/geometry/helix/error.rs

use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum HelixError {

    #[error("Tool diameter ({tool_diameter}) is larger than nominal diameter ({nominal_diameter})")]
    ToolTooLarge {
        tool_diameter: f64,
        nominal_diameter: f64,
    },

    #[error("Effective diameter must be positive (got {value})")]
    EffectiveDiameterNotPositive {
        value: f64,
    },

    #[error("Angle must be finite (got {radians})")]
    AngleNotFinite {
        radians: f64,
    },

    #[error("Angle is out of valid range (got {radians} radians)")]
    AngleOutOfRange {
        radians: f64,
    },
}