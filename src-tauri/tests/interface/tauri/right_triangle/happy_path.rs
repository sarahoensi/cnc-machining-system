// tests/interface/tauri/right_triangle/happy_path.rs

use cnc_machining_system_lib::interface::tauri::right_triangle::{
    solve_right_triangle, SolveRightTriangleRequest,
};

const EPS: f64 = 1e-9;

#[test]
fn solves_triangle_from_legs() {
    let request = SolveRightTriangleRequest::Legs {
        a_mm: 3.0,
        b_mm: 4.0,
    };

    let response = solve_right_triangle(request).expect("command should succeed");

    assert!((response.c_mm - 5.0).abs() < EPS);
    assert!((response.alpha_deg + response.beta_deg - 90.0).abs() < EPS);
}

#[test]
fn solves_triangle_from_leg_a_and_hypotenuse() {
    let request = SolveRightTriangleRequest::LegAAndHypotenuse {
        a_mm: 3.0,
        c_mm: 5.0,
    };

    let response = solve_right_triangle(request).unwrap();

    assert!((response.b_mm - 4.0).abs() < EPS);
}

#[test]
fn solves_triangle_from_leg_b_and_hypotenuse() {
    let request = SolveRightTriangleRequest::LegBAndHypotenuse {
        b_mm: 4.0,
        c_mm: 5.0,
    };

    let response = solve_right_triangle(request).unwrap();

    assert!((response.a_mm - 3.0).abs() < EPS);
}

#[test]
fn solves_triangle_from_hypotenuse_and_alpha() {
    let request = SolveRightTriangleRequest::HypotenuseAndAlpha {
        c_mm: 10.0,
        alpha_deg: 30.0,
    };

    let response = solve_right_triangle(request).unwrap();

    assert!((response.alpha_deg - 30.0).abs() < EPS);
    assert!((response.beta_deg - 60.0).abs() < EPS);
    assert!((response.alpha_deg + response.beta_deg - 90.0).abs() < EPS);
}

#[test]
fn solves_triangle_from_hypotenuse_and_beta() {
    let request = SolveRightTriangleRequest::HypotenuseAndBeta {
        c_mm: 10.0,
        beta_deg: 60.0,
    };

    let response = solve_right_triangle(request).unwrap();

    assert!((response.alpha_deg - 30.0).abs() < EPS);
    assert!((response.beta_deg - 60.0).abs() < EPS);
}

#[test]
fn solves_triangle_from_leg_a_and_alpha() {
    let request = SolveRightTriangleRequest::LegAAndAlpha {
        a_mm: 5.0,
        alpha_deg: 30.0,
    };

    let response = solve_right_triangle(request).unwrap();

    assert!((response.alpha_deg - 30.0).abs() < EPS);
    assert!((response.alpha_deg + response.beta_deg - 90.0).abs() < EPS);
    assert!((response.a_mm - 5.0).abs() < EPS);
}

#[test]
fn solves_triangle_from_leg_a_and_beta() {
    let request = SolveRightTriangleRequest::LegAAndBeta {
        a_mm: 5.0,
        beta_deg: 60.0,
    };

    let response = solve_right_triangle(request).unwrap();

    assert!((response.beta_deg - 60.0).abs() < EPS);
    assert!((response.alpha_deg - 30.0).abs() < EPS);
}

#[test]
fn solves_triangle_from_leg_b_and_alpha() {
    let request = SolveRightTriangleRequest::LegBAndAlpha {
        b_mm: 4.0,
        alpha_deg: 45.0,
    };

    let response = solve_right_triangle(request).unwrap();

    assert!((response.alpha_deg - 45.0).abs() < EPS);
    assert!((response.alpha_deg + response.beta_deg - 90.0).abs() < EPS);
}

#[test]
fn solves_triangle_from_leg_b_and_beta() {
    let request = SolveRightTriangleRequest::LegBAndBeta {
        b_mm: 4.0,
        beta_deg: 30.0,
    };

    let response = solve_right_triangle(request).unwrap();

    assert!((response.beta_deg - 30.0).abs() < EPS);
    assert!((response.alpha_deg - 60.0).abs() < EPS);
}
