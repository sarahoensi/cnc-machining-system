// domain/machining_physics/tool.rs

use crate::domain::units::{
    Diameter,
    ToothCount,
};

/// Represents a physical cutting tool used in machining.
///
/// A tool is defined by:
/// - Tool diameter
/// - Number of cutting teeth
///
/// This is a Value Object.
///
/// Invariants:
/// - Diameter must be positive and finite (enforced by `Diameter`)
/// - Tooth count must be > 0 (enforced by `ToothCount`)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Tool {
    diameter: Diameter,
    teeth: ToothCount,
}

impl Tool {

    /// Creates a new validated tool definition.
    ///
    /// All invariants are enforced by the value objects.
    pub fn new(
        diameter: Diameter,
        teeth: ToothCount,
    ) -> Self {
        Self { diameter, teeth }
    }

    /// Returns the tool diameter.
    pub fn diameter(&self) -> Diameter {
        self.diameter
    }

    /// Returns the number of cutting teeth.
    pub fn teeth(&self) -> ToothCount {
        self.teeth
    }
}