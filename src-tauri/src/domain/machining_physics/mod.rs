// domain/machining_physics/mod.rs

//pub mod physics_error;

mod error;
mod cutting_parameters;
mod cutting_solver;
mod formulas;
mod tool;

pub use error::MachiningPhysicsError;
pub use cutting_parameters::CuttingParameters;
pub use cutting_solver::MachiningSolver;
pub use tool::{Tool, ToothCount};
