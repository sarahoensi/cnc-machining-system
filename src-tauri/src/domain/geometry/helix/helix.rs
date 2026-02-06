// domain/geometry/helix/helix.rs

use std::f64::consts::PI;

use crate::domain::units::angle::Angle;
use crate::domain::units::length::{Diameter, Pitch, Length};

/// Represents a cylindrical helix.
///
/// Invariants:
/// - diameter > 0
/// - pitch > 0
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Helix {
    diameter: Diameter,
    pitch: Pitch,
}

impl Helix {
    // ---------------------------------------------------------
    // Constructor
    // ---------------------------------------------------------

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

    /// Circumference of helix cylinder
    pub fn circumference(&self) -> Length {
        let value = PI * self.diameter.mm_value();
        Length::mm(value).unwrap()
    }

    /// Helix angle
    ///
    /// angle = atan(pitch / circumference)
    pub fn helix_angle(&self) -> Angle {
        let c = self.circumference().mm_value();
        let p = self.pitch.mm_per_rev_value();

        let angle = (p / c).atan();
        Angle::radians(angle).unwrap()
    }

    /// Helix length per single revolution
    ///
    /// sqrt(circumference² + pitch²)
    pub fn length_per_revolution(&self) -> Length {
        let c = self.circumference().mm_value();
        let p = self.pitch.mm_per_rev_value();

        let value = (c.powi(2) + p.powi(2)).sqrt();
        Length::mm(value).unwrap()
    }

    /// Total helix length for given number of revolutions
    pub fn total_length(&self, revolutions: f64) -> Length {
        let single = self.length_per_revolution().mm_value();
        Length::mm(single * revolutions).unwrap()
    }

    /// Axial travel for given revolutions
    pub fn axial_travel(&self, revolutions: f64) -> Length {
        let value = self.pitch.mm_per_rev_value() * revolutions;
        Length::mm(value).unwrap()
    }
}
