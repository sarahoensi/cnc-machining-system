// interface/tauri/finishing/command.rs
use tauri::command;

use std::sync::{Arc, OnceLock};
use uuid::Uuid;

use crate::application::ApplicationError;

use crate::application::finishing::use_cases::{
    generate_finishing_plan_use_case::GenerateFinishingPlanUseCase,
    register_finishing_measurement_use_case::RegisterFinishingMeasurementUseCase,
};

use crate::domain::{
    FinishingExecutionId,
    FinishingExecutionRepository,
};

use crate::infrastructure::finishing::InMemoryFinishingExecutionRepository;

use super::request::{
    GenerateFinishingPlanRequest,
    RegisterFinishingMeasurementRequest,
    };

use super::response::FinishingExecutionResponse;


// ----------------------------------------------------
// Global repository
// ----------------------------------------------------

static REPO: OnceLock<Arc<dyn FinishingExecutionRepository>> = OnceLock::new();

fn repo() -> Arc<dyn FinishingExecutionRepository> {
    REPO
        .get_or_init(|| Arc::new(InMemoryFinishingExecutionRepository::new()))
        .clone()
}


// ----------------------------------------------------
// Commands
// ----------------------------------------------------

#[command]
pub fn generate_finishing_plan(
    request: GenerateFinishingPlanRequest,
) -> Result<FinishingExecutionResponse, String> {

    let uc = GenerateFinishingPlanUseCase::new(repo());

    let result = uc
        .execute(request.into())
        .map_err(map_error)?;

    Ok(result.into())
}


#[command]
pub fn register_finishing_measurement(
    request: RegisterFinishingMeasurementRequest,
) -> Result<FinishingExecutionResponse, String> {

    let uc = RegisterFinishingMeasurementUseCase::new(repo());

    let id = FinishingExecutionId::from_uuid(
        Uuid::parse_str(&request.execution_id)
            .map_err(|_| "Invalid execution_id")?
    );

    let result = uc
        .execute(id, request.step_number, request.measurement_mm)
        .map_err(map_error)?;

    Ok(result.into())
}



// ----------------------------------------------------
// Error mapping
// ----------------------------------------------------

fn map_error(err: ApplicationError) -> String {
    err.to_string()
}
