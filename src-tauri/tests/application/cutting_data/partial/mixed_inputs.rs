// tests/application/cutting_data/partial/mixed_inputs.rs

use cnc_machining_system_lib::application::{SolveCuttingDataInput, SolveCuttingDataUseCase};

#[test]
fn solves_full_pipeline_when_enough_data_is_provided() {
    let output = SolveCuttingDataUseCase::execute(SolveCuttingDataInput {
        cutting_speed_m_per_min: Some(200.0),
        diameter_mm: Some(10.0),
        chip_load_mm_per_tooth: Some(0.05),
        teeth: Some(4),
        ..Default::default()
    })
    .unwrap();

    assert!(output.rpm.is_some());
    assert!(output.feed_rate_mm_per_min.is_some());
}
