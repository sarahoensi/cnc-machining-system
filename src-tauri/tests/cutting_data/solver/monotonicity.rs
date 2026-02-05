// tests/cutting_data/solver/monotonicity.rs
use cnc_machining_system_lib::domain::features::cutting_data::services::solver::CuttingDataSolver;

use proptest::prelude::*;

use crate::cutting_data::common::strategies::values::*;


// ======================================================
// SPINDLE SPEED MONOTONICITY
// ======================================================

proptest! {

    #[test]
    fn spindle_speed_increases_with_cutting_speed(
        d in diameter(),
        vc1 in cutting_speed(),
        vc2 in cutting_speed(),
    ) {

        prop_assume!(vc2.value() > vc1.value());

        let n1 = CuttingDataSolver::spindle_from_vc(vc1, d);
        let n2 = CuttingDataSolver::spindle_from_vc(vc2, d);

        prop_assert!(n1.is_ok());
        prop_assert!(n2.is_ok());

        let n1 = n1.unwrap();
        let n2 = n2.unwrap();

        prop_assert!(n2.value() > n1.value());
    }
}

// ======================================================
// FEED RATE MONOTONICITY (via feed_per_tooth)
// ======================================================

proptest! {

    #[test]
    fn feed_rate_increases_with_feed_per_tooth(
        z in tooth_count(),
        n in spindle_speed(),
        fz1 in feed_per_tooth(),
        fz2 in feed_per_tooth(),
    ) {

        prop_assume!(fz2.value() > fz1.value());

        let f1 = CuttingDataSolver::feed_from_fz(fz1, z, n);
        let f2 = CuttingDataSolver::feed_from_fz(fz2, z, n);

        prop_assert!(f1.is_ok());
        prop_assert!(f2.is_ok());

        let f1 = f1.unwrap();
        let f2 = f2.unwrap();

        prop_assert!(f2.value() > f1.value());
    }
}

// ======================================================
// FEED RATE MONOTONICITY (via spindle speed)
// ======================================================

proptest! {

    #[test]
    fn feed_rate_increases_with_spindle_speed(
        z in tooth_count(),
        fz in feed_per_tooth(),
        n1 in spindle_speed(),
        n2 in spindle_speed(),
    ) {

        prop_assume!(n2.value() > n1.value());

        let f1 = CuttingDataSolver::feed_from_fz(fz, z, n1);
        let f2 = CuttingDataSolver::feed_from_fz(fz, z, n2);

        prop_assert!(f1.is_ok());
        prop_assert!(f2.is_ok());

        let f1 = f1.unwrap();
        let f2 = f2.unwrap();

        prop_assert!(f2.value() > f1.value());
    }
}

// ======================================================
// CUTTING SPEED MONOTONICITY
// ======================================================

proptest! {

    #[test]
    fn cutting_speed_increases_with_spindle_speed(
        d in diameter(),
        n1 in spindle_speed(),
        n2 in spindle_speed(),
    ) {

        prop_assume!(n2.value() > n1.value());

        let vc1 = CuttingDataSolver::vc_from_spindle(n1, d);
        let vc2 = CuttingDataSolver::vc_from_spindle(n2, d);

        prop_assert!(vc1.is_ok());
        prop_assert!(vc2.is_ok());

        let vc1 = vc1.unwrap();
        let vc2 = vc2.unwrap();

        prop_assert!(vc2.value() > vc1.value());
    }
}
