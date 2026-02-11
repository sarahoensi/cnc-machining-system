// tests/application/finishing/finishing_use_case_tests.rs

use std::sync::Arc;

use cnc_machining_system_lib::{
    application::finishing::{
        generate_finishing_plan_input::GenerateFinishingPlanInput,
        use_cases::{
            ClearFinishingMeasurementUseCase, GenerateFinishingPlanUseCase,
            RegisterFinishingMeasurementUseCase,
        },
    },
    domain::{FinishingExecutionId, FinishingExecutionRepository, FinishingMode},
    infrastructure::finishing::InMemoryFinishingExecutionRepository,
};

fn repo() -> Arc<dyn FinishingExecutionRepository> {
    Arc::new(InMemoryFinishingExecutionRepository::new())
}

fn parse_id(s: &str) -> FinishingExecutionId {
    FinishingExecutionId::from_uuid(uuid::Uuid::parse_str(s).unwrap())
}

#[test]
fn generate_finishing_plan_creates_execution() {
    let repo = repo();
    let uc = GenerateFinishingPlanUseCase::new(repo.clone());

    let input = GenerateFinishingPlanInput::ByCuts {
        mode: FinishingMode::Outer,
        start_diameter_mm: 10.0,
        target_diameter_mm: 8.0,
        cuts: 2,
    };

    let result = uc.execute(input).unwrap();

    assert_eq!(result.steps.len(), 2);
    assert!(!result.execution_id.is_empty());
}

#[test]
fn register_measurement_updates_execution() {
    let repo = repo();

    let generate = GenerateFinishingPlanUseCase::new(repo.clone());
    let register = RegisterFinishingMeasurementUseCase::new(repo.clone());

    let generated = generate
        .execute(GenerateFinishingPlanInput::ByCuts {
            mode: FinishingMode::Outer,
            start_diameter_mm: 10.0,
            target_diameter_mm: 8.0,
            cuts: 2,
        })
        .unwrap();

    let id = parse_id(&generated.execution_id);

    let updated = register.execute(id, 1, 9.5).unwrap();

    assert!(updated.steps[0].measurement_mm.is_some());
}

#[test]
fn clear_measurement_resets_state() {
    let repo = repo();

    let generate = GenerateFinishingPlanUseCase::new(repo.clone());
    let register = RegisterFinishingMeasurementUseCase::new(repo.clone());
    let clear = ClearFinishingMeasurementUseCase::new(repo.clone());

    let generated = generate
        .execute(GenerateFinishingPlanInput::ByCuts {
            mode: FinishingMode::Outer,
            start_diameter_mm: 10.0,
            target_diameter_mm: 8.0,
            cuts: 2,
        })
        .unwrap();

    let id = parse_id(&generated.execution_id);

    register.execute(id, 1, 9.5).unwrap();

    let cleared = clear.execute(id, 1).unwrap();

    assert!(cleared.steps[0].measurement_mm.is_none());
}

#[test]
fn finishing_full_workflow() {
    let repo = repo();

    let generate = GenerateFinishingPlanUseCase::new(repo.clone());
    let register = RegisterFinishingMeasurementUseCase::new(repo.clone());
    let clear = ClearFinishingMeasurementUseCase::new(repo.clone());

    let generated = generate
        .execute(GenerateFinishingPlanInput::ByCuts {
            mode: FinishingMode::Outer,
            start_diameter_mm: 10.0,
            target_diameter_mm: 8.0,
            cuts: 3,
        })
        .unwrap();

    let id = parse_id(&generated.execution_id);

    register.execute(id, 1, 9.6).unwrap();
    register.execute(id, 2, 8.9).unwrap();

    let cleared = clear.execute(id, 2).unwrap();

    assert!(cleared.steps[1].measurement_mm.is_none());
}
