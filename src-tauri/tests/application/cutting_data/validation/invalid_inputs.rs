// tests/application/cutting_data/validation/valid_inputs.rs

use cnc_machining_system_lib::application::{SolveCuttingDataInput, SolveCuttingDataUseCase};

#[test]
fn fails_when_diameter_is_zero() {

    let result = SolveCuttingDataUseCase::execute(
        SolveCuttingDataInput {
            cutting_speed_m_per_min: Some(200.0),
            diameter_mm: Some(0.0),
            ..Default::default()
        }
    );

    assert!(result.is_err());
}

#[test]
fn fails_when_teeth_is_zero() {

    let result = SolveCuttingDataUseCase::execute(
        SolveCuttingDataInput {
            rpm: Some(5000.0),
            chip_load_mm_per_tooth: Some(0.05),
            teeth: Some(0),
            diameter_mm: Some(10.0),
            ..Default::default()
        }
    );

    assert!(result.is_err());
}
