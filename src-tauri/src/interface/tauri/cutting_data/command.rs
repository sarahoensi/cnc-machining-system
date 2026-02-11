// interface/tauri/cutting_data/command.rs

use tauri::command;

use crate::application::{
    SolveCuttingDataInput,
    SolveCuttingDataUseCase,
};

use super::{
    SolveCuttingDataRequest,
    SolveCuttingDataResponse,
};

#[command]
pub fn solve_cutting_data(
    request: SolveCuttingDataRequest,
) -> Result<SolveCuttingDataResponse, String> {

    let input: SolveCuttingDataInput = request.into();

    let output = SolveCuttingDataUseCase::execute(input)
        .map_err(|e| e.to_string())?;

    Ok(output.into())
}
