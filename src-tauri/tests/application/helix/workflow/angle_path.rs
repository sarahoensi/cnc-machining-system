
use cnc_machining_system_lib::application::{SolveHelixInput, SolveHelixUseCase};
use cnc_machining_system_lib::domain::HelixMode;


#[test]
fn angle_input_generates_consistent_pitch() {

    let uc = SolveHelixUseCase;

    let result = uc.execute(
        SolveHelixInput::Angle {
            mode: HelixMode::Outer,
            diameter_mm: 10.0,
            tool_diameter_mm: 2.0,
            angle_deg: 20.0,
        }
    ).unwrap();

    assert!(result.pitch_mm_per_rev > 0.0);
}
