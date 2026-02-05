// tests/cutting_data/value_objects/boundary.rs

use cnc_machining_system_lib::domain::features::cutting_data::model::values::*;
use cnc_machining_system_lib::domain::features::cutting_data::errors::DomainError;

// ======================================================
// DIAMETER
// ======================================================

#[test]
fn diameter_accepts_smallest_valid_value() {
    assert!(DiameterMm::new(0.000001).is_ok());
}

#[test]
fn diameter_rejects_zero() {
    assert_invalid(DiameterMm::new(0.0));
}

#[test]
fn diameter_rejects_negative() {
    assert_invalid(DiameterMm::new(-1.0));
}

// ======================================================
// CUTTING SPEED
// ======================================================

#[test]
fn cutting_speed_accepts_smallest_valid_value() {
    assert!(CuttingSpeedMMin::new(0.000001).is_ok());
}

#[test]
fn cutting_speed_rejects_zero() {
    assert_invalid(CuttingSpeedMMin::new(0.0));
}

#[test]
fn cutting_speed_rejects_negative() {
    assert_invalid(CuttingSpeedMMin::new(-10.0));
}

// ======================================================
// SPINDLE SPEED
// ======================================================

#[test]
fn spindle_speed_accepts_smallest_valid_value() {
    assert!(SpindleSpeedRpm::new(0.000001).is_ok());
}

#[test]
fn spindle_speed_rejects_zero() {
    assert_invalid(SpindleSpeedRpm::new(0.0));
}

#[test]
fn spindle_speed_rejects_negative() {
    assert_invalid(SpindleSpeedRpm::new(-1.0));
}

// ======================================================
// FEED RATE
// ======================================================

#[test]
fn feed_rate_accepts_smallest_valid_value() {
    assert!(FeedRateMmMin::new(0.000001).is_ok());
}

#[test]
fn feed_rate_rejects_zero() {
    assert_invalid(FeedRateMmMin::new(0.0));
}

#[test]
fn feed_rate_rejects_negative() {
    assert_invalid(FeedRateMmMin::new(-1.0));
}

// ======================================================
// FEED PER TOOTH
// ======================================================

#[test]
fn feed_per_tooth_accepts_smallest_valid_value() {
    assert!(FeedPerToothMm::new(0.000001).is_ok());
}

#[test]
fn feed_per_tooth_rejects_zero() {
    assert_invalid(FeedPerToothMm::new(0.0));
}

#[test]
fn feed_per_tooth_rejects_negative() {
    assert_invalid(FeedPerToothMm::new(-0.1));
}

// ======================================================
// TOOTH COUNT
// ======================================================

#[test]
fn tooth_count_accepts_minimum_valid_value() {
    assert!(ToothCount::new(1).is_ok());
}

#[test]
fn tooth_count_rejects_zero() {
    assert_invalid(ToothCount::new(0));
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
