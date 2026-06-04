// tests/integration/finishing/validation.rs

use cnc_machining_system_lib::{
    application::finishing::{
        dto::{GenerateFinishingPlanInput, RegisterFinishingMeasurementInput},
        GenerateFinishingPlanUseCase, RegisterFinishingMeasurementUseCase,
    },
    domain::machining::finishing::FinishingMode,
};

#[test]
fn fails_when_step_is_out_of_range() {
    // ----------------------------------------------------
    // Setup
    // ----------------------------------------------------

    let generate_uc = GenerateFinishingPlanUseCase::new();
    let register_uc = RegisterFinishingMeasurementUseCase::new();

    // ----------------------------------------------------
    // Generate execution
    // ----------------------------------------------------

    let mut execution = generate_uc
        .execute(GenerateFinishingPlanInput::ByCuts {
            mode: FinishingMode::Outer,
            start_diameter_mm: 10.0,
            target_diameter_mm: 8.0,
            cuts: 2,
        })
        .unwrap();

    // ----------------------------------------------------
    // Try invalid step
    // ----------------------------------------------------

    let result = register_uc.execute(
        &mut execution,
        RegisterFinishingMeasurementInput {
            step_number: 99,
            measurement_mm: 9.0,
        },
    );

    // ----------------------------------------------------
    // Assert
    // ----------------------------------------------------

    assert!(result.is_err());
}
