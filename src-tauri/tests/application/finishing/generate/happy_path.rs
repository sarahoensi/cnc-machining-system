// tests/application/finishing/generate/happy_path.rs

use cnc_machining_system_lib::{
    application::finishing::{
        dto::GenerateFinishingPlanInput,
        GenerateFinishingPlanUseCase,
    },
    domain::machining::finishing::FinishingMode,
};

#[test]
fn generate_creates_execution_with_expected_steps() {

    let generate = GenerateFinishingPlanUseCase::new();

    let execution = generate
        .execute(GenerateFinishingPlanInput::ByCuts {
            mode: FinishingMode::Outer,
            start_diameter_mm: 10.0,
            target_diameter_mm: 8.0,
            cuts: 3,
        })
        .unwrap();

    assert_eq!(execution.steps().len(), 3);
    assert!(!execution.finished());
}