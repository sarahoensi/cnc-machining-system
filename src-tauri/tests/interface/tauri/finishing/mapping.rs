// tests/integration/finishing/mapping.rs

use std::sync::Arc;

use cnc_machining_system_lib::application::finishing::generate_finishing_plan_input::GenerateFinishingPlanInput;
use cnc_machining_system_lib::application::finishing::use_cases::generate_finishing_plan_use_case::GenerateFinishingPlanUseCase;


use cnc_machining_system_lib::domain::{
    FinishingMode,
    FinishingExecutionRepository,
};

use cnc_machining_system_lib::infrastructure::finishing::InMemoryFinishingExecutionRepository;


#[test]
fn generated_response_contains_expected_fields() {

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

    let step = &response.steps[0];

    assert!(step.start_mm > 0.0);
    assert!(step.planned_end_mm > 0.0);
}