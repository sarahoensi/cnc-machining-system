// tests/cutting_data/solver/knowledge_matrix.rs

use cnc_machining_system_lib::domain::features::cutting_data::services::solver::CuttingDataSolver;
use cnc_machining_system_lib::domain::features::cutting_data::model::{CuttingData, Feed, Speed};
use cnc_machining_system_lib::domain::features::cutting_data::model::values::*;

// ======================================================
// POSITIVE KNOWLEDGE COMBINATIONS
// ======================================================

// ------------------------------------------------------
// fz + cutting speed -> full solution
// ------------------------------------------------------

#[test]
fn fz_and_cutting_speed_produces_full_solution() {

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

    let partial = CuttingDataSolver::solve_partial(&data).unwrap();

    assert!(partial.spindle_speed.is_some());
    assert!(partial.feed_rate.is_some());
}

// ------------------------------------------------------
// fz + spindle speed -> full solution
// ------------------------------------------------------

#[test]
fn fz_and_spindle_speed_produces_full_solution() {

    let data = CuttingData {
        diameter: DiameterMm::new(12.0).unwrap(),
        teeth: Some(ToothCount::new(3).unwrap()),
        speed: Some(Speed::SpindleSpeed(
            SpindleSpeedRpm::new(6000.0).unwrap()
        )),
        feed: Some(Feed::FeedPerTooth(
            FeedPerToothMm::new(0.04).unwrap()
        )),
    };

    let partial = CuttingDataSolver::solve_partial(&data).unwrap();

    assert!(partial.feed_rate.is_some());
    assert!(partial.cutting_speed.is_some());
}

// ------------------------------------------------------
// feed rate + spindle speed -> full solution
// ------------------------------------------------------

#[test]
fn feed_rate_and_spindle_speed_produces_full_solution() {

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

    let partial = CuttingDataSolver::solve_partial(&data).unwrap();

    assert!(partial.feed_per_tooth.is_some());
    assert!(partial.cutting_speed.is_some());
}

// ------------------------------------------------------
// feed rate + cutting speed -> full solution
// ------------------------------------------------------

#[test]
fn feed_rate_and_cutting_speed_produces_full_solution() {

    let data = CuttingData {
        diameter: DiameterMm::new(10.0).unwrap(),
        teeth: Some(ToothCount::new(4).unwrap()),
        speed: Some(Speed::CuttingSpeed(
            CuttingSpeedMMin::new(180.0).unwrap()
        )),
        feed: Some(Feed::FeedRate(
            FeedRateMmMin::new(900.0).unwrap()
        )),
    };

    let partial = CuttingDataSolver::solve_partial(&data).unwrap();

    assert!(partial.spindle_speed.is_some());
    assert!(partial.feed_per_tooth.is_some());
}

// ------------------------------------------------------
// spindle speed only -> cutting speed
// ------------------------------------------------------

#[test]
fn spindle_only_produces_cutting_speed() {

    let data = CuttingData {
        diameter: DiameterMm::new(10.0).unwrap(),
        teeth: Some(ToothCount::new(4).unwrap()),
        speed: Some(Speed::SpindleSpeed(
            SpindleSpeedRpm::new(5000.0).unwrap()
        )),
        feed: None,
    };

    let partial = CuttingDataSolver::solve_partial(&data).unwrap();

    assert!(partial.spindle_speed.is_some());
    assert!(partial.cutting_speed.is_some());
}

// ------------------------------------------------------
// cutting speed only -> spindle speed
// ------------------------------------------------------

#[test]
fn cutting_speed_only_produces_spindle_speed() {

    let data = CuttingData {
        diameter: DiameterMm::new(15.0).unwrap(),
        teeth: Some(ToothCount::new(5).unwrap()),
        speed: Some(Speed::CuttingSpeed(
            CuttingSpeedMMin::new(220.0).unwrap()
        )),
        feed: None,
    };

    let partial = CuttingDataSolver::solve_partial(&data).unwrap();

    assert!(partial.spindle_speed.is_some());
    assert!(partial.cutting_speed.is_some());
}

// ======================================================
// NEGATIVE KNOWLEDGE COMBINATIONS
// ======================================================

// ------------------------------------------------------
// fz only -> no computable outputs
// ------------------------------------------------------

#[test]
fn feed_per_tooth_only_produces_no_outputs() {

    let data = CuttingData {
        diameter: DiameterMm::new(10.0).unwrap(),
        teeth: Some(ToothCount::new(4).unwrap()),
        speed: None,
        feed: Some(Feed::FeedPerTooth(
            FeedPerToothMm::new(0.05).unwrap()
        )),
    };

    let partial = CuttingDataSolver::solve_partial(&data).unwrap();

    assert!(partial.feed_rate.is_none());
    assert!(partial.spindle_speed.is_none());
    assert!(partial.cutting_speed.is_none());
}

// ------------------------------------------------------
// feed rate only -> no computable outputs
// ------------------------------------------------------

#[test]
fn feed_rate_only_produces_no_outputs() {

    let data = CuttingData {
        diameter: DiameterMm::new(10.0).unwrap(),
        teeth: Some(ToothCount::new(4).unwrap()),
        speed: None,
        feed: Some(Feed::FeedRate(
            FeedRateMmMin::new(1000.0).unwrap()
        )),
    };

    let partial = CuttingDataSolver::solve_partial(&data).unwrap();

    assert!(partial.feed_per_tooth.is_none());
    assert!(partial.spindle_speed.is_none());
    assert!(partial.cutting_speed.is_none());
}

// ------------------------------------------------------
// no speed and no feed -> no outputs
// ------------------------------------------------------

#[test]
fn no_speed_and_no_feed_produces_no_outputs() {

    let data = CuttingData {
        diameter: DiameterMm::new(12.0).unwrap(),
        teeth: Some(ToothCount::new(3).unwrap()),
        speed: None,
        feed: None,
    };

    let partial = CuttingDataSolver::solve_partial(&data).unwrap();

    assert!(partial.cutting_speed.is_none());
    assert!(partial.spindle_speed.is_none());
    assert!(partial.feed_rate.is_none());
    assert!(partial.feed_per_tooth.is_none());
}

// ======================================================
// INPUT PRESERVATION INVARIANT
// ======================================================

#[test]
fn solver_preserves_input_cutting_speed() {

    let vc = CuttingSpeedMMin::new(200.0).unwrap();

    let data = CuttingData {
        diameter: DiameterMm::new(10.0).unwrap(),
        teeth: Some(ToothCount::new(4).unwrap()),
        speed: Some(Speed::CuttingSpeed(vc)),
        feed: None,
    };

    let partial = CuttingDataSolver::solve_partial(&data).unwrap();

    assert_eq!(partial.cutting_speed.unwrap(), vc);
}

// ======================================================
// PARTIAL -> FULL CONSISTENCY
// ======================================================

#[test]
fn partial_solution_is_consistent_with_full_solution() {

    let data = CuttingData {
        diameter: DiameterMm::new(10.0).unwrap(),
        teeth: Some(ToothCount::new(4).unwrap()),
        speed: Some(Speed::CuttingSpeed(
            CuttingSpeedMMin::new(210.0).unwrap()
        )),
        feed: Some(Feed::FeedPerTooth(
            FeedPerToothMm::new(0.06).unwrap()
        )),
    };

    let partial = CuttingDataSolver::solve_partial(&data).unwrap();
    let full = CuttingDataSolver::solve_full(&data).unwrap();

    assert_eq!(partial.spindle_speed.unwrap(), full.spindle_speed);
    assert_eq!(partial.feed_rate.unwrap(), full.feed_rate);
}
