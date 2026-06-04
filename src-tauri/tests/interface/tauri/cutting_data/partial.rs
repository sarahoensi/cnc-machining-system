// tests/interface/tauri/cutting_data/partial.rs

use cnc_machining_system_lib::interface::cutting_data::{
    solve_cutting_data, SolveCuttingDataRequest,
};

#[test]
fn solves_only_rpm_when_vc_and_diameter_given() {
    let request = SolveCuttingDataRequest {
        cutting_speed_m_per_min: Some(200.0),
        diameter_mm: Some(10.0),
        rpm: None,
        chip_load_mm_per_tooth: None,
        feed_rate_mm_per_min: None,
        teeth: None,
    };

    let result = solve_cutting_data(request).unwrap();

    assert!(result.rpm.is_some());
    assert!(result.feed_rate_mm_per_min.is_none());
    assert!(result.chip_load_mm_per_tooth.is_none());
}

#[test]
fn returns_empty_solution_when_no_data_given() {
    let request = SolveCuttingDataRequest::default();

    let result = solve_cutting_data(request).unwrap();

    assert!(result.rpm.is_none());
    assert!(result.feed_rate_mm_per_min.is_none());
}
