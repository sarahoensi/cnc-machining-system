// interface/tauri/right_triangle/command.rs

//! Tauri command endpoint for right-triangle solving.
//!
//! This module defines the frontend integration boundary for right-triangle
//! geometry requests and standardized error mapping.

use tauri::command;

use crate::interface::tauri::error::{map_application_error, TauriError};

use crate::application::SolveRightTriangleUseCase;

use super::{SolveRightTriangleRequest, SolveRightTriangleResponse};

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

    use_case
        .execute(input)
        .map(Into::into)
        .map_err(map_application_error)
}
