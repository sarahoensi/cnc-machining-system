// domain/machining_physics/cutting_result.rs

use crate::domain::{ChipLoad, CuttingSpeed, FeedRate, Rpm};

/// Represents a consistent set of calculated machining parameters.
///
/// A `CuttingResult` bundles together values derived from cutting
/// physics formulas and tool configuration.
///
/// The values are mathematically linked:
///
/// - Cutting speed ↔ RPM ↔ Tool diameter
/// - Feed rate ↔ Chip load ↔ RPM ↔ Tooth count
///
/// Instances of this type are typically produced by machining
/// calculation services or pipelines.
#[derive(Debug, Copy, Clone)]
pub struct CuttingResult {
    /// Surface cutting speed (m/min).
    pub cutting_speed: CuttingSpeed,

    /// Spindle speed (revolutions per minute).
    pub rpm: Rpm,

    /// Chip load per tooth (mm/tooth).
    pub chip_load: ChipLoad,

    /// Linear feed rate (mm/min).
    pub feed_rate: FeedRate,
}
