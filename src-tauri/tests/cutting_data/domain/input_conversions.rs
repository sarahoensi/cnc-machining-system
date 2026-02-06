// tests/cutting_data/domain/input_conversions.rs
use std::convert::TryFrom;

use cnc_machining_system_lib::domain::features::cutting_data::errors::DomainError;
use cnc_machining_system_lib::domain::features::cutting_data::model::CuttingData;
use cnc_machining_system_lib::domain::features::cutting_data::input::raw::RawCuttingInput;

use crate::cutting_data::common::builders::*;

// ======================================================
// REQUIRED FIELD VALIDATION
// ======================================================

#[test]
fn missing_d_returns_missing_field_error() {
    let raw = missing_d(valid_raw());

    let err = CuttingData::try_from(raw).unwrap_err();
    assert_eq!(err, DomainError::MissingField("d"));
}

#[test]
fn missing_z_allows_none() {
    let raw = missing_z(valid_raw());

    let result = CuttingData::try_from(raw);
    assert!(result.is_ok(), "Expected Ok but got: {:?}", result);
    
    let data = result.unwrap();
    assert!(data.teeth.is_none(), "Expected teeth to be None when z is missing");
}

// ======================================================
// NUMERIC VALIDATION
// ======================================================

#[test]
fn diameter_must_be_positive() {
    let mut raw = valid_raw();
    raw.d = Some(0.0);

    assert_invalid_value(raw);
}

#[test]
fn tooth_count_must_be_positive() {
    let mut raw = valid_raw();
    raw.z = Some(0);

    assert_invalid_value(raw);
}

#[test]
fn cutting_speed_must_be_positive() {
    let mut raw = valid_raw();
    raw.vc = Some(-1.0);

    assert_invalid_value(raw);
}

#[test]
fn spindle_speed_must_be_positive() {
    let mut raw = with_spindle(valid_raw());
    raw.n = Some(0.0);

    assert_invalid_value(raw);
}

#[test]
fn feed_rate_must_be_positive() {
    let mut raw = with_feed_rate(valid_raw());
    raw.f = Some(-1.0);

    assert_invalid_value(raw);
}

#[test]
fn feed_per_tooth_must_be_positive() {
    let mut raw = with_feed_per_tooth(valid_raw());
    raw.fz = Some(0.0);

    assert_invalid_value(raw);
}

// ======================================================
// HELPER
// ======================================================

fn assert_invalid_value(raw: RawCuttingInput) {
    let err = CuttingData::try_from(raw).unwrap_err();

    match err {
        DomainError::InvalidValue(_) => {}
        other => panic!("Expected InvalidValue, got {:?}", other),
    }
}
