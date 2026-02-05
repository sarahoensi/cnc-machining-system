use cnc_machining_system_lib::domain::features::cutting_data::{CuttingData, CuttingDataSolver, raw::RawCuttingInput};

// tests/cutting_data/pipeline/equivalence.rs
#[test]
fn equivalent_inputs_produce_same_full_solution() {

    let raw_a = RawCuttingInput {
        d: Some(10.0),
        z: Some(4),
        vc: Some(200.0),
        fz: Some(0.05),
        ..Default::default()
    };

    let domain_a = CuttingData::try_from(raw_a).unwrap();
    let full_a = CuttingDataSolver::solve_full(&domain_a).unwrap();

    let raw_b = RawCuttingInput {
        d: Some(10.0),
        z: Some(4),
        n: Some(full_a.spindle_speed.value()),
        f: Some(full_a.feed_rate.value()),
        ..Default::default()
    };

    let domain_b = CuttingData::try_from(raw_b).unwrap();
    let full_b = CuttingDataSolver::solve_full(&domain_b).unwrap();

    assert_eq!(full_a, full_b);
}
