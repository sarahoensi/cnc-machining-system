use super::super::CalculateCuttingDataUseCase;
use crate::domain::features::cutting_data::input::raw::RawCuttingInput;

#[test]
fn calculates_spindle_speed_from_vc() {

    let input = RawCuttingInput {
        d: Some(10.0),
        vc: Some(200.0),
        z: Some(4),
        ..Default::default()
    };

    let result = CalculateCuttingDataUseCase::partial(input).unwrap();

    let spindle = result.spindle_speed.unwrap().value();

    assert!((spindle - 6366.0).abs() < 10.0);
}

#[test]
fn calculates_feed_from_fz() {

    let input = RawCuttingInput {
        d: Some(10.0),
        vc: Some(200.0),
        z: Some(4),
        fz: Some(0.05),
        ..Default::default()
    };

    let result = CalculateCuttingDataUseCase::partial(input).unwrap();

    let feed = result.feed_rate.unwrap().value();

    assert!((feed - 1273.0).abs() < 10.0);
}
