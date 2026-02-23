// domain/geometry/geometry_errors.rs


use super::right_triangle::RightTriangleError;
use super::helix::HelixError;

#[derive(Debug)]
pub enum GeometryError {
    RightTriangle(RightTriangleError),
    Helix(HelixError),
}

impl From<RightTriangleError> for GeometryError {
    fn from(err: RightTriangleError) -> Self {
        GeometryError::RightTriangle(err)
    }
}

impl From<HelixError> for GeometryError {
    fn from(err: HelixError) -> Self {
        GeometryError::Helix(err)
    }
}


use std::fmt;

impl fmt::Display for GeometryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GeometryError::RightTriangle(e) => write!(f, "{e}"),
            GeometryError::Helix(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for GeometryError {}