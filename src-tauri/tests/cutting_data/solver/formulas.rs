// tests/cutting_data/solver/formulas.rs
use cnc_machining_system_lib::domain::features::cutting_data::services::solver::CuttingDataSolver;

use proptest::prelude::*;

use crate::cutting_data::common::strategies::values::*;
use crate::cutting_data::common::approx::approx_eq;

// ======================================================
// ROUNDTRIP FORMULA PROPERTIES
// ======================================================
//
// These tests verify the algebraic correctness of the conversion formulas.
// They test that the mathematical relationships hold: if we convert from
// one unit system to another and back, we recover the original value.
//
// This is distinct from solver/roundtrip.rs which tests domain invariants
// across the full CuttingData conversion pipeline.
//
// Here we focus on the pure math: formulas are correct inverses of each other.

proptest! {

    #[test]
    fn vc_spindle_roundtrip(
        d in diameter(),
        vc0 in cutting_speed(),
    ) {

        let n = CuttingDataSolver::spindle_from_vc(vc0, d);
        prop_assert!(n.is_ok());

        let n = n.unwrap();

        let vc1 = CuttingDataSolver::vc_from_spindle(n, d);
        prop_assert!(vc1.is_ok());

        let vc1 = vc1.unwrap();

        prop_assert!(
            approx_eq(vc1.value(), vc0.value(), 1e-9)
        );
    }

    #[test]
    fn spindle_vc_roundtrip(
        d in diameter(),
        n0 in spindle_speed(),
    ) {

        let vc = CuttingDataSolver::vc_from_spindle(n0, d);
        prop_assert!(vc.is_ok());

        let vc = vc.unwrap();

        let n1 = CuttingDataSolver::spindle_from_vc(vc, d);
        prop_assert!(n1.is_ok());

        let n1 = n1.unwrap();

        prop_assert!(
            approx_eq(n1.value(), n0.value(), 1e-9)
        );
    }

    #[test]
    fn feed_roundtrip(
        z in tooth_count(),
        n in spindle_speed(),
        fz0 in feed_per_tooth(),
    ) {

        let f = CuttingDataSolver::feed_from_fz(fz0, z, n);
        prop_assert!(f.is_ok());

        let f = f.unwrap();

        let fz1 = CuttingDataSolver::fz_from_feed(f, z, n);
        prop_assert!(fz1.is_ok());

        let fz1 = fz1.unwrap();

        prop_assert!(
            approx_eq(fz1.value(), fz0.value(), 1e-9)
        );
    }
}

// ======================================================
// NUMERIC STABILITY PROPERTIES
// ======================================================

proptest! {

    #[test]
    fn solver_outputs_are_positive_and_finite(
        d in diameter(),
        z in tooth_count(),
        vc in cutting_speed(),
        fz in feed_per_tooth(),
    ) {

        let n = CuttingDataSolver::spindle_from_vc(vc, d);
        prop_assert!(n.is_ok());

        let n = n.unwrap();

        let f = CuttingDataSolver::feed_from_fz(fz, z, n);
        prop_assert!(f.is_ok());

        let f = f.unwrap();

        // ---- Spindle speed stability ----
        prop_assert!(n.value().is_finite());
        prop_assert!(n.value() > 0.0);

        // ---- Feed rate stability ----
        prop_assert!(f.value().is_finite());
        prop_assert!(f.value() > 0.0);
    }
}

// ======================================================
// MONOTONICITY PROPERTIES
// ======================================================

proptest! {

    #[test]
    fn spindle_speed_is_monotonic_in_cutting_speed(
        d in diameter(),
        vc1 in cutting_speed(),
        vc2 in cutting_speed(),
    ) {

        // Only compare when ordered
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
// DETERMINISM PROPERTY
// ======================================================

proptest! {

    #[test]
    fn formulas_are_deterministic(
        d in diameter(),
        vc in cutting_speed(),
    ) {

        let n1 = CuttingDataSolver::spindle_from_vc(vc, d);
        let n2 = CuttingDataSolver::spindle_from_vc(vc, d);

        prop_assert!(n1.is_ok());
        prop_assert!(n2.is_ok());

        let n1 = n1.unwrap();
        let n2 = n2.unwrap();

        prop_assert_eq!(n1, n2);
    }
}
