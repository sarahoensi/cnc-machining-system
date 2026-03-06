// tests/application/finishing/register/validation.rs

use cnc_machining_system_lib::{
    application::finishing::{
        dto::RegisterFinishingMeasurementInput,
        RegisterFinishingMeasurementUseCase,
    },
};

use super::super::fixtures::*;

#[test]
fn register_fails_when_step_number_is_zero() {
    let repo = repo();
    let register = RegisterFinishingMeasurementUseCase::new(repo.clone());

    let id = create_execution(repo.clone(), 3);

    let result = register.execute(RegisterFinishingMeasurementInput {
        execution_id: id,
        step_number: 0,
        measurement_mm: 9.5,
    });

    assert!(result.is_err());
}

#[test]
fn register_fails_when_step_number_out_of_range() {
    let repo = repo();
    let register = RegisterFinishingMeasurementUseCase::new(repo.clone());

    let id = create_execution(repo.clone(), 2);

    let result = register.execute(RegisterFinishingMeasurementInput {
        execution_id: id,
        step_number: 99,
        measurement_mm: 9.5,
    });

    assert!(result.is_err());
}

#[test]
fn register_fails_when_measurement_passes_target() {
    let repo = repo();
    let register = RegisterFinishingMeasurementUseCase::new(repo.clone());

    let id = create_execution(repo.clone(), 2);

    // Outer finishing → cannot go below target
    let result = register.execute(RegisterFinishingMeasurementInput {
        execution_id: id,
        step_number: 1,
        measurement_mm: 7.5,
    });

    assert!(result.is_err());
}