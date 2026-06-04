// tests/interface/tauri/cutting_data/validation.rs

use cnc_machining_system_lib::interface::cutting_data::{
    solve_cutting_data, SolveCuttingDataRequest,
};

#[test]
fn fails_when_teeth_is_zero() {
    let request = SolveCuttingDataRequest {
        feed_rate_mm_per_min: Some(500.0),
        rpm: Some(5000.0),
        teeth: Some(0), // invalid
        diameter_mm: Some(10.0),
        ..Default::default()
    };

    let result = solve_cutting_data(request);

    assert!(result.is_err());
}

#[test]
fn fails_when_diameter_is_invalid() {
    let request = SolveCuttingDataRequest {
        cutting_speed_m_per_min: Some(200.0),
        diameter_mm: Some(0.0),
        ..Default::default()
    };

    let result = solve_cutting_data(request);

    assert!(result.is_err());
}
