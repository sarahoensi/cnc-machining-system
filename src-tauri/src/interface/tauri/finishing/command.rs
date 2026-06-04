use tauri::{command, State};

use crate::AppState;

use crate::application::finishing::{
    FinishingExecutionOutput, GenerateFinishingPlanUseCase, RegisterFinishingMeasurementInput,
    RegisterFinishingMeasurementUseCase,
};

use crate::interface::finishing::{
    FinishingExecutionResponse, GenerateFinishingPlanRequest, RegisterFinishingMeasurementRequest,
};

use crate::interface::tauri::error::{map_application_error, TauriError};

//
// Generate plan
//

#[command]
pub fn generate_finishing_plan(
    state: State<AppState>,
    request: GenerateFinishingPlanRequest,
) -> Result<FinishingExecutionResponse, TauriError> {
    let uc = GenerateFinishingPlanUseCase::new();

    let execution = uc.execute(request.into()).map_err(map_application_error)?;

    let mut guard = state
        .finishing_execution
        .lock()
        .map_err(|_| TauriError::message("Finishing state is unavailable"))?;
    *guard = Some(execution.clone());

    let output: FinishingExecutionOutput = (&execution).into();

    Ok(output.into())
}

//
// Register measurement
//

#[command]
pub fn register_finishing_measurement(
    state: State<AppState>,
    request: RegisterFinishingMeasurementRequest,
) -> Result<FinishingExecutionResponse, TauriError> {
    let uc = RegisterFinishingMeasurementUseCase::new();

    let mut guard = state
        .finishing_execution
        .lock()
        .map_err(|_| TauriError::message("Finishing state is unavailable"))?;

    let execution = guard.as_mut().ok_or_else(|| TauriError {
        message: "No active finishing execution".to_string(),
        field_errors: None,
    })?;

    let input = RegisterFinishingMeasurementInput {
        step_number: request.step_number,
        measurement_mm: request.measurement_mm,
    };

    let result = uc
        .execute(execution, input)
        .map_err(map_application_error)?;

    Ok(result.into())
}
