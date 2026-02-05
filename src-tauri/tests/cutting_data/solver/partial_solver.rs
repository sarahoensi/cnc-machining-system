// tests/cutting_data/solver/partial_solver.rs
use cnc_machining_system_lib::domain::features::cutting_data::services::solver::CuttingDataSolver;
use cnc_machining_system_lib::domain::features::cutting_data::model::{CuttingData, Feed, Speed};
use cnc_machining_system_lib::domain::features::cutting_data::model::values::*;

// ======================================================
// SPEED ONLY (Cutting speed input)
// ======================================================

#[test]
fn partial_solver_with_only_speed_computes_spindle_and_cutting_speed() {

    let data = CuttingData {
        diameter: DiameterMm::new(10.0).unwrap(),
        teeth: ToothCount::new(4).unwrap(),
        speed: Some(Speed::CuttingSpeed(
            CuttingSpeedMMin::new(200.0).unwrap()
        )),
        feed: None,
    };

    let partial = CuttingDataSolver::solve_partial(&data).unwrap();

    assert!(partial.cutting_speed.is_some());
    assert!(partial.spindle_speed.is_some());

    assert!(partial.feed_rate.is_none());
    assert!(partial.feed_per_tooth.is_none());
}

// ======================================================
// SPEED ONLY (Spindle speed input)  ⭐ NEW
// ======================================================

#[test]
fn partial_solver_with_only_spindle_computes_cutting_speed() {

    let data = CuttingData {
        diameter: DiameterMm::new(10.0).unwrap(),
        teeth: ToothCount::new(4).unwrap(),
        speed: Some(Speed::SpindleSpeed(
            SpindleSpeedRpm::new(6000.0).unwrap()
        )),
        feed: None,
    };

    let partial = CuttingDataSolver::solve_partial(&data).unwrap();

    assert!(partial.spindle_speed.is_some());
    assert!(partial.cutting_speed.is_some());

    assert!(partial.feed_rate.is_none());
    assert!(partial.feed_per_tooth.is_none());
}

// ======================================================
// FEED ONLY (Feed per tooth)
// ======================================================

#[test]
fn partial_solver_with_only_feed_does_not_compute_feed_rate_without_spindle() {

    let data = CuttingData {
        diameter: DiameterMm::new(10.0).unwrap(),
        teeth: ToothCount::new(4).unwrap(),
        speed: None,
        feed: Some(Feed::FeedPerTooth(
            FeedPerToothMm::new(0.05).unwrap()
        )),
    };

    let partial = CuttingDataSolver::solve_partial(&data).unwrap();

    assert!(partial.feed_rate.is_none());
    assert!(partial.feed_per_tooth.is_none());
}

// ======================================================
// FEED ONLY (Feed rate) ⭐ NEW
// ======================================================

#[test]
fn partial_solver_with_only_feed_rate_does_not_compute_fz_without_spindle() {

    let data = CuttingData {
        diameter: DiameterMm::new(10.0).unwrap(),
        teeth: ToothCount::new(4).unwrap(),
        speed: None,
        feed: Some(Feed::FeedRate(
            FeedRateMmMin::new(1000.0).unwrap()
        )),
    };

    let partial = CuttingDataSolver::solve_partial(&data).unwrap();

    assert!(partial.feed_rate.is_none());
    assert!(partial.feed_per_tooth.is_none());
}

// ======================================================
// SPEED + FEED PER TOOTH (FULL RESOLUTION)
// ======================================================

#[test]
fn partial_solver_with_speed_and_feed_computes_everything() {

    let data = CuttingData {
        diameter: DiameterMm::new(10.0).unwrap(),
        teeth: ToothCount::new(4).unwrap(),
        speed: Some(Speed::CuttingSpeed(
            CuttingSpeedMMin::new(200.0).unwrap()
        )),
        feed: Some(Feed::FeedPerTooth(
            FeedPerToothMm::new(0.05).unwrap()
        )),
    };

    let partial = CuttingDataSolver::solve_partial(&data).unwrap();

    assert!(partial.cutting_speed.is_some());
    assert!(partial.spindle_speed.is_some());
    assert!(partial.feed_rate.is_some());
    assert!(partial.feed_per_tooth.is_some());
}

