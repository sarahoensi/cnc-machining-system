use crate::application::ApplicationError;
use crate::domain::{GeometryError, StrategyError};
use crate::domain::units::UnitsError;

pub fn map_geometry_error(err: GeometryError) -> ApplicationError {
    ApplicationError::Domain {
        code: "geometry_error",
        message: err.to_string(),
    }
}

pub fn map_strategy_error(err: StrategyError) -> ApplicationError {
    ApplicationError::Domain {
        code: "strategy_error",
        message: err.to_string(),
    }
}

pub fn map_unit_error(err: UnitsError) -> ApplicationError {
    ApplicationError::Domain {
        code: "unit_error",
        message: err.to_string(),
    }
}