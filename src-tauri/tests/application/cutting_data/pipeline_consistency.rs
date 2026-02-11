// tests/application/cutting_data/pipeline_consistency.rs

use cnc_machining_system_lib::application::{SolveCuttingDataInput, SolveCuttingDataUseCase};

#[test]
fn solving_from_different_inputs_produces_consistent_results() {
    let from_vc = SolveCuttingDataUseCase::execute(
        SolveCuttingDataInput::FromCuttingSpeed {
            cutting_speed_m_per_min: 200.0,
            diameter_mm: 10.0,
            chip_load_mm_per_tooth: 0.05,
            teeth: 4,
        }
    ).unwrap();

    let from_rpm = SolveCuttingDataUseCase::execute(
        SolveCuttingDataInput::FromRpm {
            rpm: from_vc.rpm,
            chip_load_mm_per_tooth: 0.05,
            teeth: 4,
            diameter_mm: 10.0,
        }
    ).unwrap();

    assert!((from_vc.feed_rate_mm_per_min - from_rpm.feed_rate_mm_per_min).abs() < 1e-9);
}
