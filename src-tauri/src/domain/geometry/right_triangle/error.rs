// domain/geometry/right_triangle/error.rs

use thiserror::Error;

use crate::domain::units::UnitsError;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum RightTriangleError {
    #[error(transparent)]
    Unit(#[from] UnitsError),


    #[error("Hypotenuse ({hypotenuse}) must be greater than leg ({leg})")]
    HypotenuseTooShort {
        leg: f64,
        hypotenuse: f64,
    },

    #[error("Numerical instability detected")]
    NumericalInstability,
}