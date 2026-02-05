// tests/cutting_data/solver/idempotence.rs

use cnc_machining_system_lib::domain::features::cutting_data::{
    CuttingData, CuttingDataSolver, raw::RawCuttingInput,
};

// ======================================================
// SOLVER IDEMPOTENCE
// ======================================================

/// Verifies that solve_full is idempotent:
/// solve_full(domain) → rebuild input → solve_full → results are identical
#[test]
fn solve_full_is_idempotent() {
    // Step 1: Create initial raw input
    let raw_input = RawCuttingInput {
        d: Some(10.0),
        z: Some(4),
        vc: Some(200.0),
        fz: Some(0.05),
        ..Default::default()
    };

    // Step 2: Convert to domain model
    let domain = CuttingData::try_from(raw_input).unwrap();

    // Step 3: Solve full
    let solution_1 = CuttingDataSolver::solve_full(&domain).unwrap();

    // Step 4: Rebuild RawCuttingInput from full solution
    // Note: Use cutting_speed (vc) from the solution, not spindle_speed (n)
    let rebuilt_raw = RawCuttingInput {
        d: Some(solution_1.diameter.value()),
        z: Some(solution_1.teeth.value()),
        vc: Some(solution_1.cutting_speed.value()),
        n: None,
        f: None,
        fz: Some(solution_1.feed_per_tooth.value()),
    };

    // Step 5: Convert rebuilt input back to domain
    let domain_rebuilt = CuttingData::try_from(rebuilt_raw).unwrap();

    // Step 6: Solve full again
    let solution_2 = CuttingDataSolver::solve_full(&domain_rebuilt).unwrap();

    // Step 7: Assert solutions are identical
    assert_eq!(solution_1, solution_2);
}

/// Verifies idempotence with spindle speed input variant
#[test]
fn solve_full_is_idempotent_with_spindle_speed() {
    // Step 1: Create initial raw input with spindle speed
    let raw_input = RawCuttingInput {
        d: Some(8.0),
        z: Some(2),
        n: Some(6000.0),
        f: Some(800.0),
        ..Default::default()
    };

    // Step 2: Convert to domain model
    let domain = CuttingData::try_from(raw_input).unwrap();

    // Step 3: Solve full
    let solution_1 = CuttingDataSolver::solve_full(&domain).unwrap();

    // Step 4: Rebuild RawCuttingInput from full solution
    // Note: Use spindle_speed (n) from the solution, not cutting_speed (vc)
    let rebuilt_raw = RawCuttingInput {
        d: Some(solution_1.diameter.value()),
        z: Some(solution_1.teeth.value()),
        vc: None,
        n: Some(solution_1.spindle_speed.value()),
        f: Some(solution_1.feed_rate.value()),
        fz: None,
    };

    // Step 5: Convert rebuilt input back to domain
    let domain_rebuilt = CuttingData::try_from(rebuilt_raw).unwrap();

    // Step 6: Solve full again
    let solution_2 = CuttingDataSolver::solve_full(&domain_rebuilt).unwrap();

    // Step 7: Assert solutions are identical
    assert_eq!(solution_1, solution_2);
}

/// Verifies idempotence with varied diameter and tooth count
#[test]
fn solve_full_is_idempotent_with_varied_parameters() {
    // Step 1: Create initial raw input with different parameters
    let raw_input = RawCuttingInput {
        d: Some(15.0),
        z: Some(6),
        vc: Some(150.0),
        fz: Some(0.08),
        ..Default::default()
    };

    // Step 2: Convert to domain model
    let domain = CuttingData::try_from(raw_input).unwrap();

    // Step 3: Solve full
    let solution_1 = CuttingDataSolver::solve_full(&domain).unwrap();

    // Step 4: Rebuild RawCuttingInput from full solution
    // Note: Use cutting_speed (vc) from the solution, not spindle_speed (n)
    let rebuilt_raw = RawCuttingInput {
        d: Some(solution_1.diameter.value()),
        z: Some(solution_1.teeth.value()),
        vc: Some(solution_1.cutting_speed.value()),
        n: None,
        f: None,
        fz: Some(solution_1.feed_per_tooth.value()),
    };

    // Step 5: Convert rebuilt input back to domain
    let domain_rebuilt = CuttingData::try_from(rebuilt_raw).unwrap();

    // Step 6: Solve full again
    let solution_2 = CuttingDataSolver::solve_full(&domain_rebuilt).unwrap();

    // Step 7: Assert solutions are identical
    assert_eq!(solution_1, solution_2);
}
