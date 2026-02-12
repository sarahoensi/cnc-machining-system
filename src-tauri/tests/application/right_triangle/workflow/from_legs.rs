use cnc_machining_system_lib::application::{
    SolveRightTriangleInput,
    SolveRightTriangleUseCase,
};

#[test]
fn solves_triangle_from_legs() {

    let use_case = SolveRightTriangleUseCase;

    let result = use_case.execute(
        SolveRightTriangleInput::Legs {
            a_mm: 3.0,
            b_mm: 4.0,
        }
    ).unwrap();

    assert!((result.c_mm - 5.0).abs() < 1e-9);
}
