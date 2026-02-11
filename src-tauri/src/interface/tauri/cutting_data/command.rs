// interface/tauri/utting_data/command.rs

use tauri::command;

use crate::application::{SolveCuttingDataInput, SolveCuttingDataUseCase};

use super::{
    SolveCuttingDataRequest,
    SolveCuttingDataResponse,
};

// interface/tauri/utting_data/command.rs

#[command]
pub fn solve_cutting_data(
    request: SolveCuttingDataRequest,
) -> Result<SolveCuttingDataResponse, String> {

    let input = SolveCuttingDataInput::try_from(request)
        .map_err(|e| e.to_string())?;

    let output = SolveCuttingDataUseCase::execute(input)
        .map_err(|e| e.to_string())?;

    Ok(output.into())
}
