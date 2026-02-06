// domain/geometry/right_triangle/right_triangle_solver.rs

use crate::domain::geometry::geometry_error::GeometryError;
use crate::domain::geometry::right_triangle::RightTriangle;

use crate::domain::units::angle::Angle;
use crate::domain::units::length::Length;

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
        let a_mm = a.mm_value();
        let b_mm = b.mm_value();

        let c_val = (a_mm.powi(2) + b_mm.powi(2)).sqrt();
        let c = Length::mm_positive(c_val).map_err(|_| GeometryError::InvalidTriangle)?;

        let alpha = clamp_unit(a_mm / c_val).asin();
        let beta = clamp_unit(b_mm / c_val).asin();

        Self::build(a, b, c, alpha, beta)
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

        if a_mm >= c_mm - EPS {
            return Err(GeometryError::ImpossibleTriangle);
        }

        let b_val = (c_mm.powi(2) - a_mm.powi(2)).sqrt();
        let b = Length::mm_positive(b_val).map_err(|_| GeometryError::InvalidTriangle)?;

        Self::from_legs(a, b)
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

        if b_mm >= c_mm - EPS {
            return Err(GeometryError::ImpossibleTriangle);
        }

        let a_val = (c_mm.powi(2) - b_mm.powi(2)).sqrt();
        let a = Length::mm_positive(a_val).map_err(|_| GeometryError::InvalidTriangle)?;

        Self::from_legs(a, b)
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
        let alpha_rad = alpha.radians_value();

        let a_val = c_mm * alpha_rad.sin();
        let b_val = c_mm * alpha_rad.cos();

        let a = Length::mm_positive(a_val).map_err(|_| GeometryError::InvalidTriangle)?;
        let b = Length::mm_positive(b_val).map_err(|_| GeometryError::InvalidTriangle)?;

        Self::from_legs(a, b)
    }

    // ---------------------------------------------------------
    // a + alpha (leg + angle opposite that leg)
    // ---------------------------------------------------------
    pub fn from_leg_and_opposite_angle(
        a: Length,
        alpha: Angle,
    ) -> Result<RightTriangle, GeometryError> {
        validate_acute(alpha)?;

        let a_mm = a.mm_value();
        let alpha_rad = alpha.radians_value();

        let c_val = a_mm / alpha_rad.sin();
        let c = Length::mm_positive(c_val).map_err(|_| GeometryError::InvalidTriangle)?;

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
        let alpha_rad = alpha.radians_value();

        let a_val = b_mm * alpha_rad.tan();
        let a = Length::mm_positive(a_val).map_err(|_| GeometryError::InvalidTriangle)?;

        Self::from_legs(a, b)
    }

    // ---------------------------------------------------------
    // INTERNAL builder (guarantees invariants)
    // ---------------------------------------------------------
    fn build(
        a: Length,
        b: Length,
        c: Length,
        alpha_rad: f64,
        beta_rad: f64,
    ) -> Result<RightTriangle, GeometryError> {
        let alpha = Angle::radians(alpha_rad).map_err(|_| GeometryError::InvalidTriangle)?;
        let beta = Angle::radians(beta_rad).map_err(|_| GeometryError::InvalidTriangle)?;

        Ok(RightTriangle::new(a, b, c, alpha, beta))
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

fn clamp_unit(v: f64) -> f64 {
    v.clamp(-1.0, 1.0)
}
