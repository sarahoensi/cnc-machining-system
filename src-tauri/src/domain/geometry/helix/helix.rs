// domain/geometry/helix/helix.rs

use std::f64::consts::PI;

use crate::domain::{
    units::{AcuteAngle, Diameter, Length, Pitch, PositiveLength},
    GeometryError,
};

use super::HelixError;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum HelixMode {
    Inner,
    Outer,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Helix {
    diameter: Diameter,
    pitch: Pitch,
}

impl Helix {

    // ---------------- Constructors ----------------

    pub fn new(diameter: Diameter, pitch: Pitch) -> Self {
        Self { diameter, pitch }
    }

    pub fn from_pitch(
        mode: HelixMode,
        nominal: Diameter,
        tool: Diameter,
        pitch: Pitch,
    ) -> Result<Self, GeometryError> {

        let diameter = Self::effective_diameter(mode, nominal, tool)?;

        Ok(Self::new(diameter, pitch))
    }

    pub fn from_angle(
        mode: HelixMode,
        nominal: Diameter,
        tool: Diameter,
        angle: AcuteAngle,
    ) -> Result<Self, GeometryError> {

        let diameter = Self::effective_diameter(mode, nominal, tool)?;

        let d = diameter.mm_value();
        let a = angle.radians_value();

        let pitch =
            Pitch::mm_per_rev_unchecked(a.tan() * PI * d);

        Ok(Self::new(diameter, pitch))
    }

    // ---------------- Internal helpers ----------------

    fn effective_diameter(
        mode: HelixMode,
        nominal: Diameter,
        tool: Diameter,
    ) -> Result<Diameter, GeometryError> {

        let d = nominal.mm_value();
        let r_tool = tool.mm_value() / 2.0;

        let effective = match mode {

            HelixMode::Inner => {
                if r_tool >= d {
                    return Err(
                        HelixError::ToolTooLarge {
                            tool_diameter: tool.mm_value(),
                            nominal_diameter: d,
                        }.into()
                    );
                }

                d - r_tool
            }

            HelixMode::Outer => d + r_tool,
        };

        Ok(Diameter::mm_unchecked(effective))
    }

    // ---------------- Accessors ----------------

    pub fn diameter(&self) -> Diameter {
        self.diameter
    }

    pub fn pitch(&self) -> Pitch {
        self.pitch
    }

    // ---------------- Derived geometry ----------------

    pub fn circumference(&self) -> PositiveLength {
        PositiveLength::mm_unchecked(PI * self.diameter.mm_value())
    }

    pub fn helix_angle(&self) -> AcuteAngle {
        let p = self.pitch.mm_per_rev_value();
        let c = self.circumference().mm_value();

        AcuteAngle::radians_unchecked((p / c).atan())
    }

    pub fn length_per_revolution(&self) -> PositiveLength {
        let p = self.pitch.mm_per_rev_value();
        let c = self.circumference().mm_value();

        PositiveLength::mm_unchecked((c * c + p * p).sqrt())
    }

    pub fn total_length(&self, revolutions: f64) -> Length {
        let l = self.length_per_revolution().mm_value();
        Length::mm_unchecked(l * revolutions)
    }

    pub fn axial_travel(&self, revolutions: f64) -> Length {
        Length::mm_unchecked(self.pitch.mm_per_rev_value() * revolutions)
    }
}

// --------------- TESTS -------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::approx::{approx_eq, DEFAULT_EPS};

    #[test]
    fn circumference_formula() {
        let h = Helix::new(Diameter::mm(10.0).unwrap(), Pitch::mm_per_rev(2.0).unwrap());

        let expected = PI * 10.0;

        assert!(approx_eq(
            h.circumference().mm_value(),
            expected,
            DEFAULT_EPS
        ));
    }

    #[test]
    fn axial_travel_identity() {
        let h = Helix::new(Diameter::mm(8.0).unwrap(), Pitch::mm_per_rev(3.0).unwrap());

        let travel = h.axial_travel(1.0);

        assert!(approx_eq(travel.mm_value(), 3.0, DEFAULT_EPS));
    }

    #[test]
    fn total_length_identity() {
        let h = Helix::new(Diameter::mm(6.0).unwrap(), Pitch::mm_per_rev(2.0).unwrap());

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
