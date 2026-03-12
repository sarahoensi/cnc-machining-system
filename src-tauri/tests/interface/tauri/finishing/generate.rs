// tests/integration/finishing/generate.rs

use cnc_machining_system_lib::{
    application::finishing::{
        dto::GenerateFinishingPlanInput,
        GenerateFinishingPlanUseCase,
    },
    domain::machining::finishing::FinishingMode,
};

#[test]
fn generates_plan_via_use_case() {

    let uc = GenerateFinishingPlanUseCase::new();

    let input = GenerateFinishingPlanInput::ByCuts {
        mode: FinishingMode::Outer,
        start_diameter_mm: 10.0,
        target_diameter_mm: 8.0,
        cuts: 3,
    };

    let execution = uc.execute(input).unwrap();

    assert_eq!(execution.steps().len(), 3);
    assert_eq!(execution.active_step(), Some(1));
    assert!(!execution.finished());
}