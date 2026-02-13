// tests/interface/tauri/cutting_data/mapping.rs

use cnc_machining_system_lib::interface::cutting_data::SolveCuttingDataRequest;
use cnc_machining_system_lib::application::SolveCuttingDataInput;

#[test]
fn request_maps_to_application_input() {

    let request = SolveCuttingDataRequest {
        rpm: Some(5000.0),
        diameter_mm: Some(10.0),
        ..Default::default()
    };

    let input: SolveCuttingDataInput = request.into();

    assert_eq!(input.rpm, Some(5000.0));
    assert_eq!(input.diameter_mm, Some(10.0));
}
