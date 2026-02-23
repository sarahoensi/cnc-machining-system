// domain/machining_physics/tool.rs

use crate::domain::units::Diameter;

use super::MachiningPhysicsError;

/// Number of cutting teeth on a tool.
///
/// Invariant:
/// - Must be strictly greater than zero.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ToothCount(u32);

impl ToothCount {
    /// Creates a new validated tooth count.
    pub fn new(value: u32) -> Result<Self, MachiningPhysicsError> {
        if value == 0 {
            return Err(MachiningPhysicsError::InvalidToothCount {
                value,
            });
        }

        Ok(Self(value))
    }

    /// Returns the number of teeth.
    pub fn value(self) -> u32 {
        self.0
    }
}

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
    /// Diameter invariants are enforced by `Diameter`.
    /// Tooth count invariants are enforced by `ToothCount`.
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