// tests/integration/tauri/finishing/register.rs

use cnc_machining_system_lib::interface::finishing::{
    generate_finishing_plan,
    register_finishing_measurement,
    GenerateFinishingPlanRequest,
    RegisterFinishingMeasurementRequest,
};

use cnc_machining_system_lib::interface::tauri::finishing::FinishingMode;

#[test]
fn registers_measurement_via_tauri() {

    let generated = generate_finishing_plan(
        GenerateFinishingPlanRequest::ByCuts {
            mode: FinishingMode::Outer,
            start_diameter_mm: 10.0,
            target_diameter_mm: 8.0,
            cuts: 3,
        }
    ).unwrap();

    let response = register_finishing_measurement(
        RegisterFinishingMeasurementRequest {
            execution_id: generated.execution_id.clone(),
            step_number: 1,
            measurement_mm: 9.6,
        }
    ).unwrap();

    assert_eq!(response.steps[0].measurement_mm, Some(9.6));
}
