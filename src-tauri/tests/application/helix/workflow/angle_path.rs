use cnc_machining_system_lib::application::{SolveHelixInput, SolveHelixUseCase};
use cnc_machining_system_lib::domain::HelixMode;

#[test]
fn angle_input_generates_consistent_pitch() {
    let uc = SolveHelixUseCase;

    let result = uc
        .execute(SolveHelixInput::Angle {
            mode: HelixMode::Outer,
            diameter: 10.0,
            tool_diameter: 2.0,
            angle: 20.0,
        })
        .unwrap();

    assert!(result.pitch > 0.0);
    assert_close(result.pitch, 12.5779083550566);
    assert_close(result.angle, 20.0);
}

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < 1e-9,
        "expected {actual} to be close to {expected}"
    );
}
