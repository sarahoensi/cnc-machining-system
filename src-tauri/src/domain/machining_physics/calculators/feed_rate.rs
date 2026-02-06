// domain/machining_physics/calculators/feed_rate.rs

use crate::domain::units::errors::UnitError;
use crate::domain::units::machining::ChipLoad;
use crate::domain::units::motion::{FeedRate, Rpm};

use crate::domain::machining_physics::tool::ToothCount;

pub struct FeedRateCalculator;

impl FeedRateCalculator {
    pub fn feed_rate_from_chip_load(
        chip_load: ChipLoad,
        rpm: Rpm,
        teeth: ToothCount,
    ) -> Result<FeedRate, UnitError> {
        let f = chip_load.mm_per_tooth_value() * rpm.value() * teeth.value() as f64;
        FeedRate::mm_per_min(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::units::machining::ChipLoad;
    use crate::domain::units::motion::Rpm;

    #[test]
    fn feed_is_chipload_times_rpm_times_teeth() {
        let chip = ChipLoad::mm_per_tooth(0.05).unwrap();
        let rpm = Rpm::new(1000.0).unwrap();
        let teeth = ToothCount::new(4).unwrap();

        let feed = FeedRateCalculator::feed_rate_from_chip_load(chip, rpm, teeth).unwrap();
        assert!((feed.mm_per_min_value() - 200.0).abs() < 1e-9);
    }
}
