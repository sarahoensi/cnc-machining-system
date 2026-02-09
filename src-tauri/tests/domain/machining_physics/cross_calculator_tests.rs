// tests/domain/machining_physics/cross_calculator_tests.rs

use cnc_machining_system_lib::domain::*;
use proptest::prelude::*;

use super::property_strategies::*;

fn approx(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

proptest! {

    #[test]
    fn full_chain_consistency(
        d in diameter(),
        vc in cutting_speed(),
        chip in chip_load(),
        z in tooth_count()
    ) {

        let rpm = SpindleSpeedCalculator::rpm_from_cutting_speed(vc, d).unwrap();

        let feed = FeedRateCalculator::feed_rate_from_chip_load(chip, rpm, z).unwrap();

        let chip2 = ChipLoadCalculator::chip_load_from_feed_rate(feed, rpm, z).unwrap();

        prop_assert!(approx(chip.mm_per_tooth_value(), chip2.mm_per_tooth_value()));
    }

}
