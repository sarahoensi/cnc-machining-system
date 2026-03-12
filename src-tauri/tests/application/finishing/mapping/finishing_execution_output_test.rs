// tests/application/finishing/mapping/finishing_execution_output_tests.rs

use cnc_machining_system_lib::{
    application::finishing::{
        dto::FinishingExecutionOutput,
        dto::GenerateFinishingPlanInput,
        GenerateFinishingPlanUseCase,
        RegisterFinishingMeasurementUseCase,
        RegisterFinishingMeasurementInput,
    },
    domain::{
        machining::finishing::FinishingMode
    },
};

//
// ---------------------------------------------------------
// Measurement mapping
// ---------------------------------------------------------
//

#[test]
fn mapping_preserves_measurement_values() {

    let generate = GenerateFinishingPlanUseCase::new();
    let mut execution = generate
        .execute(GenerateFinishingPlanInput::ByCuts {
            mode: FinishingMode::Outer,
            start_diameter_mm: 10.0,
            target_diameter_mm: 8.0,
            cuts: 2,
        })
        .unwrap();

    let register = RegisterFinishingMeasurementUseCase::new();

    register.execute(
        &mut execution,
        RegisterFinishingMeasurementInput {
            step_number: 1,
            measurement_mm: 9.5,
        },
    ).unwrap();

    let output: FinishingExecutionOutput = (&execution).into();

    assert_eq!(output.steps[0].measurement_mm, Some(9.5));
}

//
// ---------------------------------------------------------
// None measurement mapping
// ---------------------------------------------------------
//

#[test]
fn mapping_preserves_none_measurements() {

    let generate = GenerateFinishingPlanUseCase::new();

    let execution = generate
        .execute(GenerateFinishingPlanInput::ByCuts {
            mode: FinishingMode::Outer,
            start_diameter_mm: 10.0,
            target_diameter_mm: 8.0,
            cuts: 2,
        })
        .unwrap();

    let output: FinishingExecutionOutput = (&execution).into();

    assert!(output.steps[0].measurement_mm.is_none());
}

//
// ---------------------------------------------------------
// Step ordering
// ---------------------------------------------------------
//

#[test]
fn mapping_preserves_step_ordering() {

    let generate = GenerateFinishingPlanUseCase::new();

    let execution = generate
        .execute(GenerateFinishingPlanInput::ByCuts {
            mode: FinishingMode::Outer,
            start_diameter_mm: 10.0,
            target_diameter_mm: 8.0,
            cuts: 3,
        })
        .unwrap();

    let output: FinishingExecutionOutput = (&execution).into();

    for (domain_step, output_step) in execution.steps().iter().zip(output.steps.iter()) {
        assert_eq!(domain_step.index(), output_step.index);
    }
}

//
// ---------------------------------------------------------
// Active step mapping
// ---------------------------------------------------------
//

#[test]
fn mapping_preserves_active_step() {

    let generate = GenerateFinishingPlanUseCase::new();

    let execution = generate
        .execute(GenerateFinishingPlanInput::ByCuts {
            mode: FinishingMode::Outer,
            start_diameter_mm: 10.0,
            target_diameter_mm: 8.0,
            cuts: 3,
        })
        .unwrap();

    let output: FinishingExecutionOutput = (&execution).into();

    assert_eq!(output.active_step, Some(1));
}