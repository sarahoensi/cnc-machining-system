// tests/cutting_data/value_objects/validation.rs

use cnc_machining_system_lib::domain::features::cutting_data::model::values::*;
use cnc_machining_system_lib::domain::features::cutting_data::errors::DomainError;

// ======================================================
// DIAMETER VALIDATION
// ======================================================

#[test]
fn diameter_rejects_nan() {
    assert_invalid(DiameterMm::new(f64::NAN));
}

#[test]
fn diameter_rejects_infinity() {
    assert_invalid(DiameterMm::new(f64::INFINITY));
}

#[test]
fn diameter_rejects_negative_infinity() {
    assert_invalid(DiameterMm::new(f64::NEG_INFINITY));
}

// ======================================================
// CUTTING SPEED VALIDATION
// ======================================================

#[test]
fn cutting_speed_rejects_nan() {
    assert_invalid(CuttingSpeedMMin::new(f64::NAN));
}

#[test]
fn cutting_speed_rejects_infinity() {
    assert_invalid(CuttingSpeedMMin::new(f64::INFINITY));
}

// ======================================================
// SPINDLE SPEED VALIDATION
// ======================================================

#[test]
fn spindle_speed_rejects_nan() {
    assert_invalid(SpindleSpeedRpm::new(f64::NAN));
}

#[test]
fn spindle_speed_rejects_infinity() {
    assert_invalid(SpindleSpeedRpm::new(f64::INFINITY));
}

// ======================================================
// FEED RATE VALIDATION
// ======================================================

#[test]
fn feed_rate_rejects_nan() {
    assert_invalid(FeedRateMmMin::new(f64::NAN));
}

#[test]
fn feed_rate_rejects_infinity() {
    assert_invalid(FeedRateMmMin::new(f64::INFINITY));
}

// ======================================================
// FEED PER TOOTH VALIDATION
// ======================================================

#[test]
fn feed_per_tooth_rejects_nan() {
    assert_invalid(FeedPerToothMm::new(f64::NAN));
}

#[test]
fn feed_per_tooth_rejects_infinity() {
    assert_invalid(FeedPerToothMm::new(f64::INFINITY));
}

// ======================================================
// HELPER
// ======================================================

fn assert_invalid<T: std::fmt::Debug>(result: Result<T, DomainError>) {
    match result {
        Err(DomainError::InvalidValue(_)) => {}
        other => panic!("Expected InvalidValue error, got {:?}", other),
    }
}
