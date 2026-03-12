// tests/application/finishing/register/edit.rs

use cnc_machining_system_lib::{
    application::finishing::{
        dto::{GenerateFinishingPlanInput, RegisterFinishingMeasurementInput},
        GenerateFinishingPlanUseCase,
        RegisterFinishingMeasurementUseCase,
    },
    domain::machining::finishing::FinishingMode,
};

#[test]
fn edit_overwrites_existing_measurement() {

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
            measurement_mm: 9.6,
        },
    ).unwrap();

    register.execute(
        &mut execution,
        RegisterFinishingMeasurementInput {
            step_number: 1,
            measurement_mm: 9.4,
        },
    ).unwrap();

    assert_eq!(
        execution.steps()[0].measurement().map(|d| d.mm_value()),
        Some(9.4)
    );
}

#[test]
fn edit_recalculates_remaining_steps() {

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

    let step2_before = execution.steps()[1].planned_end().mm_value();

    register.execute(
        &mut execution,
        RegisterFinishingMeasurementInput {
            step_number: 1,
            measurement_mm: 9.3,
        },
    ).unwrap();

    let step2_after = execution.steps()[1].planned_end().mm_value();

    assert_ne!(step2_before, step2_after);
}

#[test]
fn edit_does_not_change_previous_steps() {

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

    let step1_before = execution.steps()[0].measurement();

    register.execute(
        &mut execution,
        RegisterFinishingMeasurementInput {
            step_number: 2,
            measurement_mm: 8.7,
        },
    ).unwrap();

    assert_eq!(execution.steps()[0].measurement(), step1_before);
}

#[test]
fn edit_preserves_target_diameter() {

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
            measurement_mm: 9.5,
        },
    ).unwrap();

    let last_step = execution.steps().last().unwrap();

    let target = execution.plan().target().mm_value();
    let last_end = last_step.planned_end().mm_value();

    assert!((last_end - target).abs() < 1e-9);
}