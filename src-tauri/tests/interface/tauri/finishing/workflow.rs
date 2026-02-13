// tests/integration/tauri/finishing/workflow.rs

use cnc_machining_system_lib::interface::finishing::{
    generate_finishing_plan,
    register_finishing_measurement,
    GenerateFinishingPlanRequest,
    RegisterFinishingMeasurementRequest,
};

use cnc_machining_system_lib::interface::tauri::finishing::FinishingMode;

#[test]
fn finishing_full_workflow_via_tauri() {

    let generated = generate_finishing_plan(
        GenerateFinishingPlanRequest::ByCuts {
            mode: FinishingMode::Outer,
            start_diameter_mm: 10.0,
            target_diameter_mm: 8.0,
            cuts: 3,
        }
    ).unwrap();

    let id = generated.execution_id;

    let after_step1 = register_finishing_measurement(
        RegisterFinishingMeasurementRequest {
            execution_id: id.clone(),
            step_number: 1,
            measurement_mm: 9.6,
        }
    ).unwrap();

    assert_eq!(after_step1.steps[0].measurement_mm, Some(9.6));

    let after_step2 = register_finishing_measurement(
        RegisterFinishingMeasurementRequest {
            execution_id: id.clone(),
            step_number: 2,
            measurement_mm: 8.9,
        }
    ).unwrap();

    assert_eq!(after_step2.steps[1].measurement_mm, Some(8.9));
}
