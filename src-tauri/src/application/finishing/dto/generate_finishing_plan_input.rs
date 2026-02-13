//! Input DTO for finishing plan generation.
//!
//! This module defines operator-facing planning modes accepted by the
//! finishing plan generation use case.


use crate::domain::FinishingMode;

/// Input DTO describing how to generate a finishing execution plan.
///
/// This is an application input contract. It transports the requested
/// finishing mode, start/target diameters, and planning strategy.
///
/// Validation expectations:
/// - Diameter values must satisfy domain diameter constraints.
/// - `cuts` and `radial_engagement_mm` must satisfy domain planning rules.
///
/// Unit expectations:
/// - All lengths are in millimeters (`mm`).
pub enum GenerateFinishingPlanInput {

    /// Plan by evenly distributing stock removal across a fixed number of cuts.
    ByCuts {
        /// Finishing strategy mode defined by the domain.
        mode: FinishingMode,
        /// Initial diameter before finishing (`mm`).
        start_diameter_mm: f64,
        /// Final target diameter after finishing (`mm`).
        target_diameter_mm: f64,
        /// Number of finishing cuts to schedule.
        cuts: u32,
    },

    /// Plan by using a fixed radial engagement for each step.
    ByRadialEngagement {
        /// Finishing strategy mode defined by the domain.
        mode: FinishingMode,
        /// Initial diameter before finishing (`mm`).
        start_diameter_mm: f64,
        /// Final target diameter after finishing (`mm`).
        target_diameter_mm: f64,
        /// Per-step radial engagement (`mm`).
        radial_engagement_mm: f64,
    },
}
