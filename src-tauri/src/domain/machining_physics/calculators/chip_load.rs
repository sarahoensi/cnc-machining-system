// domain/machining_physics/calculators/chip_load.rs

use crate::domain::units::errors::UnitError;
use crate::domain::units::machining::ChipLoad;
use crate::domain::units::motion::{FeedRate, Rpm};

use crate::domain::machining_physics::tool::ToothCount;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::units::machining::ChipLoad;
    use crate::domain::units::motion::{FeedRate, Rpm};

    #[test]
    fn chipload_roundtrip() {
        let chip = ChipLoad::mm_per_tooth(0.05).unwrap();
        let rpm = Rpm::new(1000.0).unwrap();
        let teeth = ToothCount::new(4).unwrap();

        let feed = FeedRate::mm_per_min(chip.mm_per_tooth_value() * rpm.value() * 4.0).unwrap();
        let chip2 = ChipLoadCalculator::chip_load_from_feed_rate(feed, rpm, teeth).unwrap();

        assert!((chip2.mm_per_tooth_value() - 0.05).abs() < 1e-9);
    }
}
