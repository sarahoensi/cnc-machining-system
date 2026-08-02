// happy_path.rs

use cnc_machining_system_lib::interface::helix::{solve_helix, HelixMode, SolveHelixRequest};

#[test]
fn solves_from_pitch_outer() {
    let request = SolveHelixRequest::Pitch {
        mode: HelixMode::Outer,
        diameter: 10.0,
        tool_diameter: 2.0,
        pitch: 4.0,
    };

    let response = solve_helix(request).unwrap();

    assert!(response.pitch > 0.0);
    assert!(response.angle > 0.0);
    assert_close(response.pitch, 4.0);
    assert_close(response.angle, 6.60254999433);
}

#[test]
fn solves_from_angle_inner() {
    let request = SolveHelixRequest::Angle {
        mode: HelixMode::Inner,
        diameter: 10.0,
        tool_diameter: 2.0,
        angle: 20.0,
    };

    let response = solve_helix(request).unwrap();

    assert!(response.pitch > 0.0);
    assert_close(response.pitch, 10.2910159268645);
    assert_close(response.angle, 20.0);
}

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < 1e-9,
        "expected {actual} to be close to {expected}"
    );
}
