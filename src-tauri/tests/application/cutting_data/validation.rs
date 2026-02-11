// tests/application/cutting_data/validation.rs

use cnc_machining_system_lib::application::SolveCuttingDataInput;
use cnc_machining_system_lib::application::SolveCuttingDataUseCase;

#[test]
fn fails_when_diameter_is_zero() {
    let result = SolveCuttingDataUseCase::execute(
        SolveCuttingDataInput::FromCuttingSpeed {
            cutting_speed_m_per_min: 200.0,
            diameter_mm: 0.0,
            chip_load_mm_per_tooth: 0.05,
            teeth: 4,
        }
    );

    assert!(result.is_err());
}

#[test]
fn fails_when_teeth_is_zero() {
    let result = SolveCuttingDataUseCase::execute(
        SolveCuttingDataInput::FromRpm {
            rpm: 5000.0,
            chip_load_mm_per_tooth: 0.05,
            teeth: 0,
            diameter_mm: 10.0,
        }
    );

    assert!(result.is_err());
}
