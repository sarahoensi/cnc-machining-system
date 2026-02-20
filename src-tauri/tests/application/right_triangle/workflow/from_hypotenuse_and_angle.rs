use cnc_machining_system_lib::application::{
    SolveRightTriangleInput,
    SolveRightTriangleUseCase,
};

const EPS: f64 = 1e-9;

#[test]
fn solves_triangle_from_hypotenuse_and_alpha() {

    let use_case = SolveRightTriangleUseCase;

    let result = use_case.execute(
        SolveRightTriangleInput::HypotenuseAndAlpha {
            c_mm: 10.0,
            alpha_deg: 30.0,
        }
    ).unwrap();

    // a = c * sin(30°) = 10 * 0.5 = 5
    assert!((result.a_mm - 5.0).abs() < EPS);

    // Ekstra robusthet:
    assert!((result.alpha_deg - 30.0).abs() < EPS);
    assert!((result.alpha_deg + result.beta_deg - 90.0).abs() < EPS);
}

#[test]
fn solves_triangle_from_hypotenuse_and_beta() {

    let use_case = SolveRightTriangleUseCase;

    let result = use_case.execute(
        SolveRightTriangleInput::HypotenuseAndBeta {
            c_mm: 10.0,
            beta_deg: 60.0,
        }
    ).unwrap();

    assert!((result.a_mm - 5.0).abs() < EPS);
    assert!((result.alpha_deg - 30.0).abs() < EPS);
}