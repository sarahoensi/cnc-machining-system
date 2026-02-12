// domain/geometry/circle/circle.rs

use std::f64::consts::PI;

use crate::domain::{Angle, Diameter, Length, Radius};

/// Represents a mathematically valid circle.
///
/// Circle values are expressed via a validated `Radius` to ensure unit and
/// numerical correctness in machining calculations.
///
/// Invariants: radius is positive and finite.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Circle {
    radius: Radius,
}

impl Circle {
    // ---------------------------------------------------------
    // Constructors
    // ---------------------------------------------------------

    /// Construct a `Circle` from a validated `Radius`.
    ///
    /// The `Radius` type enforces positivity and finiteness; callers should
    /// construct `Radius` via its domain-safe constructors.
    pub fn from_radius(radius: Radius) -> Self {
        Self { radius }
    }

    /// Construct a `Circle` from a validated `Diameter`.
    ///
    /// Invariant: the provided `Diameter` is positive and finite.
    pub fn from_diameter(diameter: Diameter) -> Self {
        Self {
            radius: diameter.radius(),
        }
    }

    // ---------------------------------------------------------
    // Accessors
    // ---------------------------------------------------------

    /// Returns the validated `Radius` of the circle.
    pub fn radius(&self) -> Radius {
        self.radius
    }

    /// Returns the corresponding `Diameter` for the circle.
    pub fn diameter(&self) -> Diameter {
        self.radius.diameter()
    }

    // ---------------------------------------------------------
    // Derived geometric properties
    // ---------------------------------------------------------

    /// Circumference of the circle (2πr) as a domain `Length`.
    ///
    /// Units: millimetres. The returned `Length` is finite and positive.
    pub fn circumference(&self) -> Length {
        let value = 2.0 * PI * self.radius.mm_value();
        Length::mm(value).unwrap()
    }

    /// Area of the circle (πr²) in square millimetres.
    pub fn area(&self) -> f64 {
        PI * self.radius.mm_value().powi(2)
    }

    /// Arc length for a given `Angle` (r * θ) as a domain `Length`.
    ///
    /// Units: millimetres. The `Angle` must be finite; callers should use
    /// `Angle` domain constructors which validate values.
    pub fn arc_length(&self, angle: Angle) -> Length {
        let value = self.radius.mm_value() * angle.radians_value();
        Length::mm(value).unwrap()
    }

    /// Sector area for a given `Angle` as a fraction of the full circle area.
    ///
    /// Units: square millimetres. Computation preserves unit correctness via
    /// `Angle` and `Radius` domain types.
    pub fn sector_area(&self, angle: Angle) -> f64 {
        let fraction = angle.radians_value() / (2.0 * PI);
        self.area() * fraction
    }
}

// ------------------ TESTS -------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::approx::{approx_eq, DEFAULT_EPS};
    use crate::domain::Angle;

    #[test]
    fn constructors_are_consistent() {
        let r = Radius::mm(5.0).unwrap();
        let d = Diameter::mm(10.0).unwrap();

        let c1 = Circle::from_radius(r);
        let c2 = Circle::from_diameter(d);

        assert!(approx_eq(
            c1.radius().mm_value(),
            c2.radius().mm_value(),
            DEFAULT_EPS
        ));
    }

    #[test]
    fn circumference_formula() {
        let c = Circle::from_radius(Radius::mm(3.0).unwrap());

        let expected = 2.0 * PI * 3.0;

        assert!(approx_eq(
            c.circumference().mm_value(),
            expected,
            DEFAULT_EPS
        ));
    }

    #[test]
    fn area_formula() {
        let c = Circle::from_radius(Radius::mm(3.0).unwrap());

        let expected = PI * 9.0;

        assert!(approx_eq(
            c.area(),
            expected,
            DEFAULT_EPS
        ));
    }

    #[test]
    fn arc_length_formula() {
        let c = Circle::from_radius(Radius::mm(2.0).unwrap());
        let angle = Angle::radians(PI).unwrap();

        let expected = 2.0 * PI;

        assert!(approx_eq(
            c.arc_length(angle).mm_value(),
            expected,
            DEFAULT_EPS
        ));
    }

    #[test]
    fn sector_area_formula() {
        let c = Circle::from_radius(Radius::mm(2.0).unwrap());
        let angle = Angle::radians(PI).unwrap();

        let expected = 0.5 * c.area();

        assert!(approx_eq(
            c.sector_area(angle),
            expected,
            DEFAULT_EPS
        ));
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;
    use crate::test_utils::approx::{approx_eq, DEFAULT_EPS};
    use crate::domain::Angle;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn arc_length_full_circle_equals_circumference(radius in 1e-9f64..1e6f64) {
            let c = Circle::from_radius(Radius::mm(radius).unwrap());

            let full = c.arc_length(
                Angle::radians(2.0 * PI).unwrap()
            );

            prop_assert!(approx_eq(
                full.mm_value(),
                c.circumference().mm_value(),
                DEFAULT_EPS
            ));
        }
    }

    proptest! {
        #[test]
        fn sector_area_full_circle_equals_area(radius in 1e-9f64..1e6f64) {
            let c = Circle::from_radius(Radius::mm(radius).unwrap());

            let full = c.sector_area(
                Angle::radians(2.0 * PI).unwrap()
            );

            prop_assert!(approx_eq(
                full,
                c.area(),
                DEFAULT_EPS
            ));
        }
    }

    proptest! {
        #[test]
        fn circumference_increases_with_radius(a in 1e-9f64..1e6f64,
                                              b in 1e-9f64..1e6f64) {

            prop_assume!(a < b);

            let c1 = Circle::from_radius(Radius::mm(a).unwrap());
            let c2 = Circle::from_radius(Radius::mm(b).unwrap());

            prop_assert!(c1.circumference() < c2.circumference());
        }
    }
}

