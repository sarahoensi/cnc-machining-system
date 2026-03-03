// domain/error.rs 

use thiserror::Error;

use crate::domain::{
    machining_strategy::StrategyError, 
    geometry::GeometryError, 
    machining_physics::MachiningPhysicsError, 
    units::UnitsError
};

/// Root error type for the entire domain layer.
///
/// All domain-level failures are wrapped in this enum.
#[derive(Debug, Error, Clone, PartialEq)]
pub enum DomainError {

    #[error(transparent)]
    Units(#[from] UnitsError),

    #[error(transparent)]
    Geometry(#[from] GeometryError),

    #[error(transparent)]
    MachiningPhysics(#[from] MachiningPhysicsError),

    #[error(transparent)]
    MachiningStrategy(#[from] StrategyError),
}