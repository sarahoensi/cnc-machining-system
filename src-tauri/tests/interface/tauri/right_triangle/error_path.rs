// tests/interface/tauri/right_triangle/error_path.rs

use cnc_machining_system_lib::interface::tauri::right_triangle::{
    solve_right_triangle,
    SolveRightTriangleRequest,
};

#[test]
fn fails_when_lengths_are_invalid() {

    let request = SolveRightTriangleRequest::Legs {
        a_mm: 0.0, // invalid
        b_mm: 4.0,
    };

    let result = solve_right_triangle(request);

    assert!(result.is_err());
}

#[test]
fn fails_when_hypotenuse_is_smaller_than_leg() {

    let request = SolveRightTriangleRequest::LegAAndHypotenuse {
        a_mm: 5.0,
        c_mm: 3.0, // impossible
    };

    let result = solve_right_triangle(request);

    assert!(result.is_err());
}

#[test]
fn fails_when_angle_is_zero() {

    let request = SolveRightTriangleRequest::HypotenuseAndAlpha {
        c_mm: 10.0,
        alpha_deg: 0.0, // invalid (must be > 0 and < 90)
    };

    let result = solve_right_triangle(request);

    assert!(result.is_err());
}

#[test]
fn fails_when_angle_is_ninety() {

    let request = SolveRightTriangleRequest::HypotenuseAndAlpha {
        c_mm: 10.0,
        alpha_deg: 90.0, // invalid
    };

    let result = solve_right_triangle(request);

    assert!(result.is_err());
}

#[test]
fn fails_when_beta_is_invalid() {

    let request = SolveRightTriangleRequest::LegAAndBeta {
        a_mm: 5.0,
        beta_deg: 90.0, // invalid
    };

    let result = solve_right_triangle(request);

    assert!(result.is_err());
}

#[test]
fn fails_when_leg_is_negative() {

    let request = SolveRightTriangleRequest::LegBAndAlpha {
        b_mm: -2.0, // invalid
        alpha_deg: 30.0,
    };

    let result = solve_right_triangle(request);

    assert!(result.is_err());
}