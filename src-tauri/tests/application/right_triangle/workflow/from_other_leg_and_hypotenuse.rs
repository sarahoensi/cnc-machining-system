use cnc_machining_system_lib::application::{SolveRightTriangleInput, SolveRightTriangleUseCase};

const EPS: f64 = 1e-9;

#[test]
fn solves_triangle_from_leg_b_and_hypotenuse() {
    let use_case = SolveRightTriangleUseCase;

    let result = use_case
        .execute(SolveRightTriangleInput::LegBAndHypotenuse {
            b_mm: 4.0,
            c_mm: 5.0,
        })
        .unwrap();

    assert!((result.a_mm - 3.0).abs() < EPS);

    // Ekstra robusthet
    assert!((result.alpha_deg + result.beta_deg - 90.0).abs() < EPS);
}
