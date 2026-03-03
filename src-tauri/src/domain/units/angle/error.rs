// units/angle/error.rs

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum AngleError {
    #[error("Angle must be finite, got {value}")]
    NotFinite {
        value: f64,
    },

    #[error("Angle must be acute (0° < θ < 90°), got {degrees}°")]
    NotAcute {
        degrees: f64,
    },
}