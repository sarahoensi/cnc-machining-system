// interface/tauri/utting_data/command.rs

use tauri::command;

use crate::application::SolveCuttingDataUseCase;

use super::{
    SolveCuttingDataRequest,
    SolveCuttingDataResponse,
};

#[command]
pub fn solve_cutting_data(
    request: SolveCuttingDataRequest,
) -> Result<SolveCuttingDataResponse, String> {

    let use_case = SolveCuttingDataUseCase;

    let input = request.into();

    let result = use_case
        .execute(input)
        .map_err(|e| e.to_string())?;

    Ok(result.into())
}
