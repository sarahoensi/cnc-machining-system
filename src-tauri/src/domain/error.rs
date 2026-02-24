// domain/error.rs 

use thiserror::Error;

use crate::domain::{
    units::UnitsError,
    geometry::GeometryError,
    machining_physics::MachiningPhysicsError,
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
    MachiningPhysics(#[from] MachiningPhysicsError),
}