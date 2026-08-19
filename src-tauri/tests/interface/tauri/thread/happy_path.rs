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
    assert!(options.types.iter().any(|option| option.value == "npt"));
    let npt_sizes = options
        .npt
        .iter()
        .map(|option| option.value.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        npt_sizes,
        vec![
            "1/8", "1/4", "3/8", "1/2", "3/4", "1", "1 1/4", "1 1/2", "2", "2 1/2", "3", "3 1/2",
            "4", "5", "6", "8", "10", "12", "14", "16", "18", "20", "24"
        ]
    );
    assert!(options.npt.iter().all(|option| option.pitches.len() == 1));
    assert!(options.npt.iter().any(|option| {
        option.value == "1"
            && option.pitches[0].value == "11.5"
            && option.pitches[0].label == "11.5 TPI"
            && option.pitches[0].series == "NPT"
            && option.pitches[0].is_default_pitch
    }));
    assert!(options.npt.iter().any(|option| {
        option.value == "24" && option.pitches[0].tap_drill_basis.contains("not populated")
    }));
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

#[test]
fn solves_npt_thread_from_database_values() {
    for (size, pitch, expected_drill_diameter, expected_thread_depth) in [
        ("1/8", "27", 8.433, 0.753),
        ("1/4", "18", 11.112, 1.129),
        ("3/8", "18", 14.287, 1.129),
        ("1/2", "14", 17.859, 1.451),
        ("3/4", "14", 23.019, 1.451),
        ("1", "11.5", 28.972, 1.767),
        ("1 1/4", "11.5", 37.703, 1.767),
        ("1 1/2", "11.5", 43.656, 1.767),
        ("2", "11.5", 55.562, 1.767),
        ("2 1/2", "8", 66.536, 2.540),
        ("3", "8", 82.312, 2.540),
        ("3 1/2", "8", 94.932, 2.540),
        ("4", "8", 107.553, 2.540),
        ("5", "8", 134.384, 2.540),
        ("6", "8", 161.191, 2.540),
        ("8", "8", 211.674, 2.540),
        ("10", "8", 265.311, 2.540),
        ("12", "8", 315.793, 2.540),
        ("14", "8", 347.344, 2.540),
        ("16", "8", 397.827, 2.540),
        ("18", "8", 448.310, 2.540),
        ("20", "8", 498.792, 2.540),
        ("24", "8", 595.186, 2.540),
    ] {
        let result = solve_thread_from_path(
            Path::new(DB_PATH),
            SolveThreadRequest {
                thread_type: "npt".to_string(),
                size: size.to_string(),
                pitch: pitch.to_string(),
            },
        )
        .unwrap();

        assert_close(result.drill_diameter_mm, expected_drill_diameter);
        assert_close(result.thread_depth_mm, expected_thread_depth);
    }
}

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < 0.000001,
        "expected {actual} to be close to {expected}"
    );
}
