use cnc_machining_system_lib::application::{HelixMode, SolveHelixInput, SolveHelixUseCase};

#[test]
fn fails_when_diameter_is_invalid() {

    let uc = SolveHelixUseCase;

    let result = uc.execute(
        SolveHelixInput::Pitch {
            mode: HelixMode::Outer,
            diameter_mm: 0.0,
            tool_diameter_mm: 2.0,
            pitch_mm_per_rev: 4.0,
        }
    );

    assert!(result.is_err());
}
