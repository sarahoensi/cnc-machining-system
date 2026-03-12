// tests/integration/finishing/mapping.rs

use cnc_machining_system_lib::{
    application::finishing::{
        dto::{GenerateFinishingPlanInput, FinishingExecutionOutput},
        GenerateFinishingPlanUseCase,
    },
    domain::machining::finishing::FinishingMode,
};

#[test]
fn generated_output_contains_expected_fields() {

    let uc = GenerateFinishingPlanUseCase::new();

    let execution = uc
        .execute(GenerateFinishingPlanInput::ByCuts {
            mode: FinishingMode::Outer,
            start_diameter_mm: 10.0,
            target_diameter_mm: 8.0,
            cuts: 3,
        })
        .unwrap();

    let output: FinishingExecutionOutput = (&execution).into();

    let step = &output.steps[0];

    assert!(step.start_mm > 0.0);
    assert!(step.planned_end_mm > 0.0);
}