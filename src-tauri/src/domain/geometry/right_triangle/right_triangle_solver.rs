// domain/geometry/right_triangle/right_triangle_solver.rs

use crate::domain::geometry::geometry_error::GeometryError;
use crate::domain::{Angle, Length, RightTriangle};

const EPS: f64 = 1e-12;

pub struct RightTriangleSolver;

impl RightTriangleSolver {
    // ---------------------------------------------------------
    // a + b (two legs)
    // ---------------------------------------------------------
    pub fn from_legs(
        a: Length,
        b: Length,
    ) -> Result<RightTriangle, GeometryError> {

        if !is_positive_finite(a) || !is_positive_finite(b) {
            return Err(GeometryError::InvalidTriangle);
        }

        Ok(RightTriangle::new(a, b))
    }

    // ---------------------------------------------------------
    // a + c (leg + hypotenuse)
    // ---------------------------------------------------------
    pub fn from_leg_and_hypotenuse(
        a: Length,
        c: Length,
    ) -> Result<RightTriangle, GeometryError> {

        let a_mm = a.mm_value();
        let c_mm = c.mm_value();

        if !is_positive_finite(a) || !is_positive_finite(c) {
            return Err(GeometryError::InvalidTriangle);
        }

        if a_mm >= c_mm {
            return Err(GeometryError::ImpossibleTriangle);
        }

        let mut b_sq = c_mm.powi(2) - a_mm.powi(2);

        // Numerisk toleranse
        if b_sq < 0.0 && b_sq > -EPS {
            b_sq = 0.0;
        }

        if b_sq < 0.0 {
            return Err(GeometryError::ImpossibleTriangle);
        }

        let b = Length::mm_positive(b_sq.sqrt())
            .map_err(|_| GeometryError::InvalidTriangle)?;

        Ok(RightTriangle::new(a, b))
    }

    // ---------------------------------------------------------
    // b + c (other leg + hypotenuse)
    // ---------------------------------------------------------
    pub fn from_other_leg_and_hypotenuse(
        b: Length,
        c: Length,
    ) -> Result<RightTriangle, GeometryError> {

        let b_mm = b.mm_value();
        let c_mm = c.mm_value();

        if !is_positive_finite(b) || !is_positive_finite(c) {
            return Err(GeometryError::InvalidTriangle);
        }

        if b_mm >= c_mm {
            return Err(GeometryError::ImpossibleTriangle);
        }

        let mut a_sq = c_mm.powi(2) - b_mm.powi(2);

        if a_sq < 0.0 && a_sq > -EPS {
            a_sq = 0.0;
        }

        if a_sq < 0.0 {
            return Err(GeometryError::ImpossibleTriangle);
        }

        let a = Length::mm_positive(a_sq.sqrt())
            .map_err(|_| GeometryError::InvalidTriangle)?;

        Ok(RightTriangle::new(a, b))
    }

    // ---------------------------------------------------------
    // c + alpha (hypotenuse + acute angle)
    // ---------------------------------------------------------
    pub fn from_hypotenuse_and_angle(
        c: Length,
        alpha: Angle,
    ) -> Result<RightTriangle, GeometryError> {

        validate_acute(alpha)?;

        let c_mm = c.mm_value();
        let rad = alpha.radians_value();

        let a = Length::mm_positive(c_mm * rad.sin())
            .map_err(|_| GeometryError::InvalidTriangle)?;

        let b = Length::mm_positive(c_mm * rad.cos())
            .map_err(|_| GeometryError::InvalidTriangle)?;

        Ok(RightTriangle::new(a, b))
    }

    // ---------------------------------------------------------
    // a + alpha (leg opposite alpha)
    // ---------------------------------------------------------
    pub fn from_leg_and_opposite_angle(
        a: Length,
        alpha: Angle,
    ) -> Result<RightTriangle, GeometryError> {

        validate_acute(alpha)?;

        let a_mm = a.mm_value();
        let rad = alpha.radians_value();

        let s = rad.sin();

        if s.abs() < EPS {
            return Err(GeometryError::InvalidTriangle);
        }

        let c = Length::mm_positive(a_mm / s)
            .map_err(|_| GeometryError::InvalidTriangle)?;

        Self::from_leg_and_hypotenuse(a, c)
    }

    // ---------------------------------------------------------
    // b + alpha (leg adjacent to alpha)
    // ---------------------------------------------------------
    pub fn from_adjacent_leg_and_angle(
        b: Length,
        alpha: Angle,
    ) -> Result<RightTriangle, GeometryError> {

        validate_acute(alpha)?;

        let b_mm = b.mm_value();
        let rad = alpha.radians_value();

        let a = Length::mm_positive(b_mm * rad.tan())
            .map_err(|_| GeometryError::InvalidTriangle)?;

        Ok(RightTriangle::new(a, b))
    }
}

// ---------------------------------------------------------
// Helpers
// ---------------------------------------------------------

fn validate_acute(angle: Angle) -> Result<(), GeometryError> {
    let deg = angle.degrees_value();
    if deg <= 0.0 || deg >= 90.0 {
        return Err(GeometryError::InvalidTriangle);
    }
    Ok(())
}

fn is_positive_finite(length: Length) -> bool {
    let v = length.mm_value();
    v.is_finite() && v > 0.0
}

// ----------- TESTS --------

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn validate_acute_rejects_invalid() {
        assert!(validate_acute(Angle::degrees(0.0).unwrap()).is_err());
        assert!(validate_acute(Angle::degrees(90.0).unwrap()).is_err());
    }

    #[test]
    fn positive_finite_check() {
        assert!(is_positive_finite(Length::mm_positive(3.0).unwrap()));
    }
}
