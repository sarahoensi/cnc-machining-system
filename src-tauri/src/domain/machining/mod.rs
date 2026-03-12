// domain/machining/mod.rs

pub mod cutting_data;
pub mod finishing;

pub use cutting_data::{CuttingParameters, Tool, CuttingSolver, CuttingError};
