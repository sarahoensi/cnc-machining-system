// tests/integration/finishing/validation.rs

use cnc_machining_system_lib::application::finishing::generate_finishing_plan_input::GenerateFinishingPlanInput;
use uuid::Uuid;
use std::sync::Arc;

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
fn fails_when_execution_id_is_invalid() {
    let result = Uuid::parse_str("not-a-uuid");
    assert!(result.is_err());
}


#[test]
fn fails_when_step_is_out_of_range() {

    let repo: Arc<dyn FinishingExecutionRepository> =
        Arc::new(InMemoryFinishingExecutionRepository::new());

    let generate_uc = GenerateFinishingPlanUseCase::new(repo.clone());
    let register_uc = RegisterFinishingMeasurementUseCase::new(repo.clone());

    let generated = generate_uc
        .execute(GenerateFinishingPlanInput::ByCuts {
            mode: FinishingMode::Outer,
            start_diameter_mm: 10.0,
            target_diameter_mm: 8.0,
            cuts: 2,
        })
        .unwrap();

    let uuid = Uuid::parse_str(&generated.execution_id).unwrap();
    let id = FinishingExecutionId::from_uuid(uuid);

    let result = register_uc.execute(id, 99, 9.0);

    assert!(result.is_err());
}