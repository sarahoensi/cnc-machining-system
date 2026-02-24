// tests/integration/finishing/workflow.rs

use std::sync::Arc;
use cnc_machining_system_lib::application::finishing::generate_finishing_plan_input::GenerateFinishingPlanInput;
use uuid::Uuid;

use cnc_machining_system_lib::application::finishing::use_cases::{
    generate_finishing_plan_use_case::GenerateFinishingPlanUseCase,
    register_finishing_measurement_use_case::RegisterFinishingMeasurementUseCase,
};


use cnc_machining_system_lib::domain::{
    FinishingExecutionRepository,
    FinishingMode,
    FinishingExecutionId,
};

use cnc_machining_system_lib::infrastructure::finishing::InMemoryFinishingExecutionRepository;


#[test]
fn finishing_full_workflow_via_use_cases() {

    // Shared in-memory repository
    let repo: Arc<dyn FinishingExecutionRepository> =
        Arc::new(InMemoryFinishingExecutionRepository::new());

    let generate_uc = GenerateFinishingPlanUseCase::new(repo.clone());
    let register_uc = RegisterFinishingMeasurementUseCase::new(repo.clone());

    // -------------------------------------------------
    // Step 1: Generate plan
    // -------------------------------------------------

    let generated = generate_uc
        .execute(GenerateFinishingPlanInput::ByCuts {
            mode: FinishingMode::Outer,
            start_diameter_mm: 10.0,
            target_diameter_mm: 8.0,
            cuts: 3,
        })
        .unwrap();

    let uuid = Uuid::parse_str(&generated.execution_id).unwrap();
    let id = FinishingExecutionId::from_uuid(uuid);

    // -------------------------------------------------
    // Step 2: Register first measurement
    // -------------------------------------------------

    let after_step1 = register_uc
        .execute(id, 1, 9.6)
        .unwrap();

    assert_eq!(after_step1.steps[0].measurement_mm, Some(9.6));

    // -------------------------------------------------
    // Step 3: Register second measurement
    // -------------------------------------------------

    let after_step2 = register_uc
        .execute(id, 2, 8.9)
        .unwrap();

    assert_eq!(after_step2.steps[1].measurement_mm, Some(8.9));
}