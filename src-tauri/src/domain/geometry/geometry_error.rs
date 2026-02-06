// domain/geometry/geometry_errors.rs

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum GeometryError {
    InvalidTriangle,
    ImpossibleTriangle,
    DivisionByZero,
    InvalidHelix,
}

impl fmt::Display for GeometryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GeometryError::InvalidTriangle => write!(f, "Invalid triangle"),
            GeometryError::ImpossibleTriangle => write!(f, "Triangle cannot exist"),
            GeometryError::DivisionByZero => write!(f, "Division by zero"),
            GeometryError::InvalidHelix => write!(f, "Invalid helix parameters"),
        }
    }
}

impl std::error::Error for GeometryError {}
