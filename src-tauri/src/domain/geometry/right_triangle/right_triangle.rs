// domain/geometry/right_triangle/right_triangle.rs

use std::f64::consts::FRAC_PI_2;

use crate::domain::{
    units::{AcuteAngle, Angle, PositiveLength},
    GeometryError,
};

use super::RightTriangleError;

const EPS: f64 = 1e-12;

/// Represents a mathematically valid right triangle.
///
/// Canonical representation: two legs (`a`, `b`).
///
/// All other properties are derived from these legs.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RightTriangle {
    a: PositiveLength,
    b: PositiveLength,
}

impl RightTriangle {
    // ---------------------------------------------------------
    // Constructors
    // ---------------------------------------------------------

    pub fn from_legs(a: PositiveLength, b: PositiveLength) -> Self {
        Self { a, b }
    }

    pub fn from_leg_and_hypotenuse(
        a: PositiveLength,
        c: PositiveLength,
    ) -> Result<Self, GeometryError> {
        let a_mm = a.mm_value();
        let c_mm = c.mm_value();

        if a_mm >= c_mm {
            return Err(RightTriangleError::HypotenuseTooShort {
                leg: a_mm,
                hypotenuse: c_mm,
            }
            .into());
        }

        // numerically stable: c² − a² = (c−a)(c+a)
        let diff = c_mm - a_mm;
        let sum = c_mm + a_mm;

        let b = PositiveLength::mm_unchecked((diff * sum).sqrt());

        Ok(Self { a, b })
    }

    pub fn from_other_leg_and_hypotenuse(
        b: PositiveLength,
        c: PositiveLength,
    ) -> Result<Self, GeometryError> {
        let b_mm = b.mm_value();
        let c_mm = c.mm_value();

        if b_mm >= c_mm {
            return Err(RightTriangleError::HypotenuseTooShort {
                leg: b_mm,
                hypotenuse: c_mm,
            }
            .into());
        }

        let diff = c_mm - b_mm;
        let sum = c_mm + b_mm;

        let a = PositiveLength::mm_unchecked((diff * sum).sqrt());

        Ok(Self { a, b })
    }

    pub fn from_hypotenuse_and_angle(c: PositiveLength, alpha: AcuteAngle) -> Self {
        let c_mm = c.mm_value();
        let rad = alpha.radians_value();

        let a = PositiveLength::mm_unchecked(c_mm * rad.sin());
        let b = PositiveLength::mm_unchecked(c_mm * rad.cos());

        Self { a, b }
    }

    pub fn from_leg_and_opposite_angle(
        a: PositiveLength,
        alpha: AcuteAngle,
    ) -> Result<Self, GeometryError> {
        let a_mm = a.mm_value();
        let rad = alpha.radians_value();

        let sin = rad.sin();
        let cos = rad.cos();

        if sin.abs() < EPS {
            return Err(RightTriangleError::NumericalInstability.into());
        }

        let b = PositiveLength::mm_unchecked(a_mm * cos / sin);

        Ok(Self { a, b })
    }

    pub fn from_adjacent_leg_and_angle(
        b: PositiveLength,
        alpha: AcuteAngle,
    ) -> Result<Self, GeometryError> {
        let b_mm = b.mm_value();
        let rad = alpha.radians_value();

        let sin = rad.sin();
        let cos = rad.cos();

        if cos.abs() < EPS {
            return Err(RightTriangleError::NumericalInstability.into());
        }

        let a = PositiveLength::mm_unchecked(b_mm * sin / cos);

        Ok(Self { a, b })
    }

    // ---------------------------------------------------------
    // Beta wrappers
    // ---------------------------------------------------------

    pub fn from_leg_a_and_beta(a: PositiveLength, beta: AcuteAngle) -> Result<Self, GeometryError> {
        let alpha = AcuteAngle::radians_unchecked(FRAC_PI_2 - beta.radians_value());

        Self::from_leg_and_opposite_angle(a, alpha)
    }

    pub fn from_leg_b_and_beta(b: PositiveLength, beta: AcuteAngle) -> Result<Self, GeometryError> {
        let alpha = AcuteAngle::radians_unchecked(FRAC_PI_2 - beta.radians_value());

        Self::from_adjacent_leg_and_angle(b, alpha)
    }

    pub fn from_hypotenuse_and_beta(c: PositiveLength, beta: AcuteAngle) -> Self {
        let alpha = AcuteAngle::radians_unchecked(FRAC_PI_2 - beta.radians_value());

        Self::from_hypotenuse_and_angle(c, alpha)
    }

