// tests/application/finishing/register/locking.rs

use cnc_machining_system_lib::{
    application::finishing::{
        dto::{GenerateFinishingPlanInput, RegisterFinishingMeasurementInput},
        GenerateFinishingPlanUseCase,
        RegisterFinishingMeasurementUseCase,
    },
    domain::machining::finishing::FinishingMode,
};

#[test]
fn edit_previous_step_after_later_measurement_fails() {

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

    register.execute(
        &mut execution,
        RegisterFinishingMeasurementInput {
            step_number: 1,
            measurement_mm: 9.6,
        },
    ).unwrap();

    register.execute(
        &mut execution,
        RegisterFinishingMeasurementInput {
            step_number: 2,
            measurement_mm: 8.9,
        },
    ).unwrap();

    let result = register.execute(
        &mut execution,
        RegisterFinishingMeasurementInput {
            step_number: 1,
            measurement_mm: 9.5,
        },
    );

    assert!(result.is_err());
}

#[test]
fn edit_last_measured_step_is_allowed() {

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

    register.execute(
        &mut execution,
        RegisterFinishingMeasurementInput {
            step_number: 1,
            measurement_mm: 9.6,
        },
    ).unwrap();

    register.execute(
        &mut execution,
        RegisterFinishingMeasurementInput {
            step_number: 2,
            measurement_mm: 8.9,
        },
    ).unwrap();

    let result = register.execute(
        &mut execution,
        RegisterFinishingMeasurementInput {
            step_number: 2,
            measurement_mm: 8.7,
        },
    );

    assert!(result.is_ok());
}