// tests/cutting_data/acceptance/minimal_input.rs

use cnc_machining_system_lib::domain::features::cutting_data::raw::RawCuttingInput;
use crate::cutting_data::acceptance::helpers::*;

// ======================================================
// DIAMETER + SPINDLE SPEED -> CUTTING SPEED
// ======================================================

#[test]
fn user_enters_diameter_and_spindle_speed() {

    let raw = RawCuttingInput {
        d: Some(10.0),
        z: Some(4),
        n: Some(6000.0),
        ..Default::default()
    };

    let partial = solve_partial(raw);

    assert!(partial.cutting_speed.is_some());
    assert!(partial.feed_rate.is_none());
}

// ======================================================
// DIAMETER + CUTTING SPEED -> SPINDLE SPEED
// ======================================================

#[test]
fn user_enters_diameter_and_cutting_speed() {

    let raw = RawCuttingInput {
        d: Some(10.0),
        z: Some(4),
        vc: Some(200.0),
        ..Default::default()
    };

    let partial = solve_partial(raw);

    assert!(partial.spindle_speed.is_some());
    assert!(partial.feed_rate.is_none());
}

// ======================================================
// FEED PER TOOTH + TOOTH COUNT + SPINDLE -> FEED RATE
// ======================================================

#[test]
fn user_enters_feed_per_tooth_tooth_count_and_spindle() {

    let raw = RawCuttingInput {
        d: Some(10.0),
        z: Some(4),
        n: Some(6000.0),
        fz: Some(0.05),
        ..Default::default()
    };

    let partial = solve_partial(raw);

    assert!(partial.feed_rate.is_some());
}

// ======================================================
// FEED RATE + TOOTH COUNT + SPINDLE -> FEED PER TOOTH
// ======================================================

#[test]
fn user_enters_feed_rate_tooth_count_and_spindle() {

    let raw = RawCuttingInput {
        d: Some(10.0),
        z: Some(4),
        n: Some(6000.0),
        f: Some(1200.0),
        ..Default::default()
    };

    let partial = solve_partial(raw);

    assert!(partial.feed_per_tooth.is_some());
}

// ======================================================
// VERIFY SYSTEM DOES NOT OVER-COMPUTE
// ======================================================

#[test]
fn diameter_and_spindle_does_not_compute_feed() {

    let raw = RawCuttingInput {
        d: Some(10.0),
        z: Some(4),
        n: Some(6000.0),
        ..Default::default()
    };

    let partial = solve_partial(raw);

    assert!(partial.feed_rate.is_none());
    assert!(partial.feed_per_tooth.is_none());
}
