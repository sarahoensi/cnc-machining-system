use cnc_machining_system_lib::application::{
    SolveRightTriangleInput,
    SolveRightTriangleUseCase,
};

#[test]
fn fails_when_leg_is_zero() {

    let use_case = SolveRightTriangleUseCase;

    let result = use_case.execute(
        SolveRightTriangleInput::Legs {
            a_mm: 0.0, // invalid
            b_mm: 4.0,
        }
    );

    assert!(result.is_err());
}

#[test]
fn fails_when_hypotenuse_smaller_than_leg() {

    let use_case = SolveRightTriangleUseCase;

    let result = use_case.execute(
        SolveRightTriangleInput::LegAAndHypotenuse {
            a_mm: 5.0,
            c_mm: 3.0, // impossible
        }
    );

    assert!(result.is_err());
}

#[test]
fn fails_when_angle_is_invalid() {

    let use_case = SolveRightTriangleUseCase;

    let result = use_case.execute(
        SolveRightTriangleInput::HypotenuseAndAlpha {
            c_mm: 10.0,
            alpha_deg: 90.0, // invalid (must be < 90)
        }
    );

    assert!(result.is_err());
}

#[test]
fn fails_when_beta_is_invalid() {

    let use_case = SolveRightTriangleUseCase;

    let result = use_case.execute(
        SolveRightTriangleInput::LegAAndBeta {
            a_mm: 5.0,
            beta_deg: 0.0,
        }
    );

    assert!(result.is_err());
}

#[test]
fn fails_when_leg_is_negative() {

    let use_case = SolveRightTriangleUseCase;

    let result = use_case.execute(
        SolveRightTriangleInput::LegBAndAlpha {
            b_mm: -2.0,
            alpha_deg: 30.0,
        }
    );

    assert!(result.is_err());
}