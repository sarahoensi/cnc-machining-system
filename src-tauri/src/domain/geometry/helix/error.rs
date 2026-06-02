// domain/geometry/helix/error.rs

use thiserror::Error;

use crate::domain::units::UnitsError;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum HelixError {
    #[error(transparent)]
    Unit(#[from] UnitsError),

    #[error(
        "Tool diameter ({tool_diameter}) is larger than nominal diameter ({nominal_diameter})"
    )]
    ToolTooLarge {
        tool_diameter: f64,
        nominal_diameter: f64,
    },

    #[error("Angle is out of valid range (got {radians} radians)")]
    AngleOutOfRange { radians: f64 },
}
