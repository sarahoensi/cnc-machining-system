// cutting_data/services/tests/unit/solve_partial.rs

use super::super::approx;
use super::super::super::solver::CuttingDataSolver;
use crate::domain::features::cutting_data::model::{CuttingData, Feed, Speed};
use crate::domain::features::cutting_data::model::values::*;

#[test]
fn solve_partial_only_speed() {
    let data = CuttingData {
        diameter: DiameterMm::new(10.0).unwrap(),
        teeth: Some(ToothCount::new(4).unwrap()),
        speed: Some(Speed::CuttingSpeed(CuttingSpeedMMin::new(200.0).unwrap())),
        feed: None,
    };

    let p = CuttingDataSolver::solve_partial(&data).unwrap();
    assert!(p.cutting_speed.is_some());
    assert!(p.spindle_speed.is_some());
    assert!(p.feed_rate.is_none());
    assert!(p.feed_per_tooth.is_none());
}

#[test]
fn solve_partial_only_feed_does_not_compute_without_spindle() {
    let data = CuttingData {
        diameter: DiameterMm::new(10.0).unwrap(),
        teeth: Some(ToothCount::new(4).unwrap()),
        speed: None,
        feed: Some(Feed::FeedPerTooth(FeedPerToothMm::new(0.05).unwrap())),
    };

    let p = CuttingDataSolver::solve_partial(&data).unwrap();
    assert!(p.feed_rate.is_none());
    assert!(p.feed_per_tooth.is_none());
}

#[test]
fn solve_partial_speed_plus_fz_computes_everything() {
    let data = CuttingData {
        diameter: DiameterMm::new(10.0).unwrap(),
        teeth: Some(ToothCount::new(4).unwrap()),
        speed: Some(Speed::CuttingSpeed(CuttingSpeedMMin::new(200.0).unwrap())),
        feed: Some(Feed::FeedPerTooth(FeedPerToothMm::new(0.05).unwrap())),
    };

    let p = CuttingDataSolver::solve_partial(&data).unwrap();

    assert!(p.cutting_speed.is_some());
    assert!(p.spindle_speed.is_some());
    assert!(p.feed_rate.is_some());
    assert!(p.feed_per_tooth.is_some());

    approx(p.spindle_speed.unwrap().value(), 6366.2, 1.0);
    approx(p.feed_rate.unwrap().value(), 1273.24, 2.0);
    approx(p.feed_per_tooth.unwrap().value(), 0.05, 1e-12);
    approx(p.cutting_speed.unwrap().value(), 200.0, 1e-12);
}

#[test]
fn solve_partial_speed_plus_feed_rate_computes_fz() {
    let data = CuttingData {
        diameter: DiameterMm::new(8.0).unwrap(),
        teeth: Some(ToothCount::new(2).unwrap()),
        speed: Some(Speed::SpindleSpeed(SpindleSpeedRpm::new(10000.0).unwrap())),
        feed: Some(Feed::FeedRate(FeedRateMmMin::new(800.0).unwrap())),
    };

    let p = CuttingDataSolver::solve_partial(&data).unwrap();

    assert!(p.cutting_speed.is_some());
    assert!(p.spindle_speed.is_some());
    assert!(p.feed_rate.is_some());
    assert!(p.feed_per_tooth.is_some());

    approx(p.spindle_speed.unwrap().value(), 10000.0, 1e-12);
    approx(p.feed_rate.unwrap().value(), 800.0, 1e-12);
    approx(p.feed_per_tooth.unwrap().value(), 0.04, 0.0001);
    approx(p.cutting_speed.unwrap().value(), 251.33, 0.5);
}

#[test]
fn solve_partial_no_speed_no_feed_returns_none_for_computed_fields() {
    let data = CuttingData {
        diameter: DiameterMm::new(10.0).unwrap(),
        teeth: Some(ToothCount::new(4).unwrap()),
        speed: None,
        feed: None,
    };

    let p = CuttingDataSolver::solve_partial(&data).unwrap();

    assert!(p.cutting_speed.is_none());
    assert!(p.spindle_speed.is_none());
    assert!(p.feed_rate.is_none());
    assert!(p.feed_per_tooth.is_none());
}
