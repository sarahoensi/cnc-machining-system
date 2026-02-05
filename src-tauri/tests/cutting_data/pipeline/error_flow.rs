// tests/cutting_data/pipeline/error_flow.rs

use cnc_machining_system_lib::domain::features::cutting_data::{CuttingData, DomainError, raw::RawCuttingInput};
#[test]
fn raw_invalid_combination_returns_error() {

    let raw = RawCuttingInput {
        d: Some(10.0),
        z: Some(4),
        vc: Some(200.0),
        n: Some(6000.0),
        ..Default::default()
    };

    let err = CuttingData::try_from(raw).unwrap_err();

    assert_eq!(err, DomainError::InvalidSpeedMode);
}
