// cutting_data/mod.rs

mod input;
mod solver;
mod solution;
mod errors;

// Public API
pub use input::raw::RawCuttingInput;
pub use input::valid::ValidCuttingInput;
pub use solution::CuttingDataSolution;
pub use solver::solve;
pub use errors::DomainError;
