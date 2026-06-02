use cnc_machining_system_lib::application::{SolveHelixInput, SolveHelixUseCase};
use cnc_machining_system_lib::domain::HelixMode;

#[test]
fn fails_when_diameter_is_invalid() {
    let uc = SolveHelixUseCase;

    let result = uc.execute(SolveHelixInput::Pitch {
        mode: HelixMode::Outer,
        diameter: 0.0,
        tool_diameter: 2.0,
        pitch: 4.0,
    });

    assert!(result.is_err());
}
