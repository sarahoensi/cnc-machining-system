// tests/integration/finishing/workflow.rs

use cnc_machining_system_lib::{
    application::finishing::{
        dto::{GenerateFinishingPlanInput, RegisterFinishingMeasurementInput},
        GenerateFinishingPlanUseCase,
        RegisterFinishingMeasurementUseCase,
    },
    domain::machining::finishing::FinishingMode,
};

#[test]
fn finishing_full_workflow_via_use_cases() {

    let generate_uc = GenerateFinishingPlanUseCase::new();
    let register_uc = RegisterFinishingMeasurementUseCase::new();

    // -------------------------------------------------
    // Step 1: Generate execution
    // -------------------------------------------------

    let mut execution = generate_uc
        .execute(GenerateFinishingPlanInput::ByCuts {
            mode: FinishingMode::Outer,
            start_diameter_mm: 10.0,
            target_diameter_mm: 8.0,
            cuts: 3,
        })
        .unwrap();

    // -------------------------------------------------
    // Step 2: Register first measurement
    // -------------------------------------------------

    register_uc
        .execute(
            &mut execution,
            RegisterFinishingMeasurementInput {
                step_number: 1,
                measurement_mm: 9.6,
            },
        )
        .unwrap();

    assert_eq!(
        execution.steps()[0].measurement().map(|d| d.mm_value()),
        Some(9.6)
    );

    // -------------------------------------------------
    // Step 3: Register second measurement
    // -------------------------------------------------

    register_uc
        .execute(
            &mut execution,
            RegisterFinishingMeasurementInput {
                step_number: 2,
                measurement_mm: 8.9,
            },
        )
        .unwrap();

    assert_eq!(
        execution.steps()[1].measurement().map(|d| d.mm_value()),
        Some(8.9)
    );
}