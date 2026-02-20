use cnc_machining_system_lib::application::{
    SolveRightTriangleInput,
    SolveRightTriangleUseCase,
};

const EPS: f64 = 1e-9;

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
        SolveRightTriangleInput::LegAAndHypotenuse {
            a_mm: 3.0,
            c_mm: 5.0,
        }
    ).unwrap();

    // Sammenlign alle sider
    assert!((from_legs.a_mm - from_leg_hyp.a_mm).abs() < EPS);
    assert!((from_legs.b_mm - from_leg_hyp.b_mm).abs() < EPS);
    assert!((from_legs.c_mm - from_leg_hyp.c_mm).abs() < EPS);

    // Sammenlign vinkler
    assert!((from_legs.alpha_deg - from_leg_hyp.alpha_deg).abs() < EPS);
    assert!((from_legs.beta_deg - from_leg_hyp.beta_deg).abs() < EPS);
}