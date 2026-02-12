#[test]
fn solves_triangle_from_other_leg_and_hypotenuse() {

    let use_case = SolveRightTriangleUseCase;

    let result = use_case.execute(
        SolveRightTriangleInput::OtherLegAndHypotenuse {
            b_mm: 4.0,
            c_mm: 5.0,
        }
    ).unwrap();

    assert!((result.a_mm - 3.0).abs() < 1e-9);
}
