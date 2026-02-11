// tests/application/finishing/mod.rs

use cnc_machining_system_lib::application::finishing::use_cases::RegisterFinishingMeasurementUseCase;

use super::fixtures::*;

#[test]
fn register_fails_when_step_out_of_range() {
    let repo = repo();
    let register = RegisterFinishingMeasurementUseCase::new(repo.clone());

    let id = create_execution(repo.clone(), 1);

    let result = register.execute(id, 99, 8.0);

    assert!(result.is_err());
}

#[test]
fn register_fails_when_execution_not_found() {
    let repo = repo();
    let register = RegisterFinishingMeasurementUseCase::new(repo.clone());

    let random_id = cnc_machining_system_lib::domain::FinishingExecutionId::new();

    let result = register.execute(random_id, 1, 8.0);

    assert!(result.is_err());
}
