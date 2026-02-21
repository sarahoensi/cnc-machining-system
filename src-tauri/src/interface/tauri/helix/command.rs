//! Tauri command endpoint for helix-solving integration.
//!
//! This module exposes a frontend-facing command that converts helix request
//! DTOs into application inputs and returns serialized helix outputs.

// interface/tauri/helix/command.rs

use tauri::command;

use crate::{application::SolveHelixUseCase, interface::tauri::error::{TauriError, map_application_error}};
//use crate::application::ApplicationError;

use super::{
    SolveHelixRequest,
    SolveHelixResponse,
};

/// Solves helix geometry used in machining setup workflows.
///
/// Purpose:
/// - Exposes helix solving (pitch-path or angle-path) to the frontend.
///
/// Expected input:
/// - A [`SolveHelixRequest`] containing mode, nominal diameter, tool diameter,
///   and either pitch or angle.
///
/// Output meaning:
/// - Returns [`SolveHelixResponse`] with effective diameter, pitch, angle, and
///   circumference values for UI display.
///
/// Use case triggered:
/// - Calls [`SolveHelixUseCase::execute`].
///
/// Frontend error scenarios:
/// - Returns `Err(String)` for invalid units/domain constraints.
/// - Returns `Err(String)` when helix construction rules reject the request.
///
/// Workflow assumptions:
/// - The command is stateless and can be called independently.
#[command]
pub fn solve_helix(
    request: SolveHelixRequest,
) -> Result<SolveHelixResponse, TauriError> {

    let use_case = SolveHelixUseCase;
    let input = request.into();

    use_case
        .execute(input)
        .map(Into::into)
        .map_err(map_application_error)
}
