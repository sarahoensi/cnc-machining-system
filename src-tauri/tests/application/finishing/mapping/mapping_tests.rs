// tests/application/finishing/mapping/finishing_execution_output_tests.rs

use cnc_machining_system_lib::{
    application::finishing::finishing_output_mapper::to_output,
    application::finishing::use_cases::RegisterFinishingMeasurementUseCase,
};

use crate::application::finishing::fixtures::*;

//
// ---------------------------------------------------------
// Measurement mapping
// ---------------------------------------------------------
//

#[test]
fn mapping_preserves_measurement_values() {
    let repo = repo();
    let id = create_execution(repo.clone(), 2);

    let register = RegisterFinishingMeasurementUseCase::new(repo.clone());
    register.execute(id, 1, 9.5).unwrap();

    let execution = repo.get(id).unwrap();
    let output = to_output(&execution);

    assert_eq!(output.steps[0].measurement_mm, Some(9.5));
}

//
// ---------------------------------------------------------
// None measurement mapping
// ---------------------------------------------------------
//

#[test]
fn mapping_preserves_none_measurements() {
    let repo = repo();
    let id = create_execution(repo.clone(), 2);

    let execution = repo.get(id).unwrap();
    let output = to_output(&execution);

    assert!(output.steps[0].measurement_mm.is_none());
}

//
// ---------------------------------------------------------
// Step ordering
// ---------------------------------------------------------
//

#[test]
fn mapping_preserves_step_ordering() {
    let repo = repo();
    let id = create_execution(repo.clone(), 3);

    let execution = repo.get(id).unwrap();
    let output = to_output(&execution);

    for (domain_step, output_step) in execution.steps().iter().zip(output.steps.iter()) {
        assert_eq!(domain_step.step_number(), output_step.step_number);
    }
}

//
// ---------------------------------------------------------
// Target preservation
// ---------------------------------------------------------
//

#[test]
fn mapping_preserves_target_diameter() {
    let repo = repo();
    let id = create_execution(repo.clone(), 3);

    let execution = repo.get(id).unwrap();
    let output = to_output(&execution);

    let domain_target = execution.plan().target().mm_value();
    let output_target = output.target_diameter_mm;

    assert!((domain_target - output_target).abs() < 1e-9);
}

//
// ---------------------------------------------------------
// Execution ID mapping
// ---------------------------------------------------------
//

#[test]
fn mapping_preserves_execution_id() {
    let repo = repo();
    let id = create_execution(repo.clone(), 2);

    let execution = repo.get(id).unwrap();
    let output = to_output(&execution);

    assert_eq!(output.execution_id, execution.id().to_string());
}
