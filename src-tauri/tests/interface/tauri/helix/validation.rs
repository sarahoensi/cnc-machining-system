// validation.rs

use cnc_machining_system_lib::interface::helix::{
    solve_helix,
    SolveHelixRequest,
};

use cnc_machining_system_lib::interface::tauri::helix::HelixMode;

#[test]
fn fails_when_diameter_is_invalid() {

    let request = SolveHelixRequest::Pitch {
        mode: HelixMode::Outer,
        diameter: 0.0, // invalid
        tool_diameter: 2.0,
        pitch: 4.0,
    };

    let result = solve_helix(request);

    assert!(result.is_err());
}

#[test]
fn fails_when_angle_is_invalid() {

    let request = SolveHelixRequest::Angle {
        mode: HelixMode::Inner,
        diameter: 10.0,
        tool_diameter: 2.0,
        angle: 90.0, // tan(90°) → invalid pitch
    };

    let result = solve_helix(request);

    assert!(result.is_err());
}
