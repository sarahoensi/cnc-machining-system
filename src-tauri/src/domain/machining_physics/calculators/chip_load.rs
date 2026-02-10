// domain/machining_physics/calculators/chip_load.rs

use crate::domain::UnitError;
use crate::domain::{ChipLoad, FeedRate, Rpm, ToothCount};

pub struct ChipLoadCalculator;

impl ChipLoadCalculator {
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

