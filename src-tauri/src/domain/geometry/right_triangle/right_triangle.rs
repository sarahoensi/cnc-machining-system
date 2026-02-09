// domain/geometry/right_triangle/right_triangle.rs

use crate::domain::units::angle::Angle;
use crate::domain::units::length::Length;

const TRIG_CLAMP_EPS: f64 = 1e-15;

/// Represents a mathematically valid right triangle.
/// Canonical representation: two legs (a, b).
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RightTriangle {
    a: Length,
    b: Length,
}

impl RightTriangle {
    // ---------------------------------------------------------
    // Constructor (crate-private, solver owns validation)
    // ---------------------------------------------------------
    pub(crate) fn new(a: Length, b: Length) -> Self {
        Self { a, b }
    }

    // ---------------------------------------------------------
    // Accessors
    // ---------------------------------------------------------

    pub fn a(&self) -> Length {
        self.a
    }

    pub fn b(&self) -> Length {
        self.b
    }

    /// Hypotenuse
    pub fn c(&self) -> Length {
        let a = self.a.mm_value();
        let b = self.b.mm_value();

        let c = (a * a + b * b).sqrt();
        Length::mm_positive(c).unwrap()
    }

    pub fn alpha(&self) -> Angle {
        let ratio = clamp_unit(self.a.mm_value() / self.c().mm_value());
        Angle::radians(ratio.asin()).unwrap()
    }

    pub fn beta(&self) -> Angle {
        Angle::degrees(90.0 - self.alpha().degrees_value()).unwrap()
    }

    /// Always 90°
    pub fn gamma(&self) -> Angle {
        Angle::degrees(90.0).unwrap()
    }

    // ---------------------------------------------------------
    // Derived geometric properties
    // ---------------------------------------------------------

    pub fn area(&self) -> f64 {
        self.a.mm_value() * self.b.mm_value() / 2.0
    }

    pub fn perimeter(&self) -> Length {
        Length::mm(
            self.a.mm_value()
                + self.b.mm_value()
                + self.c().mm_value(),
        )
        .unwrap()
    }

    /// Height from right angle to hypotenuse
    pub fn altitude_to_hypotenuse(&self) -> Length {
        let h = (2.0 * self.area()) / self.c().mm_value();
        Length::mm_positive(h).unwrap()
    }

    pub fn projection_a_on_c(&self) -> Length {
        let val = self.a.mm_value().powi(2) / self.c().mm_value();
        Length::mm_positive(val).unwrap()
    }

    pub fn projection_b_on_c(&self) -> Length {
        let val = self.b.mm_value().powi(2) / self.c().mm_value();
        Length::mm_positive(val).unwrap()
    }

    pub fn sin_alpha(&self) -> f64 {
        self.a.mm_value() / self.c().mm_value()
    }

    pub fn cos_alpha(&self) -> f64 {
        self.b.mm_value() / self.c().mm_value()
    }

    pub fn tan_alpha(&self) -> f64 {
        self.a.mm_value() / self.b.mm_value()
    }
}

// ---------------------------------------------------------
// Helpers
// ---------------------------------------------------------

fn clamp_unit(v: f64) -> f64 {
    if v.is_nan() {
        return 0.0;
    }
    v.clamp(-1.0 + TRIG_CLAMP_EPS, 1.0 - TRIG_CLAMP_EPS)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::test_utils::approx::{approx_eq, DEFAULT_EPS};

    fn sample_triangle() -> RightTriangle {
        RightTriangle::new(
            Length::mm_positive(3.0).unwrap(),
            Length::mm_positive(4.0).unwrap(),
        )
    }

    #[test]
    fn pythagoras_identity() {
        let t = sample_triangle();

        let lhs =
            t.a().mm_value().powi(2) + t.b().mm_value().powi(2);

        let rhs = t.c().mm_value().powi(2);

        assert!(approx_eq(lhs, rhs, DEFAULT_EPS));
    }

    #[test]
    fn angle_sum_identity() {
        let t = sample_triangle();

        let sum =
            t.alpha().degrees_value() + t.beta().degrees_value();

        assert!(approx_eq(sum, 90.0, DEFAULT_EPS));
    }

    #[test]
    fn projection_identity() {
        let t = sample_triangle();

        let sum = t.projection_a_on_c().mm_value()
            + t.projection_b_on_c().mm_value();

        assert!(approx_eq(
            sum,
            t.c().mm_value(),
            DEFAULT_EPS
        ));
    }

    #[test]
    fn altitude_identity() {
        let t = sample_triangle();

        let expected =
            (t.a().mm_value() * t.b().mm_value())
                / t.c().mm_value();

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

        let via_ch =
            t.c().mm_value()
                * t.altitude_to_hypotenuse().mm_value()
                / 2.0;

        assert!(approx_eq(via_ab, via_ch, DEFAULT_EPS));
    }

    #[test]
    fn trig_identity() {
        let t = sample_triangle();

        let s = t.sin_alpha();
        let c = t.cos_alpha();

        assert!(approx_eq(
            s.powi(2) + c.powi(2),
            1.0,
            DEFAULT_EPS
        ));
    }
}
