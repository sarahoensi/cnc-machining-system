// tests/application/finishing/generate/validation.rs

use cnc_machining_system_lib::{
    application::finishing::{
        dto::GenerateFinishingPlanInput,
        GenerateFinishingPlanUseCase,
    },
    domain::machining::finishing::FinishingMode,
};

#[test]
fn generate_fails_when_cuts_is_zero() {

    let generate = GenerateFinishingPlanUseCase::new();

    let result = generate.execute(
        GenerateFinishingPlanInput::ByCuts {
            mode: FinishingMode::Outer,
            start_diameter_mm: 10.0,
            target_diameter_mm: 8.0,
            cuts: 0,
        }
    );

    assert!(result.is_err());
}