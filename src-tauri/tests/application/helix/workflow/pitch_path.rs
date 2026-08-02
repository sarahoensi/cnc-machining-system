use cnc_machining_system_lib::application::{SolveHelixInput, SolveHelixUseCase};
use cnc_machining_system_lib::domain::HelixMode;

#[test]
fn pitch_input_preserves_pitch() {
    let uc = SolveHelixUseCase;

    let input = SolveHelixInput::Pitch {
        mode: HelixMode::Outer,
        diameter: 10.0,
        tool_diameter: 2.0,
        pitch: 4.0,
    };

    let result = uc.execute(input).unwrap();

    assert!((result.pitch - 4.0).abs() < 1e-9);
    assert_close(result.angle, 6.60254999433);
}

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < 1e-9,
        "expected {actual} to be close to {expected}"
    );
}
