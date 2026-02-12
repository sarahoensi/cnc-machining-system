// tests/integration/tauri/finishing/generate.rs

use cnc_machining_system_lib::interface::{
    GenerateFinishingPlanRequest, generate_finishing_plan, tauri::finishing::FinishingMode

};

//use cnc_machining_system_lib::interface::FinishingMode;

#[test]
fn generates_plan_via_tauri() {

    let request = GenerateFinishingPlanRequest::ByCuts {
        mode: FinishingMode::Outer,
        start_diameter_mm: 10.0,
        target_diameter_mm: 8.0,
        cuts: 3,
    };

    let response = generate_finishing_plan(request).unwrap();

    assert_eq!(response.steps.len(), 3);
    assert!(!response.execution_id.is_empty());
}
