// domain/geometry/geometry_errors.rs

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum GeometryError {
    InvalidTriangle,
    ImpossibleTriangle,
    DivisionByZero,
    InvalidHelix,
}

impl fmt::Display for GeometryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GeometryError::InvalidTriangle => write!(f, "Invalid triangle"),
            GeometryError::ImpossibleTriangle => write!(f, "Triangle cannot exist"),
            GeometryError::DivisionByZero => write!(f, "Division by zero"),
            GeometryError::InvalidHelix => write!(f, "Invalid helix parameters"),
        }
    }
}

impl std::error::Error for GeometryError {}

// -------- TESTS -----------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_messages_are_correct() {
        assert_eq!(
            GeometryError::InvalidTriangle.to_string(),
            "Invalid triangle"
        );

        assert_eq!(
            GeometryError::ImpossibleTriangle.to_string(),
            "Triangle cannot exist"
        );

        assert_eq!(
            GeometryError::DivisionByZero.to_string(),
            "Division by zero"
        );

        assert_eq!(
            GeometryError::InvalidHelix.to_string(),
            "Invalid helix parameters"
        );
    }

    #[test]
    fn equality_works() {
        assert_eq!(
            GeometryError::InvalidTriangle,
            GeometryError::InvalidTriangle
        );

        assert_ne!(
            GeometryError::InvalidTriangle,
            GeometryError::ImpossibleTriangle
        );
    }

    #[test]
    fn implements_std_error() {
        fn assert_error<E: std::error::Error>() {}

        assert_error::<GeometryError>();
    }
}
