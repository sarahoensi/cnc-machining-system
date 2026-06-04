// tests/application/finishing/register/validation.rs

use cnc_machining_system_lib::{
    application::finishing::{
        dto::{GenerateFinishingPlanInput, RegisterFinishingMeasurementInput},
        GenerateFinishingPlanUseCase, RegisterFinishingMeasurementUseCase,
    },
    domain::machining::finishing::FinishingMode,
};

#[test]
fn register_fails_when_step_number_is_zero() {
    let generate = GenerateFinishingPlanUseCase::new();
    let register = RegisterFinishingMeasurementUseCase::new();

    let mut execution = generate
        .execute(GenerateFinishingPlanInput::ByCuts {
            mode: FinishingMode::Outer,
            start_diameter_mm: 10.0,
            target_diameter_mm: 8.0,
            cuts: 3,
        })
        .unwrap();

    let result = register.execute(
        &mut execution,
        RegisterFinishingMeasurementInput {
            step_number: 0,
            measurement_mm: 9.5,
        },
    );

    assert!(result.is_err());
}

#[test]
fn register_fails_when_step_number_out_of_range() {
    let generate = GenerateFinishingPlanUseCase::new();
    let register = RegisterFinishingMeasurementUseCase::new();

    let mut execution = generate
        .execute(GenerateFinishingPlanInput::ByCuts {
            mode: FinishingMode::Outer,
            start_diameter_mm: 10.0,
            target_diameter_mm: 8.0,
            cuts: 2,
        })
        .unwrap();

    let result = register.execute(
        &mut execution,
        RegisterFinishingMeasurementInput {
            step_number: 99,
            measurement_mm: 9.5,
        },
    );

    assert!(result.is_err());
}

#[test]
fn register_fails_when_measurement_passes_target() {
    let generate = GenerateFinishingPlanUseCase::new();
    let register = RegisterFinishingMeasurementUseCase::new();

    let mut execution = generate
        .execute(GenerateFinishingPlanInput::ByCuts {
            mode: FinishingMode::Outer,
            start_diameter_mm: 10.0,
            target_diameter_mm: 8.0,
            cuts: 2,
        })
        .unwrap();

    // Outer finishing → cannot go below target
    let result = register.execute(
        &mut execution,
        RegisterFinishingMeasurementInput {
            step_number: 1,
            measurement_mm: 7.5,
        },
    );

    assert!(result.is_err());
}
