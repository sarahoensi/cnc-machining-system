// domain/units/error.rs

use std::fmt;

/// Common validation errors for unit value objects.
#[derive(Debug, Clone, Copy, PartialEq,)]
pub enum UnitError {
    NotFinite(&'static str),
    NonPositiveValue(&'static str),
    NegativeValue(&'static str),
    OutOfRange {
        ty: &'static str,
        min: f64,
        max: f64,
        actual: f64,
    },
}

impl fmt::Display for UnitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnitError::NotFinite(ty) => write!(f, "{ty} must be finite"),
            UnitError::NonPositiveValue(ty) => write!(f, "{ty} must be > 0"),
            UnitError::NegativeValue(ty) => write!(f, "{ty} must be >= 0"),
            UnitError::OutOfRange { ty, min, max, actual } => {
                write!(f, "{ty} out of range [{min}, {max}]: {actual}")
            }
        }
    }
}

impl std::error::Error for UnitError {}

// -------- TESTS -----------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_not_finite() {
        let err = UnitError::NotFinite("Angle");
        assert_eq!(err.to_string(), "Angle must be finite");
    }

    #[test]
    fn display_non_positive() {
        let err = UnitError::NonPositiveValue("Length");
        assert_eq!(err.to_string(), "Length must be > 0");
    }

    #[test]
    fn display_negative_value() {
        let err = UnitError::NegativeValue("Length");
        assert_eq!(err.to_string(), "Length must be >= 0");
    }

    #[test]
    fn display_out_of_range() {
        let err = UnitError::OutOfRange {
            ty: "Percentage",
            min: 0.0,
            max: 100.0,
            actual: 120.0,
        };

        assert_eq!(
            err.to_string(),
            "Percentage out of range [0, 100]: 120"
        );
    }

    #[test]
    fn equality_works() {
        assert_eq!(
            UnitError::NotFinite("Angle"),
            UnitError::NotFinite("Angle")
        );

        assert_ne!(
            UnitError::NotFinite("Angle"),
            UnitError::NotFinite("Length")
        );
    }

    #[test]
    fn implements_std_error() {
        fn assert_error<E: std::error::Error>() {}
        assert_error::<UnitError>();
    }
}
