// domain/geometry/right_triangle/right_triangle_solver.rs

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
        // No validation needed — units guarantee positivity
        Ok(RightTriangle::new(a, b))
    }

    // ---------------------------------------------------------
    // a + c
    // ---------------------------------------------------------

    pub fn from_leg_and_hypotenuse(
        a: PositiveLength,
        c: PositiveLength,
    ) -> Result<RightTriangle, GeometryError> {

        let b = Self::solve_leg_and_hypotenuse(a, c)?;
        Ok(RightTriangle::new(a, b))
    }

    // ---------------------------------------------------------
    // b + c
    // ---------------------------------------------------------

    pub fn from_other_leg_and_hypotenuse(
        b: PositiveLength,
        c: PositiveLength,
    ) -> Result<RightTriangle, GeometryError> {

        let a = Self::solve_leg_and_hypotenuse(b, c)?;
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

        let a = PositiveLength::mm(a_val)?;

        let b = PositiveLength::mm(b_val)?;

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
    let rad = alpha.radians_value();

    let sin = rad.sin();
    let cos = rad.cos();

    if !sin.is_finite() || !cos.is_finite() {
        return Err(RightTriangleError::NumericalInstability.into());
    }

    if sin.abs() < EPS {
        return Err(RightTriangleError::NumericalInstability.into());
    }

    // b = a * cos / sin   (more stable than a / tan)
    let b_val = a_mm * cos / sin;

    if !b_val.is_finite() || b_val <= 0.0 {
        return Err(RightTriangleError::NumericalInstability.into());
    }

    let b = PositiveLength::mm(b_val)?;


    Ok(RightTriangle::new(a, b))
}

    // ---------------------------------------------------------
    // b + alpha
    // ---------------------------------------------------------

pub fn from_adjacent_leg_and_angle(
    b: PositiveLength,
    alpha: AcuteAngle,
) -> Result<RightTriangle, GeometryError> {

    let b_mm = b.mm_value();
    let rad = alpha.radians_value();

    let sin = rad.sin();
    let cos = rad.cos();

    if !sin.is_finite() || !cos.is_finite() {
        return Err(RightTriangleError::NumericalInstability.into());
    }

    if cos.abs() < EPS {
        return Err(RightTriangleError::NumericalInstability.into());
    }

    // a = b * sin / cos  (more stable than b * tan)
    let a_val = b_mm * sin / cos;

    if !a_val.is_finite() || a_val <= 0.0 {
        return Err(RightTriangleError::NumericalInstability.into());
    }

    let a = PositiveLength::mm(a_val)?;
        

    Ok(RightTriangle::new(a, b))
}
    // ---------------------------------------------------------
    // Beta wrappers
    // ---------------------------------------------------------

    pub fn from_leg_a_and_beta(
    a: PositiveLength,
    beta: AcuteAngle,
) -> Result<RightTriangle, GeometryError> {

    let mut alpha_rad =
    std::f64::consts::FRAC_PI_2 - beta.radians_value();

if alpha_rad < 0.0 && alpha_rad > -EPS {
    alpha_rad = 0.0;
}

let alpha = AcuteAngle::radians(alpha_rad)
    .map_err(|_| RightTriangleError::NumericalInstability)?;

    Self::from_leg_and_opposite_angle(a, alpha)
}

    pub fn from_leg_b_and_beta(
    b: PositiveLength,
    beta: AcuteAngle,
) -> Result<RightTriangle, GeometryError> {

    let mut alpha_rad =
    std::f64::consts::FRAC_PI_2 - beta.radians_value();

if alpha_rad < 0.0 && alpha_rad > -EPS {
    alpha_rad = 0.0;
}

let alpha = AcuteAngle::radians(alpha_rad)
    .map_err(|_| RightTriangleError::NumericalInstability)?;

    Self::from_adjacent_leg_and_angle(b, alpha)
}

    pub fn from_hypotenuse_and_beta(
    c: PositiveLength,
    beta: AcuteAngle,
) -> Result<RightTriangle, GeometryError> {

    let mut alpha_rad =
    std::f64::consts::FRAC_PI_2 - beta.radians_value();

if alpha_rad < 0.0 && alpha_rad > -EPS {
    alpha_rad = 0.0;
}

let alpha = AcuteAngle::radians(alpha_rad)
    .map_err(|_| RightTriangleError::NumericalInstability)?;

    Self::from_hypotenuse_and_angle(c, alpha)
}

    // ---------------------------------------------------------
    // Internal helpers
    // ---------------------------------------------------------

   fn solve_leg_and_hypotenuse(
    leg: PositiveLength,
    hyp: PositiveLength,
) -> Result<PositiveLength, GeometryError> {

    let leg_mm = leg.mm_value();
    let hyp_mm = hyp.mm_value();

    if leg_mm >= hyp_mm {
        return Err(
            RightTriangleError::HypotenuseTooShort {
                leg: leg_mm,
                hypotenuse: hyp_mm,
            }
            .into(),
        );
    }

    // numerically stable
    let diff = hyp_mm - leg_mm;
    let sum  = hyp_mm + leg_mm;
    let other_sq = diff * sum;

    let other_val = safe_sqrt(other_sq)?;

    PositiveLength::mm(other_val)?;

    let other = PositiveLength::mm(other_val)?;

    Ok(other)
        
}

    
}

// ---------------------------------------------------------
// Numeric helper
// ---------------------------------------------------------

fn safe_sqrt(value: f64) -> Result<f64, GeometryError> {

    if !value.is_finite() {
        return Err(RightTriangleError::NumericalInstability.into());
    }

    if value < -EPS {
        return Err(RightTriangleError::NumericalInstability.into());
    }

    Ok(value.max(0.0).sqrt())
}