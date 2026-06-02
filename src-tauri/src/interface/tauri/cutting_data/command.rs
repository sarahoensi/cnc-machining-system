//! Tauri command endpoint for cutting-data solving.
//!
//! This module exposes a frontend-callable command that translates UI payloads
//! into application input and returns a serialized response DTO.

// interface/tauri/cutting_data/command.rs

use tauri::command;

use crate::interface::tauri::error::{map_application_error, TauriError};

use crate::application::{SolveCuttingDataInput, SolveCuttingDataUseCase};

use super::{SolveCuttingDataRequest, SolveCuttingDataResponse};

/// Solves and completes cutting data for a machining setup request.
///
/// Purpose:
/// - Exposes the cutting-data calculation workflow to the frontend.
///
/// Expected input:
/// - A [`SolveCuttingDataRequest`] containing optional known machining values
///   such as cutting speed, rpm, chip load, feed rate, tooth count, and diameter.
///
/// Output meaning:
/// - Returns [`SolveCuttingDataResponse`] with validated and derived fields.
/// - Values remain optional when the provided data cannot derive them safely.
///
/// Use case triggered:
/// - Calls [`SolveCuttingDataUseCase::execute`].
///
/// Frontend error scenarios:
/// - Returns `Err(String)` when unit/domain validation fails.
/// - Returns `Err(String)` when an application-level calculation path is invalid.
///
/// Workflow assumptions:
/// - The command is stateless; each request is solved independently.
#[command]
pub fn solve_cutting_data(
    request: SolveCuttingDataRequest,
) -> Result<SolveCuttingDataResponse, TauriError> {
    let input: SolveCuttingDataInput = request.into();

    let output = SolveCuttingDataUseCase::execute(input).map_err(map_application_error)?;

    Ok(output.into())
}
