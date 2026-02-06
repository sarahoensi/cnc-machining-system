// tests/cutting_data/solver/full_solver.rs
use cnc_machining_system_lib::domain::features::cutting_data::services::solver::CuttingDataSolver;
use cnc_machining_system_lib::domain::features::cutting_data::model::{CuttingData, Feed, Speed};
use cnc_machining_system_lib::domain::features::cutting_data::model::values::*;
use cnc_machining_system_lib::domain::features::cutting_data::errors::DomainError;

use crate::cutting_data::common::approx::approx_eq;

// ======================================================
// SUCCESS CASES
// ======================================================

#[test]
fn solve_full_from_cutting_speed_and_feed_per_tooth() {

    let data = CuttingData {
        diameter: DiameterMm::new(10.0).unwrap(),
        teeth: Some(ToothCount::new(4).unwrap()),
        speed: Some(Speed::CuttingSpeed(
            CuttingSpeedMMin::new(200.0).unwrap()
        )),
        feed: Some(Feed::FeedPerTooth(
            FeedPerToothMm::new(0.05).unwrap()
        )),
    };

    let result = CuttingDataSolver::solve_full(&data);
    assert!(result.is_ok(), "solve_full failed: {:?}", result);

    let result = result.unwrap();

    assert!(result.cutting_speed.value() > 0.0);
    assert!(result.spindle_speed.value() > 0.0);
    assert!(result.feed_rate.value() > 0.0);
    assert!(result.feed_per_tooth.value() > 0.0);
}

#[test]
fn solve_full_from_spindle_speed_and_feed_rate() {

    let data = CuttingData {
        diameter: DiameterMm::new(8.0).unwrap(),
        teeth: Some(ToothCount::new(2).unwrap()),
        speed: Some(Speed::SpindleSpeed(
            SpindleSpeedRpm::new(10000.0).unwrap()
        )),
        feed: Some(Feed::FeedRate(
            FeedRateMmMin::new(800.0).unwrap()
        )),
    };

    let result = CuttingDataSolver::solve_full(&data);
    assert!(result.is_ok(), "solve_full failed: {:?}", result);

    let result = result.unwrap();

    assert!(result.cutting_speed.value() > 0.0);
    assert!(result.spindle_speed.value() > 0.0);
    assert!(result.feed_rate.value() > 0.0);
    assert!(result.feed_per_tooth.value() > 0.0);
}

// ======================================================
// ERROR CASES
// ======================================================

#[test]
fn solve_full_errors_when_speed_missing() {

    let data = CuttingData {
        diameter: DiameterMm::new(10.0).unwrap(),
        teeth: Some(ToothCount::new(4).unwrap()),
        speed: None,
        feed: Some(Feed::FeedPerTooth(
            FeedPerToothMm::new(0.05).unwrap()
        )),
    };

    let result = CuttingDataSolver::solve_full(&data);
    assert!(result.is_err());

    let err = result.unwrap_err();

    assert!(matches!(err, DomainError::MissingField("cutting_speed")));
}

#[test]
fn solve_full_errors_when_feed_missing() {

    let data = CuttingData {
        diameter: DiameterMm::new(10.0).unwrap(),
        teeth: Some(ToothCount::new(4).unwrap()),
        speed: Some(Speed::CuttingSpeed(
            CuttingSpeedMMin::new(200.0).unwrap()
        )),
        feed: None,
    };

    let result = CuttingDataSolver::solve_full(&data);
    assert!(result.is_err());

    let err = result.unwrap_err();

    assert!(matches!(err, DomainError::MissingField("feed_rate")));
}

// ======================================================
// CONSISTENCY GUARANTEE
// ======================================================

#[test]
fn solve_full_produces_self_consistent_results() {

    let data = CuttingData {
        diameter: DiameterMm::new(12.0).unwrap(),
        teeth: Some(ToothCount::new(3).unwrap()),
        speed: Some(Speed::CuttingSpeed(
            CuttingSpeedMMin::new(180.0).unwrap()
        )),
        feed: Some(Feed::FeedPerTooth(
            FeedPerToothMm::new(0.07).unwrap()
        )),
    };

    let result = CuttingDataSolver::solve_full(&data);
    assert!(result.is_ok(), "solve_full failed: {:?}", result);

    let result = result.unwrap();

    let n_check = CuttingDataSolver::spindle_from_vc(
        result.cutting_speed,
        result.diameter,
    );

    assert!(n_check.is_ok());

    let n_check = n_check.unwrap();

    assert!(approx_eq(
        n_check.value(),
        result.spindle_speed.value(),
        1e-9,
    ));
}
// ======================================================
// OPTIONAL TOOTH COUNT BEHAVIOR
// ======================================================

#[test]
fn solve_full_errors_when_teeth_missing() {

    let data = CuttingData {
        diameter: DiameterMm::new(10.0).unwrap(),
        teeth: None,
        speed: Some(Speed::CuttingSpeed(
            CuttingSpeedMMin::new(200.0).unwrap()
        )),
        feed: Some(Feed::FeedPerTooth(
            FeedPerToothMm::new(0.05).unwrap()
        )),
    };

    let result = CuttingDataSolver::solve_full(&data);
    assert!(result.is_err(), "Expected solve_full to fail without teeth");

    let err = result.unwrap_err();
    assert!(matches!(err, DomainError::MissingField("teeth")));
}

#[test]
fn solve_partial_speed_works_without_teeth() {

    let data = CuttingData {
        diameter: DiameterMm::new(10.0).unwrap(),
        teeth: None,
        speed: Some(Speed::CuttingSpeed(
            CuttingSpeedMMin::new(200.0).unwrap()
        )),
        feed: None,
    };

    let result = CuttingDataSolver::solve_partial(&data);
    assert!(result.is_ok(), "solve_partial should succeed with speed but no teeth");

    let partial = result.unwrap();
    assert!(partial.cutting_speed.is_some(), "Should compute cutting speed");
    assert!(partial.spindle_speed.is_some(), "Should compute spindle speed");
    assert!(partial.feed_rate.is_none(), "Should not compute feed without teeth");
    assert!(partial.feed_per_tooth.is_none(), "Should not compute feed_per_tooth without teeth");
}

#[test]
fn solve_partial_feed_fails_without_teeth() {

    let data = CuttingData {
        diameter: DiameterMm::new(10.0).unwrap(),
        teeth: None,
        speed: Some(Speed::CuttingSpeed(
            CuttingSpeedMMin::new(200.0).unwrap()
        )),
        feed: Some(Feed::FeedPerTooth(
            FeedPerToothMm::new(0.05).unwrap()
        )),
    };

    let result = CuttingDataSolver::solve_partial(&data);
    assert!(result.is_ok(), "solve_partial should succeed");

    let partial = result.unwrap();
    assert!(partial.feed_rate.is_none(), "Feed rate should be None without teeth");
    assert!(partial.feed_per_tooth.is_none(), "Feed per tooth should be None without teeth");
}