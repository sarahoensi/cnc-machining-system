// src/application/shared/error.rs

use crate::domain::GeometryError;
use crate::domain::UnitError;

#[derive(Debug)]
pub enum ApplicationError {
    Geometry(GeometryError),
    Unit(UnitError),
}

impl std::fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationError::Geometry(e) => write!(f, "{e}"),
            ApplicationError::Unit(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for ApplicationError {}

impl From<GeometryError> for ApplicationError {
    fn from(err: GeometryError) -> Self {
        ApplicationError::Geometry(err)
    }
}

impl From<UnitError> for ApplicationError {
    fn from(err: UnitError) -> Self {
        ApplicationError::Unit(err)
    }
}
