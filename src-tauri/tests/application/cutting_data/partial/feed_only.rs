// application/cutting_data/partial/feed_only.rs

use cnc_machining_system_lib::application::{SolveCuttingDataInput, SolveCuttingDataUseCase};

#[test]
fn solves_chip_load_from_feed_rpm_and_teeth() {

    let output = SolveCuttingDataUseCase::execute(
        SolveCuttingDataInput {
            feed_rate_mm_per_min: Some(800.0),
            rpm: Some(5000.0),
            teeth: Some(4),
            ..Default::default()
        }
    ).unwrap();

    assert!(output.chip_load_mm_per_tooth.is_some());
}
