// interface/tauri/right_triangle/command.rs

use tauri::command;
use serde::Serialize;

use crate::application::SolveRightTriangleUseCase;
use crate::application::ApplicationError;

use super::{
    SolveRightTriangleRequest,
    SolveRightTriangleResponse,
};

#[derive(Debug, Serialize)]
pub struct TauriError {
    message: String,
}

// ---------------------------------------------------------
// Command
// ---------------------------------------------------------

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

fn map_application_error(err: ApplicationError) -> TauriError {
    TauriError {
        message: err.to_string(),
    }
}
