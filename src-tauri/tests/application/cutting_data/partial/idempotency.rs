// tests/applocation//cutting_data/partial/idempotency.rs

use cnc_machining_system_lib::application::{SolveCuttingDataInput, SolveCuttingDataUseCase};

#[test]
fn solving_twice_does_not_change_result() {
    let first = SolveCuttingDataUseCase::execute(SolveCuttingDataInput {
        rpm: Some(5000.0),
        diameter_mm: Some(10.0),
        ..Default::default()
    })
    .unwrap();

    let second = SolveCuttingDataUseCase::execute(SolveCuttingDataInput {
        rpm: first.rpm,
        cutting_speed_m_per_min: first.cutting_speed_m_per_min,
        chip_load_mm_per_tooth: first.chip_load_mm_per_tooth,
        feed_rate_mm_per_min: first.feed_rate_mm_per_min,
        diameter_mm: Some(10.0),
        ..Default::default()
    })
    .unwrap();

    assert_eq!(first.feed_rate_mm_per_min, second.feed_rate_mm_per_min);
}
