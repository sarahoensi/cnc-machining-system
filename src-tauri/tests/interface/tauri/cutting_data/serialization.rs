// tests/interface/tauri/cutting_data/serialization.rs

use cnc_machining_system_lib::interface::cutting_data::SolveCuttingDataRequest;

#[test]
fn deserializes_valid_json() {

    let json = r#"
    {
        "rpm": 5000,
        "diameter_mm": 10
    }
    "#;

    let req: SolveCuttingDataRequest = serde_json::from_str(json).unwrap();

    assert_eq!(req.rpm, Some(5000.0));
}
