// src/domain/features/cutting_data/input/tests.rs

use super::super::raw::RawCuttingInput;
use crate::domain::features::cutting_data::errors::DomainError;
use crate::domain::features::cutting_data::model::CuttingData;

#[test]
fn missing_d_returns_missing_field_d() {
    let raw = RawCuttingInput {
        d: None,
        z: Some(4),
        vc: Some(200.0),
        ..Default::default()
    };

    let err = CuttingData::try_from(raw).unwrap_err();
    assert_eq!(err, DomainError::MissingField("d"));
}

#[test]
fn missing_z_returns_missing_field_z() {
    let raw = RawCuttingInput {
        d: Some(10.0),
        z: None,
        vc: Some(200.0),
        ..Default::default()
    };

    let err = CuttingData::try_from(raw).unwrap_err();
    assert_eq!(err, DomainError::MissingField("z"));
}

#[test]
fn invalid_diameter_zero_is_rejected() {
    let raw = RawCuttingInput {
        d: Some(0.0),
        z: Some(4),
        vc: Some(200.0),
        ..Default::default()
    };

    let err = CuttingData::try_from(raw).unwrap_err();
    matches_invalid_value(err);
}

#[test]
fn invalid_diameter_negative_is_rejected() {
    let raw = RawCuttingInput {
        d: Some(-10.0),
        z: Some(4),
        vc: Some(200.0),
        ..Default::default()
    };

    let err = CuttingData::try_from(raw).unwrap_err();
    matches_invalid_value(err);
}

#[test]
fn invalid_tooth_count_zero_is_rejected() {
    let raw = RawCuttingInput {
        d: Some(10.0),
        z: Some(0),
        vc: Some(200.0),
        ..Default::default()
    };

    let err = CuttingData::try_from(raw).unwrap_err();
    matches_invalid_value(err);
}

// ---------------- SPEED MODE COMBINATIONS ----------------

#[test]
fn speed_mode_vc_only_is_ok() {
    let raw = RawCuttingInput {
        d: Some(10.0),
        z: Some(4),
        vc: Some(200.0),
        n: None,
        ..Default::default()
    };

    let data = CuttingData::try_from(raw).unwrap();
    assert!(data.speed.is_some());
}

#[test]
fn speed_mode_n_only_is_ok() {
    let raw = RawCuttingInput {
        d: Some(10.0),
        z: Some(4),
        vc: None,
        n: Some(6000.0),
        ..Default::default()
    };

    let data = CuttingData::try_from(raw).unwrap();
    assert!(data.speed.is_some());
}

#[test]
fn speed_mode_none_none_is_ok_for_partial() {
    let raw = RawCuttingInput {
        d: Some(10.0),
        z: Some(4),
        vc: None,
        n: None,
        ..Default::default()
    };

    let data = CuttingData::try_from(raw).unwrap();
    assert!(data.speed.is_none());
}

#[test]
fn speed_mode_both_vc_and_n_is_error() {
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

// ---------------- FEED MODE COMBINATIONS ----------------

#[test]
fn feed_mode_f_only_is_ok() {
    let raw = RawCuttingInput {
        d: Some(10.0),
        z: Some(4),
        vc: Some(200.0), // speed present, but not required just to parse feed mode
        f: Some(1000.0),
        fz: None,
        ..Default::default()
    };

    let data = CuttingData::try_from(raw).unwrap();
    assert!(data.feed.is_some());
}

#[test]
fn feed_mode_fz_only_is_ok() {
    let raw = RawCuttingInput {
        d: Some(10.0),
        z: Some(4),
        vc: Some(200.0),
        f: None,
        fz: Some(0.05),
        ..Default::default()
    };

    let data = CuttingData::try_from(raw).unwrap();
    assert!(data.feed.is_some());
}

#[test]
fn feed_mode_none_none_is_ok_for_partial() {
    let raw = RawCuttingInput {
        d: Some(10.0),
        z: Some(4),
        vc: Some(200.0),
        f: None,
        fz: None,
        ..Default::default()
    };

    let data = CuttingData::try_from(raw).unwrap();
    assert!(data.feed.is_none());
}

#[test]
fn feed_mode_both_f_and_fz_is_error() {
    let raw = RawCuttingInput {
        d: Some(10.0),
        z: Some(4),
        vc: Some(200.0),
        f: Some(1000.0),
        fz: Some(0.05),
        ..Default::default()
    };

    let err = CuttingData::try_from(raw).unwrap_err();
    assert_eq!(err, DomainError::InvalidFeedMode);
}

// ---------------- INVALID NUMERIC VALUES (MODE-SPECIFIC) ----------------

#[test]
fn invalid_vc_zero_is_rejected() {
    let raw = RawCuttingInput {
        d: Some(10.0),
        z: Some(4),
        vc: Some(0.0),
        ..Default::default()
    };

    let err = CuttingData::try_from(raw).unwrap_err();
    matches_invalid_value(err);
}

#[test]
fn invalid_n_negative_is_rejected() {
    let raw = RawCuttingInput {
        d: Some(10.0),
        z: Some(4),
        n: Some(-1.0),
        ..Default::default()
    };

    let err = CuttingData::try_from(raw).unwrap_err();
    matches_invalid_value(err);
}

#[test]
fn invalid_f_zero_is_rejected() {
    let raw = RawCuttingInput {
        d: Some(10.0),
        z: Some(4),
        vc: Some(200.0),
        f: Some(0.0),
        ..Default::default()
    };

    let err = CuttingData::try_from(raw).unwrap_err();
    matches_invalid_value(err);
}

#[test]
fn invalid_fz_negative_is_rejected() {
    let raw = RawCuttingInput {
        d: Some(10.0),
        z: Some(4),
        vc: Some(200.0),
        fz: Some(-0.01),
        ..Default::default()
    };

    let err = CuttingData::try_from(raw).unwrap_err();
    matches_invalid_value(err);
}

// ---------------- helpers ----------------

fn matches_invalid_value(err: DomainError) {
    match err {
        DomainError::InvalidValue(_) => {}
        other => panic!("Expected DomainError::InvalidValue(..), got: {:?}", other),
    }
}

#[test]
fn invalid_vc_negative_is_rejected() {
    let raw = RawCuttingInput {
        d: Some(10.0),
        z: Some(4),
        vc: Some(-1.0),
        ..Default::default()
    };

    let err = CuttingData::try_from(raw).unwrap_err();
    matches_invalid_value(err);
}

#[test]
fn invalid_n_zero_is_rejected() {
    let raw = RawCuttingInput {
        d: Some(10.0),
        z: Some(4),
        n: Some(0.0),
        ..Default::default()
    };

    let err = CuttingData::try_from(raw).unwrap_err();
    matches_invalid_value(err);
}

#[test]
fn invalid_f_negative_is_rejected() {
    let raw = RawCuttingInput {
        d: Some(10.0),
        z: Some(4),
        vc: Some(200.0),
        f: Some(-1.0),
        ..Default::default()
    };

    let err = CuttingData::try_from(raw).unwrap_err();
    matches_invalid_value(err);
}

#[test]
fn invalid_fz_zero_is_rejected() {
    let raw = RawCuttingInput {
        d: Some(10.0),
        z: Some(4),
        vc: Some(200.0),
        fz: Some(0.0),
        ..Default::default()
    };

    let err = CuttingData::try_from(raw).unwrap_err();
    matches_invalid_value(err);
}
