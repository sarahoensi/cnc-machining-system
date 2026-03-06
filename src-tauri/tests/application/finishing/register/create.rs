// tests/application/finishing/register/create.rs

use cnc_machining_system_lib::{
    application::finishing::{
        dto::RegisterFinishingMeasurementInput,
        RegisterFinishingMeasurementUseCase,
    },
};

use crate::application::finishing::fixtures::{create_execution, repo};


#[test]
fn register_updates_only_selected_step() {
    let repo = repo();
    let register = RegisterFinishingMeasurementUseCase::new(repo.clone());

    let id = create_execution(repo.clone(), 3);

    register.execute(
        RegisterFinishingMeasurementInput {
            execution_id: id,
            step_number: 2,
            measurement_mm: 8.7,
        }
    ).unwrap();

    let execution = repo.get(id).unwrap();

    assert!(execution.steps()[1].measurement().is_some());
    assert!(execution.steps()[0].measurement().is_none());
    assert!(execution.steps()[2].measurement().is_none());
}


#[test]
fn register_creates_measurement() {
    let repo = repo();
    let register = RegisterFinishingMeasurementUseCase::new(repo.clone());

    let id = create_execution(repo.clone(), 2);

    register.execute(
        RegisterFinishingMeasurementInput {
            execution_id: id,
            step_number: 1,
            measurement_mm: 9.5,
        }
    ).unwrap();

    let execution = repo.get(id).unwrap();

    assert_eq!(
        execution.steps()[0].measurement().map(|d| d.mm_value()),
        Some(9.5)
    );
}