//! Finishing workflow application module.
//!
//! This module exposes finishing use cases that coordinate domain planning and
//! execution aggregates for diameter reduction operations. It fits the machining
//! system by managing the plan generation lifecycle and measured-step updates.

// application/finishing/mod.rs

mod dto;
pub mod use_cases;
mod mapping;

pub use dto::*;
//pub use use_cases::*;
pub use mapping::*;
