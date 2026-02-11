// tests/application/cutting_data/from_cutting_speed.rs

use cnc_machining_system_lib::application::SolveCuttingDataInput;
use cnc_machining_system_lib::application::SolveCuttingDataUseCase;

#[test]
fn solves_pipeline_from_cutting_speed() {
    let output = SolveCuttingDataUseCase::execute(
        SolveCuttingDataInput::FromCuttingSpeed {
            cutting_speed_m_per_min: 200.0,
            diameter_mm: 10.0,
            chip_load_mm_per_tooth: 0.05,
            teeth: 4,
        }
    ).unwrap();

    assert!(output.rpm > 0.0);
    assert!(output.feed_rate_mm_per_min > 0.0);
}
