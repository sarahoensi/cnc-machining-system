use cnc_machining_system_lib::interface::thread::{
    list_thread_options_from_path, solve_thread_from_path, SolveThreadRequest,
};
use std::path::Path;

const DB_PATH: &str = "data/threads.sqlite";

#[test]
fn lists_thread_options_for_frontend_dropdowns() {
    let options = list_thread_options_from_path(Path::new(DB_PATH)).unwrap();

    assert!(options.types.iter().any(|option| option.value == "metric"));
    assert!(options.metric.iter().any(|option| option.value == "M10"));
    assert!(options.unc.iter().any(|option| option.value == "1/4"));
    assert!(options
        .unc
        .iter()
        .any(|option| option.value == "1 1/8" && option.label == "1 1/8"));
    assert!(options
        .unf
        .iter()
        .any(|option| option.value == "1 1/4" && option.label == "1 1/4"));
}

#[test]
fn solves_thread_via_tauri_command() {
    let result = solve_thread_from_path(
        Path::new(DB_PATH),
        SolveThreadRequest {
            thread_type: "metric".to_string(),
            size: "M10".to_string(),
            pitch: "1.5".to_string(),
        },
    )
    .unwrap();

    assert_close(result.drill_diameter_mm, 8.5);
    assert_close(result.thread_depth_mm, 0.8119);
}

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < 0.000001,
        "expected {actual} to be close to {expected}"
    );
}
