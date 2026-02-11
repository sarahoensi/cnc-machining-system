// application/features/cutting_data/calculate_cutting_data.rs

use crate::domain::features::cutting_data::*;
use crate::domain::features::cutting_data::input::raw::RawCuttingInput;


pub struct CalculateCuttingDataUseCase;

impl CalculateCuttingDataUseCase {

    pub fn partial(
        raw: RawCuttingInput
    ) -> Result<CuttingDataPartialSolution, DomainError> {

        let data = CuttingData::try_from(raw)?;
        CuttingDataSolver::solve_partial(&data)
    }

    pub fn full(
        raw: RawCuttingInput
    ) -> Result<CuttingDataFullSolution, DomainError> {

        let data = CuttingData::try_from(raw)?;
        CuttingDataSolver::solve_full(&data)
    }
}
