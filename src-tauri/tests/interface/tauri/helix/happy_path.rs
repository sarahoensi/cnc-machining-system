// happy_path.rs

use cnc_machining_system_lib::interface::{
    SolveHelixRequest, solve_helix, tauri::helix::HelixMode
};


#[test]
fn solves_from_pitch_outer() {

    let request = SolveHelixRequest::Pitch {
        mode: HelixMode::Outer,
        diameter_mm: 10.0,
        tool_diameter_mm: 2.0,
        pitch_mm_per_rev: 4.0,
    };

    let response = solve_helix(request).unwrap();

    assert!(response.pitch_mm_per_rev > 0.0);
    assert!(response.effective_diameter_mm > 0.0);
    assert!(response.angle_deg > 0.0);
}

#[test]
fn solves_from_angle_inner() {

    let request = SolveHelixRequest::Angle {
        mode: HelixMode::Inner,
        diameter_mm: 10.0,
        tool_diameter_mm: 2.0,
        angle_deg: 20.0,
    };

    let response = solve_helix(request).unwrap();

    assert!(response.pitch_mm_per_rev > 0.0);
    assert!(response.circumference_mm > 0.0);
}
