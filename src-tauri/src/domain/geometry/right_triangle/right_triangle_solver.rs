//domain/right_triangle/right_triangle_solver.rs

use crate::domain::{
    GeometryError,
    geometry::right_triangle::{RightTriangle, RightTriangleError},
    units::{AcuteAngle, PositiveLength},
};

const EPS: f64 = 1e-12;

/// Provides validated construction routines for `RightTriangle`.
pub struct RightTriangleSolver;

impl RightTriangleSolver {

    // ---------------------------------------------------------
    // a + b
    // ---------------------------------------------------------

    pub fn from_legs(
    a: PositiveLength,
    b: PositiveLength,
) -> Result<RightTriangle, GeometryError> {

    let a_mm = a.mm_value();
    let b_mm = b.mm_value();

    if a_mm <= 0.0 {
        return Err(
            RightTriangleError::LegNotPositive { value: a_mm }.into()
        );
    }

    if b_mm <= 0.0 {
        return Err(
            RightTriangleError::LegNotPositive { value: b_mm }.into()
        );
    }

    Ok(RightTriangle::new(a, b))
}

    // ---------------------------------------------------------
    // a + c
    // ---------------------------------------------------------

    pub fn from_leg_and_hypotenuse(
        a: PositiveLength,
        c: PositiveLength,
    ) -> Result<RightTriangle, GeometryError> {

        let a_mm = a.mm_value();
        let c_mm = c.mm_value();

        if a_mm >= c_mm {
            return Err(
                RightTriangleError::HypotenuseTooShort {
                    leg: a_mm,
                    hypotenuse: c_mm,
                }
                .into(),
            );
        }

        let b_sq = c_mm.powi(2) - a_mm.powi(2);
        let b_val = safe_sqrt(b_sq)?;

        let b = PositiveLength::mm(b_val)
            .map_err(|_| RightTriangleError::LegNotPositive { value: b_val })?;

        Ok(RightTriangle::new(a, b))
    }

    // ---------------------------------------------------------
    // b + c
    // ---------------------------------------------------------

    pub fn from_other_leg_and_hypotenuse(
        b: PositiveLength,
        c: PositiveLength,
    ) -> Result<RightTriangle, GeometryError> {

        let b_mm = b.mm_value();
        let c_mm = c.mm_value();

        if b_mm >= c_mm {
            return Err(
                RightTriangleError::HypotenuseTooShort {
                    leg: b_mm,
                    hypotenuse: c_mm,
                }
                .into(),
            );
        }

        let a_sq = c_mm.powi(2) - b_mm.powi(2);
        let a_val = safe_sqrt(a_sq)?;

        let a = PositiveLength::mm(a_val)
            .map_err(|_| RightTriangleError::LegNotPositive { value: a_val })?;

        Ok(RightTriangle::new(a, b))
    }

    // ---------------------------------------------------------
    // c + alpha
    // ---------------------------------------------------------

    pub fn from_hypotenuse_and_angle(
        c: PositiveLength,
        alpha: AcuteAngle,
    ) -> Result<RightTriangle, GeometryError> {

        

        let c_mm = c.mm_value();
        let rad = alpha.radians_value();

        let a_val = c_mm * rad.sin();
        let b_val = c_mm * rad.cos();

        let a = PositiveLength::mm(a_val)
            .map_err(|_| RightTriangleError::LegNotPositive { value: a_val })?;

        let b = PositiveLength::mm(b_val)
            .map_err(|_| RightTriangleError::LegNotPositive { value: b_val })?;

        Ok(RightTriangle::new(a, b))
    }

    // ---------------------------------------------------------
    // a + alpha
    // ---------------------------------------------------------

    pub fn from_leg_and_opposite_angle(
        a: PositiveLength,
        alpha: AcuteAngle,
    ) -> Result<RightTriangle, GeometryError> {


        let a_mm = a.mm_value();
        let s = alpha.radians_value().sin();

        if s.abs() < EPS {
            return Err(RightTriangleError::DivisionByZero.into());
        }

        let c_val = a_mm / s;

        let c = PositiveLength::mm(c_val)
            .map_err(|_| RightTriangleError::HypotenuseNotPositive { value: c_val })?;

        Self::from_leg_and_hypotenuse(a, c)
    }

    // ---------------------------------------------------------
    // b + alpha
    // ---------------------------------------------------------

    pub fn from_adjacent_leg_and_angle(
        b: PositiveLength,
        alpha: AcuteAngle,
    ) -> Result<RightTriangle, GeometryError> {


        let b_mm = b.mm_value();
        let t = alpha.radians_value().tan();

        if !t.is_finite() {
            return Err(RightTriangleError::NumericalInstability.into());
        }

        let a_val = b_mm * t;

        let a = PositiveLength::mm(a_val)
            .map_err(|_| RightTriangleError::LegNotPositive { value: a_val })?;

        Ok(RightTriangle::new(a, b))
    }

    // ---------------------------------------------------------
    // Beta wrappers
    // ---------------------------------------------------------

    pub fn from_leg_a_and_beta(
        a: PositiveLength,
        beta: AcuteAngle,
    ) -> Result<RightTriangle, GeometryError> {


        let alpha = AcuteAngle::degrees(90.0 - beta.degrees_value())
            .map_err(|_| RightTriangleError::NumericalInstability)?;

        Self::from_leg_and_opposite_angle(a, alpha)
    }

    pub fn from_leg_b_and_beta(
        b: PositiveLength,
        beta: AcuteAngle,
    ) -> Result<RightTriangle, GeometryError> {


        let alpha = AcuteAngle::degrees(90.0 - beta.degrees_value())
            .map_err(|_| RightTriangleError::NumericalInstability)?;

        Self::from_adjacent_leg_and_angle(b, alpha)
    }

    pub fn from_hypotenuse_and_beta(
        c: PositiveLength,
        beta: AcuteAngle,
    ) -> Result<RightTriangle, GeometryError> {


        let alpha = AcuteAngle::degrees(90.0 - beta.degrees_value())
            .map_err(|_| RightTriangleError::NumericalInstability)?;

        Self::from_hypotenuse_and_angle(c, alpha)
    }
}

// ---------------------------------------------------------
// Helpers
// ---------------------------------------------------------



fn safe_sqrt(value: f64) -> Result<f64, GeometryError> {
    if value < 0.0 && value > -EPS {
        return Ok(0.0);
    }

    if value < 0.0 {
        return Err(RightTriangleError::NumericalInstability.into());
    }

    Ok(value.sqrt())
}