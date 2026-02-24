// domain/geometry/geometry_error.rs

use thiserror::Error;

use super::right_triangle::RightTriangleError;
use super::helix::HelixError;

#[derive(Debug, Error)]
pub enum GeometryError {

    #[error(transparent)]
    RightTriangle(#[from] RightTriangleError),

    #[error(transparent)]
    Helix(#[from] HelixError),
}