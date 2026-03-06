// tests/application/finishing/register/locking.rs

use cnc_machining_system_lib::{
    application::finishing::{
        dto::RegisterFinishingMeasurementInput,
        RegisterFinishingMeasurementUseCase,
    },
};

use crate::application::finishing::fixtures::{create_execution, repo};

#[test]
fn edit_previous_step_after_later_measurement_fails() {
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

    let result = register.execute(RegisterFinishingMeasurementInput {
        execution_id: id,
        step_number: 1,
        measurement_mm: 9.5,
    });

    assert!(result.is_err());
}

#[test]
fn edit_last_measured_step_is_allowed() {
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

    let result = register.execute(RegisterFinishingMeasurementInput {
        execution_id: id,
        step_number: 2,
        measurement_mm: 8.7,
    });

    assert!(result.is_ok());
}