// tests/cutting_data/acceptance/helpers.rs

use cnc_machining_system_lib::domain::features::cutting_data::{
    CuttingData, CuttingDataFullSolution, CuttingDataPartialSolution, CuttingDataSolver, raw::RawCuttingInput
};


pub fn solve_partial(raw: RawCuttingInput) -> CuttingDataPartialSolution {
    let domain = CuttingData::try_from(raw)
        .expect("raw input should be valid");

    CuttingDataSolver::solve_partial(&domain)
        .expect("partial solver should succeed")
}

pub fn solve_full(raw: RawCuttingInput) -> CuttingDataFullSolution {
    let domain = CuttingData::try_from(raw)
        .expect("raw input should be valid");

    CuttingDataSolver::solve_full(&domain)
        .expect("full solver should succeed")
}
