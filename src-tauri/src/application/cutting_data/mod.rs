//! Cutting-data orchestration for milling parameter workflows.
//!
//! This module exposes use cases and DTOs that coordinate domain calculators
//! for spindle speed, cutting speed, chip load, and feed rate. It fits the
//! machining system by completing partial operator inputs into a consistent
//! cutting-data set for downstream execution planning.

// application/cutting_data/mod.rs

pub mod dto;
pub mod solve_cutting_data_use_case;



