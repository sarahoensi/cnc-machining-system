// domain/geometry/helix/error.rs

#[derive(Debug, Clone, PartialEq)]
pub enum HelixError {
    ToolTooLarge {
        tool_diameter: f64,
        nominal_diameter: f64,
    },

    EffectiveDiameterNotPositive {
        value: f64,
    },

    AngleNotFinite {
        radians: f64,
    },

    AngleOutOfRange {
        radians: f64,
    },
}

use std::fmt;

impl fmt::Display for HelixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for HelixError {}