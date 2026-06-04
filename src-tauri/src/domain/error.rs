// domain/error.rs

use thiserror::Error;

use crate::domain::{
    geometry::GeometryError, machining::finishing::FinishingError, machining::CuttingError,
    machining::CylinderWeightError, units::UnitsError,
};

/// Root error type for the entire domain layer.
///
/// All domain-level failures are wrapped in this enum.
#[derive(Debug, Error)]
pub enum DomainError {
    #[error(transparent)]
    Units(#[from] UnitsError),

    #[error(transparent)]
    Geometry(#[from] GeometryError),

    #[error(transparent)]
    MachiningPhysics(#[from] CuttingError),

    #[error(transparent)]
    MachiningStrategy(#[from] FinishingError),

    #[error(transparent)]
    CylinderWeight(#[from] CylinderWeightError),
}
