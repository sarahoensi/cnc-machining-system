// cutting_data/services/tests/unit/conversions.rs


use super::super::approx;
use super::super::super::solver::CuttingDataSolver;
use crate::domain::features::cutting_data::model::values::*;

#[test]
fn vc_to_spindle_matches_expected() {
    let d = DiameterMm::new(10.0).unwrap();
    let vc = CuttingSpeedMMin::new(200.0).unwrap();

    let n = CuttingDataSolver::spindle_from_vc(vc, d).unwrap();
    approx(n.value(), 6366.2, 1.0);
}

#[test]
fn spindle_to_vc_matches_expected() {
    let d = DiameterMm::new(8.0).unwrap();
    let n = SpindleSpeedRpm::new(10000.0).unwrap();

    let vc = CuttingDataSolver::vc_from_spindle(n, d).unwrap();
    approx(vc.value(), 251.33, 0.5);
}

#[test]
fn fz_to_feed_matches_expected() {
    let z = ToothCount::new(4).unwrap();
    let n = SpindleSpeedRpm::new(6366.2).unwrap();
    let fz = FeedPerToothMm::new(0.05).unwrap();

    let f = CuttingDataSolver::feed_from_fz(fz, z, n).unwrap();
    approx(f.value(), 1273.24, 2.0);
}

#[test]
fn feed_to_fz_matches_expected() {
    let z = ToothCount::new(2).unwrap();
    let n = SpindleSpeedRpm::new(10000.0).unwrap();
    let f = FeedRateMmMin::new(800.0).unwrap();

    let fz = CuttingDataSolver::fz_from_feed(f, z, n).unwrap();
    approx(fz.value(), 0.04, 0.0001);
}
