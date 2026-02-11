// tests/application/finishing/generate/happy_path.rs

use cnc_machining_system_lib::{
    application::finishing::{
        generate_finishing_plan_input::GenerateFinishingPlanInput,
        use_cases::GenerateFinishingPlanUseCase,
    },
    domain::FinishingMode,
};

use super::super::fixtures::*;

#[test]
fn generate_creates_execution_with_expected_steps() {
    let repo = repo();
    let generate = GenerateFinishingPlanUseCase::new(repo.clone());

    let result = generate
        .execute(GenerateFinishingPlanInput::ByCuts {
            mode: FinishingMode::Outer,
            start_diameter_mm: 10.0,
            target_diameter_mm: 8.0,
            cuts: 3,
        })
        .unwrap();

    assert_eq!(result.steps.len(), 3);
    assert!(!result.execution_id.is_empty());
}

#[test]
fn generate_creates_unique_execution_ids() {
    let repo = repo();
    let generate = GenerateFinishingPlanUseCase::new(repo.clone());

    let a = generate
        .execute(GenerateFinishingPlanInput::ByCuts {
            mode: FinishingMode::Outer,
            start_diameter_mm: 10.0,
            target_diameter_mm: 8.0,
            cuts: 2,
        })
        .unwrap();

    let b = generate
        .execute(GenerateFinishingPlanInput::ByCuts {
            mode: FinishingMode::Outer,
            start_diameter_mm: 10.0,
            target_diameter_mm: 8.0,
            cuts: 2,
        })
        .unwrap();

    assert_ne!(a.execution_id, b.execution_id);
}
