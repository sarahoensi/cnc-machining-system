// tests/application/cutting_data/partial/rpm_only.rs

use cnc_machining_system_lib::application::{SolveCuttingDataInput, SolveCuttingDataUseCase};

#[test]
fn solves_cutting_speed_from_rpm_and_diameter() {
    let output = SolveCuttingDataUseCase::execute(SolveCuttingDataInput {
        rpm: Some(5000.0),
        diameter_mm: Some(10.0),
        ..Default::default()
    })
    .unwrap();

    assert!(output.cutting_speed_m_per_min.is_some());
    assert!(output.feed_rate_mm_per_min.is_none());
    assert!(output.chip_load_mm_per_tooth.is_none());
    assert_close(output.cutting_speed_m_per_min.unwrap(), 157.07963267948966);
}

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < 1e-9,
        "expected {actual} to be close to {expected}"
    );
}
