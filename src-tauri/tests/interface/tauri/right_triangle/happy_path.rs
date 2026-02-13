// tests/interface/tauri/right_triangle/happy_path.rs

use cnc_machining_system_lib::interface::right_triangle::{
    solve_right_triangle,
};

use cnc_machining_system_lib::interface::tauri::right_triangle::{
    SolveRightTriangleRequest,
};

#[test]
fn solves_triangle_from_legs() {

    let request = SolveRightTriangleRequest::Legs {
        a_mm: 3.0,
        b_mm: 4.0,
    };

    let response = solve_right_triangle(request)
        .expect("command should succeed");

    assert!((response.c_mm - 5.0).abs() < 1e-9);
    assert!((response.alpha_deg + response.beta_deg - 90.0).abs() < 1e-9);
}

#[test]
fn solves_triangle_from_leg_and_hypotenuse() {

    let request = SolveRightTriangleRequest::LegAndHypotenuse {
        a_mm: 3.0,
        c_mm: 5.0,
    };

    let response = solve_right_triangle(request).unwrap();

    assert!((response.b_mm - 4.0).abs() < 1e-9);
}

#[test]
fn solves_triangle_from_other_leg_and_hypotenuse() {

    let request = SolveRightTriangleRequest::OtherLegAndHypotenuse {
        b_mm: 4.0,
        c_mm: 5.0,
    };

    let response = solve_right_triangle(request).unwrap();

    assert!((response.a_mm - 3.0).abs() < 1e-9);
}

#[test]
fn solves_triangle_from_hypotenuse_and_angle() {

    let request = SolveRightTriangleRequest::HypotenuseAndAngle {
        c_mm: 10.0,
        alpha_deg: 30.0,
    };

    let response = solve_right_triangle(request).unwrap();

    assert!(response.a_mm > 0.0);
    assert!(response.b_mm > 0.0);
}
