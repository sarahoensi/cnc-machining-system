// tests/domain/machining_physics/chip_load_tests.rs

use cnc_machining_system_lib::domain::tool::ToothCount;
use cnc_machining_system_lib::domain::*;
use cnc_machining_system_lib::test_utils::approx::{approx_eq, DEFAULT_EPS};

fn rpm(v: f64) -> Rpm {
    Rpm::new(v).unwrap()
}

fn chip(v: f64) -> ChipLoad {
    ChipLoad::mm_per_tooth(v).unwrap()
}

fn feed(v: f64) -> FeedRate {
    FeedRate::mm_per_min(v).unwrap()
}

fn teeth(v: u32) -> ToothCount {
    ToothCount::new(v).unwrap()
}

#[test]
fn chipload_basic_formula() {
    let result = ChipLoadCalculator::chip_load_from_feed_rate(
        feed(200.0),
        rpm(1000.0),
        teeth(4),
    )
    .unwrap();

    assert!(approx_eq(result.mm_per_tooth_value(), 0.05, DEFAULT_EPS));
}

#[test]
fn chipload_roundtrip_with_feed_calculator() {
    let original_chip = chip(0.07);

    let feed = FeedRateCalculator::feed_rate_from_chip_load(
        original_chip,
        rpm(800.0),
        teeth(3),
    )
    .unwrap();

    let reconstructed = ChipLoadCalculator::chip_load_from_feed_rate(
        feed,
        rpm(800.0),
        teeth(3),
    )
    .unwrap();

    assert!(approx_eq(
        original_chip.mm_per_tooth_value(),
        reconstructed.mm_per_tooth_value(),
        DEFAULT_EPS
    ));
}

#[test]
fn rejects_zero_teeth() {
    let teeth = ToothCount::new(0);
    assert!(teeth.is_err());
}

#[test]
fn scaling_feed_scales_chipload() {
    let c1 = ChipLoadCalculator::chip_load_from_feed_rate(
        feed(100.0),
        rpm(1000.0),
        teeth(4),
    )
    .unwrap();

    let c2 = ChipLoadCalculator::chip_load_from_feed_rate(
        feed(200.0),
        rpm(1000.0),
        teeth(4),
    )
    .unwrap();

    assert!(approx_eq(
        c2.mm_per_tooth_value(),
        c1.mm_per_tooth_value() * 2.0,
        DEFAULT_EPS
    ));
}
