// tests/interface/tauri/right_triangle/error_path.rs
use cnc_machining_system_lib::interface::{
    solve_right_triangle,
};

use cnc_machining_system_lib::interface::tauri::right_triangle::{
    SolveRightTriangleRequest,
};

#[test]
fn fails_when_lengths_are_invalid() {

    let request = SolveRightTriangleRequest::Legs {
        a_mm: 0.0,
        b_mm: 4.0,
    };

    let result = solve_right_triangle(request);

    assert!(result.is_err());
}

#[test]
fn fails_when_hypotenuse_is_smaller_than_leg() {

    let request = SolveRightTriangleRequest::LegAndHypotenuse {
        a_mm: 5.0,
        c_mm: 3.0,
    };

    let result = solve_right_triangle(request);

    assert!(result.is_err());
}

#[test]
fn fails_when_angle_is_invalid() {

    let request = SolveRightTriangleRequest::HypotenuseAndAngle {
        c_mm: 10.0,
        alpha_deg: 0.0,
    };

    let result = solve_right_triangle(request);

    assert!(result.is_err());
}
