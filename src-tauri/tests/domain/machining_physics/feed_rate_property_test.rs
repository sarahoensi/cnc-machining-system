// tests/domain/machining_physics/feed_rate_tests.rs

use proptest::prelude::*;
use cnc_machining_system_lib::domain::*;
use cnc_machining_system_lib::test_utils::approx::{approx_eq};


use super::property_strategies::*;

proptest! {
    #[test]
    fn feed_rate_and_chipload_are_inverse(
        chip in chip_load(),
        n in rpm(),
        z in tooth_count(),
    ) {

        let f = FeedRateCalculator::feed_rate_from_chip_load(chip, n, z).unwrap();

        let chip2 = ChipLoadCalculator::chip_load_from_feed_rate(f, n, z).unwrap();

        prop_assert!(approx_eq(
            chip2.mm_per_tooth_value(),
            chip.mm_per_tooth_value(),
            1e-9
        ));
    }
}
