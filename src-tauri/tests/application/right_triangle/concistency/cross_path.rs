#[test]
fn different_inputs_produce_same_triangle() {

    let use_case = SolveRightTriangleUseCase;

    let from_legs = use_case.execute(
        SolveRightTriangleInput::Legs {
            a_mm: 3.0,
            b_mm: 4.0,
        }
    ).unwrap();

    let from_leg_hyp = use_case.execute(
        SolveRightTriangleInput::LegAndHypotenuse {
            a_mm: 3.0,
            c_mm: 5.0,
        }
    ).unwrap();

    assert!((from_legs.b_mm - from_leg_hyp.b_mm).abs() < 1e-9);
}
