use cnc_machining_system_lib::application::{SolveRightTriangleInput, SolveRightTriangleUseCase};

#[test]
fn output_contains_consistent_triangle_geometry() {
    let use_case = SolveRightTriangleUseCase;

    let result = use_case
        .execute(SolveRightTriangleInput::Legs {
            a_mm: 3.0,
            b_mm: 4.0,
        })
        .unwrap();

    // Pythagoras invariant
    assert!((result.a_mm.powi(2) + result.b_mm.powi(2) - result.c_mm.powi(2)).abs() < 1e-9);

    // Angle sum invariant
    assert!((result.alpha_deg + result.beta_deg - 90.0).abs() < 1e-9);
}
