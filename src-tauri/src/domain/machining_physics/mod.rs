// domain/machining_physics/mod.rs

pub mod physics_error;

mod calculators;
mod cutting_input;
mod cutting_result;
mod tool;

pub use calculators::*;
pub use cutting_input::CuttingInput;
pub use cutting_result::CuttingResult;
pub use tool::{Tool, ToothCount};
