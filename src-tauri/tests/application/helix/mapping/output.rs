use cnc_machining_system_lib::application::{SolveHelixInput, SolveHelixUseCase};
use cnc_machining_system_lib::domain::HelixMode;

#[test]
fn output_contains_consistent_geometry_values() {
    let uc = SolveHelixUseCase;

    let result = uc
        .execute(SolveHelixInput::Pitch {
            mode: HelixMode::Outer,
            diameter: 10.0,
            tool_diameter: 2.0,
            pitch: 4.0,
        })
        .unwrap();

    assert!(result.angle > 0.0);
}
