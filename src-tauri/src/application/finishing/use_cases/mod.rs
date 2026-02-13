//! Finishing use cases for planning and execution updates.
//!
//! This module contains application workflows that coordinate the finishing
//! domain planner and execution aggregate through repository boundaries.

// application/finishing/use_cases/mod.rs

pub mod generate_finishing_plan_use_case;
pub mod register_finishing_measurement_use_case;

pub use generate_finishing_plan_use_case::*;
pub use register_finishing_measurement_use_case::*;
