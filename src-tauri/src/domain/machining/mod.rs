// domain/machining/mod.rs

pub mod cutting_data;
pub mod cylinder_weight;
pub mod finishing;

pub use cutting_data::{CuttingError, CuttingParameters, CuttingSolver, Tool};
pub use cylinder_weight::{CylinderSpec, CylinderWeightError, CylinderWeightSolver, Material};
