// tests/interface/tauri/cutting_data/happy_path.rs

use cnc_machining_system_lib::interface::cutting_data::{
    solve_cutting_data, SolveCuttingDataRequest,
};

#[test]
fn solves_full_pipeline_via_tauri() {
    let request = SolveCuttingDataRequest {
        cutting_speed_m_per_min: Some(200.0),
        diameter_mm: Some(10.0),
        chip_load_mm_per_tooth: Some(0.05),
        teeth: Some(4),
        rpm: None,
        feed_rate_mm_per_min: None,
    };

    let result = solve_cutting_data(request).unwrap();

    assert!(result.rpm.unwrap() > 0.0);
    assert!(result.feed_rate_mm_per_min.unwrap() > 0.0);
}
