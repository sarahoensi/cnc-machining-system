use cnc_machining_system_lib::application::{SolveHelixInput, SolveHelixUseCase};
use cnc_machining_system_lib::domain::HelixMode;

#[test]
fn output_contains_consistent_geometry_values() {

    let uc = SolveHelixUseCase;

    let result = uc.execute(
        SolveHelixInput::Pitch {
            mode: HelixMode::Outer,
            diameter_mm: 10.0,
            tool_diameter_mm: 2.0,
            pitch_mm_per_rev: 4.0,
        }
    ).unwrap();

    assert!(result.circumference_mm > 0.0);
    assert!(result.angle_deg > 0.0);
}
