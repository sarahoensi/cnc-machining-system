// interface/tauri/finishing/command.rs

use tauri::command;

use crate::application::finishing::{
    GenerateFinishingPlanUseCase,
    RegisterFinishingMeasurementUseCase,
    ClearFinishingMeasurementUseCase,
};

use crate::application::ApplicationError;

use super::request::{
    GenerateFinishingPlanRequest,
    RegisterFinishingMeasurementRequest,
    ClearFinishingMeasurementRequest,
};

use super::response::FinishingExecutionResponse;


// ----------------------------------------------------
// Commands
// ----------------------------------------------------

#[command]
pub fn generate_finishing_plan(
    request: GenerateFinishingPlanRequest,
) -> Result<FinishingExecutionResponse, String> {

    let uc = GenerateFinishingPlanUseCase;

    let result = uc
        .execute(request.into())
        .map_err(map_error)?;

    Ok(result.into())
}


#[command]
pub fn register_finishing_measurement(
    mut execution: crate::domain::FinishingExecution,
    request: RegisterFinishingMeasurementRequest,
) -> Result<FinishingExecutionResponse, String> {

    let uc = RegisterFinishingMeasurementUseCase;

    let result = uc
        .execute(&mut execution, request.step_number, request.measurement_mm)
        .map_err(map_error)?;

    Ok(result.into())
}


#[command]
pub fn clear_finishing_measurement(
    mut execution: crate::domain::FinishingExecution,
    request: ClearFinishingMeasurementRequest,
) -> Result<FinishingExecutionResponse, String> {

    let uc = ClearFinishingMeasurementUseCase;

    let result = uc
        .execute(&mut execution, request.step_number)
        .map_err(map_error)?;

    Ok(result.into())
}


// ----------------------------------------------------
// Error mapping
// ----------------------------------------------------

fn map_error(err: ApplicationError) -> String {
    err.to_string()
}
