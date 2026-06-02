//! Tauri command endpoint for helix-solving integration.
//!
//! This module exposes a frontend-facing command that converts helix request
//! DTOs into application inputs and returns serialized helix outputs.

// interface/tauri/helix/command.rs

use tauri::command;

use crate::{
    application::SolveHelixUseCase,
    interface::tauri::error::{map_application_error, TauriError},
};

use super::{SolveHelixRequest, SolveHelixResponse};

#[command]
pub fn solve_helix(request: SolveHelixRequest) -> Result<SolveHelixResponse, TauriError> {
    let use_case = SolveHelixUseCase;
    let input = request.into();

    match use_case.execute(input) {
        Ok(result) => Ok(result.into()),

        Err(err) => {
            println!("APPLICATION ERROR: {:?}", err);
            Err(map_application_error(err))
        }
    }
}
