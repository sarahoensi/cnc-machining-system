// domain/geometry/helix/helix_angle.rs

use std::f64::consts::PI;

use crate::domain::Angle;
use crate::domain::geometry::GeometryError;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HelixAngle(Angle);

impl HelixAngle {

    pub fn new(angle: Angle) -> Result<Self, GeometryError> {

        let rad = angle.radians_value();

        if !rad.is_finite() {
            return Err(GeometryError::NotFinite);
        }

        if rad <= 0.0 || rad >= PI / 2.0 {
            return Err(GeometryError::OutOfRange);
        }

        Ok(Self(angle))
    }

    pub fn angle(self) -> Angle {
        self.0
    }

    pub fn radians_value(self) -> f64 {
        self.0.radians_value()
    }

    pub fn degrees_value(self) -> f64 {
        self.0.degrees_value()
    }
}

// ---------------- TESTS ----------------

#[cfg(test)]
mod tests {

    use super::*;
    use crate::test_utils::approx::{approx_eq, DEFAULT_EPS};

    #[test]
    fn accepts_valid_angle() {

        let angle = Angle::degrees(30.0).unwrap();

        let result = HelixAngle::new(angle);

        assert!(result.is_ok());
    }

    #[test]
    fn rejects_zero_angle() {

        let angle = Angle::degrees(0.0).unwrap();

        let result = HelixAngle::new(angle);

        assert!(matches!(result, Err(GeometryError::OutOfRange)));
    }

    #[test]
    fn rejects_ninety_degrees() {

        let angle = Angle::degrees(90.0).unwrap();

        let result = HelixAngle::new(angle);

        assert!(matches!(result, Err(GeometryError::OutOfRange)));
    }

    #[test]
    fn rejects_above_ninety() {

        let angle = Angle::degrees(120.0).unwrap();

        let result = HelixAngle::new(angle);

        assert!(matches!(result, Err(GeometryError::OutOfRange)));
    }

    #[test]
    fn rejects_negative_angle() {

        let angle = Angle::degrees(-10.0).unwrap();

        let result = HelixAngle::new(angle);

        assert!(matches!(result, Err(GeometryError::OutOfRange)));
    }

    #[test]
    fn preserves_original_angle_value() {

        let angle = Angle::degrees(42.5).unwrap();

        let helix_angle = HelixAngle::new(angle).unwrap();

        assert!(approx_eq(
            helix_angle.degrees_value(),
            42.5,
            DEFAULT_EPS
        ));
    }

    #[test]
    fn radians_and_degrees_are_consistent() {

        let angle = Angle::degrees(25.0).unwrap();

        let helix_angle = HelixAngle::new(angle).unwrap();

        let reconstructed = helix_angle.radians_value().to_degrees();

        assert!(approx_eq(
            reconstructed,
            helix_angle.degrees_value(),
            DEFAULT_EPS
        ));
    }
}
