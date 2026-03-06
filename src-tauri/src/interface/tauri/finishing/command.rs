use tauri::{command, State};
use uuid::Uuid;

use crate::AppState;
use crate::domain::FinishingExecutionId;

use crate::application::finishing::{
    GenerateFinishingPlanUseCase, RegisterFinishingMeasurementInput, RegisterFinishingMeasurementUseCase
};

use crate::interface::finishing::{
    FinishingExecutionResponse,
    GenerateFinishingPlanRequest,
    RegisterFinishingMeasurementRequest,
};

use crate::interface::tauri::error::{
    TauriError,
    map_application_error,
};

use crate::application::{ApplicationError, ValidationErrors};


// ----------------------------------------------------
// Generate plan
// ----------------------------------------------------

#[command]
pub fn generate_finishing_plan(
    state: State<AppState>,
    request: GenerateFinishingPlanRequest,
) -> Result<FinishingExecutionResponse, TauriError> {

    let uc = GenerateFinishingPlanUseCase::new(
        state.finishing_repo.clone(),
    );

    let result = uc
        .execute(request.into())
        .map_err(map_application_error)?;

    Ok(result.into())
}


// ----------------------------------------------------
// Register measurement
// ----------------------------------------------------

#[command]
pub fn register_finishing_measurement(
    state: State<AppState>,
    request: RegisterFinishingMeasurementRequest,
) -> Result<FinishingExecutionResponse, TauriError> {

    let uc = RegisterFinishingMeasurementUseCase::new(
        state.finishing_repo.clone(),
    );

    // ----------------------------------------------------
    // Validate UUID at interface boundary
    // ----------------------------------------------------

    let uuid = Uuid::parse_str(&request.execution_id)
        .map_err(|_| {
            map_application_error(
                ApplicationError::Validation({
                    let mut v = ValidationErrors::new();
                    v.push("execution_id", "invalid_uuid", "Invalid execution_id");
                    v
                })
            )
        })?;

    let id = FinishingExecutionId::from_uuid(uuid);

    // ----------------------------------------------------
    // Build application input
    // ----------------------------------------------------

    let input = RegisterFinishingMeasurementInput {
        execution_id: id,
        step_number: request.step_number,
        measurement_mm: request.measurement_mm,
    };

    // ----------------------------------------------------
    // Execute use case
    // ----------------------------------------------------

    let result = uc
        .execute(input)
        .map_err(map_application_error)?;

    Ok(result.into())
}