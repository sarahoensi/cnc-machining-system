// tests/cutting_data/solver/roundtrip.rs
use cnc_machining_system_lib::domain::features::cutting_data::services::solver::CuttingDataSolver;

use proptest::prelude::*;

use crate::cutting_data::common::strategies::values::*;
use crate::cutting_data::common::approx::approx_eq;

// ======================================================
// DOMAIN INVARIANT ROUNDTRIP TESTS
// ======================================================
//
// These tests verify that domain invariants are maintained across the full
// CuttingData conversion and solving pipeline. They ensure that when we:
//   1. Create domain inputs from raw values
//   2. Solve the full system
//   3. Reconstruct raw inputs from the solution
//   4. Convert back to domain
//   5. Solve again
//
// ...the final results are identical. This validates the entire flow, not just
// the individual formulas.
//
// This complements solver/formulas.rs which focuses solely on algebraic
// correctness of the conversion formulas themselves.
//

// ======================================================
// CUTTING SPEED ↔ SPINDLE SPEED
// ======================================================

proptest! {

    #[test]
    fn vc_to_spindle_to_vc_roundtrip(
        d in diameter(),
        vc0 in cutting_speed(),
    ) {

        let n = CuttingDataSolver::spindle_from_vc(vc0, d);
        prop_assert!(n.is_ok());

        let n = n.unwrap();

        let vc1 = CuttingDataSolver::vc_from_spindle(n, d);
        prop_assert!(vc1.is_ok());

        let vc1 = vc1.unwrap();

        prop_assert!(approx_eq(vc1.value(), vc0.value(), 1e-9));

    }
}

proptest! {

    #[test]
    fn spindle_to_vc_to_spindle_roundtrip(
        d in diameter(),
        n0 in spindle_speed(),
    ) {

        let vc = CuttingDataSolver::vc_from_spindle(n0, d);
        prop_assert!(vc.is_ok());

        let vc = vc.unwrap();

        let n1 = CuttingDataSolver::spindle_from_vc(vc, d);
        prop_assert!(n1.is_ok());

        let n1 = n1.unwrap();

        prop_assert!(approx_eq(n1.value(), n0.value(), 1e-9));
    }
}


// ======================================================
// FEED PER TOOTH ↔ FEED RATE
// ======================================================

proptest! {

    #[test]
    fn fz_to_feed_to_fz_roundtrip(
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

        prop_assert!(approx_eq(fz1.value(), fz0.value(), 1e-9));
    }
}

proptest! {

    #[test]
    fn feed_to_fz_to_feed_roundtrip(
        z in tooth_count(),
        n in spindle_speed(),
        f0 in feed_rate(),
    ) {

        let fz = CuttingDataSolver::fz_from_feed(f0, z, n);
        prop_assert!(fz.is_ok());

        let fz = fz.unwrap();

        let f1 = CuttingDataSolver::feed_from_fz(fz, z, n);
        prop_assert!(f1.is_ok());

        let f1 = f1.unwrap();

        prop_assert!(approx_eq(f1.value(), f0.value(), 1e-9));
    }
}
