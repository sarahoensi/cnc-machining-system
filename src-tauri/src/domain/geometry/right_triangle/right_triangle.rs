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
