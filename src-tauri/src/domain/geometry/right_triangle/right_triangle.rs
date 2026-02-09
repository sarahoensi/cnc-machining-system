// domain/geometry/right_triangle/right_triangle.rs

use crate::domain::units::length::Length;
use crate::domain::units::angle::Angle;


const EPS: f64 = 1e-9;

/// Represents a mathematically valid right triangle.
///
/// Invariants:
/// - a² + b² = c²
/// - gamma = 90°
/// - alpha + beta = 90°
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RightTriangle {
    a: Length, // leg
    b: Length, // leg
    c: Length, // hypotenuse

    alpha: Angle,
    beta: Angle,
}

impl RightTriangle {
    // ---------------------------------------------------------
    // Constructor (crate-private)
    // Only solver should construct triangles.
    // ---------------------------------------------------------
    pub(crate) fn new(
        a: Length,
        b: Length,
        c: Length,
        alpha: Angle,
        beta: Angle,
    ) -> Self {
        debug_assert!(is_valid_pythagoras(a, b, c));
        debug_assert!(is_valid_angles(alpha, beta));

        Self { a, b, c, alpha, beta }
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

    pub fn c(&self) -> Length {
        self.c
    }

    pub fn alpha(&self) -> Angle {
        self.alpha
    }

    pub fn beta(&self) -> Angle {
        self.beta
    }

    /// Always 90°
    pub fn gamma(&self) -> Angle {
        Angle::degrees(90.0).unwrap()
    }

    // ---------------------------------------------------------
    // Derived geometric properties
    // ---------------------------------------------------------

    /// Triangle area = (a * b) / 2
    pub fn area(&self) -> f64 {
        (self.a.mm_value() * self.b.mm_value()) / 2.0
    }

    /// Perimeter = a + b + c
    pub fn perimeter(&self) -> Length {
        Length::mm(self.a.mm_value() + self.b.mm_value() + self.c.mm_value())
            .unwrap()
    }

    /// Height from hypotenuse to right angle vertex
    pub fn altitude_to_hypotenuse(&self) -> Length {
        let h = (self.a.mm_value() * self.b.mm_value()) / self.c.mm_value();
        Length::mm(h).unwrap()
    }

    /// Projection of side a onto hypotenuse
    pub fn projection_a_on_c(&self) -> Length {
        let val = self.a.mm_value().powi(2) / self.c.mm_value();
        Length::mm(val).unwrap()
    }

    /// Projection of side b onto hypotenuse
    pub fn projection_b_on_c(&self) -> Length {
        let val = self.b.mm_value().powi(2) / self.c.mm_value();
        Length::mm(val).unwrap()
    }

    /// Ratio a / c
    pub fn sin_alpha(&self) -> f64 {
        self.a.mm_value() / self.c.mm_value()
    }

    /// Ratio b / c
    pub fn cos_alpha(&self) -> f64 {
        self.b.mm_value() / self.c.mm_value()
    }

    /// Ratio a / b
    pub fn tan_alpha(&self) -> f64 {
        self.a.mm_value() / self.b.mm_value()
    }
}

// ---------------------------------------------------------
// Validation helpers
// ---------------------------------------------------------

fn is_valid_pythagoras(a: Length, b: Length, c: Length) -> bool {
    let lhs = a.mm_value().powi(2) + b.mm_value().powi(2);
    let rhs = c.mm_value().powi(2);

    (lhs - rhs).abs() < EPS
}

fn is_valid_angles(alpha: Angle, beta: Angle) -> bool {
    let sum = alpha.degrees_value() + beta.degrees_value();
    (sum - 90.0).abs() < 1e-6
}

// --------------- TESTS ----------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::test_utils::approx::{approx_eq, DEFAULT_EPS};

    fn sample_triangle() -> RightTriangle {
        // 3-4-5 triangle
        RightTriangle::new(
            Length::mm(3.0).unwrap(),
            Length::mm(4.0).unwrap(),
            Length::mm(5.0).unwrap(),
            Angle::degrees(36.86989765).unwrap(),
            Angle::degrees(53.13010235).unwrap(),
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
            (t.a().mm_value() * t.b().mm_value()) / t.c().mm_value();

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
        let via_ch = t.c().mm_value()
            * t.altitude_to_hypotenuse().mm_value()
            / 2.0;

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

#[cfg(test)]
mod property_tests {
    use super::*;
    use crate::domain::test_utils::approx::{approx_eq, DEFAULT_EPS};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn pythagoras_property(a in 1e-6f64..1e4f64,
                              b in 1e-6f64..1e4f64) {

            let c_val = (a.powi(2) + b.powi(2)).sqrt();

            let alpha = (a / c_val).asin().to_degrees();
            let beta = 90.0 - alpha;

            let t = RightTriangle::new(
                Length::mm(a).unwrap(),
                Length::mm(b).unwrap(),
                Length::mm(c_val).unwrap(),
                Angle::degrees(alpha).unwrap(),
                Angle::degrees(beta).unwrap(),
            );

            let lhs = a.powi(2) + b.powi(2);
            let rhs = t.c().mm_value().powi(2);

            prop_assert!(approx_eq(lhs, rhs, DEFAULT_EPS));
        }
    }
}
