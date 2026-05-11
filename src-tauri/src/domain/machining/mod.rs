// domain/machining/mod.rs

pub mod cutting_data;
pub mod cylinder_weight;
pub mod finishing;

pub use cutting_data::{CuttingParameters, Tool, CuttingSolver, CuttingError};
pub use cylinder_weight::{CylinderSpec, CylinderWeightError, CylinderWeightSolver, Material};