    // ---------------------------------------------------------
    // Accessors
    // ---------------------------------------------------------

    pub fn a(&self) -> PositiveLength {
        self.a
    }
    pub fn b(&self) -> PositiveLength {
        self.b
    }

    // ---------------------------------------------------------
    // Internal helper
    // ---------------------------------------------------------

    fn legs(&self) -> (f64, f64) {
        (self.a.mm_value(), self.b.mm_value())
    }

    // ---------------------------------------------------------
    // Derived geometry
    // ---------------------------------------------------------

    pub fn c(&self) -> PositiveLength {
        let (a, b) = self.legs();
        PositiveLength::mm_unchecked((a * a + b * b).sqrt())
    }

    pub fn alpha(&self) -> AcuteAngle {
        let (a, b) = self.legs();
        AcuteAngle::radians_unchecked(a.atan2(b))
    }

    pub fn beta(&self) -> AcuteAngle {
        let (a, b) = self.legs();
        AcuteAngle::radians_unchecked(b.atan2(a))
    }

    pub fn gamma(&self) -> Angle {
        Angle::radians_unchecked(FRAC_PI_2)
    }

    pub fn area(&self) -> f64 {
        let (a, b) = self.legs();
        0.5 * a * b
    }

    pub fn perimeter(&self) -> PositiveLength {
        let (a, b) = self.legs();
        PositiveLength::mm_unchecked(a + b + self.c().mm_value())
    }

    pub fn altitude_to_hypotenuse(&self) -> PositiveLength {
        let (a, b) = self.legs();
        let c = self.c().mm_value();
        PositiveLength::mm_unchecked((a * b) / c)
    }

    pub fn projection_a_on_c(&self) -> PositiveLength {
        let (a, _) = self.legs();
        let c = self.c().mm_value();
        PositiveLength::mm_unchecked((a * a) / c)
    }

    pub fn projection_b_on_c(&self) -> PositiveLength {
        let (_, b) = self.legs();
        let c = self.c().mm_value();
        PositiveLength::mm_unchecked((b * b) / c)
    }

    pub fn sin_alpha(&self) -> f64 {
        let (a, _) = self.legs();
        a / self.c().mm_value()
    }

    pub fn cos_alpha(&self) -> f64 {
        let (_, b) = self.legs();
        b / self.c().mm_value()
    }

    pub fn tan_alpha(&self) -> f64 {
        let (a, b) = self.legs();
        a / b
    }
}
// ---------------------------------------------------------
// Helpers
// ---------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::approx::{approx_eq, DEFAULT_EPS};

    fn sample_triangle() -> RightTriangle {
        RightTriangle::from_legs(
            PositiveLength::mm(3.0).unwrap(),
            PositiveLength::mm(4.0).unwrap(),
        )
    }

    #[test]
    fn pythagoras_identity() {
        let t = sample_triangle();

        let lhs = t.a().mm_value().powi(2) + t.b().mm_value().powi(2);

        let rhs = t.c().mm_value().powi(2);

        assert!(approx_eq(lhs, rhs, DEFAULT_EPS));
    }

    #[test]
    fn angle_sum_identity() {
        let t = sample_triangle();

        let sum = t.alpha().degrees_value() + t.beta().degrees_value();

        assert!(approx_eq(sum, 90.0, DEFAULT_EPS));
    }

    #[test]
    fn projection_identity() {
        let t = sample_triangle();

        let sum = t.projection_a_on_c().mm_value() + t.projection_b_on_c().mm_value();

        assert!(approx_eq(sum, t.c().mm_value(), DEFAULT_EPS));
    }

    #[test]
    fn altitude_identity() {
        let t = sample_triangle();

        let expected = (t.a().mm_value() * t.b().mm_value()) / t.c().mm_value();

        assert!(approx_eq(
            t.altitude_to_hypotenuse().mm_value(),
            expected,
            DEFAULT_EPS
        ));
    }

    #[test]
    fn area_consistency() {
        let t = sample_triangle();

        let via_ab = t.area();

        let via_ch = t.c().mm_value() * t.altitude_to_hypotenuse().mm_value() / 2.0;

        assert!(approx_eq(via_ab, via_ch, DEFAULT_EPS));
    }

    #[test]
    fn trig_identity() {
        let t = sample_triangle();

        let s = t.sin_alpha();
        let c = t.cos_alpha();

        assert!(approx_eq(s.powi(2) + c.powi(2), 1.0, DEFAULT_EPS));
    }
}
