// tests/application/cutting_data/partial/cross_path.rs

use cnc_machining_system_lib::application::{SolveCuttingDataInput, SolveCuttingDataUseCase};

#[test]
fn different_inputs_produce_consistent_feed_rate() {
    let a = SolveCuttingDataUseCase::execute(SolveCuttingDataInput {
        cutting_speed_m_per_min: Some(200.0),
        diameter_mm: Some(10.0),
        chip_load_mm_per_tooth: Some(0.05),
        teeth: Some(4),
        ..Default::default()
    })
    .unwrap();

    let b = SolveCuttingDataUseCase::execute(SolveCuttingDataInput {
        rpm: a.rpm,
        chip_load_mm_per_tooth: Some(0.05),
        teeth: Some(4),
        diameter_mm: Some(10.0),
        ..Default::default()
    })
    .unwrap();

    let feed_a = a.feed_rate_mm_per_min.unwrap();
    let feed_b = b.feed_rate_mm_per_min.unwrap();

    assert!((feed_a - feed_b).abs() < 1e-9);
}
