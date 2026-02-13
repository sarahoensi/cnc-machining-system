//! Tauri command endpoints for finishing planning and measurement updates.
//!
//! This module is the integration boundary between frontend finishing screens
//! and application-layer finishing use cases.

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

/// Generates a finishing execution plan for a machining operation.
///
/// Purpose:
/// - Exposes finishing-plan generation to the frontend.
///
/// Expected input:
/// - A [`GenerateFinishingPlanRequest`] with planning mode and diameter targets.
///
/// Output meaning:
/// - Returns [`FinishingExecutionResponse`] containing a new `execution_id` and
///   ordered planned steps for operator follow-up.
///
/// Use case triggered:
/// - Calls [`GenerateFinishingPlanUseCase::execute`].
///
/// Frontend error scenarios:
/// - Returns `Err(String)` for invalid input values rejected by domain rules.
/// - Returns `Err(String)` for planning failures or repository persistence failures.
///
/// Workflow assumptions:
/// - The returned `execution_id` should be used in subsequent
///   `register_finishing_measurement` calls.
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


/// Registers measured diameter feedback for one finishing step.
///
/// Purpose:
/// - Exposes step-by-step finishing lifecycle updates to the frontend.
///
/// Expected input:
/// - A [`RegisterFinishingMeasurementRequest`] with:
///   - `execution_id` from a prior generated plan
///   - `step_number` to update
///   - measured diameter in millimeters
///
/// Output meaning:
/// - Returns updated [`FinishingExecutionResponse`] including registered
///   measurement data.
///
/// Use case triggered:
/// - Calls [`RegisterFinishingMeasurementUseCase::execute`].
///
/// Frontend error scenarios:
/// - Returns `Err(String)` when `execution_id` is not a valid UUID string.
/// - Returns `Err(String)` for unknown execution IDs, invalid measurements,
///   invalid workflow transitions, or repository errors.
///
/// Workflow assumptions:
/// - This command is intended to run after `generate_finishing_plan`.
/// - Step updates are expected to follow the execution lifecycle constraints.
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

/// Maps application-layer errors into frontend-safe string responses.
///
/// Errors returned by this mapper are intended for UI display/logging and are
/// expected outcomes for invalid requests or constrained workflow states.
fn map_error(err: ApplicationError) -> String {
    err.to_string()
}
