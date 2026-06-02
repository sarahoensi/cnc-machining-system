// tests

use cnc_machining_system_lib::interface::tauri::right_triangle::SolveRightTriangleRequest;
use serde_json::json;

#[test]
fn deserializes_valid_json() {
    let json = json!({
        "type": "Legs",
        "a_mm": 3.0,
        "b_mm": 4.0
    });

    let req: SolveRightTriangleRequest = serde_json::from_value(json).unwrap();

    match req {
        SolveRightTriangleRequest::Legs { a_mm, b_mm } => {
            assert_eq!(a_mm, 3.0);
            assert_eq!(b_mm, 4.0);
        }
        _ => panic!(),
    }
}

#[test]
fn fails_when_json_missing_fields() {
    let json = serde_json::json!({
        "type": "Legs",
        "a_mm": 3.0
    });

    let result: Result<SolveRightTriangleRequest, _> = serde_json::from_value(json);

    assert!(result.is_err());
}
