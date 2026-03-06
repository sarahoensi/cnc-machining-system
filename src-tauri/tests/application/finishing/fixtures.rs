// tests/application/finishing/fixtures.rs

use std::sync::Arc;

use cnc_machining_system_lib::{
    application::finishing::{
        dto::GenerateFinishingPlanInput,
        GenerateFinishingPlanUseCase,
    },
    domain::{
        FinishingExecutionId,
        FinishingExecutionRepository,
        FinishingMode,
    },
    infrastructure::finishing::InMemoryFinishingExecutionRepository,
};

pub fn repo() -> Arc<dyn FinishingExecutionRepository> {
    Arc::new(InMemoryFinishingExecutionRepository::new())
}

pub fn parse_id(s: &str) -> FinishingExecutionId {
    FinishingExecutionId::from_uuid(uuid::Uuid::parse_str(s).unwrap())
}

pub fn create_execution(
    repo: Arc<dyn FinishingExecutionRepository>,
    cuts: u32,
) -> FinishingExecutionId {

    let generate = GenerateFinishingPlanUseCase::new(repo.clone());

    let generated = generate
        .execute(GenerateFinishingPlanInput::ByCuts {
            mode: FinishingMode::Outer,
            start_diameter_mm: 10.0,
            target_diameter_mm: 8.0,
            cuts,
        })
        .unwrap();

    parse_id(&generated.execution_id)
}