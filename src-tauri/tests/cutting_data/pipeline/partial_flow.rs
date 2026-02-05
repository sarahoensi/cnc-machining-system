use cnc_machining_system_lib::domain::features::cutting_data::{CuttingData, CuttingDataSolver, raw::RawCuttingInput};

// tests/cutting_data/pipeline/partial_flow.rs
#[test]
fn raw_speed_only_produces_partial_solution() {
    let raw = RawCuttingInput {
        d: Some(10.0),
        z: Some(4),
        vc: Some(200.0),
        ..Default::default()
    };

    let domain = CuttingData::try_from(raw).unwrap();

    let partial = CuttingDataSolver::solve_partial(&domain).unwrap();

    assert!(partial.spindle_speed.is_some());
    assert!(partial.feed_rate.is_none());
}
