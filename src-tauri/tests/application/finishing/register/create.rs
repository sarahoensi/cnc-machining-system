// tests/application/finishing/register/create.rs

use cnc_machining_system_lib::{
    application::finishing::{
        dto::{GenerateFinishingPlanInput, RegisterFinishingMeasurementInput},
        GenerateFinishingPlanUseCase,
        RegisterFinishingMeasurementUseCase,
    },
    domain::machining::finishing::FinishingMode,
};

#[test]
fn register_updates_only_selected_step() {

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
            step_number: 2,
            measurement_mm: 8.7,
        }
    ).unwrap();

    assert!(execution.steps()[1].measurement().is_some());
    assert!(execution.steps()[0].measurement().is_none());
    assert!(execution.steps()[2].measurement().is_none());
}


#[test]
fn register_creates_measurement() {

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

    register.execute(
        &mut execution,
        RegisterFinishingMeasurementInput {
            step_number: 1,
            measurement_mm: 9.5,
        }
    ).unwrap();

    assert_eq!(
        execution.steps()[0].measurement().map(|d| d.mm_value()),
        Some(9.5)
    );
}