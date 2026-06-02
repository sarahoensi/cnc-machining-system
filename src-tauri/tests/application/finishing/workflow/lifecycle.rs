// tests/application/finishing/workflow/lifecycle.rs

use cnc_machining_system_lib::{
    application::finishing::{
        dto::{GenerateFinishingPlanInput, RegisterFinishingMeasurementInput},
        GenerateFinishingPlanUseCase, RegisterFinishingMeasurementUseCase,
    },
    domain::machining::finishing::FinishingMode,
};

#[test]
fn full_finishing_workflow() {
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

    // --- Step 1 measurement ---
    register
        .execute(
            &mut execution,
            RegisterFinishingMeasurementInput {
                step_number: 1,
                measurement_mm: 9.6,
            },
        )
        .unwrap();

    assert_eq!(
        execution.steps()[0].measurement().map(|d| d.mm_value()),
        Some(9.6)
    );

    // --- Step 2 measurement ---
    register
        .execute(
            &mut execution,
            RegisterFinishingMeasurementInput {
                step_number: 2,
                measurement_mm: 8.9,
            },
        )
        .unwrap();

    assert_eq!(
        execution.steps()[1].measurement().map(|d| d.mm_value()),
        Some(8.9)
    );

    // --- Edit step 2 ---
    register
        .execute(
            &mut execution,
            RegisterFinishingMeasurementInput {
                step_number: 2,
                measurement_mm: 8.7,
            },
        )
        .unwrap();

    // ✔ measurement updated
    assert_eq!(
        execution.steps()[1].measurement().map(|d| d.mm_value()),
        Some(8.7)
    );

    // ✔ step 1 unchanged
    assert_eq!(
        execution.steps()[0].measurement().map(|d| d.mm_value()),
        Some(9.6)
    );

    // ✔ target still reached
    let last_step = execution.steps().last().unwrap();
    let target = execution.plan().target().mm_value();

    assert!((last_step.planned_end().mm_value() - target).abs() < 1e-9);
}
