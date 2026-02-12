// domain/geometry/geometry_errors.rs
//! Geometry errors for validated domain primitives.
//!
//! This module defines `GeometryError`, the canonical set of validation and
//! constraint failures returned by geometry constructors and solvers. Errors map
//! to physical and mathematical validation concerns encountered during machining
//! calculations, such as invalid triangle dimensions, non-finite values, and
//! out-of-range parameters.
//!
//! These errors are intended to be handled by callers to provide actionable
//! feedback when inputs violate domain invariants.

use std::fmt;

/// Errors that can occur when constructing or solving geometry values.
///
/// This enum enumerates high-level validation failures produced by geometry
/// types (triangles, helices, circles) when inputs violate domain invariants.
/// Callers should match on variants to interpret failures in machining contexts.
#[derive(Debug, Clone, PartialEq)]
pub enum GeometryError {
    /// Input values do not satisfy basic triangle requirements.
    ///
    /// Examples: non-positive side lengths or degenerate side combinations.
    InvalidTriangle,

    /// Triangle parameters violate the triangle inequality and cannot form a triangle.
    ///
    /// Invariant: for a valid triangle, each side must be less than the sum of the others.
    ImpossibleTriangle,

    /// An operation would require division by zero.
    ///
    /// Indicates an invalid configuration where a denominator evaluates to zero
    /// in geometric formulas used by machining calculations.
    DivisionByZero,

    /// Helix parameters are not consistent with a valid helix.
    ///
    /// Examples: non-finite pitch or an invalid pitch-to-radius relationship for the
    /// expected physical model.
    InvalidHelix,

    /// A numeric value was not finite (NaN or infinite).
    ///
    /// Invariant: geometry inputs and intermediate results are expected to be finite.
    NotFinite,

    /// A value lies outside the acceptable physical or mathematical range.
    ///
    /// Use for domain limits such as angle ranges, realistic machining dimensions,
    /// or other constrained parameters.
    OutOfRange,
}

impl fmt::Display for GeometryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GeometryError::InvalidTriangle => write!(f, "Invalid triangle"),
            GeometryError::ImpossibleTriangle => write!(f, "Triangle cannot exist"),
            GeometryError::DivisionByZero => write!(f, "Division by zero"),
            GeometryError::InvalidHelix => write!(f, "Invalid helix parameters"),
            GeometryError::NotFinite => write!(f, "Value is not finite"),
            GeometryError::OutOfRange => write!(f, "Value is out of valid range"),
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

        assert_eq!(
            GeometryError::NotFinite.to_string(),
            "Value is not finite"
        );
        assert_eq!(
            GeometryError::OutOfRange.to_string(),
            "Value is out of valid range"
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
