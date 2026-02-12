// validation.rs

use cnc_machining_system_lib::interface::{
    solve_helix,
    SolveHelixRequest,
};

use cnc_machining_system_lib::interface::tauri::helix::HelixMode;

#[test]
fn fails_when_diameter_is_invalid() {

    let request = SolveHelixRequest::Pitch {
        mode: HelixMode::Outer,
        diameter_mm: 0.0, // invalid
        tool_diameter_mm: 2.0,
        pitch_mm_per_rev: 4.0,
    };

    let result = solve_helix(request);

    assert!(result.is_err());
}

#[test]
fn fails_when_angle_is_invalid() {

    let request = SolveHelixRequest::Angle {
        mode: HelixMode::Inner,
        diameter_mm: 10.0,
        tool_diameter_mm: 2.0,
        angle_deg: 90.0, // tan(90°) → invalid pitch
    };

    let result = solve_helix(request);

    assert!(result.is_err());
}
