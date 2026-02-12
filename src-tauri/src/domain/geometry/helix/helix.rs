// domain/geometry/helix/helix.rs


use std::f64::consts::PI;

use crate::domain::{Angle, Diameter, Length};
use crate::domain::units::Pitch;

/// Represents a cylindrical helix used in machining geometry.
///
/// Encapsulates `diameter` and `pitch` to derive helix-specific measures.
///
/// Invariants: `diameter` and `pitch` are positive and finite.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Helix {
    diameter: Diameter,
    pitch: Pitch,
}

impl Helix {
    // ---------------------------------------------------------
    // Constructor
    // ---------------------------------------------------------

    /// Create a new `Helix` from validated `Diameter` and `Pitch`.
    ///
    /// Both `Diameter` and `Pitch` enforce unit correctness and positivity.
    pub fn new(diameter: Diameter, pitch: Pitch) -> Self {
        Self { diameter, pitch }
    }

    // ---------------------------------------------------------
    // Accessors
    // ---------------------------------------------------------

    /// Returns the helix `Diameter` (domain type).
    pub fn diameter(&self) -> Diameter {
        self.diameter
    }

    /// Returns the helix `Pitch` (millimetres per revolution).
    pub fn pitch(&self) -> Pitch {
        self.pitch
    }

    // ---------------------------------------------------------
    // Derived geometric properties
    // ---------------------------------------------------------

    /// Circumference of the helix cylinder (π * diameter) as a domain `Length`.
    ///
    /// Units: millimetres. The result is finite and positive.
    pub fn circumference(&self) -> Length {
        let value = PI * self.diameter.mm_value();
        Length::mm(value).unwrap()
    }

    /// Helix angle computed as atan(pitch / circumference).
    ///
    /// Returned `Angle` is finite and in radians; callers should use domain
    /// facilities to inspect degrees or radians as needed.
    pub fn helix_angle(&self) -> Angle {
        let c = self.circumference().mm_value();
        let p = self.pitch.mm_per_rev_value();

        let angle = (p / c).atan();
        Angle::radians(angle).unwrap()
    }

    /// Helix length per single revolution: sqrt(circumference² + pitch²).
    ///
    /// Units: millimetres. Useful for computing actual toolpath lengths.
    pub fn length_per_revolution(&self) -> Length {
        let c = self.circumference().mm_value();
        let p = self.pitch.mm_per_rev_value();

        let value = (c.powi(2) + p.powi(2)).sqrt();
        Length::mm(value).unwrap()
    }

    /// Total helix length for a given number of revolutions.
    ///
    /// The `revolutions` parameter may be fractional; units are millimetres.
    pub fn total_length(&self, revolutions: f64) -> Length {
        let single = self.length_per_revolution().mm_value();
        Length::mm(single * revolutions).unwrap()
    }

    /// Axial travel for a given number of revolutions (pitch * revolutions).
    ///
    /// Units: millimetres. Positive or negative revolutions reflect direction.
    pub fn axial_travel(&self, revolutions: f64) -> Length {
        let value = self.pitch.mm_per_rev_value() * revolutions;
        Length::mm(value).unwrap()
    }
}

// --------------- TESTS -------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::approx::{approx_eq, DEFAULT_EPS};

    #[test]
    fn circumference_formula() {
        let h = Helix::new(
            Diameter::mm(10.0).unwrap(),
            Pitch::mm_per_rev(2.0).unwrap(),
        );

        let expected = PI * 10.0;

        assert!(approx_eq(
            h.circumference().mm_value(),
            expected,
            DEFAULT_EPS
        ));
    }

    #[test]
    fn axial_travel_identity() {
        let h = Helix::new(
            Diameter::mm(8.0).unwrap(),
            Pitch::mm_per_rev(3.0).unwrap(),
        );

        let travel = h.axial_travel(1.0);

        assert!(approx_eq(
            travel.mm_value(),
            3.0,
            DEFAULT_EPS
        ));
    }

    #[test]
    fn total_length_identity() {
        let h = Helix::new(
            Diameter::mm(6.0).unwrap(),
            Pitch::mm_per_rev(2.0).unwrap(),
        );

        let single = h.length_per_revolution();
        let total = h.total_length(5.0);

        assert!(approx_eq(
            total.mm_value(),
            single.mm_value() * 5.0,
            DEFAULT_EPS
        ));
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;
    use crate::test_utils::approx::{approx_eq, DEFAULT_EPS};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn length_per_revolution_identity(
            diameter in 1e-6f64..1e4f64,
            pitch in 1e-6f64..1e4f64
        ) {
            let h = Helix::new(
                Diameter::mm(diameter).unwrap(),
                Pitch::mm_per_rev(pitch).unwrap()
            );

            let c = h.circumference().mm_value();
            let expected = (c.powi(2) + pitch.powi(2)).sqrt();

            prop_assert!(approx_eq(
                h.length_per_revolution().mm_value(),
                expected,
                DEFAULT_EPS
            ));
        }
    }

    proptest! {
        #[test]
        fn helix_angle_identity(
            diameter in 1e-6f64..1e4f64,
            pitch in 1e-6f64..1e4f64
        ) {
            let h = Helix::new(
                Diameter::mm(diameter).unwrap(),
                Pitch::mm_per_rev(pitch).unwrap()
            );

            let c = h.circumference().mm_value();
            let angle = h.helix_angle().radians_value();

            prop_assert!(approx_eq(
                angle.tan(),
                pitch / c,
                DEFAULT_EPS
            ));
        }
    }

    proptest! {
        #[test]
        fn axial_travel_identity(
            diameter in 1e-6f64..1e4f64,
            pitch in 1e-6f64..1e4f64,
            revs in -1e3f64..1e3f64
        ) {
            let h = Helix::new(
                Diameter::mm(diameter).unwrap(),
                Pitch::mm_per_rev(pitch).unwrap()
            );

            let travel = h.axial_travel(revs);

            prop_assert!(approx_eq(
                travel.mm_value(),
                pitch * revs,
                DEFAULT_EPS
            ));
        }
    }
}
