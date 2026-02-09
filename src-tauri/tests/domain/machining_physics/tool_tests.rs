// tests/domain/machining_physics/tool_tests.rs

use cnc_machining_system_lib::domain::*;

#[test]
fn tooth_count_valid() {
    let t = ToothCount::new(4).unwrap();
    assert_eq!(t.value(), 4);
}

#[test]
fn tooth_count_rejects_zero() {
    assert!(ToothCount::new(0).is_err());
}

#[test]
fn tool_stores_properties() {
    let tool = Tool::new(
        Diameter::mm(10.0).unwrap(),
        ToothCount::new(4).unwrap(),
    );

    assert_eq!(tool.teeth().value(), 4);
    assert_eq!(tool.diameter().mm_value(), 10.0);
}
