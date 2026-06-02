// units/angle/error.rs

use crate::domain::units::core::NumericError;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum AngleError {
    #[error(transparent)]
    Numeric(#[from] NumericError),

    #[error("Angle must be acute (0° < θ < 90°), got {0}°")]
    NotAcute(f64),
}
