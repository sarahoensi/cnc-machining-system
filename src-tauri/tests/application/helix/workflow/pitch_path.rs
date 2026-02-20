use cnc_machining_system_lib::application::{SolveHelixInput, SolveHelixUseCase};
use cnc_machining_system_lib::domain::HelixMode;


#[test]
fn pitch_input_preserves_pitch() {

    let uc = SolveHelixUseCase;

    let input = SolveHelixInput::Pitch {
        mode: HelixMode::Outer,
        diameter_mm: 10.0,
        tool_diameter_mm: 2.0,
        pitch_mm_per_rev: 4.0,
    };

    let result = uc.execute(input).unwrap();

    assert!((result.pitch_mm_per_rev - 4.0).abs() < 1e-9);
}
