// tests/application/finishing/generate/validation.rs

use cnc_machining_system_lib::application::finishing::{
    generate_finishing_plan_input::GenerateFinishingPlanInput,
    use_cases::GenerateFinishingPlanUseCase,
};

use super::super::fixtures::*;

#[test]
fn generate_fails_when_cuts_is_zero() {
    let repo = repo();
    let generate = GenerateFinishingPlanUseCase::new(repo);

    let result = generate.execute(
        GenerateFinishingPlanInput::ByCuts {
            mode: cnc_machining_system_lib::domain::FinishingMode::Outer,
            start_diameter_mm: 10.0,
            target_diameter_mm: 8.0,
            cuts: 0,
        }
    );

    assert!(result.is_err());
}
