// tests/application/finishing/register/edit.rs

use cnc_machining_system_lib::{
    application::finishing::{
        dto::RegisterFinishingMeasurementInput,
        RegisterFinishingMeasurementUseCase,
    },
};

use crate::application::finishing::fixtures::{create_execution, repo};


#[test]
fn edit_overwrites_existing_measurement() {
    let repo = repo();
    let register = RegisterFinishingMeasurementUseCase::new(repo.clone());

    let id = create_execution(repo.clone(), 2);

    register.execute(RegisterFinishingMeasurementInput {
        execution_id: id,
        step_number: 1,
        measurement_mm: 9.6,
    }).unwrap();

    register.execute(RegisterFinishingMeasurementInput {
        execution_id: id,
        step_number: 1,
        measurement_mm: 9.4,
    }).unwrap();

    let execution = repo.get(id).unwrap();

    assert_eq!(
        execution.steps()[0].measurement().map(|d| d.mm_value()),
        Some(9.4)
    );
}


#[test]
fn edit_recalculates_remaining_steps() {
    let repo = repo();
    let register = RegisterFinishingMeasurementUseCase::new(repo.clone());

    let id = create_execution(repo.clone(), 3);

    register.execute(RegisterFinishingMeasurementInput {
        execution_id: id,
        step_number: 1,
        measurement_mm: 9.6,
    }).unwrap();

    let before = repo.get(id).unwrap();
    let step2_before = before.steps()[1].planned_end().mm_value();

    register.execute(RegisterFinishingMeasurementInput {
        execution_id: id,
        step_number: 1,
        measurement_mm: 9.3,
    }).unwrap();

    let after = repo.get(id).unwrap();
    let step2_after = after.steps()[1].planned_end().mm_value();

    assert_ne!(step2_before, step2_after);
}


#[test]
fn edit_does_not_change_previous_steps() {
    let repo = repo();
    let register = RegisterFinishingMeasurementUseCase::new(repo.clone());

    let id = create_execution(repo.clone(), 3);

    register.execute(RegisterFinishingMeasurementInput {
        execution_id: id,
        step_number: 1,
        measurement_mm: 9.6,
    }).unwrap();

    register.execute(RegisterFinishingMeasurementInput {
        execution_id: id,
        step_number: 2,
        measurement_mm: 8.9,
    }).unwrap();

    let before = repo.get(id).unwrap();
    let step1_before = before.steps()[0].measurement();

    register.execute(RegisterFinishingMeasurementInput {
        execution_id: id,
        step_number: 2,
        measurement_mm: 8.7,
    }).unwrap();

    let after = repo.get(id).unwrap();

    assert_eq!(after.steps()[0].measurement(), step1_before);
}


#[test]
fn edit_preserves_target_diameter() {
    let repo = repo();
    let register = RegisterFinishingMeasurementUseCase::new(repo.clone());

    let id = create_execution(repo.clone(), 3);

    register.execute(RegisterFinishingMeasurementInput {
        execution_id: id,
        step_number: 1,
        measurement_mm: 9.5,
    }).unwrap();

    let execution = repo.get(id).unwrap();

    let last_step = execution.steps().last().unwrap();

    let target = execution.plan().target().mm_value();
    let last_end = last_step.planned_end().mm_value();

    assert!((last_end - target).abs() < 1e-9);
}