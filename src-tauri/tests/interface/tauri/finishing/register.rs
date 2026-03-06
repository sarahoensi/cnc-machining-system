// tests/integration/finishing/register.rs

use std::sync::Arc;

use cnc_machining_system_lib::application::finishing::dto::{
    GenerateFinishingPlanInput,
    RegisterFinishingMeasurementInput,
};

use cnc_machining_system_lib::application::finishing::{
    GenerateFinishingPlanUseCase,
    RegisterFinishingMeasurementUseCase,
};

use cnc_machining_system_lib::domain::{
    FinishingExecutionRepository,
    FinishingMode,
    FinishingExecutionId,
};

use cnc_machining_system_lib::infrastructure::finishing::InMemoryFinishingExecutionRepository;

use uuid::Uuid;

#[test]
fn registers_measurement_via_use_cases() {

    // ----------------------------------------------------
    // Shared repository
    // ----------------------------------------------------

    let repo: Arc<dyn FinishingExecutionRepository> =
        Arc::new(InMemoryFinishingExecutionRepository::new());

    let generate_uc = GenerateFinishingPlanUseCase::new(repo.clone());
    let register_uc = RegisterFinishingMeasurementUseCase::new(repo.clone());

    // ----------------------------------------------------
    // Step 1: Generate plan
    // ----------------------------------------------------

    let generated = generate_uc
        .execute(GenerateFinishingPlanInput::ByCuts {
            mode: FinishingMode::Outer,
            start_diameter_mm: 10.0,
            target_diameter_mm: 8.0,
            cuts: 3,
        })
        .unwrap();

    // Convert returned id → domain id
    let uuid = Uuid::parse_str(&generated.execution_id)
        .expect("valid uuid");

    let id = FinishingExecutionId::from_uuid(uuid);

    // ----------------------------------------------------
    // Step 2: Register measurement
    // ----------------------------------------------------

    let input = RegisterFinishingMeasurementInput {
        execution_id: id,
        step_number: 1,
        measurement_mm: 9.6,
    };

    let response = register_uc
        .execute(input)
        .unwrap();

    // ----------------------------------------------------
    // Step 3: Assert
    // ----------------------------------------------------

    assert_eq!(response.steps[0].measurement_mm, Some(9.6));
}