// tests/integration/finishing/generate.rs

use std::sync::Arc;

use cnc_machining_system_lib::application::finishing::generate_finishing_plan_input::GenerateFinishingPlanInput;
use cnc_machining_system_lib::application::finishing::use_cases::generate_finishing_plan_use_case::GenerateFinishingPlanUseCase;


use cnc_machining_system_lib::domain::{
    FinishingMode,
    FinishingExecutionRepository,
};

use cnc_machining_system_lib::infrastructure::finishing::InMemoryFinishingExecutionRepository;


#[test]
fn generates_plan_via_use_case() {

    let repo: Arc<dyn FinishingExecutionRepository> =
        Arc::new(InMemoryFinishingExecutionRepository::new());

    let uc = GenerateFinishingPlanUseCase::new(repo);

    let input = GenerateFinishingPlanInput::ByCuts {
        mode: FinishingMode::Outer,
        start_diameter_mm: 10.0,
        target_diameter_mm: 8.0,
        cuts: 3,
    };

    let response = uc.execute(input).unwrap();

    assert_eq!(response.steps.len(), 3);
    assert!(!response.execution_id.is_empty());
}