// ======================================================
// SPINDLE + FEED RATE
// ======================================================

#[test]
fn partial_solver_with_spindle_and_feed_rate_computes_fz_and_cutting_speed() {

    let data = CuttingData {
        diameter: DiameterMm::new(8.0).unwrap(),
        teeth: ToothCount::new(2).unwrap(),
        speed: Some(Speed::SpindleSpeed(
            SpindleSpeedRpm::new(10000.0).unwrap()
        )),
        feed: Some(Feed::FeedRate(
            FeedRateMmMin::new(800.0).unwrap()
        )),
    };

    let partial = CuttingDataSolver::solve_partial(&data).unwrap();

    assert!(partial.cutting_speed.is_some());
    assert!(partial.spindle_speed.is_some());
    assert!(partial.feed_rate.is_some());
    assert!(partial.feed_per_tooth.is_some());
}

// ======================================================
// SPINDLE + FEED PER TOOTH ⭐ NEW
// ======================================================

#[test]
fn partial_solver_with_spindle_and_feed_per_tooth_computes_feed_rate_and_vc() {

    let data = CuttingData {
        diameter: DiameterMm::new(10.0).unwrap(),
        teeth: ToothCount::new(4).unwrap(),
        speed: Some(Speed::SpindleSpeed(
            SpindleSpeedRpm::new(6000.0).unwrap()
        )),
        feed: Some(Feed::FeedPerTooth(
            FeedPerToothMm::new(0.05).unwrap()
        )),
    };

    let partial = CuttingDataSolver::solve_partial(&data).unwrap();

    assert!(partial.feed_rate.is_some());
    assert!(partial.cutting_speed.is_some());
}

// ======================================================
// CUTTING SPEED + FEED RATE ⭐ NEW
// ======================================================

#[test]
fn partial_solver_with_cutting_speed_and_feed_rate_computes_spindle_and_fz() {

    let data = CuttingData {
        diameter: DiameterMm::new(10.0).unwrap(),
        teeth: ToothCount::new(4).unwrap(),
        speed: Some(Speed::CuttingSpeed(
            CuttingSpeedMMin::new(200.0).unwrap()
        )),
        feed: Some(Feed::FeedRate(
            FeedRateMmMin::new(1000.0).unwrap()
        )),
    };

    let partial = CuttingDataSolver::solve_partial(&data).unwrap();

    assert!(partial.spindle_speed.is_some());
    assert!(partial.feed_per_tooth.is_some());
}

// ======================================================
// INPUT PRESERVATION INVARIANT ⭐ NEW
// ======================================================

#[test]
fn partial_solver_preserves_input_cutting_speed() {

    let vc = CuttingSpeedMMin::new(200.0).unwrap();

    let data = CuttingData {
        diameter: DiameterMm::new(10.0).unwrap(),
        teeth: ToothCount::new(4).unwrap(),
        speed: Some(Speed::CuttingSpeed(vc)),
        feed: None,
    };

    let partial = CuttingDataSolver::solve_partial(&data).unwrap();

    assert_eq!(partial.cutting_speed.unwrap(), vc);
}

// ======================================================
// NO SPEED NO FEED
// ======================================================

#[test]
fn partial_solver_with_no_speed_and_no_feed_returns_empty_computations() {

    let data = CuttingData {
        diameter: DiameterMm::new(10.0).unwrap(),
        teeth: ToothCount::new(4).unwrap(),
        speed: None,
        feed: None,
    };

    let partial = CuttingDataSolver::solve_partial(&data).unwrap();

    assert!(partial.cutting_speed.is_none());
    assert!(partial.spindle_speed.is_none());
    assert!(partial.feed_rate.is_none());
    assert!(partial.feed_per_tooth.is_none());
}
