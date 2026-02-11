// interface/features/cutting_data/tauri_commands.rs

use crate::application::features::cutting_data::*;
use crate::domain::features::cutting_data::*;
use crate::domain::features::cutting_data::input::raw::RawCuttingInput;


/// Calculate partial cutting data
#[tauri::command]
pub fn calculate_partial_cutting_data(
    input: RawCuttingInput,
) -> Result<CuttingDataPartialSolution, String> {

    CalculateCuttingDataUseCase::partial(input)
        .map_err(|e| e.to_string())
}

/// Calculate full cutting data
#[tauri::command]
pub fn calculate_full_cutting_data(
    input: RawCuttingInput,
) -> Result<CuttingDataFullSolution, String> {

    CalculateCuttingDataUseCase::full(input)
        .map_err(|e| e.to_string())
}
