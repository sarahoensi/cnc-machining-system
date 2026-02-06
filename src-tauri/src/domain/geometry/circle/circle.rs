// domain/geometry/circle/circle.rs

use std::f64::consts::PI;

use crate::domain::units::length::{Diameter, Radius, Length};

/// Represents a mathematically valid circle.
///
/// Invariants:
/// - radius > 0
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Circle {
    radius: Radius,
}

impl Circle {
    // ---------------------------------------------------------
    // Constructors
    // ---------------------------------------------------------

    pub fn from_radius(radius: Radius) -> Self {
        Self { radius }
    }

    pub fn from_diameter(diameter: Diameter) -> Self {
        Self {
            radius: diameter.radius(),
        }
    }

    // ---------------------------------------------------------
    // Accessors
    // ---------------------------------------------------------

    pub fn radius(&self) -> Radius {
        self.radius
    }

    pub fn diameter(&self) -> Diameter {
        self.radius.diameter()
    }

    // ---------------------------------------------------------
    // Derived geometric properties
    // ---------------------------------------------------------

    /// Circumference = 2πr
    pub fn circumference(&self) -> Length {
        let value = 2.0 * PI * self.radius.mm_value();
        Length::mm(value).unwrap()
    }

    /// Area = πr²
    pub fn area(&self) -> f64 {
        PI * self.radius.mm_value().powi(2)
    }

    /// Arc length = r * θ
    pub fn arc_length(&self, angle: crate::domain::units::angle::Angle) -> Length {
        let value = self.radius.mm_value() * angle.radians_value();
        Length::mm(value).unwrap()
    }

    /// Sector area = (θ / 2π) * circle area
    pub fn sector_area(&self, angle: crate::domain::units::angle::Angle) -> f64 {
        let fraction = angle.radians_value() / (2.0 * PI);
        self.area() * fraction
    }
}
