// domain/geometry/right_triangle/right_triangle.rs

use crate::domain::units::{AcuteAngle, Angle, Length, PositiveLength};

const TRIG_CLAMP_EPS: f64 = 1e-15;

/// Represents a mathematically valid right triangle.
///
/// Canonical representation: two legs (`a`, `b`). All derived measures are
/// computed from these legs to maintain a single source of truth.
///
/// Invariants: leg lengths are positive and finite; hypotenuse is computed
/// from the Pythagorean relation and is positive.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RightTriangle {
    a: PositiveLength,
    b: PositiveLength,
}

impl RightTriangle {
    // ---------------------------------------------------------
    // Constructor (crate-private, solver owns validation)
    // ---------------------------------------------------------
    pub(crate) fn new(a: PositiveLength, b: PositiveLength) -> Self {
        Self { a, b }
    }

    // ---------------------------------------------------------
    // Accessors
    // ---------------------------------------------------------

    /// Returns the first leg length `a`.
    pub fn a(&self) -> PositiveLength {
        self.a
    }

    /// Returns the second leg length `b`.
    pub fn b(&self) -> PositiveLength {
        self.b
    }

    /// Hypotenuse `c` computed as sqrt(a² + b²).
    ///
    /// Returns a validated positive `Length` representing the hypotenuse.
    pub fn c(&self) -> PositiveLength {
        let a = self.a.mm_value();
        let b = self.b.mm_value();

        let c = (a * a + b * b).sqrt();
        PositiveLength::mm(c).unwrap()
    }

    /// Acute angle `alpha` opposite leg `a`.
    ///
    /// Returns an `Angle` in radians constructed using validated trig input.
    pub fn alpha(&self) -> AcuteAngle {
        let ratio = clamp_unit(self.a.mm_value() / self.c().mm_value());
        AcuteAngle::radians(ratio.asin()).unwrap()
    }

    /// Complementary acute angle `beta` adjacent to leg `a`.
    pub fn beta(&self) -> AcuteAngle {
        AcuteAngle::degrees(90.0 - self.alpha().degrees_value()).unwrap()
    }

    /// Right angle `gamma` (always 90°).
    pub fn gamma(&self) -> Angle {
        Angle::degrees(90.0).unwrap()
    }

    // ---------------------------------------------------------
    // Derived geometric properties
    // ---------------------------------------------------------

    /// Area of the right triangle (1/2 * a * b) in square millimetres.
    pub fn area(&self) -> f64 {
        self.a.mm_value() * self.b.mm_value() / 2.0
    }

    /// Perimeter as a domain `Length` (a + b + c).
    pub fn perimeter(&self) -> Length {
        Length::mm(
            self.a.mm_value()
                + self.b.mm_value()
                + self.c().mm_value(),
        )
        .unwrap()
    }

    /// Height from the right angle to the hypotenuse.
    ///
    /// Returns a positive `Length` representing the altitude to the hypotenuse.
    pub fn altitude_to_hypotenuse(&self) -> PositiveLength {
        let h = (2.0 * self.area()) / self.c().mm_value();
        PositiveLength::mm(h).unwrap()
    }

    /// Projection length of leg `a` onto the hypotenuse `c`.
    pub fn projection_a_on_c(&self) -> PositiveLength {
        let val = self.a.mm_value().powi(2) / self.c().mm_value();
        PositiveLength::mm(val).unwrap()
    }

    /// Projection length of leg `b` onto the hypotenuse `c`.
    pub fn projection_b_on_c(&self) -> PositiveLength {
        let val = self.b.mm_value().powi(2) / self.c().mm_value();
        PositiveLength::mm(val).unwrap()
    }

    /// Sine of the acute angle `alpha` (a / c).
    pub fn sin_alpha(&self) -> f64 {
        self.a.mm_value() / self.c().mm_value()
    }

    /// Cosine of the acute angle `alpha` (b / c).
    pub fn cos_alpha(&self) -> f64 {
        self.b.mm_value() / self.c().mm_value()
    }

    /// Tangent of the acute angle `alpha` (a / b).
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
    use crate::test_utils::approx::{approx_eq, DEFAULT_EPS};

    fn sample_triangle() -> RightTriangle {
        RightTriangle::new(
            PositiveLength::mm(3.0).unwrap(),
            PositiveLength::mm(4.0).unwrap(),
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
