// cutting_data/services/tests/unit/round_trips.rs

use super::super::approx;
use super::super::super::solver::CuttingDataSolver;
use crate::domain::features::cutting_data::model::values::*;

#[test]
fn round_trip_vc_to_n_to_vc() {
    let d = DiameterMm::new(12.0).unwrap();
    let vc0 = CuttingSpeedMMin::new(180.0).unwrap();

    let n = CuttingDataSolver::spindle_from_vc(vc0, d).unwrap();
    let vc1 = CuttingDataSolver::vc_from_spindle(n, d).unwrap();

    approx(vc1.value(), vc0.value(), 1e-9);
}

#[test]
fn round_trip_fz_to_f_to_fz() {
    let z = ToothCount::new(3).unwrap();
    let n = SpindleSpeedRpm::new(9000.0).unwrap();
    let fz0 = FeedPerToothMm::new(0.07).unwrap();

    let f = CuttingDataSolver::feed_from_fz(fz0, z, n).unwrap();
    let fz1 = CuttingDataSolver::fz_from_feed(f, z, n).unwrap();

    approx(fz1.value(), fz0.value(), 1e-9);
}
