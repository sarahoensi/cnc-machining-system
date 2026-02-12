
use cnc_machining_system_lib::application::{SolveRightTriangleInput, SolveRightTriangleUseCase};



#[test]
fn solves_triangle_from_hypotenuse_and_angle() {

    let use_case = SolveRightTriangleUseCase;

    let result = use_case.execute(
        SolveRightTriangleInput::HypotenuseAndAngle {
            c_mm: 10.0,
            alpha_deg: 30.0,
        }
    ).unwrap();

    assert!((result.a_mm - 5.0).abs() < 1e-9);
}
