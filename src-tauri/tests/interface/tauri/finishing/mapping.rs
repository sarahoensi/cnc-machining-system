// mapping.rs

use cnc_machining_system_lib::interface::{GenerateFinishingPlanRequest, generate_finishing_plan, tauri::finishing::FinishingMode};

#[test]
fn generated_response_contains_expected_fields() {

    let response = generate_finishing_plan(
        GenerateFinishingPlanRequest::ByCuts {
            mode: FinishingMode::Outer,
            start_diameter_mm: 10.0,
            target_diameter_mm: 8.0,
            cuts: 3,
        }
    ).unwrap();

    let step = &response.steps[0];

    assert!(step.start_mm > 0.0);
    assert!(step.planned_end_mm > 0.0);
}
