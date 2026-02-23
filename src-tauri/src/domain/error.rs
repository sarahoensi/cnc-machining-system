use std::fmt;

use crate::domain::{
    units::UnitsError,
    geometry::GeometryError,
    machining_physics::MachiningPhysicsError,
};

/// Root error type for the entire domain layer.
///
/// All domain-level failures are wrapped in this enum.
#[derive(Debug)]
pub enum DomainError {
    Units(UnitsError),
    Geometry(GeometryError),
    MachiningPhysics(MachiningPhysicsError),
}

//
// Automatic conversions (critical for `?` propagation)
//

impl From<UnitsError> for DomainError {
    fn from(value: UnitsError) -> Self {
        DomainError::Units(value)
    }
}

impl From<GeometryError> for DomainError {
    fn from(value: GeometryError) -> Self {
        DomainError::Geometry(value)
    }
}

impl From<MachiningPhysicsError> for DomainError {
    fn from(value: MachiningPhysicsError) -> Self {
        DomainError::MachiningPhysics(value)
    }
}


