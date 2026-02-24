//! Tauri command endpoint for helix-solving integration.
//!
//! This module exposes a frontend-facing command that converts helix request
//! DTOs into application inputs and returns serialized helix outputs.

// interface/tauri/helix/command.rs

use tauri::command;

use crate::{application::SolveHelixUseCase, interface::tauri::error::{TauriError, map_application_error}};


use super::{
    SolveHelixRequest,
    SolveHelixResponse,
};


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
