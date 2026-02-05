// cuttig_data/services/tests/unit/solve_full.rs

use super::super::approx;
use super::super::super::solver::CuttingDataSolver;

use crate::domain::features::cutting_data::errors::DomainError;
use crate::domain::features::cutting_data::model::{CuttingData, Feed, Speed};
use crate::domain::features::cutting_data::model::values::*;

#[test]
fn solve_full_from_vc_and_fz_computes_all_fields() {
    let data = CuttingData {
        diameter: DiameterMm::new(10.0).unwrap(),
        teeth: ToothCount::new(4).unwrap(),
        speed: Some(Speed::CuttingSpeed(CuttingSpeedMMin::new(200.0).unwrap())),
        feed: Some(Feed::FeedPerTooth(FeedPerToothMm::new(0.05).unwrap())),
    };

    let r = CuttingDataSolver::solve_full(&data).unwrap();

    approx(r.cutting_speed.value(), 200.0, 1e-12);
    approx(r.spindle_speed.value(), 6366.2, 1.0);
    approx(r.feed_per_tooth.value(), 0.05, 1e-12);
    approx(r.feed_rate.value(), 1273.24, 2.0);
}

#[test]
fn solve_full_from_spindle_and_feed_rate_computes_all_fields() {
    let data = CuttingData {
        diameter: DiameterMm::new(8.0).unwrap(),
        teeth: ToothCount::new(2).unwrap(),
        speed: Some(Speed::SpindleSpeed(SpindleSpeedRpm::new(10000.0).unwrap())),
        feed: Some(Feed::FeedRate(FeedRateMmMin::new(800.0).unwrap())),
    };

    let r = CuttingDataSolver::solve_full(&data).unwrap();

    // gitt
    approx(r.spindle_speed.value(), 10000.0, 1e-12);
    approx(r.feed_rate.value(), 800.0, 1e-12);

    // beregnet
    approx(r.feed_per_tooth.value(), 0.04, 0.0001);
    approx(r.cutting_speed.value(), 251.33, 0.5);
}

#[test]
fn solve_full_errors_when_speed_missing() {
    let data = CuttingData {
        diameter: DiameterMm::new(10.0).unwrap(),
        teeth: ToothCount::new(4).unwrap(),
        speed: None,
        feed: Some(Feed::FeedPerTooth(FeedPerToothMm::new(0.05).unwrap())),
    };

    let err = CuttingDataSolver::solve_full(&data).unwrap_err();

    // Hvis DomainError::MissingField inneholder &'static str eller String, funker dette fint.
    assert!(matches!(err, DomainError::MissingField("cutting_speed")));
}

#[test]
fn solve_full_errors_when_feed_missing() {
    let data = CuttingData {
        diameter: DiameterMm::new(10.0).unwrap(),
        teeth: ToothCount::new(4).unwrap(),
        speed: Some(Speed::CuttingSpeed(CuttingSpeedMMin::new(200.0).unwrap())),
        feed: None,
    };

    let err = CuttingDataSolver::solve_full(&data).unwrap_err();
    assert!(matches!(err, DomainError::MissingField("feed_rate")));
}

#[test]
fn solve_full_errors_when_feed_present_but_spindle_not_computable() {
    // Med din solve_partial: feed kan ikke regnes uten spindle_speed.
    // Og spindle_speed kan ikke regnes uten speed.
    let data = CuttingData {
        diameter: DiameterMm::new(10.0).unwrap(),
        teeth: ToothCount::new(4).unwrap(),
        speed: None,
        feed: Some(Feed::FeedRate(FeedRateMmMin::new(800.0).unwrap())),
    };

    let err = CuttingDataSolver::solve_full(&data).unwrap_err();
    assert!(matches!(err, DomainError::MissingField("cutting_speed")));
}
