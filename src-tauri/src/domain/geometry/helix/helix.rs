use std::f64::consts::PI;

use crate::domain::units::{
    Diameter,
    Length,
    Pitch,
    PositiveLength,
    AcuteAngle,
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Helix {
    diameter: Diameter,
    pitch: Pitch,
}

impl Helix {

    pub fn new(diameter: Diameter, pitch: Pitch) -> Self {
        Self { diameter, pitch }
    }

    // ---------------------------------------------------------
    // Accessors
    // ---------------------------------------------------------

    pub fn diameter(&self) -> Diameter {
        self.diameter
    }

    pub fn pitch(&self) -> Pitch {
        self.pitch
    }

    // ---------------------------------------------------------
    // Derived geometric properties
    // ---------------------------------------------------------

    /// π * D  (always positive)
    pub fn circumference(&self) -> PositiveLength {
        let value = PI * self.diameter.mm_value();
        PositiveLength::mm(value)
            .expect("Circumference must be positive")
    }

    /// atan(p / circumference)
    /// Always 0 < θ < π/2
    pub fn helix_angle(&self) -> AcuteAngle {
        let c = self.circumference().mm_value();
        let p = self.pitch.mm_per_rev_value();

        let angle = (p / c).atan();

        AcuteAngle::radians(angle)
            .expect("Helix angle must be acute")
    }

    /// sqrt(c² + p²) (always positive)
    pub fn length_per_revolution(&self) -> PositiveLength {
        let c = self.circumference().mm_value();
        let p = self.pitch.mm_per_rev_value();

        let value = (c.powi(2) + p.powi(2)).sqrt();

        PositiveLength::mm(value)
            .expect("Helix length per revolution must be positive")
    }

    /// Total helix length for N revolutions.
    /// May be negative if revolutions is negative.
    pub fn total_length(&self, revolutions: f64) -> Length {
        let single = self.length_per_revolution().mm_value();
        Length::mm(single * revolutions)
            .expect("Finite multiplication expected")
    }

    /// Axial travel = pitch * revolutions
    /// May be negative if revolutions is negative.
    pub fn axial_travel(&self, revolutions: f64) -> Length {
        let value = self.pitch.mm_per_rev_value() * revolutions;

        Length::mm(value)
            .expect("Finite multiplication expected")
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
