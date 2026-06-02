// tests/integration/finishing/register.rs

use cnc_machining_system_lib::{
    application::finishing::{
        dto::{GenerateFinishingPlanInput, RegisterFinishingMeasurementInput},
        GenerateFinishingPlanUseCase, RegisterFinishingMeasurementUseCase,
    },
    domain::machining::finishing::FinishingMode,
};

#[test]
fn registers_measurement_via_use_cases() {
    let generate_uc = GenerateFinishingPlanUseCase::new();
    let register_uc = RegisterFinishingMeasurementUseCase::new();

    // ----------------------------------------------------
    // Step 1: Generate execution
    // ----------------------------------------------------

    let mut execution = generate_uc
        .execute(GenerateFinishingPlanInput::ByCuts {
            mode: FinishingMode::Outer,
            start_diameter_mm: 10.0,
            target_diameter_mm: 8.0,
            cuts: 3,
        })
        .unwrap();

    // ----------------------------------------------------
    // Step 2: Register measurement
    // ----------------------------------------------------

    register_uc
        .execute(
            &mut execution,
            RegisterFinishingMeasurementInput {
                step_number: 1,
                measurement_mm: 9.6,
            },
        )
        .unwrap();

    // ----------------------------------------------------
    // Step 3: Assert
    // ----------------------------------------------------

    assert_eq!(
        execution.steps()[0].measurement().map(|d| d.mm_value()),
        Some(9.6)
    );
}
