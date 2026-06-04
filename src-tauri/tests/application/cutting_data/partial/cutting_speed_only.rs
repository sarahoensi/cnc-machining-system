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
}
