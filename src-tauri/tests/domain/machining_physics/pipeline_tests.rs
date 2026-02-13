// tests/domain/machining_physics/pipeline_tests.rs

use cnc_machining_system_lib::domain::{units::*,*};
use cnc_machining_system_lib::test_utils::approx::{approx_eq, DEFAULT_EPS};

#[test]
fn full_cutting_pipeline_roundtrip() {
    let tool = Tool::new(
        Diameter::mm(10.0).unwrap(),
        ToothCount::new(4).unwrap(),
    );

    let vc = CuttingSpeed::meters_per_min(120.0).unwrap();
    let chip = ChipLoad::mm_per_tooth(0.04).unwrap();

    let rpm = SpindleSpeedCalculator::rpm_from_cutting_speed(vc, tool.diameter()).unwrap();

    let feed = FeedRateCalculator::feed_rate_from_chip_load(
        chip,
        rpm,
        tool.teeth(),
    )
    .unwrap();

    let chip_back = ChipLoadCalculator::chip_load_from_feed_rate(
        feed,
        rpm,
        tool.teeth(),
    )
    .unwrap();

    assert!(approx_eq(
        chip.mm_per_tooth_value(),
        chip_back.mm_per_tooth_value(),
        DEFAULT_EPS
    ));
}
