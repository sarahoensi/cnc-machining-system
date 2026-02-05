// tests/cutting_data/pipeline/full_flow.rs

use cnc_machining_system_lib::domain::features::cutting_data::{CuttingData, CuttingDataSolver, raw::RawCuttingInput};
use crate::cutting_data::common::approx::approx_eq;

#[test]
fn raw_input_produces_valid_full_solution() {
    let raw = RawCuttingInput {
        d: Some(10.0),
        z: Some(4),
        vc: Some(200.0),
        fz: Some(0.05),
        ..Default::default()
    };

    let domain = CuttingData::try_from(raw).unwrap();

    let full = CuttingDataSolver::solve_full(&domain).unwrap();

    // ======================================================
    // ALL VALUES ARE POSITIVE AND FINITE
    // ======================================================
    
    assert!(full.diameter.value() > 0.0, "diameter must be positive");
    assert!(full.teeth.value() > 0, "teeth must be positive");
    assert!(full.cutting_speed.value() > 0.0, "cutting speed must be positive");
    assert!(full.spindle_speed.value() > 0.0, "spindle speed must be positive");
    assert!(full.feed_rate.value() > 0.0, "feed rate must be positive");
    assert!(full.feed_per_tooth.value() > 0.0, "feed per tooth must be positive");

    assert!(full.diameter.value().is_finite(), "diameter must be finite");
    assert!(full.cutting_speed.value().is_finite(), "cutting speed must be finite");
    assert!(full.spindle_speed.value().is_finite(), "spindle speed must be finite");
    assert!(full.feed_rate.value().is_finite(), "feed rate must be finite");
    assert!(full.feed_per_tooth.value().is_finite(), "feed per tooth must be finite");

    // ======================================================
    // CROSS-FORMULA CONSISTENCY
    // ======================================================

    // Recompute cutting speed from spindle speed and diameter
    let computed_vc = CuttingDataSolver::vc_from_spindle(
        full.spindle_speed,
        full.diameter,
    ).expect("vc_from_spindle should succeed");

    assert!(
        approx_eq(computed_vc.value(), full.cutting_speed.value(), 1e-9),
        "recomputed cutting speed must match: computed={}, expected={}",
        computed_vc.value(),
        full.cutting_speed.value()
    );

    // Recompute spindle speed from cutting speed and diameter
    let computed_n = CuttingDataSolver::spindle_from_vc(
        full.cutting_speed,
        full.diameter,
    ).expect("spindle_from_vc should succeed");

    assert!(
        approx_eq(computed_n.value(), full.spindle_speed.value(), 1e-9),
        "recomputed spindle speed must match: computed={}, expected={}",
        computed_n.value(),
        full.spindle_speed.value()
    );

    // Recompute feed rate from feed per tooth
    let computed_f = CuttingDataSolver::feed_from_fz(
        full.feed_per_tooth,
        full.teeth,
        full.spindle_speed,
    ).expect("feed_from_fz should succeed");

    assert!(
        approx_eq(computed_f.value(), full.feed_rate.value(), 1e-9),
        "recomputed feed rate must match: computed={}, expected={}",
        computed_f.value(),
        full.feed_rate.value()
    );

    // Recompute feed per tooth from feed rate
    let computed_fz = CuttingDataSolver::fz_from_feed(
        full.feed_rate,
        full.teeth,
        full.spindle_speed,
    ).expect("fz_from_feed should succeed");

    assert!(
        approx_eq(computed_fz.value(), full.feed_per_tooth.value(), 1e-9),
        "recomputed feed per tooth must match: computed={}, expected={}",
        computed_fz.value(),
        full.feed_per_tooth.value()
    );
}
