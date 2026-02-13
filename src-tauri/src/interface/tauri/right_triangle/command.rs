//! Tauri command endpoint for right-triangle solving.
//!
//! This module defines the frontend integration boundary for right-triangle
//! geometry requests and standardized error mapping.

// interface/tauri/right_triangle/command.rs

use tauri::command;
use serde::Serialize;

use crate::application::SolveRightTriangleUseCase;
use crate::application::ApplicationError;

use super::{
    SolveRightTriangleRequest,
    SolveRightTriangleResponse,
};

/// Frontend-safe error payload returned by right-triangle commands.
///
/// Frontend representation:
/// - Serialized object with a user-facing `message` field.
///
/// Message safety:
/// - Message content is produced from application errors intended for UI
///   handling and display/logging.
#[derive(Debug, Serialize)]
pub struct TauriError {
    message: String,
}

// ---------------------------------------------------------
// Command
// ---------------------------------------------------------

/// Solves right-triangle geometry for machining setup.
///
/// Purpose:
/// - Exposes triangle solving paths (legs, leg+hypotenuse, etc.) to the UI.
///
/// Expected input:
/// - A [`SolveRightTriangleRequest`] variant containing the known geometric
///   values in millimeters/degrees.
///
/// Output meaning:
/// - Returns [`SolveRightTriangleResponse`] with all solved sides and angles.
///
/// Use case triggered:
/// - Calls [`SolveRightTriangleUseCase::execute`].
///
/// Frontend error scenarios:
/// - Returns `Err(TauriError)` for invalid units or impossible geometry.
/// - Errors represent expected validation outcomes for incorrect input.
///
/// Workflow assumptions:
/// - The command is stateless and suitable for repeated interactive solving.
#[command]
pub fn solve_right_triangle(
    request: SolveRightTriangleRequest,
) -> Result<SolveRightTriangleResponse, TauriError> {

    let use_case = SolveRightTriangleUseCase;

    let input = request.into();

    let result = use_case
        .execute(input)
        .map_err(map_application_error)?;

    Ok(result.into())
}


// ---------------------------------------------------------
// Error Mapping
// ---------------------------------------------------------

/// Maps application errors into the Tauri-facing error shape used by frontend
/// consumers.
fn map_application_error(err: ApplicationError) -> TauriError {
    TauriError {
        message: err.to_string(),
    }
}
