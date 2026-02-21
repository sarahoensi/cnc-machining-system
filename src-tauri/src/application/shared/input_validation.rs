use crate::application::shared::validation::ValidationErrors;
use crate::domain::units::{Angle, Length, UnitError};

/// Maps a UnitError to a stable validation code.
/// UI kan senere bruke `code` for lokalisering.
fn unit_error_code(err: &UnitError) -> &'static str {
    match err {
        UnitError::NotFinite(_) => "not_finite",
        UnitError::NonPositiveValue(_) => "non_positive",
        UnitError::NegativeValue(_) => "negative",
        UnitError::OutOfRange { .. } => "out_of_range",
    }
}

/// Validates and constructs a positive Length from raw mm input.
/// Pushes field error if invalid.
pub fn validate_length_mm_positive(
    field: &'static str,
    raw: f64,
    v: &mut ValidationErrors,
) -> Option<Length> {
    match Length::mm_positive(raw) {
        Ok(val) => Some(val),
        Err(e) => {
            v.push(field, unit_error_code(&e), e.to_string());
            None
        }
    }
}

/// Validates and constructs a Length (>= 0 allowed)
#[allow(dead_code)]
pub fn validate_length_mm(
    field: &'static str,
    raw: f64,
    v: &mut ValidationErrors,
) -> Option<Length> {
    match Length::mm(raw) {
        Ok(val) => Some(val),
        Err(e) => {
            v.push(field, unit_error_code(&e), e.to_string());
            None
        }
    }
}

/// Validates and constructs an Angle in degrees.
pub fn validate_angle_degrees(
    field: &'static str,
    raw: f64,
    v: &mut ValidationErrors,
) -> Option<Angle> {
    match Angle::degrees(raw) {
        Ok(val) => Some(val),
        Err(e) => {
            v.push(field, unit_error_code(&e), e.to_string());
            None
        }
    }
}

/// Ensures angle is acute (0 < angle < 90).
/// Use when domain requires right-triangle semantics.
pub fn ensure_acute_angle(
    field: &'static str,
    angle: Angle,
    v: &mut ValidationErrors,
) -> Option<Angle> {
    let deg = angle.degrees_value();

    if deg <= 0.0 || deg >= 90.0 {
        v.push(
            field,
            "out_of_range",
            "Vinkel må være mellom 0 og 90",
        );
        None
    } else {
        Some(angle)
    }
}