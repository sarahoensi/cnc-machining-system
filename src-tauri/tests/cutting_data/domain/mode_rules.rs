// tests/cutting_data/domain/mode_rules.rs
use std::convert::TryFrom;

use cnc_machining_system_lib::domain::features::cutting_data::errors::DomainError;
use cnc_machining_system_lib::domain::features::cutting_data::model::CuttingData;

use crate::cutting_data::common::builders::*;

// ======================================================
// SPEED MODE RULES
// ======================================================

#[test]
fn speed_mode_allows_cutting_speed_only() {
    let raw = valid_raw();

    let data = CuttingData::try_from(raw).unwrap();
    assert!(data.speed.is_some());
}

#[test]
fn speed_mode_allows_spindle_speed_only() {
    let raw = with_spindle(valid_raw());

    let data = CuttingData::try_from(raw).unwrap();
    assert!(data.speed.is_some());
}

#[test]
fn speed_mode_allows_no_speed_for_partial_input() {
    let raw = without_speed(valid_raw());

    let data = CuttingData::try_from(raw).unwrap();
    assert!(data.speed.is_none());
}

#[test]
fn speed_mode_rejects_cutting_speed_and_spindle_speed() {
    let mut raw = valid_raw();
    raw.n = Some(6000.0);

    let err = CuttingData::try_from(raw).unwrap_err();
    assert_eq!(err, DomainError::InvalidSpeedMode);
}

// ======================================================
// FEED MODE RULES
// ======================================================

#[test]
fn feed_mode_allows_feed_rate_only() {
    let raw = with_feed_rate(valid_raw());

    let data = CuttingData::try_from(raw).unwrap();
    assert!(data.feed.is_some());
}

#[test]
fn feed_mode_allows_feed_per_tooth_only() {
    let raw = with_feed_per_tooth(valid_raw());

    let data = CuttingData::try_from(raw).unwrap();
    assert!(data.feed.is_some());
}

#[test]
fn feed_mode_allows_no_feed_for_partial_input() {
    let raw = without_feed(valid_raw());

    let data = CuttingData::try_from(raw).unwrap();
    assert!(data.feed.is_none());
}

#[test]
fn feed_mode_rejects_feed_rate_and_feed_per_tooth() {
    let mut raw = with_feed_rate(valid_raw());
    raw.fz = Some(0.05);

    let err = CuttingData::try_from(raw).unwrap_err();
    assert_eq!(err, DomainError::InvalidFeedMode);
}

// ======================================================
// MODE INDEPENDENCE RULES
// ======================================================

#[test]
fn feed_mode_can_exist_without_speed_mode() {
    let raw = with_feed_rate(without_speed(valid_raw()));

    let data = CuttingData::try_from(raw).unwrap();

    assert!(data.feed.is_some());
    assert!(data.speed.is_none());
}
