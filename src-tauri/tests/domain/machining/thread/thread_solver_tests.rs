use cnc_machining_system_lib::domain::machining::thread::{
    resolve_thread_spec, ThreadSolver, ThreadType,
};

#[test]
fn metric_thread_solves_drill_diameter_and_depth() {
    let spec = resolve_thread_spec(ThreadType::Metric, "M10", "1.5").unwrap();
    let result = ThreadSolver::solve(&spec);

    assert_close(result.drill_diameter_mm, 8.5);
    assert_close(result.thread_depth_mm, 0.811899);
}

#[test]
fn unified_thread_converts_tpi_to_pitch_mm() {
    let spec = resolve_thread_spec(ThreadType::Unc, "1/4", "20").unwrap();
    let result = ThreadSolver::solve(&spec);

    assert_close(spec.pitch_mm, 1.27);
    assert_close(result.drill_diameter_mm, 5.08);
}

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < 0.000001,
        "expected {actual} to be close to {expected}"
    );
}
