// domain/machining_physics/cutting_input.rs

use crate::domain::units::{ChipLoad, CuttingSpeed, FeedRate, Rpm};

/// Represents partially known machining parameters.
///
/// This type is used as input to machining calculation logic
/// where some values are known and others must be derived.
///
/// # Typical Relationships
///
/// - Cutting speed ↔ RPM
/// - Chip load ↔ Feed rate ↔ RPM ↔ Tooth count
///
/// At least one parameter in each relationship group is usually
/// required to compute the remaining values.
///
/// # Partial Input
///
/// All fields are optional to allow flexible calculation flows.
/// Validation and completeness checks are handled by application
/// or domain calculation services.
#[derive(Debug, Copy, Clone)]
pub struct CuttingInput {
    /// Surface cutting speed (m/min).
    pub cutting_speed: Option<CuttingSpeed>,

    /// Spindle speed (revolutions per minute).
    pub rpm: Option<Rpm>,

    /// Chip load per tooth (mm/tooth).
    pub chip_load: Option<ChipLoad>,

    /// Linear feed rate (mm/min).
    pub feed_rate: Option<FeedRate>,
}
