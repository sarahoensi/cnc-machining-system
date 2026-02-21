//! Application error boundary for use-case execution.
//!
//! This module wraps domain and unit validation failures into a single error
//! type consumable by UI/API layers.

// src/application/shared/error.rs


use crate::application::shared::ValidationErrors;
use crate::domain::units::UnitError;
use crate::domain::GeometryError;
use crate::domain::StrategyError;

#[derive(Debug)]
/// Error type returned by application use cases.
///
/// Error origin:
/// - Variants wrap domain-level validation and strategy errors.
///
/// Exposure:
/// - Safe to expose at application boundaries after appropriate presentation
///   formatting by external layers.
pub enum ApplicationError {
    Validation (ValidationErrors),

    /// Geometry/domain-consistency failure originating from domain services.
    Geometry(GeometryError),
    /// Unit/value-object validation failure originating from domain constructors.
    Unit(UnitError),
    /// Domain strategy or planning-rule failure.
    Strategy(StrategyError),
}

impl ApplicationError {
    pub fn validation(&self) -> Option<&ValidationErrors> {
        match self {
            ApplicationError::Validation(v) => Some(v),
            _ => None,
        }
    }

    pub fn geometry(&self) -> Option<&GeometryError> {
        match self {
            ApplicationError::Geometry(e) => Some(e),
            _ => None,
        }
    }

    pub fn unit(&self) -> Option<&UnitError> {
        match self {
            ApplicationError::Unit(e) => Some(e),
            _ => None,
        }
    }

    pub fn strategy(&self) -> Option<&StrategyError> {
        match self {
            ApplicationError::Strategy(e) => Some(e),
            _ => None,
        }
    }
}

impl std::fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationError::Validation(v) => write!(f, "{}", v.message),
            ApplicationError::Geometry(e) => write!(f, "{e}"),
            ApplicationError::Unit(e) => write!(f, "{e}"),
            ApplicationError::Strategy(e) => write!(f, "{e}"),
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

impl From<StrategyError> for ApplicationError {
    fn from(err: StrategyError) -> Self {
        ApplicationError::Strategy(err)
    }
}
