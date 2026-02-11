// tests/application/cutting_data/from_feed_rate.rs

use cnc_machining_system_lib::application::SolveCuttingDataInput;
use cnc_machining_system_lib::application::SolveCuttingDataUseCase;

#[test]
fn solves_pipeline_from_feed_rate() {
    let output = SolveCuttingDataUseCase::execute(
        SolveCuttingDataInput::FromFeedRate {
            feed_rate_mm_per_min: 800.0,
            rpm: 5000.0,
            teeth: 4,
            diameter_mm: 10.0,
        }
    ).unwrap();

    assert!(output.chip_load_mm_per_tooth > 0.0);
    assert!(output.cutting_speed_m_per_min > 0.0);
}
