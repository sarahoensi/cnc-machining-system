// interface/tauri/helix/command.rs

use tauri::command;

use crate::application::SolveHelixUseCase;
//use crate::application::ApplicationError;

use super::{
    SolveHelixRequest,
    SolveHelixResponse,
};

#[command]
pub fn solve_helix(
    request: SolveHelixRequest,
) -> Result<SolveHelixResponse, String> {

    let use_case = SolveHelixUseCase;

    let input = request.into();

    let result = use_case
        .execute(input)
        .map_err(|e| e.to_string())?;

    Ok(result.into())
}
