use super::super::CalculateCuttingDataUseCase;
use crate::domain::features::cutting_data::input::raw::RawCuttingInput;

#[test]
fn full_solution_returns_all_fields() {

    let input = RawCuttingInput {
        d: Some(10.0),
        vc: Some(200.0),
        z: Some(4),
        fz: Some(0.05),
        ..Default::default()
    };

    let result = CalculateCuttingDataUseCase::full(input).unwrap();

    assert!(result.spindle_speed.value() > 0.0);
    assert!(result.feed_rate.value() > 0.0);
}

#[test]
fn full_solution_fails_with_missing_input() {

    let input = RawCuttingInput {
        d: Some(10.0),
        z: Some(4),
        ..Default::default()
    };

    assert!(CalculateCuttingDataUseCase::full(input).is_err());
}
