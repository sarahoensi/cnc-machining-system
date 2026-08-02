// tests/application/cutting_data/cutting_speed_only.rs

use cnc_machining_system_lib::application::{SolveCuttingDataInput, SolveCuttingDataUseCase};

#[test]
fn solves_rpm_from_cutting_speed_and_diameter() {
    let output = SolveCuttingDataUseCase::execute(SolveCuttingDataInput {
        cutting_speed_m_per_min: Some(200.0),
        diameter_mm: Some(10.0),
        ..Default::default()
    })
    .unwrap();

    assert!(output.rpm.is_some());
    assert_close(output.rpm.unwrap(), 6366.197723675814);
}

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < 1e-9,
        "expected {actual} to be close to {expected}"
    );
}
