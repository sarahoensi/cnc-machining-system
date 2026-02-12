// tests/integration/tauri/finishing/validation.rs

use cnc_machining_system_lib::interface::{
    GenerateFinishingPlanRequest, RegisterFinishingMeasurementRequest, generate_finishing_plan, register_finishing_measurement, tauri::finishing::FinishingMode
};

#[test]
fn fails_when_execution_id_is_invalid() {

    let result = register_finishing_measurement(
        RegisterFinishingMeasurementRequest {
            execution_id: "not-a-uuid".into(),
            step_number: 1,
            measurement_mm: 9.5,
        }
    );

    assert!(result.is_err());
}

#[test]
fn fails_when_step_is_out_of_range() {

    let generated = generate_finishing_plan(
        GenerateFinishingPlanRequest::ByCuts {
            mode: FinishingMode::Outer,
            start_diameter_mm: 10.0,
            target_diameter_mm: 8.0,
            cuts: 2,
        }
    ).unwrap();

    let result = register_finishing_measurement(
        RegisterFinishingMeasurementRequest {
            execution_id: generated.execution_id,
            step_number: 99,
            measurement_mm: 9.0,
        }
    );

    assert!(result.is_err());
}
