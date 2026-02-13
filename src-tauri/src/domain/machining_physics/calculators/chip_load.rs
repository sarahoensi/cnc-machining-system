// domain/machining_physics/calculators/chip_load.rs

use crate::domain::UnitError;
use crate::domain::{ChipLoad, FeedRate, Rpm, ToothCount};

/// Computes chip load from feed, spindle speed, and tooth count.
pub struct ChipLoadCalculator;

impl ChipLoadCalculator {
    /// Derives per-tooth material engagement from programmed feed conditions.
    ///
    /// Uses `fz = F / (n * z)`, where:
    /// - `fz` is chip load in mm/tooth,
    /// - `F` is feed rate in mm/min,
    /// - `n` is spindle speed in RPM,
    /// - `z` is tooth count.
    ///
    /// # Errors
    ///
    /// Returns `UnitError::NonPositiveValue` when `n * z` is non-positive or non-finite.
    /// Returns `UnitError` if the computed chip load violates `ChipLoad` unit invariants.
    pub fn chip_load_from_feed_rate(
        feed: FeedRate,
        rpm: Rpm,
        teeth: ToothCount,
    ) -> Result<ChipLoad, UnitError> {
        let denom = rpm.value() * teeth.value() as f64;
        if denom <= 0.0 || !denom.is_finite() {
            return Err(UnitError::NonPositiveValue("ChipLoad denominator"));
        }

        let chip = feed.mm_per_min_value() / denom;
        ChipLoad::mm_per_tooth(chip)
    }
}

