// tests/domain/machining_physics/feed_rate_tests.rs

use cnc_machining_system_lib::domain::{units::*,*};
use cnc_machining_system_lib::test_utils::approx::{approx_eq, DEFAULT_EPS};

fn rpm(v: f64) -> Rpm {
    Rpm::new(v).unwrap()
}

fn chip(v: f64) -> ChipLoad {
    ChipLoad::mm_per_tooth(v).unwrap()
}

fn teeth(v: u32) -> ToothCount {
    ToothCount::new(v).unwrap()
}

#[test]
fn feed_rate_formula_correct() {
    let f = FeedRateCalculator::feed_rate_from_chip_load(
        chip(0.05),
        rpm(1000.0),
        teeth(4),
    )
    .unwrap();

    assert!(approx_eq(f.mm_per_min_value(), 200.0, DEFAULT_EPS));
}

#[test]
fn scaling_rpm_scales_feed() {
    let f1 = FeedRateCalculator::feed_rate_from_chip_load(
        chip(0.05),
        rpm(500.0),
        teeth(4),
    )
    .unwrap();

    let f2 = FeedRateCalculator::feed_rate_from_chip_load(
        chip(0.05),
        rpm(1000.0),
        teeth(4),
    )
    .unwrap();

    assert!(approx_eq(
        f2.mm_per_min_value(),
        f1.mm_per_min_value() * 2.0,
        DEFAULT_EPS
    ));
}

