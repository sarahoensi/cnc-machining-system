// domain/geometry/geometry_error.rs

use thiserror::Error;

use crate::domain::units::UnitsError;

use super::right_triangle::RightTriangleError;
use super::helix::HelixError;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum GeometryError {
    #[error(transparent)]
    Units(#[from] UnitsError),

    #[error(transparent)]
    RightTriangle(#[from] RightTriangleError),

    #[error(transparent)]
    Helix(#[from] HelixError),

    
}
