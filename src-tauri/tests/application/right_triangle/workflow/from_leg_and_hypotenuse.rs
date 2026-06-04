use cnc_machining_system_lib::application::{SolveRightTriangleInput, SolveRightTriangleUseCase};

const EPS: f64 = 1e-9;

#[test]
fn solves_triangle_from_leg_a_and_hypotenuse() {
    let use_case = SolveRightTriangleUseCase;

    let result = use_case
        .execute(SolveRightTriangleInput::LegAAndHypotenuse {
            a_mm: 3.0,
            c_mm: 5.0,
        })
        .unwrap();

    assert!((result.b_mm - 4.0).abs() < EPS);
}
